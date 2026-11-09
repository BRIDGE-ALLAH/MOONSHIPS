# MW Universe OS — Cross-Repository Consolidation Audit

**Status:** Initial evidence-based audit; migration not yet performed  
**Date:** 2026-11-09  
**Canonical destination:** BRIDGE-ALLAH/MW  
**Scope:** A-L-L-A-H / GODPROMISE / Empire of the Light / ARSY / MOONWITNESS / ROCKSOUL / THEBATTERIES / ZILL / E-ISP / MW-* and related systems.

> This is an evidence-based initial inventory, not a claim that every source repository has been fully inspected or migrated. No source repository has been deleted, archived, or modified in this audit.

## 1. Canonical decision

All active development converges into **MW (MoonWitness Universe OS)**. The user intends to work primarily in this repository rather than maintain fragmented development homes.

This means one canonical monorepo with explicit internal boundaries—not one tangled application and not a collection of duplicated backends.

- One canonical source repository: BRIDGE-ALLAH/MW.
- One authoritative technical control plane: **ARSY**, implemented as a bounded platform/domain within MW.
- **MOONSHIPS** remains the infrastructure operations product. Its existing Rust/Axum/MiniJinja runtime is the initial operational baseline.
- **MOONWITNESS** remains the evidence, knowledge, research, and provenance product domain.
- **ROCKSOUL** remains a creative/media and AI-workload product/runtime family with distinct responsibilities.
- **GODPROMISE**, **A-L-L-A-H.COM**, and **EMPIRE OF THE LIGHT** are vision, institutional, public identity, and narrative layers—not competing backend authorities.
- **THEBATTERIES**, **ZILL**, **E-ISP**, and every additional discovered project require a documented disposition before being considered consolidated.

Multiple apps, crates, packages, workers, and deployment profiles may exist inside MW. They must follow one architecture, security, and ownership system. A separate process is justified only by measured isolation, security, lifecycle, or scaling needs.

## 2. What one repository means

Copying every upstream tree into a folder would preserve fragmentation inside a monorepo. Each capability must have exactly one authoritative owner.

Allowed dispositions:
- **ADOPT** — retain as canonical implementation after license/security/quality review.
- **PORT** — migrate domain behavior into an existing MW owner; do not blindly copy implementation.
- **INTEGRATE** — consume through a versioned internal contract or adapter.
- **REBUILD** — only where no usable implementation exists and requirements justify it.
- **PRESERVE** — retain source snapshot/history/attribution as migration evidence.
- **ARCHIVE** — only after migration acceptance and dependency cutover.
- **REJECT** — unsafe, legally incompatible, obsolete, duplicate, or out of scope, with reason recorded.

No repository is to be deleted or made read-only until the migration ledger proves its contents have a disposition and the canonical implementation passes acceptance gates.

## 3. Initial repository inventory

These repositories were visible through the connected GitHub installation. README claims are not proof of implementation. Revalidate branches, commits, licenses, and source status before each migration batch.

