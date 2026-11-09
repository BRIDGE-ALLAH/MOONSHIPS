# ARSY Capability Parity Matrix — 2026-11-10

**Status:** Source-backed assessment and target contract proposal. No claim that the complete ARSY target is already implemented in MW.  
**Sources pinned:** MW main `7db7fa32e879154a437ed1be4b8e85e361305ec1`; MOONWITNESS `8f9a5de38c39ea82388178effefead06af4e2c8b`; A-L-L-A-H.COM `bf8681c714b9983861664a05f0c6d9df752f7549`.

## 1. Critical distinction

There are three different systems in the source set and they must not be conflated:

1. **MW/Pier runtime (Rust/Axum/SQLite):** operational control plane for servers, projects, resources, deployments, registries, users, tokens, federation and audit. It has real authentication and RBAC.
2. **MOONWITNESS `apps/arsy` (Fastify 5/PostgreSQL):** newer ARSY command API with a single configured operator principal, domain route modules, deployment authorization, signed capability grants, approvals, operation journal and outbox. This is not a general multi-user identity provider today.
3. **A-L-L-A-H.COM legacy Fastify ARSY / LLM gateway:** separate migration lineage. Its LLM gateway source has unscoped query paths and missing tenant propagation at call sites already documented in the security audit. It remains quarantined from wholesale import.

The intended architecture is **one ARSY authority**. These are migration inputs, not three permanent authorities.

## 2. Capability parity

| Capability | MW/Pier current evidence | MOONWITNESS ARSY current evidence | Gap / target disposition |
|---|---|---|---|
| Human authentication | Local account/password, session cookie, TOTP, API tokens | `X-ARSY-Operator-Token` hook creates one configured operator principal; missing config fails closed with 503 | Preserve MW human auth as baseline. Design federation/IdP separately; static operator token is service/operator bootstrap, not end-user SSO |
| Machine identity | API bearer tokens, peer-core token, agent/bootstrap tokens | Server-side operator token; deployment capability grants | Unify with typed service principal, audience, purpose, expiry, revocation and rotation |
| Global authorization | Owner/Admin/User roles | One operator principal; capability/policy authorization is operation-specific | Retain local role behavior during migration; define ARSY subject/tenant/resource/action policy contract |
| Project/workspace scope | Project membership Admin/Editor/Viewer; membership helper; global Admin bypass | Deployment owner scope; no general organization/workspace membership model shown | Projects are not equivalent to tenants. Introduce tenant/org/workspace context additively, with migration and explicit owner mapping |
| Tenant propagation | `AuthUser` contains id, username, role, global_role, session_id, is_peer; no tenant_id/org_id | Deployment context sets `tenantId = ownerId`; SQL queries scope by owner_id and tenant_id | Strong gap: a general tenant context is not present in MW principal; ARSY app currently uses owner ID as tenant boundary |
| Permission vocabulary | ManageUsers, ChangeGlobalRoles, ManageServers, ViewSystem, CreateProjects, ViewProject, EditProject, AdminProject, ManageFederation; source comments say project permissions are wired incrementally | Capability grants scope subject, tenant, purpose, session, actions, resources, argument hashes, time validity; policy provider can require approval | Combine role membership with resource/action ABAC; no role-only or model-only authorization |
| Delegation / approval | Peer federation grants; no equivalent signed per-operation delegated grant/approval proven in reviewed paths | Ed25519-signed capability chains and signed two-person approvals for deployment operations | Port as bounded ARSY capability service after independent crypto/key lifecycle review |
| Operations / idempotency | Deploy/task/schedule APIs exist; complete cross-domain durable operation contract not proven | Deployment operation state machine, idempotency key + request hash, durable operation events, transactional outbox and worker leases | Reuse design selectively; define domain-neutral operation envelope and adapters |
| Audit / evidence | Auth audit events and admin audit API; infrastructure events | Append-only evidence events, operation journal and persisted authorization decisions | One correlation/event envelope; preserve domain evidence and source provenance |
| Resource domains | Servers, projects, containers, databases, domains, networks, registries, backups, federation, etc. | Docker, deployments, evidence, governance, opportunities, products, assets, channels, listings, inspections, finance and oracle modules | ARSY owns cross-domain policy/registry contracts; each product domain owns business data and handlers |
| Persistence | rusqlite/SQLite current runtime | PostgreSQL with migrations and DB-backed route modules | No forced database consolidation. Add adapters/contracts, migrate only with restore/rollback and parity evidence |
| API framework | Axum 0.8 | Fastify 5 | Rust is target direction, but port contracts and behavior, not framework internals or unsafe source wholesale |
| Tests | Existing Rust tests/CI and route-specific tests; inspect actual workflow before parity claims | Unit and integration tests; PostgreSQL integration tests explicitly skip if TEST_DATABASE_URL is absent | Require clean CI with DB-backed integration, no-principal, cross-tenant, wrong-owner, expired/revoked grant, replay, stale lease and approval tests |

