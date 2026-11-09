# MW Ecosystem Architecture Decisions — 2026-11-09

**Status:** Proposed decisions for implementation planning; not a claim that the target runtime already exists.  
**Authority:** User-designated canonical repository BRIDGE-ALLAH/MW.  
**Strategy:** one ecosystem repository, modular monolith first, explicit domain ownership, incremental migration.

## Executive decision

MW is the canonical ecosystem home and shared human workspace. It is not one undifferentiated application and it does not take ownership of every domain's private data. The architecture has one technical control-plane authority (ARSY), one primary operator shell, shared identity/policy contracts, and multiple bounded product/domain modules.

**North star:** ONE UNIVERSE. ONE CONTROL PLANE. MANY CAPABILITIES. ONE ACCOUNTABLE SYSTEM.

MOONSHIPS remains the infrastructure operations product identity inside MW. MOONWITNESS remains evidence and knowledge. ROCKSOUL remains bounded cognition and creative intelligence. MOONSKY remains messaging. AQEL remains language/compiler research. MFTL and RGBL remain separate narrative and exact-text domains. M-Time remains temporal interoperability/research. ZILL WATCH is a HEADQUARTER module in prototype status. THE BATTERIES remains a narrative/source-catalog reference until its intended referent is verified.

## ADR-0001 — Canonical repository and modular-monolith boundary

**Status: Proposed — recommended for ratification**

### Decision
1. BRIDGE-ALLAH/MW is the sole default location for active ecosystem development, architecture, product planning and integration.
2. Use the existing Rust/Axum MW workspace as the operational baseline; organize new work as bounded crates/modules and route-based experiences.
3. A product identity does not automatically require a separate executable, service, repository or database.
4. Extract a process/service only for a concrete security, scaling, lifecycle, language/toolchain or failure-isolation requirement.
5. Source repositories remain intact until import, provenance, license, test, operational and recovery acceptance is demonstrated.

### Alternatives rejected
- Continue parallel active development across many repositories: rejected because it fragments source of truth and governance.
- Copy every repository into one monolithic application: rejected because it destroys domain boundaries and increases risk.
- Microservices by brand name: rejected because a name alone is not a lifecycle or trust boundary.

### Consequences
New active work should land in MW. Legacy repositories become pinned migration sources or historical references. Every import needs source SHA, destination path, license/NOTICE, transformations, exclusions, tests and rollback evidence.

## ADR-0002 — ARSY is the single technical control-plane authority

**Status: Proposed — target authority selected; implementation choice remains staged**

### Decision
ARSY owns ecosystem principals, organization/workspace/tenant mapping, authorization policy, trust/delegation, resource registry contracts, durable operation coordination, event correlation and append-only audit/evidence references. Domain modules own their business rules and data.

The target direction is a Rust implementation integrated into the MW workspace. Current MW Axum/Pier runtime is the host baseline, but it is **not yet proven to implement the complete ARSY governance contract**. The Fastify ARSY implementation in MOONWITNESS/A-L-L-A-H.COM is a migration/reference source for routes, schemas and behavior, not a second permanent Hub. EMPIRE-OF-LIGHT is a large Temps-derived platform, not a small ARSY crate. The small ARSY repo is a prototype, not the canonical implementation.

### Required migration rule
Build a capability parity matrix before porting: route/API, schema/migration, authn/authz, tenant isolation, policy, jobs/leases/retries, integrations, audit, observability, tests and deployment. Port behavior only after negative tenant tests and source-level security review. The legacy Fastify LLM gateway is blocked from wholesale import because source inspection found unscoped query paths and missing tenant propagation at call sites; proposed remediation files are not proof of merged fixes.

### Alternatives rejected
- Keep multiple global hubs (Fastify, Rust prototype, Temps-derived platform): rejected due to conflicting authority and run-state.
- Declare current MW complete ARSY because it is Rust/Axum: rejected because its existing proven role is infrastructure operations and its full governance parity is not established.
- Import Fastify wholesale: rejected until tenant/security gates pass.

## ADR-0003 — Infrastructure platform lineage: retain MW/Pier, select capabilities only

**Status: Proposed — recommended**

### Decision
Keep current MW/Pier-derived Rust/Axum infrastructure operations runtime as canonical MOONSHIPS server baseline. Preserve Pier-compatible paths, units, environment keys, agent protocol and database upgrade behavior during incremental rebranding.

Treat EMPIRE-OF-LIGHT (Temps-derived) and mwx-ships (Loco/SeaORM) as capability sources, not replacements. Evaluate each capability against a source SHA, parity, security, license, migration and rollback evidence. Do not merge platform workspaces wholesale.

