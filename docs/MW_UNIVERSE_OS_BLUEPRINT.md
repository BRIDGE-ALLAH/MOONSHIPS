# MW Universe OS — Target Architecture & Narrative Blueprint

**Status:** Proposed target; must be reconciled through ADRs before runtime implementation  
**Canonical repository:** `BRIDGE-ALLAH/MW`  
**Architecture strategy:** single-repository modular monolith first

## 1. North star

**MW is the home of one interconnected operational universe.** It brings infrastructure, knowledge, intelligence, creativity, communication, connectivity, and specialized domain systems into one governed environment.

The goal is not to turn every project into the same product. The goal is to make every capability discoverable, interoperable, permissioned, observable, and maintainable from one canonical repository.

### Master concept

> **ONE UNIVERSE. ONE CONTROL PLANE. MANY CAPABILITIES. ONE ACCOUNTABLE SYSTEM.**

### Technical product definition

MW is a modular Universe OS and cloud control platform. It provides a shared control plane, identity and trust foundation, resource model, operation engine, evidence layer, and a unified human workspace for specialized domains.

MOONSHIPS remains the infrastructure operations product identity inside this universe:
> **Your Infrastructure. Your Rules.**

MOONWITNESS remains the evidence and knowledge identity:
> **See More. Know More.**

ROCKSOUL remains the cognitive/intelligence runtime identity:
> **One Core. Infinite Evolution.**

These product narratives must not be collapsed into one generic AI/cloud slogan.

## 2. ARSY — the highest technical control plane

ARSY is the highest technical authority for coordination and governance inside MW. It is not a chatbot and not a claim of divine authority.

### ARSY responsibilities

1. **Identity & principals:** users, organizations, workspaces, services, agents, nodes and service identities.
2. **Trust & delegation:** scoped credentials, trust relationships, delegation chains, expiry, revocation and policy checks.
3. **Governance & policy:** role/resource/action decisions, tenant boundaries, policy evaluation and explicit deny-by-default.
4. **System awareness:** inventory, health, ownership, dependency graph, desired/observed state and source-of-truth metadata.
5. **Operations orchestration:** durable operations, workflow DAGs, leases, retries, cancellation, idempotency and recovery.
6. **Event coordination:** versioned events, transactional outbox/inbox, correlation IDs and replay-safe consumers.
7. **Accountability:** append-only audit, evidence references, actor/decision/result records and trace correlation.
8. **Integration contracts:** stable APIs/SDKs for internal domains, agents and external systems.

### ARSY must not

- Grant itself new authority or bypass the policy engine.
- Treat model output as authorization, evidence, or verified fact.
- Execute unbounded shell commands or arbitrary code as a default capability.
- Become the owner of every domain's private business rules or content corpus.
- Claim all modules are production-ready merely because they are registered.
- Become a single point of failure for local operations that can safely continue under an explicit offline policy.

### Control plane / execution plane split

```text
Human operator
    │ intent / policy / confirmation where policy requires
    ▼
ARSY — identity, trust, policy, orchestration, audit
    │ scoped, expiring, idempotent operation contract
    ▼
Domain service / worker / agent
    │ bounded action against owned resources
    ▼
Infrastructure or domain resource
    │ observed result + evidence
    ▼
ARSY — reconcile, record, expose status
```

The worker/agent does not own global identity, tenant policy, or canonical run state. It receives a scoped grant from ARSY and reports verifiable results.

## 3. Brain architecture — distributed intelligence, centralized governance

“Brain” means a governed set of cognitive capabilities, not one omnipotent model.

| Capability | Canonical concept | Responsibility | Authority limit |
|---|---|---|---|
| Governance brain | ARSY | context, policy, trust, coordination | cannot override human/system policy |
| Cognitive runtime | ROCKSOUL | bounded reasoning, memory, planning, reflection | no implicit tools or side effects |
| Deterministic compute | ORACLE | reproducible computation and evaluation | input/output contracts; no policy ownership |
| Worker framework | LUMEN | bounded agent/worker execution | scoped grants; no global identity or run-state ownership |
| Evidence fabric | MOONWITNESS / WITNESS | source lineage, claims, observations, uncertainty and provenance | cannot turn assertion into fact without evidence |
| Resource intelligence | inventory/ontology | resource graph, relationships and semantic classification | domain owners remain authoritative for their resources |
| Human command interface | MW Workspace / Headquarters | operator visibility, review, intent and policy configuration | UI is not a security boundary |