## 3. Evidence details

### MW/Pier
- `crates/pier-core/src/api/mod.rs` exports a broad infrastructure API surface and separates public from protected routes.
- `crates/pier-core/src/auth/middleware.rs` authenticates local sessions, bearer API tokens and peer-core grants. The resulting `AuthUser` does not carry a tenant or organization identifier.
- `crates/pier-core/src/auth/rbac/policy.rs` defines global and project-scoped permission checks. Its own source comment says project permission variants are wired incrementally and Stage 1 ships only guards used by user-management endpoints.
- `crates/pier-core/src/auth/rbac/guards.rs` resolves project roles inside handlers. Global Owner/Admin and peer principals bypass project membership. This is project RBAC, not a complete tenant hierarchy.

### MOONWITNESS ARSY
- `apps/arsy/package.json` identifies Fastify 5, Zod 4 and PostgreSQL dependencies.
- `apps/arsy/src/app.ts` registers 15 route-module families and calls `registerOperatorApiAuth`.
- `apps/arsy/src/auth/operator-token.ts` protects all `/api/v1/*` routes with one configured server-side operator token and one operator ID. Health/liveness are public. No token or operator ID fails closed.
- `apps/arsy/src/deployments/authorization.ts` checks operation owner, actor, tenant, purpose, session, action, resource, argument hash, signed capability chain, expiry/revocation, policy result and approval.
- `apps/arsy/src/deployments/operation-store.ts` persists owner-bound operations and idempotency conflict detection. Outbox leases and recovery live in the adjacent outbox store.
- Integration tests require PostgreSQL via `TEST_DATABASE_URL`; tests skip when it is absent. A unit test using FakePool is not a substitute for the real database CI job.

### A-L-L-A-H.COM legacy gateway
The earlier source audit found missing tenant propagation/unscoped access in selected LLM-gateway handlers. Keep this distinct from MOONWITNESS `apps/arsy`; do not infer the newer app has those exact defects, and do not infer the old gateway was fixed because proposed plans exist.

## 4. Target ARSY contracts — before runtime port

Define versioned language-neutral contracts first:

- `Principal`: subject_id, principal_kind (human/service/agent/peer), authentication_method, session_id, issuer, audience, issued_at, expires_at.
- `Scope`: tenant_id, organization_id, workspace_id, resource_owner_id; nullable only where the route contract explicitly allows it.
- `AuthorizationRequest`: principal, action, resource_type, resource_id, scope, purpose, argument_hash, requested_at, correlation_id.
- `AuthorizationDecision`: allow/deny, stable reason_code, policy_version, obligations, approval_required, decision_id, evaluated_at.
- `CapabilityGrant`: issuer, subject, tenant, purpose, session, actions, resources, argument_hashes, parent grant, validity window, key_id, signature, revocation reference.
- `Approval`: operation, requester, distinct approver, action/resource, policy version, decision, reason, validity and signature.
- `Operation`: operation_id, domain, tenant, actor, idempotency_key, request_hash, state, attempt, timestamps, event sequence, result/error reference.
- `AuditEvent`: actor/principal, tenant, action, resource, decision, correlation, source evidence, timestamp and immutable sequence.

Every request must fail closed when tenant/scope cannot be resolved. A platform administrator is an explicit principal class with audited, time-bounded break-glass elevation, not a magic tenant wildcard.

