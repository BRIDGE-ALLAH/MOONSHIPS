//! Per-route RBAC guards.
//!
//! Two flavours:
//!
//! * **Global-role middlewares** ([`require_global_role`]) attach to a router
//!   via `.layer(axum::middleware::from_fn(require_global_admin))`. They only
//!   need [`AuthUser`] from request extensions, so they slot in cleanly after
//!   [`crate::auth::middleware::require_auth`].
//!
//! * **Project-role helper** ([`enforce_project_role`]) is called from inside
//!   handlers that already have the `project_id` in hand (via `Path<…>`).
//!   Middleware can't see typed path params, so this is left to the handler
//!   to make the call explicit and grep-able.
//!
//! [`ProjectMembership`] is the resolved-role view returned by the helper,
//! carrying the user's effective role for the project (incl. the
//! "global Admin bypass" case).

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use rusqlite::Connection;

use super::membership;
use super::roles::{GlobalRole, ProjectRole};
use crate::auth::middleware::AuthUser;
use crate::error::AppError;

/// Result of resolving a user's role within a specific project.
///
/// `via_global_admin == true` means the user reached this project through
/// their system-wide role (Owner/Admin), not via an explicit
/// `project_members` row. Handlers can branch on this for things like
/// "show the project owner switcher only when *they* are project Admin",
/// vs "show it to any global Admin too".
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProjectMembership {
    pub project_id: String,
    pub user_id: String,
    pub role: ProjectRole,
    pub via_global_admin: bool,

    #[test]
    fn project_role_guard_denies_cross_tenant_members_even_with_project_membership() {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            "CREATE TABLE project_tenants (project_id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL);
             CREATE TABLE project_members (
                 user_id TEXT NOT NULL,
                 project_id TEXT NOT NULL,
                 project_role TEXT NOT NULL
             );
             INSERT INTO project_tenants (project_id, tenant_id)
             VALUES ('project-a', 'tenant-a'), ('project-b', 'tenant-b');
             INSERT INTO project_members (user_id, project_id, project_role)
             VALUES ('user-a', 'project-a', 'viewer'), ('user-a', 'project-b', 'admin');",
        )
        .unwrap();

        let user = AuthUser {
            id: "user-a".into(),
            username: "alice".into(),
            role: "user".into(),
            global_role: GlobalRole::User,
            tenant_id: Some("tenant-a".into()),
            session_id: "session-a".into(),
            is_peer: false,
        };

        assert!(enforce_project_role(&user, "project-a", ProjectRole::Viewer, &db).is_ok());
        assert!(matches!(
            enforce_project_role(&user, "project-b", ProjectRole::Viewer, &db),
            Err(AppError::Forbidden(_))
        ));

        let unscoped = AuthUser {
            tenant_id: None,
            ..user
        };
        assert!(matches!(
            enforce_project_role(&unscoped, "project-a", ProjectRole::Viewer, &db),
            Err(AppError::Forbidden(_))
        ));
    }
}

/// Generic factory: returns an async middleware that requires `min` global role.
///
/// Wraps the closure so call-sites read like:
/// ```ignore
/// .layer(axum::middleware::from_fn(|req, next| {
///     require_global_role(GlobalRole::Admin, req, next)
/// }))
/// ```
/// In practice we use the three pre-built helpers below
/// ([`require_global_admin`] / [`require_global_owner`] / [`require_global_user`])
/// to keep the router declarations tidy.
pub async fn require_global_role(
    min: GlobalRole,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let user = req
        .extensions()
        .get::<AuthUser>()
        .ok_or(AppError::Unauthorized)?;

    // Peers carry global_role=Admin synthetically — admit them on Admin gates
    // and the lower User gate, but refuse Owner-only routes (federation +
    // user mutations are local-trust only).
    if user.is_peer && min == GlobalRole::Owner {
        return Err(AppError::Forbidden(crate::i18n::te(
            "errors.rbac.owner_only_peer_token",
        )));
    }

    if !user.global_role.at_least(min) {
        return Err(AppError::Forbidden(crate::i18n::te_args(
            "errors.rbac.requires_global_role",
            &[("v", min.as_str())],
        )));
    }
    Ok(next.run(req).await)
}

/// Gate a router to global Admin+ (covers Owner + Admin).
pub async fn require_global_admin(req: Request, next: Next) -> Result<Response, AppError> {
    require_global_role(GlobalRole::Admin, req, next).await
}

/// Gate a router to Owner only.
pub async fn require_global_owner(req: Request, next: Next) -> Result<Response, AppError> {
    require_global_role(GlobalRole::Owner, req, next).await
}

/// Gate a router to "any authenticated user". Distinguishes "auth checked but
/// role not enforced" from a missing guard, useful for routes that need to
/// reject peer tokens but accept all human roles.
#[allow(dead_code)]
pub async fn require_global_user(req: Request, next: Next) -> Result<Response, AppError> {
    require_global_role(GlobalRole::User, req, next).await
}