### Cognitive lifecycle

```text
OBSERVE → GATHER EVIDENCE → CONTEXTUALIZE → REASON
        → PROPOSE PLAN → POLICY CHECK → EXECUTE BOUNDED TASK
        → VERIFY RESULT → RECORD EVIDENCE → LEARN WITH GOVERNANCE
```

Required invariants:
- Every claim has provenance and an explicit status (observed, reported, inferred, disputed, unknown, verified).
- Every mutating operation has actor, scope, policy decision, idempotency key, correlation ID and outcome.
- Memory is explicit, scoped, exportable, retention-governed and auditable.
- Human approval is required only where policy/risk demands it; routine safe work must be automated with machine-verifiable policy.
- A model is never its own independent verifier.

## 4. Domain map and product narratives

| Domain / identity | Primary job | Boundary |
|---|---|---|
| **MOONSHIPS** | Cloud/infrastructure operations: hosts, apps, containers, databases, domains, TLS, backups, network, agents | Owns infrastructure resource operations; ARSY governs authorization and orchestration |
| **MOONWITNESS** | Evidence-first knowledge, research, witness, source and corpus | Owns evidence and knowledge domain models; not global control plane |
| **ROCKSOUL** | Cognitive runtime, AI reasoning, memory and capability evolution | Owns bounded cognition; not global policy or unrestricted executor |
| **THE BATTERIES** | Unresolved project/domain identity from source narrative | Do not assign technical scope until original repo/docs/source are found |
| **ZILL / ZILL WATCH** | Unresolved identity; MoonWitness XV references ZILL WATCH as a module | Determine whether a product module, signal/intelligence domain, or separate concept; do not invent |
| **A-L-L-A-H.COM** | Public ecosystem identity/entry point in existing narrative sources | Public-facing identity, not backend authority |
| **GODPROMISE / EMPIRE OF THE LIGHT** | Vision, promise, institutional/cultural narrative | Preserve original source language and context; not runtime services |
| **AQEL** | Arabic-first programming language and language-neutral semantic core | Compiler/runtime domain; no privileged access by language semantics alone |
| **MOONSKY** | Messaging/channels | Messaging domain; not a second ARSY Hub |
| **MTS / ANTIKYTHERA** | Celestial time model and time/event computation | Research and deterministic time-domain capability; validate claims against astronomy/time standards |
| **E-ISP / HARBOR** | Connectivity, ISP operations and customer/service workflows | Business/industry domain; do not couple to core OS boot |
| **ORACLE** | Deterministic computation | Separate bounded compute capability behind contracts |
| **LUMEN** | Worker/agent execution framework | No second Hub or identity authority |
| **GATE / HEADQUARTER / COCKPIT / DOCK** | Human/public/operations interfaces | Route-based experiences over shared contracts; no direct privileged database access |

## 5. One repository, modular internal structure

The exact physical tree must be derived from source audits, but the intended ownership map is:

```text
MW/
├── apps/
│   └── mw-server/                 # primary runtime / operator web shell
├── crates/
│   ├── arsy-core/                 # control-plane domain contracts and policy
│   ├── arsy-identity/             # identity and tenancy
│   ├── arsy-operations/           # durable operation engine
│   ├── moonships-infra/           # infrastructure domain
│   ├── moonwitness-evidence/      # evidence/provenance domain
│   ├── rocksoul-cognition/        # bounded cognition contracts/runtime
│   ├── oracle-compute/            # deterministic computation
│   ├── lumen-workers/             # scoped worker execution
│   ├── aqel-language/             # compiler/language domain
│   ├── moonsky-messaging/         # messaging domain
│   ├── mts-time/                 # time/astronomical domain
│   └── shared-contracts/          # versioned cross-domain contracts
├── web/
│   ├── shell/                     # one navigation and identity shell
│   ├── modules/                   # route-based domain experiences
│   └── design-system/             # one canonical component/token system
├── data/
│   ├── migrations/
│   ├── fixtures/
│   └── schemas/
├── integrations/
│   ├── providers/
│   ├── protocols/
│   └── adapters/
├── deploy/
│   ├── local/
│   ├── single-node/
│   └── multi-node/
├── docs/
│   ├── ecosystem/
│   ├── architecture/
│   ├── adr/
│   ├── products/
│   └── migration/
└── tools/
    ├── repo-audit/
    ├── import-ledger/
    └── devx/
```

