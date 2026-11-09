# MW Architecture Conflict Register — 2026-11-09

Status: evidence-backed working register; no final ADR has been ratified.

This document translates the repository audit into explicit conflict boundaries. It prevents repository names, README declarations, and target diagrams from being mistaken for implemented or approved architecture.

## 1. ARSY / backend authority

### Observed sources

| Source | Current evidence | Conflict / limitation |
|---|---|---|
| `BRIDGE-ALLAH/MW` | Rust/Axum/MiniJinja infrastructure control plane (Pier-derived); existing app runtime | Has real infrastructure implementation and compatibility obligations; not automatically the whole ecosystem identity/governance domain |
| `BRIDGE-ALLAH/ARSY` | Rust/Axum/Tokio layers: arsy-core, arsy-infra, arsy-server; README explicitly says user storage is in-memory and license not selected | Small prototype; not a persistence-ready central platform |
| `BRIDGE-ALLAH/A-R-S-Y` | Go + PocketBase fork; early Docker Compose deployment MVP; README says production use is not yet appropriate | Different language, framework and product scope |
| `BRIDGE-ALLAH/MOONWITNESS/apps/arsy` | Fastify 5 + TypeScript + PostgreSQL package; workspace contracts and core package | Explicitly named canonical ARSY backend in MOONWITNESS README; conflicts with Rust-first target |
| `BRIDGE-ALLAH/EMPIRE-OF-LIGHT` | Rust workspace with >100 Temps-derived crates; README says Rust-first ARSY and Fastify is migration input | Workspace root Cargo metadata still names Temps, upstream repo and Temps contributors; must distinguish inherited infrastructure from implemented ARSY domain |
| `bjo163/moonwitnessxv` | Earlier docs describe Fastify/MoonQL Hub | Another source of Fastify-era requirements; full current-source parity audit still pending |

### Working target (provisional)

- MW is the single repository and primary application/runtime host.
- ARSY is the single authoritative control-plane domain for identity integration, authorization policy, tenant boundaries, system governance, orchestration, and cross-domain contracts.
- Reuse existing Rust/Axum MW infrastructure capabilities where they meet the requirement; do not replace them with a new parallel backend.
- Treat Fastify, Go/PocketBase and prototype Rust ARSY as migration/reference sources. Extract requirements and behavior; do not preserve duplicate global authorities.
- Do not decide whether ARSY is a crate, a set of crates, or an independently deployable process until the domain/contract/security inventory is complete.

### Required parity matrix

For each ARSY source, record:
1. Routes, methods, request/response schemas, versioning and error semantics.
2. Identity provider integration, session/token lifecycle, credential storage and key rotation.
3. RBAC/ABAC/policy evaluation, tenant resolution and cross-tenant isolation.
4. Persistence schema, migrations, transaction boundaries and data ownership.
5. Jobs, queues, scheduler, leases, retries, idempotency and restart recovery.
6. Integrations, webhooks, external APIs and secrets boundaries.
7. Audit events, evidence/provenance and observability.
8. Tests, CI, deployment, backup/restore and operational runbooks.

No implementation is declared canonical solely by README statements.

## 2. Gateway architecture

### Observed sources

- `MW-GATEWAY`: Agentgateway-derived Rust workspace, Apache-2.0; README scopes it as LLM/MCP/A2A connectivity, routing, authn/authz, guardrail and observability gateway. Cargo workspace includes agentgateway, LLM, policy, HTTP, xDS and related crates.
- `ROCKSOUL-WX`: Rust/Axum workspace with typed workload contracts, privacy/egress policy, execution planning, provider routing, acceptance checks and persisted metadata receipts.

### Provisional boundary

- MW-GATEWAY may own protocol connectivity/data-plane behavior (LLM, MCP, A2A, HTTP proxy).
- ROCKSOUL-WX may own outcome-oriented workload planning, policy evaluation and execution receipts.
- ARSY owns ecosystem identity, tenancy, governance, authorization authority and orchestration decisions.
- Neither gateway may become a competing identity authority, canonical ecosystem database, human approval authority, or global ARSY hub.
- If both are integrated, use versioned contracts and an explicit trust boundary; avoid duplicate policy evaluation with divergent semantics.

### Required comparison

Compare authn/authz, policy language, workload contract, privacy classification, egress enforcement, provider catalog, secrets, routing, receipts, streaming, tool invocation, audit events, persistence, deployment topology, failure handling and upstream update strategy.

## 3. Identity

- `MW-ID` root README and project docs identify the source as Kanidm. It is an upstream identity platform candidate, not proof of a bespoke MW identity service.
- Identity source-of-truth, authentication, federation, authorization policy and product profile must be separated.
- ARSY remains the ecosystem's policy-enforcement and identity-integration authority; a specialist identity provider may authenticate identities without owning every domain permission.
- Evaluate OIDC/OAuth2/WebAuthn/passkeys, groups, service identities, SCIM/LDAP needs, tenant mapping, account recovery, audit, HA, backup, licensing and operational burden before selecting.
- Do not copy or rename Kanidm internals before fork ancestry, local changes, license and deployment requirements are verified.

## 4. Data layer / ORM

