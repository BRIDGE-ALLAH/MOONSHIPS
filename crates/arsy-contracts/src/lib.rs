//! Transport-neutral ARSY contracts.
//!
//! This crate defines data contracts only. It does not authenticate principals,
//! evaluate policy, verify signatures, grant permissions, or access persistence.
//! Callers must fail closed when scope cannot be resolved.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrincipalKind {
    Human,
    Service,
    Agent,
    Peer,
    PlatformAdmin,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Principal {
    pub subject_id: String,
    pub kind: PrincipalKind,
    pub issuer: String,
    pub audience: String,
    pub authentication_method: String,
    pub session_id: Option<String>,
    pub issued_at: String,
    pub expires_at: String,
}

impl Principal {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("subject_id", &self.subject_id)?;
        required("issuer", &self.issuer)?;
        required("audience", &self.audience)?;
        required("authentication_method", &self.authentication_method)?;
        required("issued_at", &self.issued_at)?;
        required("expires_at", &self.expires_at)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    /// Mandatory for every tenant-scoped business operation.
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub workspace_id: Option<String>,
    pub resource_owner_id: Option<String>,
}

impl Scope {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("tenant_id", &self.tenant_id)?;
        optional_required("organization_id", self.organization_id.as_deref())?;
        optional_required("workspace_id", self.workspace_id.as_deref())?;
        optional_required("resource_owner_id", self.resource_owner_id.as_deref())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantRole {
    Owner,
    Admin,
    Member,
}

impl TenantRole {
    /// Map the existing MW global role into the additive legacy tenant role.
    /// Unknown role values fail closed instead of silently becoming members.
    pub fn from_legacy_global_role(value: &str) -> Result<Self, ContractError> {
        match value {
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            "user" => Ok(Self::Member),
            _ => Err(ContractError::InvalidField("global_role")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantStatus {
    Active,
    Suspended,
    Archived,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tenant {
    pub tenant_id: String,
    pub slug: String,
    pub name: String,
    pub status: TenantStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl Tenant {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("tenant_id", &self.tenant_id)?;
        required("slug", &self.slug)?;
        required("name", &self.name)?;
        required("created_at", &self.created_at)?;
        required("updated_at", &self.updated_at)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TenantMembership {
    pub tenant_id: String,
    pub user_id: String,
    pub role: TenantRole,
    pub created_at: String,
}

impl TenantMembership {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("tenant_id", &self.tenant_id)?;
        required("user_id", &self.user_id)?;
        required("created_at", &self.created_at)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantMappingSource {
    LegacyBackfill,
    OperatorAssignment,
    Migration,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectTenantMapping {
    pub project_id: String,
    pub tenant_id: String,
    pub source: TenantMappingSource,
    pub created_at: String,
}

impl ProjectTenantMapping {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("project_id", &self.project_id)?;
        required("tenant_id", &self.tenant_id)?;
        required("created_at", &self.created_at)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceRef {
    pub resource_type: String,
    pub resource_id: String,
}

impl ResourceRef {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("resource_type", &self.resource_type)?;
        required("resource_id", &self.resource_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationRequest {
    pub principal: Principal,
    pub scope: Scope,
    pub action: String,
    pub resource: ResourceRef,
    pub purpose: String,
    pub argument_hash: String,
    pub correlation_id: String,
    pub requested_at: String,
}

impl AuthorizationRequest {
    pub fn validate(&self) -> Result<(), ContractError> {
        self.principal.validate()?;
        self.scope.validate()?;
        self.resource.validate()?;
        required("action", &self.action)?;
        required("purpose", &self.purpose)?;
        required("argument_hash", &self.argument_hash)?;
        required("correlation_id", &self.correlation_id)?;
        required("requested_at", &self.requested_at)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationEffect {
    Allow,
    Deny,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationDecision {
    pub decision_id: String,
    pub effect: AuthorizationEffect,
    pub reason_code: String,
    pub policy_version: String,
    pub approval_required: bool,
    pub evaluated_at: String,
}

impl AuthorizationDecision {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("decision_id", &self.decision_id)?;
        required("reason_code", &self.reason_code)?;
        required("policy_version", &self.policy_version)?;
        required("evaluated_at", &self.evaluated_at)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityGrant {
    pub grant_id: String,
    pub parent_grant_id: Option<String>,
    pub issuer_id: String,
    pub subject_id: String,
    pub scope: Scope,
    pub purpose: String,
    pub session_id: String,
    pub actions: Vec<String>,
    pub resources: Vec<String>,
    pub argument_hashes: Vec<String>,
    pub issued_at: String,
    pub expires_at: String,
    pub key_id: String,
    pub signature: String,
}

impl CapabilityGrant {
    /// Structural validation only. Signature verification and revocation checks
    /// must be performed by a trusted authorization service.
    pub fn validate(&self) -> Result<(), ContractError> {
        required("grant_id", &self.grant_id)?;
        required("issuer_id", &self.issuer_id)?;
        required("subject_id", &self.subject_id)?;
        self.scope.validate()?;
        required("purpose", &self.purpose)?;
        required("session_id", &self.session_id)?;
        non_empty_list("actions", &self.actions)?;
        non_empty_list("resources", &self.resources)?;
        non_empty_list("argument_hashes", &self.argument_hashes)?;
        required("issued_at", &self.issued_at)?;
        required("expires_at", &self.expires_at)?;
        required("key_id", &self.key_id)?;
        required("signature", &self.signature)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecision {
    Approved,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Approval {
    pub approval_id: String,
    pub operation_id: String,
    pub scope: Scope,
    pub requester_id: String,
    pub approver_id: String,
    pub action: String,
    pub resource: ResourceRef,
    pub policy_version: String,
    pub decision: ApprovalDecision,
    pub reason_code: String,
    pub issued_at: String,
    pub expires_at: String,
    pub key_id: String,
    pub signature: String,
}

impl Approval {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("approval_id", &self.approval_id)?;
        required("operation_id", &self.operation_id)?;
        self.scope.validate()?;
        required("requester_id", &self.requester_id)?;
        required("approver_id", &self.approver_id)?;
        if self.requester_id == self.approver_id {
            return Err(ContractError::SelfApproval);
        }
        required("action", &self.action)?;
        self.resource.validate()?;
        required("policy_version", &self.policy_version)?;
        required("reason_code", &self.reason_code)?;
        required("issued_at", &self.issued_at)?;
        required("expires_at", &self.expires_at)?;
        required("key_id", &self.key_id)?;
        required("signature", &self.signature)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationEnvelope {
    pub operation_id: String,
    pub domain: String,
    pub scope: Scope,
    pub actor_id: String,
    pub idempotency_key: String,
    pub request_hash: String,
    pub status: OperationStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl OperationEnvelope {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("operation_id", &self.operation_id)?;
        required("domain", &self.domain)?;
        self.scope.validate()?;
        required("actor_id", &self.actor_id)?;
        if self.idempotency_key.trim().len() < 8 {
            return Err(ContractError::InvalidField("idempotency_key"));
        }
        required("request_hash", &self.request_hash)?;
        required("created_at", &self.created_at)?;
        required("updated_at", &self.updated_at)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditEvent {
    pub event_id: String,
    pub sequence: u64,
    pub scope: Scope,
    pub actor_id: String,
    pub action: String,
    pub resource: ResourceRef,
    pub effect: AuthorizationEffect,
    pub reason_code: String,
    pub correlation_id: String,
    pub source_evidence_ref: Option<String>,
    pub occurred_at: String,
}

impl AuditEvent {
    pub fn validate(&self) -> Result<(), ContractError> {
        required("event_id", &self.event_id)?;
        self.scope.validate()?;
        required("actor_id", &self.actor_id)?;
        required("action", &self.action)?;
        self.resource.validate()?;
        required("reason_code", &self.reason_code)?;
        required("correlation_id", &self.correlation_id)?;
        optional_required("source_evidence_ref", self.source_evidence_ref.as_deref())?;
        required("occurred_at", &self.occurred_at)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractError {
    EmptyField(&'static str),
    InvalidField(&'static str),
    SelfApproval,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(f, "{field} must not be empty"),
            Self::InvalidField(field) => write!(f, "{field} is invalid"),
            Self::SelfApproval => write!(f, "requester and approver must be different principals"),
        }
    }
}

impl std::error::Error for ContractError {}

fn required(field: &'static str, value: &str) -> Result<(), ContractError> {
    if value.trim().is_empty() {
        Err(ContractError::EmptyField(field))
    } else {
        Ok(())
    }
}

fn optional_required(field: &'static str, value: Option<&str>) -> Result<(), ContractError> {
    if value.is_some_and(|v| v.trim().is_empty()) {
        Err(ContractError::InvalidField(field))
    } else {
        Ok(())
    }
}

fn non_empty_list(field: &'static str, values: &[String]) -> Result<(), ContractError> {
    if values.is_empty() || values.iter().any(|v| v.trim().is_empty()) {
        Err(ContractError::InvalidField(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn principal() -> Principal {
        Principal {
            subject_id: "user-1".into(),
            kind: PrincipalKind::Human,
            issuer: "mw".into(),
            audience: "mw-api".into(),
            authentication_method: "session_cookie".into(),
            session_id: Some("session-1".into()),
            issued_at: "2026-11-10T00:00:00Z".into(),
            expires_at: "2026-11-10T01:00:00Z".into(),
        }
    }

    fn scope() -> Scope {
        Scope {
            tenant_id: "tenant-a".into(),
            organization_id: Some("org-a".into()),
            workspace_id: Some("workspace-a".into()),
            resource_owner_id: Some("owner-a".into()),
        }
    }

    fn resource() -> ResourceRef {
        ResourceRef {
            resource_type: "deployment".into(),
            resource_id: "deployment-1".into(),
        }
    }

    #[test]
    fn authorization_request_requires_tenant_and_complete_scope() {
        let request = AuthorizationRequest {
            principal: principal(),
            scope: Scope {
                tenant_id: " ".into(),
                ..scope()
            },
            action: "deployment:read".into(),
            resource: resource(),
            purpose: "operator-request".into(),
            argument_hash: "hash".into(),
            correlation_id: "correlation-1".into(),
            requested_at: "2026-11-10T00:00:00Z".into(),
        };
        assert_eq!(
            request.validate(),
            Err(ContractError::EmptyField("tenant_id"))
        );
    }

    #[test]
    fn authorization_request_accepts_explicit_tenant_context() {
        let request = AuthorizationRequest {
            principal: principal(),
            scope: scope(),
            action: "deployment:read".into(),
            resource: resource(),
            purpose: "operator-request".into(),
            argument_hash: "hash".into(),
            correlation_id: "correlation-1".into(),
            requested_at: "2026-11-10T00:00:00Z".into(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn approval_rejects_self_approval() {
        let approval = Approval {
            approval_id: "approval-1".into(),
            operation_id: "operation-1".into(),
            scope: scope(),
            requester_id: "same-user".into(),
            approver_id: "same-user".into(),
            action: "deployment:delete".into(),
            resource: resource(),
            policy_version: "v1".into(),
            decision: ApprovalDecision::Approved,
            reason_code: "approved".into(),
            issued_at: "2026-11-10T00:00:00Z".into(),
            expires_at: "2026-11-10T01:00:00Z".into(),
            key_id: "key-1".into(),
            signature: "signature".into(),
        };
        assert_eq!(approval.validate(), Err(ContractError::SelfApproval));
    }

    #[test]
    fn operation_requires_idempotency_key() {
        let operation = OperationEnvelope {
            operation_id: "op-1".into(),
            domain: "moonships".into(),
            scope: scope(),
            actor_id: "user-1".into(),
            idempotency_key: "short".into(),
            request_hash: "hash".into(),
            status: OperationStatus::Queued,
            created_at: "2026-11-10T00:00:00Z".into(),
            updated_at: "2026-11-10T00:00:00Z".into(),
        };
        assert_eq!(
            operation.validate(),
            Err(ContractError::InvalidField("idempotency_key"))
        );
    }

    #[test]
    fn legacy_global_roles_map_explicitly_and_unknown_roles_fail_closed() {
        assert_eq!(
            TenantRole::from_legacy_global_role("owner"),
            Ok(TenantRole::Owner)
        );
        assert_eq!(
            TenantRole::from_legacy_global_role("admin"),
            Ok(TenantRole::Admin)
        );
        assert_eq!(
            TenantRole::from_legacy_global_role("user"),
            Ok(TenantRole::Member)
        );
        assert_eq!(
            TenantRole::from_legacy_global_role("superuser"),
            Err(ContractError::InvalidField("global_role"))
        );
    }

    #[test]
    fn project_tenant_mapping_requires_both_sides() {
        let mapping = ProjectTenantMapping {
            project_id: "project-1".into(),
            tenant_id: "".into(),
            source: TenantMappingSource::LegacyBackfill,
            created_at: "2026-11-10T00:00:00Z".into(),
        };
        assert_eq!(
            mapping.validate(),
            Err(ContractError::EmptyField("tenant_id"))
        );
    }

    #[test]
    fn principal_kind_serializes_stably() {
        assert_eq!(
            serde_json::to_string(&PrincipalKind::PlatformAdmin).unwrap(),
            "\"platform_admin\""
        );
    }

    #[test]
    fn strict_contract_rejects_unknown_fields() {
        let json = r#"{"tenant_id":"tenant-a","unexpected":"value"}"#;
        assert!(serde_json::from_str::<Scope>(json).is_err());
    }
}