This is a **target map**, not permission to create all directories immediately. The current Rust/Axum/MiniJinja application remains the baseline until a migration ADR proves a safer path. React/Next.js surfaces from source repositories may be adapted as route-based modules or embedded static assets only after evaluating accessibility, bundle, routing, auth and operational costs. Do not create separate deployed apps by default.

## 6. Platform capability ownership

One canonical owner per capability:

- Identity, authorization and tenancy: ARSY.
- Resource registry and desired/observed reconciliation: ARSY contracts + domain-owned resource adapters.
- Durable operation state and scheduling: ARSY operation engine.
- Infrastructure execution: MOONSHIPS domain adapters and scoped agents.
- Model/provider connectivity: one AI gateway contract; choose implementation after overlap audit.
- Deterministic compute: ORACLE.
- Evidence and source lineage: MOONWITNESS/WITNESS.
- Cognitive runtime and memory: ROCKSOUL.
- Worker framework: LUMEN.
- Storage: one canonical storage contract with local and object-store adapters.
- ORM and migrations: one domain persistence strategy, introduced incrementally.
- UI tokens/components: one design system.
- Observability: shared traces, metrics, logs and audit correlation; no duplicate incompatible telemetry plane.

## 7. Runtime and frontend constraints

- Keep one canonical repository for all development.
- Prefer one primary web shell and route-based domain modules.
- Preserve current Rust/Axum/MiniJinja deployment and user experience while auditing alternatives.
- Do not migrate to React/Next.js wholesale without measured reasons and an incremental coexistence plan.
- Do not split services merely because a source repo had a separate app.
- Isolate a worker/process only where security, resource limits, language/toolchain, scaling, or failure isolation requires it.
- Keep local/single-node operation viable; multi-node/enterprise capabilities must be tested separately.
- Keep current Pier runtime compatibility names and data paths until tested compatibility migration.

## 8. Narrative and theological guardrails

The ecosystem may draw inspiration from values such as amanah (responsibility), mizan (balance), ilm (knowledge), adl (justice), and careful witnessing. These values can inform product ethics and governance.

The names ARSY, A-L-L-A-H, GODPROMISE, and religious references must be handled with respect. MW/ARSY are human-made software architecture. Do not imply that a product represents Allah, al-'Arsh, revelation, divine knowledge, or divine endorsement. Do not treat a technical graph or score as a religious verdict. Scripture corpora must preserve canonical text integrity, source attribution, and clear separation between text, metadata, evidence, translation, interpretation, and commentary.

## 9. Architecture decisions required before major code imports

1. Canonical ARSY implementation and Rust/Axum migration boundary.
2. Whether MW remains the existing Axum/MiniJinja app with modular routes or adopts a hybrid web workspace.
3. Canonical persistence/ORM strategy, SQLite compatibility and multi-node database adapter.
4. Identity and tenant model.
5. Resource identity and desired/observed-state contract.
6. Durable operation engine, leases, idempotency and recovery.
7. Agent/worker trust and capability sandbox.
8. Evidence graph, provenance and event contracts.
9. Canonical gateway and messaging ownership.
10. Storage/backup/restore abstraction.
11. Design-system and frontend module ownership.
12. Import order and upstream license/attribution policy.

## 10. Delivery sequence

- **Wave 0:** complete source inventory and exact-SHA audit.
- **Wave 1:** architecture and ownership ADRs; resolve duplicate ARSY and platform authorities.
- **Wave 2:** establish shared contracts, provenance/import tooling and repository governance.
- **Wave 3:** import foundations and domain modules in dependency order.
- **Wave 4:** integrate product routes and shared UI shell.
- **Wave 5:** automated integration, security, license, backup/restore, release and deployment evidence.
- **Wave 6:** deprecate/archive source repos only after successful import and retention checks.

## 11. Completion definition

The ecosystem is consolidated only when:
- All accessible repositories have an explicit disposition.
- All chosen source capabilities are imported with provenance and license compliance.
- There is exactly one canonical ARSY authority.
- Every domain has one owner and versioned contracts.
- The unified application builds and tests from a clean checkout.
- User-visible routes accurately represent implemented capability.
- No duplicated identity, policy, scheduler, gateway, database or resource authority remains.
- Production readiness is demonstrated with deployment, monitoring, backup/restore and rollback evidence.