- `MW-ORM` Cargo package remains `sea-orm`, repository points to SeaQL/SeaORM, version 2.0.4, license MIT OR Apache-2.0; it includes an added `moonwitness-module` workspace member.
- Current MW source baseline uses rusqlite/SQLite and embedded ordered migrations; optional SQLx/Redis dependencies do not establish SQLx as primary domain persistence.
- The SeaORM fork is a candidate for evaluation, not an adopted ORM.
- Required prototype: map one representative MW domain and its migrations, tenant filters, transactions, SQLite behavior, Postgres behavior, concurrency, connection lifecycle, error mapping, testability and upgrade path. Compare against current rusqlite and direct SQL before selecting.
- Preserve SQLite local/single-node compatibility until an explicit migration ADR and tested rollback path exist.

## 5. Object storage

- `MW-STORAGE` is RustFS-derived and its README states broad S3-compatible object storage direction.
- Treat it as a separate infrastructure service candidate, not an internal MW persistence replacement.
- Validate exact S3 API compatibility, bucket/object policy, IAM integration, encryption, versioning, retention/legal hold, lifecycle, replication, erasure coding, healing, observability, upgrade/rollback and restore drills.
- Evidence and corpus domains must retain content hash, original source, rights, acquisition metadata and transformation lineage independently of storage location.

## 6. Frontend and design system

Observed options:
- MW existing server-rendered Axum/MiniJinja product UI.
- `MW-UI`: Next.js/React/Tailwind/shadcn ecosystem, MIT declared in package.json.
- `MW-BOILERPLATE`: Rust/Dioxus static frontend starter; README says no backend/auth/database and compile/render verification pending.
- `MOONWITNESS` and `EMPIRE-OF-LIGHT`: Next.js/React app workspaces.

Provisional rule:
- Keep the current MW UI as the default control-plane shell while it remains fit for purpose.
- Shared design tokens, accessibility rules and visual components may be extracted from MW-UI after package/license and dependency review.
- Specialist apps may use React/Dioxus only where independent UX, offline/native runtime or deployment justifies it.
- Do not introduce a second shell or rewrite the main frontend before route inventory, auth contract, design token mapping, accessibility tests, build/deploy cost and migration plan are documented.

## 7. Infrastructure platform lineage

- Current MW is Pier-derived.
- EMPIRE-OF-LIGHT is Temps-derived. Its Cargo workspace has a large number of Temps crates and still declares repository/homepage `gotempsh/temps`, author `Temps Contributors`, version `0.1.0-beta.60`, license `MIT OR Apache-2.0`.
- These are two substantial infrastructure platform lineages. They may contain overlapping deployment, database, networking, backup, observability and agent functionality.
- Do not merge both workspaces wholesale. Compare domain capability, migration history, security, license notices, operational compatibility and test evidence. Select individual capabilities by evidence and explicit ADR.
- Preserve Pier runtime identifiers/data paths already used by MW. Preserve Temps identifiers and attribution in any approved derived source.

## 8. Product and domain boundaries

- MOONWITNESS repository has an explicit product catalog and domain ownership registry; its README says several experiences are targets/prototypes, not production-ready apps.
- ROCKSOUL-EX is a local-first VS Code extension with mission-scoped patching, explicit approval and evidence-oriented verification. It is a strong specialist developer-experience candidate, not a backend authority.
- ROCKSOUL-WX is a gateway/workload service candidate, not all of ROCKSOUL's creative/narrative runtime.
- ZILL WATCH is documented as a MOONWITNESS HEADQUARTER module, not a separate app or repository in the observed inventory.
- THE BATTERIES remains unresolved: no exact implementation or repo was verified in this pass.
- Preserve the difference between narrative identity, product, application, module, service, library, data corpus and external dependency.

## 9. ADR sequence

Draft in this order, each with evidence, alternatives, consequences, security, licensing and migration:
1. ADR-0001 — MW as canonical repository and modular-monolith-first boundary.
2. ADR-0002 — ARSY domain authority, contracts and runtime boundary.
3. ADR-0003 — Infrastructure platform lineage: Pier/MW vs Temps/EMPIRE capability selection.
4. ADR-0004 — Gateway composition: MW-GATEWAY and ROCKSOUL-WX.
5. ADR-0005 — Identity provider integration and ARSY authorization authority.
6. ADR-0006 — Persistence strategy: rusqlite baseline, SeaORM evaluation, SQLite/Postgres.
7. ADR-0007 — Object storage service boundary and RustFS integration.
8. ADR-0008 — Frontend shell and shared UI/design system.
9. ADR-0009 — MOONWITNESS / ROCKSOUL / MOONSKY / AQEL / ZILL / THE BATTERIES domain ownership and migration sequence.

## 10. Non-negotiable gates

- Source commit SHA and upstream ancestry recorded.
- License and NOTICE obligations verified file-by-file and dependency-by-dependency.
- Secret scan and dependency/advisory review completed.
- Domain owner and data source-of-truth documented.
- Contract compatibility and tenant-isolation tests pass.
- Build/test/CI evidence captured from the exact candidate commit.
- Backup, restore, rollback and operational runbook exist for stateful/runtime changes.
- No source repo is archived or deleted until all approved content is imported, provenance is preserved, links/consumers are migrated, releases are reproducible, and recovery is proven.