| Repository | Observed role/evidence | Initial disposition |
|---|---|---|
| BRIDGE-ALLAH/MW | Renamed MOONSHIPS repo; Rust workspace; Axum + MiniJinja; AGPL-3.0 | **CANONICAL DESTINATION**; preserve current runtime and product identity |
| BRIDGE-ALLAH/MOONWITNESS | Evidence-first knowledge/operations monorepo; Next.js apps, Fastify ARSY, Python ORACLE, shared packages; mixed maturity and production NO-GO disclosed | **PORT / ADOPT BY DOMAIN**; migrate docs, apps, evidence and contracts; no second backend authority |
| BRIDGE-ALLAH/EMPIRE-OF-LIGHT | Rust/Cargo platform and Temps-derived workspace; Rust-first ARSY claims; Apache-2.0 | **DEEP AUDIT / SELECTIVE ADOPT**; reconcile overlap with MW before code migration |
| BRIDGE-ALLAH/ARSY | Small Rust Axum workspace; in-memory repositories; persistence adapters described as future work; no license selected | **REFERENCE / PORT SELECTIVELY**; not production authority as-is |
| BRIDGE-ALLAH/A-R-S-Y | Go/PocketBase-derived early-stage Docker Compose control plane; MIT; limited MVP | **REFERENCE / PRESERVE**; no second ARSY runtime in MW |
| BRIDGE-ALLAH/ROCKSOUL-WX | Rust-first outcome-driven AI gateway; workload contracts, provider routing, policy, receipts, Axum API, Dioxus deck | **ADOPT OR INTEGRATE** after code/security/test audit against MW-GATEWAY |
| BRIDGE-ALLAH/ROCKSOUL-EX | Local-first VS Code engineering companion; mission-scoped AI patches, explicit approval and proof; VSIX release line | **ADOPT AS PRODUCT/TOOL DOMAIN**; preserve local-first security model |
| BRIDGE-ALLAH/MW-GATEWAY | Agentgateway-derived LLM/MCP/A2A connectivity gateway; Apache-2.0 | **ADOPT/INTEGRATE** only after overlap map with ROCKSOUL-WX; preserve upstream attribution |
| BRIDGE-ALLAH/MW-ID | Kanidm-derived identity provider; MPL-2.0 | **INTEGRATE AS UPSTREAM COMPONENT** only after MPL obligations review |
| BRIDGE-ALLAH/MW-BAAS | Loco-derived Rust application framework; Apache-2.0 | **REFERENCE / SELECTIVE ADOPT**; avoid introducing a second backend framework without ADR |
| BRIDGE-ALLAH/MW-ORM | MoonWitness modular relational ORM target; Apache-2.0; README distinguishes target from completed implementation | **AUDIT / PROTOTYPE FIRST**; compare with MW rusqlite/SQLx/SeaORM usage |
| BRIDGE-ALLAH/MW-STORAGE | RustFS-derived object storage; Apache-2.0; upstream identity remains prominent | **INTEGRATE AS OPTIONAL SERVICE/DEPENDENCY**; verify license, provenance, maintenance and operational fit |
| BRIDGE-ALLAH/MW-IDE | Lapce-derived Rust editor evolving into evidence-driven workspace; Apache-2.0 | **ADOPT AS OPTIONAL DEVELOPER PRODUCT**; separate app/runtime boundary inside monorepo |
| BRIDGE-ALLAH/MW-UI | React/Next/Tailwind/shadcn component foundation; MIT | **SELECTIVE ADOPT**; current MW frontend is MiniJinja, so no forced rewrite |
| BRIDGE-ALLAH/MW-BOILERPLATE | Rust/Dioxus Moonbase starter; no backend/database/auth; compile/render verification pending | **REFERENCE / SELECTIVE ADOPT**; not default frontend architecture |
| BRIDGE-ALLAH/AGILITY | GPT4Free/g4f-derived multi-provider model access | **SECURITY + LEGAL REVIEW / LIKELY REJECT OR ISOLATE**; assess provider terms, credentials, abuse and supply chain |
| BRIDGE-ALLAH/wtagent | Browser/web-session agent with command/file automation claims | **SECURITY REVIEW / ISOLATE**; high-risk session, credential and command execution boundaries |
| BRIDGE-ALLAH/MOONWITNESS-GATEWAY | Repository visible; README/code not yet reviewed | **UNCLASSIFIED**; inspect tree, commits, license and code |
| BRIDGE-ALLAH/FS-BOILERPLATE | Repository visible; README/code not yet reviewed | **UNCLASSIFIED**; inspect tree and license |
| BRIDGE-ALLAH/BOILERPLATE-INTELEGENT-ENTERPRISE-FULLSTACK | Bun/Fastify/Next full-stack starter with optional Rust direction | **LEGACY/REFERENCE**; compare unique assets; no competing backend |
| bjo163/arah-saas-development | Only repository visible under owner bjo163 through current connected installation; private | **ACCESS LIMITED / AUDIT BLOCKED** |

## 4. Names and repositories still unaccounted for

The following names are in scope but were not located in the connected repository inventory:
- bjo163 repositories beyond bjo163/arah-saas-development.
- A-L-L-A-H.COM source repository under bjo163, referenced by EMPIRE-OF-LIGHT docs but not visible in this connection.
- THEBATTERIES and associated repositories.
- ZILL / ZILL WATCH repositories or standalone source.
- Additional GODPROMISE, E-ISP, JIZZ, SIDRA, ORACLE, LUMEN, WITNESS FABRIC, or related repos not listed above.

**Access blocker:** the connected GitHub installation currently exposes only one bjo163 repository. This is a visibility/permission limitation, not evidence that other repositories do not exist. Full audit requires granting the GitHub connection access to the intended bjo163 repositories or making their source archives available.

## 5. Proposed ecosystem narrative and ownership

- **GODPROMISE** — vision, promise, ethical intent, values; not a technical service.
- **A-L-L-A-H.COM** — public ecosystem identity, public home and discovery entry; not a second backend.
- **EMPIRE OF THE LIGHT** — institutional, enterprise, cultural and community universe; not a competing runtime or repo.
- **MW / Universe OS** — canonical engineering repository and integrated platform/product universe.
- **ARSY** — highest technical control plane inside MW: identity/trust, governance, policy, orchestration, capability registry and accountable coordination. It is not an unrestricted AI and does not claim divine authority.
- **MOONWITNESS** — evidence, knowledge, research, provenance and accountable intelligence product domain.
- **MOONSHIPS** — infrastructure operations product; retain its existing product identity inside MW unless separately decided.
- **ROCKSOUL** — creative/media expression and specialized AI workload execution.
- **THEBATTERIES** — unresolved; inspect source before assigning a role.
- **ZILL** — unresolved; MOONWITNESS registry references ZILL WATCH for rights/integrity/quality/policy inspection, but confirm original source intent.
- **E-ISP.ID** — connectivity/service business domain; customer and commercial workflows remain distinct from infrastructure primitives.