/// Resolve a `resource_id` (a `services.id`) to its owning `project_id`,
/// then enforce `min_role` on that project. Combines the two lookups so
/// handlers can do a one-liner gate at the top of the function.
///
/// Returns `Forbidden` (for non-admins) if the resource exists but belongs
/// to no project — there's no project to scope membership against. Global
/// Owner / Admin and peer requests bypass the project lookup entirely so
/// legacy orphan rows (e.g. compose stacks created before projects existed,
/// rows whose project was deleted with `ON DELETE SET NULL`) remain reachable
/// from the admin UI.
pub fn enforce_resource_role(
    state: &crate::state::SharedState,
    user: &AuthUser,
    resource_id: &str,
    min: ProjectRole,
) -> Result<ProjectMembership, AppError> {
    // Fast path matching `enforce_project_role`: global admins/peers don't
    // need the resource→project lookup to succeed. Doing this *before* the
    // DB query also avoids 403'ing the Owner on services with NULL
    // project_id, which was the root cause of the post-RBAC "Loading…"
    // hang on legacy compose stacks.
    if user.is_peer || user.global_role.at_least(GlobalRole::Admin) {
        return Ok(ProjectMembership {
            project_id: String::new(),
            user_id: user.id.clone(),
            role: ProjectRole::Admin,
            via_global_admin: true,
        });
    }

    let db = state
        .db
        .lock()
        .map_err(|e| AppError::Internal(anyhow::anyhow!("DB lock: {e}")))?;
    let project_id = super::membership::project_for_resource(&db, resource_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("project lookup: {e}")))?
        .ok_or_else(|| AppError::Forbidden(crate::i18n::te("errors.rbac.resource_not_bound")))?;
    enforce_project_role(user, &project_id, min, &db)
}

fn project_belongs_to_tenant(
    conn: &Connection,
    project_id: &str,
    tenant_id: &str,
) -> rusqlite::Result<bool> {
    use rusqlite::OptionalExtension;

    let mapped: Option<String> = conn
        .query_row(
            "SELECT tenant_id FROM project_tenants WHERE project_id = ?1",
            [project_id],
            |row| row.get(0),
        )
        .optional()?;
    Ok(mapped.as_deref() == Some(tenant_id))
}

/// Look up the caller's effective role within `project_id` and return it,
/// or fail with 403 if the role is below `min`. The DB connection is
/// short-lived — we open it inside the helper so handlers don't need to
/// thread the mutex guard through every call site.
///
/// Global Owner / Admin always pass with `ProjectRole::Admin` and
/// `via_global_admin = true`.
pub fn enforce_project_role(
    user: &AuthUser,
    project_id: &str,
    min: ProjectRole,
    conn: &Connection,
) -> Result<ProjectMembership, AppError> {
    if user.is_peer || user.global_role.at_least(GlobalRole::Admin) {
        return Ok(ProjectMembership {
            project_id: project_id.to_string(),
            user_id: user.id.clone(),
            role: ProjectRole::Admin,
            via_global_admin: true,
        });
    }

    let tenant_id = user.tenant_id.as_deref().ok_or_else(|| {
        AppError::Forbidden(crate::i18n::te_args(
            "errors.rbac.not_project_member",
            &[("v", project_id)],
        ))
    })?;
    let in_tenant = project_belongs_to_tenant(conn, project_id, tenant_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("project tenant lookup: {e}")))?;
    if !in_tenant {
        return Err(AppError::Forbidden(crate::i18n::te_args(
            "errors.rbac.not_project_member",
            &[("v", project_id)],
        )));
    }

    let role = membership::role_for(conn, &user.id, project_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("membership lookup: {e}")))?
        .ok_or_else(|| {
            AppError::Forbidden(crate::i18n::te_args(
                "errors.rbac.not_project_member",
                &[("v", project_id)],
            ))
        })?;

    if !role.at_least(min) {
        return Err(AppError::Forbidden(crate::i18n::te_args(
            "errors.rbac.requires_project_role",
            &[("v", min.as_str())],
        )));
    }
    Ok(ProjectMembership {
        project_id: project_id.to_string(),
        user_id: user.id.clone(),
        role,
        via_global_admin: false,
    })
}

#[cfg(test)]
mod tenant_tests {
    use super::*;

    #[test]
    fn project_tenant_check_denies_cross_tenant_and_unmapped_projects() {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            "CREATE TABLE project_tenants (project_id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL);
             INSERT INTO project_tenants (project_id, tenant_id)
             VALUES ('project-a', 'tenant-a'), ('project-b', 'tenant-b');",
        )
        .unwrap();

        assert!(project_belongs_to_tenant(&db, "project-a", "tenant-a").unwrap());
        assert!(!project_belongs_to_tenant(&db, "project-a", "tenant-b").unwrap());
        assert!(!project_belongs_to_tenant(&db, "unmapped-project", "tenant-a").unwrap());
    }
}