## 5. Migration and acceptance gates

### Gate A — contract and data mapping
- Document current MW users, global roles, project memberships, peer grants and service/agent credentials.
- Define deterministic project-to-tenant mapping; do not assume project_id equals tenant_id.
- Define owner-to-tenant mapping for MOONWITNESS ARSY's existing owner-scoped rows.
- Define API compatibility and versioning, audit event correlation, key rotation and rollback.

### Gate B — security tests
Required matrix: anonymous; user in tenant A; user in tenant B; project member without tenant membership; tenant admin; platform admin; peer; service principal; expired/revoked credential; delegated capability; expired/revoked capability; wrong action/resource/argument hash; replayed idempotency key; distinct approver; unavailable policy provider.

Expected result: no cross-tenant read/write, no owner bypass, no self-approval where two-person approval is required, no fallback to global data when scope is missing.

### Gate C — implementation
1. Add typed ARSY contract types and pure policy evaluation tests; do not alter existing login/session semantics.
2. Add tenant context as an additive layer with migration/backfill and explicit legacy-single-tenant behavior.
3. Add authorization decision and audit correlation at one vertical slice (read-only project/resource list).
4. Add signed capability and approval enforcement to one bounded destructive operation (deployment) only after key lifecycle review.
5. Add durable operation/outbox adapter and worker recovery tests.
6. Port additional domains only after parity and security acceptance.

### Gate D — release
Clean-checkout Rust build/tests, PostgreSQL contract integration, migration forward/backward rehearsal, backup/restore, API compatibility, tenant negative tests, secret/license scans and operator runbook.


## 6.1 Additive SQLite tenant foundation (migration 73)

The audit branch now adds migration 73 in `crates/pier-core/src/db/schema.rs`:

- `tenants`: tenant registry with active/suspended/archived lifecycle.
- `tenant_members`: explicit user membership and owner/admin/member role.
- `project_tenants`: one tenant mapping per existing project, with provenance (`legacy_backfill`, `operator_assignment`, `migration`).
- Existing users are backfilled into `legacy-default`; their tenant role is mapped from the existing typed global role (owner/admin/user → owner/admin/member).
- Existing projects are mapped to `legacy-default` with `legacy_backfill` provenance.
- All writes use additive table creation and `INSERT OR IGNORE`; no existing user/project row is rewritten or deleted.

Three focused SQLite tests cover role and project backfill, idempotent replay, and an empty installation. - The Rust contract crate now also defines typed tenant, tenant membership, tenant role, lifecycle status and project-to-tenant mapping records; legacy global-role conversion is explicit and rejects unknown values. Two additional tests cover role mapping and missing tenant assignment.

This migration is intentionally data-foundation only: current AuthUser, routes, SQL query scoping and RBAC behavior remain unchanged. Before multi-tenant enforcement, add explicit operator assignment and conflict checks, tenant-aware AuthUser propagation, route-level tenant predicates, and cross-tenant negative tests. Do not interpret the legacy default tenant as proof that historical projects were independently tenant-isolated.

## 7. First implementation increment (additive, no runtime behavior change)

The audit branch now contains `crates/arsy-contracts` as an isolated Rust workspace crate. It defines serde contracts for principal, tenant scope, resource, authorization request/decision, signed capability grant, two-person approval, operation envelope and audit event. Structural validation is explicit; self-approval and missing tenant scope are rejected. Unit tests cover mandatory tenant context, explicit scope, self-approval, idempotency-key length, stable principal serialization and strict unknown-field rejection.

This crate does **not** authenticate users, evaluate policy, verify cryptographic signatures, connect to SQLite/PostgreSQL, alter existing routes or change the current login/session behavior. Its first use is a typed boundary for subsequent integration. CI must pass before this increment is treated as accepted.

## 8. Current decision

**Do not wholesale port either Fastify application.** Keep MW/Pier as the live operational baseline; use MOONWITNESS ARSY as the stronger reference for signed deployment authorization and durable operations; use A-L-L-A-H.COM as a legacy source only, with its LLM gateway quarantined. First implementation increment is the versioned ARSY contract and tenant mapping, not a new login system or database replacement.