The separate Tauri/Rust/React moonships repository is an optional native operator workbench/client. It is not the main MW browser shell and not a second server control plane.

### Consequences
No big-bang rewrite. Feature selection happens per bounded capability. Existing production state is preserved unless an independently tested migration is approved.

## ADR-0004 — Identity, authentication and authorization boundaries

**Status: Proposed — target model selected; provider selection open**

### Decision
Separate:
- identity source/provider (authentication and account lifecycle);
- ARSY authorization (permissions, tenant/resource scope, policy and delegation);
- domain-local data ownership;
- machine/service identities (scoped, expiring and revocable).

ARSY is the ecosystem authorization/policy authority. A specialist IdP such as the Kanidm-derived MW-ID may authenticate identities but is not automatically selected or allowed to replace ARSY authorization.

MOONSKY currently has an accepted standalone operator-identity ADR and owns its own schema. Preserve that boundary during the first migration. Shared ecosystem SSO is a target only after an explicit OIDC/federation contract, operator/account mapping, credential rotation, WebSocket authorization, break-glass recovery and rollback are tested. Do not silently merge accounts or sessions.

### Consequences
One user experience is the goal, not an unsafe shared database or universal admin token. Every service identity gets least privilege and a documented owner.

## ADR-0005 — Persistence, schema ownership and storage

**Status: Proposed — preserve current baseline**

### Decision
1. Keep MW's current SQLite/rusqlite persistence and embedded migration ordering for the existing runtime.
2. Preserve local/single-node operability. Do not change the primary database or ORM during the brand/consolidation pass.
3. Domain owners retain their schema/data ownership until an explicit API and migration plan moves it.
4. Evaluate MW-ORM/SeaORM with one representative domain prototype, including SQLite/Postgres behavior, tenant filtering, transactions, migration rollback and concurrency; do not adopt by repository name.
5. MOONSKY's PostgreSQL schema remains MOONSKY-owned. Q4Q's Supabase runtime remains source-of-truth until an approved migration exists.
6. MW-STORAGE/RustFS is a separate object-storage service candidate, never a substitute for relational storage.
7. MFTL owns narrative records and assessment; RGBL owns exact corpus text and provenance. Cross-domain references use stable IDs and versioned contracts.

### Consequences
There is no forced database consolidation. A unified ecosystem API and identity model can coexist with domain-owned stores during migration.

## ADR-0006 — Primary frontend and experience architecture

**Status: Proposed — keep current shell**

### Decision
Keep the existing Axum/MiniJinja web UI as the primary MW control-plane shell during the first integration waves. Build one navigation, account context, permission-aware route registry and design-token contract.

Use MW-UI as a source for selective design-token/component adaptation after license/dependency/accessibility review. Do not replace the shell wholesale with Next.js. Headquarter/other React surfaces can be migrated as route-based modules or static assets when API/auth parity exists; do not deploy separate apps by default.

The Tauri moonships workbench remains an optional native client with its own threat boundary and native-side permission checks.

### Consequences
The user sees one primary frontend while product identities retain distinct visual voice and workflows. Prototype/synthetic/offline/unavailable states remain explicit.

## ADR-0007 — Gateway and external integration strategy

**Status: Proposed — no gateway implementation selected yet**

### Decision
Define one MW gateway contract for provider routing, model/API connectivity, MCP/A2A, egress policy, credential references, usage metering, tenant isolation and audit. Compare MW-GATEWAY (agentgateway-derived), ROCKSOUL-WX and the existing Fastify gateway against this contract.

Do not import a gateway implementation until upstream provenance, license/NOTICE, authn/authz, tenant isolation, SSRF/DNS rebinding, TLS validation, credential storage, egress allow-list, rate limits, streaming, audit and failure behavior are independently verified.

### Consequences
No open proxy or duplicate gateway. Provider credentials stay server-side and are never exposed to browser clients or logs.

## ADR-0008 — Domain ownership and maturity

**Status: Proposed — ownership map for migration**