Sacred language must be handled respectfully. No software, model, company or product may claim to represent God, possess divine authority, or carry divine endorsement. Ethical values may inspire human governance principles but do not replace security, evidence or accountability.

## 6. Target technical architecture (not all implemented)

1. **ARSY control plane:** identity, trust, authorization, policy, tenant/workspace boundaries, capability registry, orchestration, audit and human approval boundaries.
2. **Shared foundations:** API contracts, events/outbox, persistence, secrets, configuration, observability, notifications, storage interfaces and developer tooling.
3. **Domain engines:** evidence/provenance, AI workload routing, infrastructure operations, identity integration, object storage, deterministic computation, creative workflow and ISP service workflow.
4. **Product experiences:** MOONSHIPS, MOONWITNESS, ROCKSOUL, E-ISP/HARBOR, developer tools and operational consoles.
5. **Adapters:** Docker, SSH, cloud APIs, model providers, MCP/A2A, storage backends and connectivity/payment providers.

Start as a modular monolith and selectively integrate independently deployable upstream components. Do not turn every folder into a service or add Kubernetes/microservices for appearance.

### ARSY brain model

ARSY is a governed coordination brain, not one all-powerful AI model:
- **Awareness:** aggregate verified health/state observations from domain owners.
- **Identity & Trust:** authenticate people, services, agents, nodes and workload identities.
- **Policy & Authority:** decide permitted actions under explicit policies and delegated scopes.
- **Intelligence:** analyze evidence and propose options with provenance and uncertainty.
- **Orchestration:** create bounded plans and dispatch work to specialized executors.
- **Execution boundary:** domain-owned workers execute authorized, scoped, idempotent operations.
- **Witness/Audit:** preserve decision, actor, policy version, inputs, outputs and result evidence.

AI recommendations never grant their own permissions. High-impact or irreversible actions require explicit approval policy. Every mutation must be authorized, auditable, idempotent or recoverable, and owned by one domain.

## 7. Architectural conflicts to resolve

1. **ARSY duplication:** Rust ARSY, Go/PocketBase A-R-S-Y, Fastify ARSY in MOONWITNESS, and Rust-first ARSY claims in EMPIRE-OF-LIGHT. Choose one canonical technical authority; others become migration references.
2. **AI gateway overlap:** ROCKSOUL-WX and MW-GATEWAY. Define workload policy/router versus protocol connectivity proxy boundaries.
3. **Backend/data overlap:** MW Rust/Axum + rusqlite, EMPIRE Rust/Cargo, MOONWITNESS Fastify/Postgres, MW-BAAS/Loco, MW-ORM, Go/PocketBase. Map capability/data ownership before merging.
4. **Frontend divergence:** MW uses Axum/MiniJinja; MOONWITNESS and MW-UI use Next/React; MW-BOILERPLATE uses Dioxus. Preserve MOONSHIPS UI initially; choose other profiles by product need.
5. **Licensing/provenance:** Kanidm/MPL-2.0, RustFS/Apache-2.0, Lapce/Apache-2.0, Loco/Apache-2.0, Agentgateway/Apache-2.0, PocketBase/MIT and other upstream-derived code need file-level notices, license checks and provenance.
6. **Maturity:** MOONWITNESS README explicitly says production NO-GO for its current stack. A target architecture is not proof of readiness.

## 8. Migration ledger requirements

For every source component record:
- Source repository, branch, commit SHA, path and license.
- Domain and business responsibility.
- Tests/CI/security evidence and maturity.
- Target path and single canonical owner in MW.
- Disposition (ADOPT / PORT / INTEGRATE / PRESERVE / ARCHIVE / REJECT).
- Conflict/overlap analysis and dependencies.
- Required legal notices and attribution.
- Migration batch, verification commands, acceptance evidence and rollback.
- Status and target commit/PR reference.

A copied README or plan is not a migrated capability.

## 9. Execution phases

### P0 — Access and source census
- Obtain visibility to all intended bjo163 repositories and all BRIDGE-ALLAH repositories.
- Enumerate repos, branches, tags, commits, releases, issues/PRs, default branch, archived status, license and last activity.
- Record immutable source SHAs.
- Inspect trees, manifests, code, docs, assets, workflows, secret templates and deployment artifacts.
- Acceptance: every in-scope repository is accounted for or marked inaccessible with owner/action.