| Domain / identity | Canonical responsibility | Current evidence status | Boundary |
|---|---|---|---|
| ARSY | Identity integration, policy, tenancy, orchestration, audit | Multiple implementations/prototypes; target Rust parity incomplete | One global technical authority |
| MOONSHIPS | Servers, apps, containers, databases, domains, TLS, backups, infrastructure ops | Current MW runtime is the operational baseline | Infrastructure execution under ARSY policy |
| MOONWITNESS / WITNESS | Evidence, observations, knowledge and source lineage | Multiple source workspaces; production gates vary | Cannot turn assertion into fact without evidence |
| RGBL | Exact text/corpus identity, edition, passage and provenance | Separate corpus source | Canonical text owner |
| MFTL | STORY/narrative, claims and downstream assessment | Small dedicated source tree | Does not own exact corpus, canonical event or person registries |
| ROCKSOUL | Bounded cognition, memory, planning and evaluation | Rust prototype with explicit no-autonomous-tools boundary | Model output is never authority/verifier |
| MOONSKY | Messaging channels and delivery | Standalone service with own schema/operator identity | SSO integration is gated; schema stays domain-owned |
| AQEL | Arabic-first language/compiler | Very small pre-release spec | No production compiler claim |
| M-Time / Antikythera | Temporal interoperability and research | Multi-crate research system | Does not redefine international time standards or religious authority |
| ZILL WATCH | Quality, rights and policy inspection module within HEADQUARTER | Verified route/docs/policy code; explicitly prototype | Not a separate app or live inspection service yet |
| THE BATTERIES | Unresolved narrative/source-catalog reference | Mentioned with IsRockFeel and fictional-character references; no technical product confirmed | No implementation/domain assignment until referent is verified |
| MW-GATEWAY / ROCKSOUL-WX | Gateway implementation candidates | Upstream-derived/candidate code | No canonical gateway selected |
| MW-UI | Design-system source | Large React/Next/Tailwind tree | Selective extraction only |
| MW-ORM | ORM candidate | SeaORM-derived | No adoption until prototype |
| MW-ID | Identity-provider candidate | Kanidm-derived | Authentication provider only if selected; not ARSY policy |
| MW-STORAGE | Object-storage candidate | RustFS-derived | Separate S3 service |
| EMPIRE-OF-LIGHT | Platform capability source | Large Temps-derived workspace | Selective capabilities; preserve attribution |

### Maturity vocabulary
Every route and capability must be labelled one of: implemented, partial, prototype, planned, parked, external dependency, legacy, blocked or unknown. Directory presence and mock UI are not production evidence.

## ADR-0009 — Brand and narrative hierarchy

**Status: Proposed — product architecture, not theological claim**

### Decision
MW is the canonical ecosystem/repository umbrella. MOONSHIPS is the infrastructure operations product. MOONWITNESS is the evidence/knowledge identity. ROCKSOUL is the cognitive/creative identity. MOONSKY, AQEL, MFTL/RGBL, M-Time, ZILL WATCH and other verified domains keep their own names and bounded jobs.

The visual DNA for the shared ecosystem is GRUNGE × RIOT × FUNKY × GEN-Z: underground-future texture, expressive typography, high-contrast utility, irreverent energy and clear operator affordances. Keep usability, accessibility, trust cues and truthful system state ahead of decorative effects.

A-L-L-A-H / Empire of the Light and related language may be represented as the user's institutional/narrative universe with respect. Software, ARSY and MW must never claim divine status, revelation, divine endorsement or theological authority. Technical metrics and policy decisions are human-made system outputs, not spiritual verdicts.

## Implementation sequence

1. Complete W0 metadata and recursive source audit, including exact source commits, license files, workflows, releases, submodules and LFS.
2. Complete W1 route/schema/tenant/API parity and ratify the ADRs above.
3. Complete W2 license/NOTICE/SBOM, secret scan, dependency/advisory and upstream-diff gates.
4. Build import ledger and versioned shared contracts before importing runtime code.
5. Integrate in dependency order: ARSY identity/policy/operations contracts; MOONSHIPS infrastructure; evidence/corpus; bounded ROCKSOUL; MOONSKY federation; AQEL/M-Time; gateway/storage/UI adapters.
6. Run clean-checkout build/test/security/license CI, migration/rollback, tenant negative tests, backup/restore and operational drills.
7. Retire legacy repos only after import acceptance and owner-approved archival.

## Open blockers

- Full 142-repository metadata/tree inventory is not complete; current audit covers the accessible inventory baseline plus priority source trees.
- ARSY route/schema/auth/tenant parity is not complete and the legacy Fastify LLM gateway has a security import blocker.
- Upstream lineage, license compatibility and NOTICE obligations have not been fully audited for all candidates.
- No build/test/restore/deployment evidence was executed by this source-tree pass.
- THE BATTERIES referent is unresolved; ZILL WATCH is known but only as a prototype HEADQUARTER module.