### P1 — Capability and legal audit
- Inspect source code, not only README.
- Map identity, auth, policy, tenancy, orchestration, AI gateway, infrastructure, storage, ORM, evidence, analytics, creative, ISP and developer capabilities.
- Check licenses, notices, dependency security, secrets and dangerous execution paths.
- Acceptance: every capability has one target owner and legal/security disposition.

### P2 — Canonical architecture and brain contracts
- Publish ADRs for ARSY authority, backend/runtime boundaries, UI profiles, persistence, eventing, agents, gateway split, storage and identity.
- Define versioned internal API/event contracts and action authority/approval matrix.
- Acceptance: no unresolved duplicate authority for critical capabilities.

### P3 — Monorepo foundation
- Establish MW workspace layout and dependency rules without disrupting MOONSHIPS runtime.
- Add source provenance, third-party notices, migration ledger, architecture docs and CI boundaries.
- Acceptance: clean checkout of MW still builds/tests existing MOONSHIPS and validates new foundations.

### P4 — Bounded migration batches
- Migrate docs/brand/blueprints first, then shared contracts, selected libraries/services, then apps.
- One capability per batch; compare paths before copy; preserve upstream history where feasible.
- Run unit, integration, security, license, build and operational checks after each batch.
- Acceptance: each batch has source SHA, target commit, test evidence and rollback plan.

### P5 — Cutover and retirement
- Update source README to point to MW only after relevant content is migrated and verified.
- Archive/read-only source repos only after dependencies are removed and the user explicitly authorizes archival.
- Acceptance: active development can happen from MW; old repos remain historical references and are no longer required.

## 10. Immediate next actions

- [ ] Resolve GitHub access to all bjo163 repos, especially A-L-L-A-H.COM, THEBATTERIES and ZILL.
- [ ] Complete code/manifest/license/CI/issue/PR audit of the 20 visible BRIDGE-ALLAH repositories.
- [ ] Inspect MW TODO and existing roadmap to avoid duplicate tasks.
- [ ] Create canonical ADRs and migration ledger.
- [ ] Map MOONSHIPS runtime into MW without breaking installer, paths, services, env vars or SQLite data.
- [ ] Audit EMPIRE-OF-LIGHT Rust platform against MW before selecting code migration.
- [ ] Audit MOONWITNESS apps and Fastify ARSY behavior; migrate requirements, not a duplicate backend.
- [ ] Decide gateway, ORM, identity, storage, frontend and agent ownership.
- [ ] Begin reversible migration batches only after baseline tests and source SHAs are recorded.

## 11. Non-goals and safety rails

- No destructive repository deletion, force-push or mass overwrite.
- No secrets, credentials, private keys, tokens, production env files or customer data in Git.
- No blind copy of whole repositories.
- No second backend authority beside canonical ARSY/MW.
- No forced React/Dioxus rewrite of the existing MOONSHIPS interface.
- No claims of completed migration, feature parity or production readiness without tests.
- No automatic source archival.
- Preserve upstream licenses, NOTICE files, copyrights, trademarks and provenance.


## 12. Additional baseline findings from the MW repository itself

A second pass inspected the current MW TODO, Cargo workspace manifest, and MOONWITNESS platform ownership map.

- MW already has a detailed 61 KB engineering backlog. Its opening section is still titled **MOONSHIPS — Consolidated Engineering Backlog**, and its metadata says branch `rebrand/moonships-full-brand`. That branch label is stale relative to the renamed repository's current `main` default branch and should be corrected only after checking the actual TODO ownership/status convention.
- The root Cargo workspace still declares `repository = https://github.com/BRIDGE-ALLAH/MOONSHIPS`. This is a stale repository metadata reference after the rename; it should become `https://github.com/BRIDGE-ALLAH/MW` in a small metadata-only change.
- README installation instructions still clone `BRIDGE-ALLAH/MOONSHIPS.git`. GitHub may redirect this URL, but canonical documentation should use `BRIDGE-ALLAH/MW.git`.
- The existing MW backlog is intentionally rigorous about preserving Pier runtime identifiers, AGPL-3.0 obligations, customer data, and no unattended destructive production operations. These constraints remain binding during consolidation.
- MOONWITNESS has a detailed capability ownership map (document 33) that already assigns ARSY the control-plane role for caller identity, authorization, policy/approval integration, orchestration and canonical audit. It also states HEADQUARTER is the operator UI, Docker connector is read-only, and queue transport is not business truth. This work must be reconciled into MW's architecture rather than recreated from scratch.
- MOONWITNESS ownership map records an existing Docker workload ownership ADR and multiple tracked issues. Reuse these decisions as migration inputs after validating their source commit and current status; do not open duplicate implementation work without comparing existing issue/PR state.

These are documentation/metadata alignment findings only. No runtime compatibility identifiers, services, paths, database schema, or deployment behavior were changed.
