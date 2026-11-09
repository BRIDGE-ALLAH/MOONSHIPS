# MW Ecosystem Consolidation — Execution TODO

## 2026-11-09 execution snapshot

This snapshot separates completed evidence collection from remaining acceptance work. A task remains open until its full acceptance criteria pass; source-tree inspection alone does not close build, security, license or migration gates.

| Workstream | Current status | Evidence / remaining gate |
|---|---|---|
| W0.01 repository visibility inventory | Baseline captured | 142 public bjo163 search results, 20 visible BRIDGE-ALLAH repos, 1 separately accessible bjo163 repo; rerun at final audit and reconcile any inaccessible/history gaps |
| W0.02 metadata | Partial | Search/list JSON snapshot exists; languages, releases, tags, latest commits, licenses/files, workflows, CI, submodules/LFS and assets remain incomplete |
| W0.03 recursive source audit | Partial | Priority tree audit added for 16 repos, including the MOONWITNESS HEADQUARTER prototype/source registry; full recursive audit of all accessible candidates and file-level review remain open |
| W0.04 unresolved names | Partial | ZILL WATCH confirmed as a HEADQUARTER prototype module; THE BATTERIES appears in MOONWITNESS narrative/source catalog only; owner-wide bjo163 search yielded false positives, technical referent unresolved |
| W1.01 ARSY comparison | Partial / blocked | Fastify source findings recorded; Rust/Axum parity, full route/auth/tenant matrix and security-negative tests remain open |
| W1.02 MOONSHIPS comparison | Partial | Current MW retained; mwx-ships and Tauri workbench assessed at source/docs level; feature/security parity and CI evidence remain open |
| W1.03 platform capability ownership | Drafted | Proposed ADR-0001–0009 and domain ownership map added; owner ratification and source-backed contracts remain open |
| W1.04 identity, data, UI and gateway | Drafted | Preserve current baselines; provider/gateway/ORM/storage selections remain gated on prototypes and security/license review |
| W2–W6 | Not started / gated | License/SBOM, secret and dependency scans, import ledger, code imports, unified routes, release/restore and legacy retirement must follow W1 |

**Current safety posture:** no source repository was modified, archived or deleted. No runtime code was imported. No production system or customer data was touched. The draft PR is not merged and no CI-green claim is made.

---


**Canonical repository:** `BRIDGE-ALLAH/MW`  
**Parent tracker:** [Issue #18](https://github.com/BRIDGE-ALLAH/MW/issues/18)  
**Execution model:** sequential dependency waves; read-only audit before imports; no destructive source-repository actions.

## Global task contract

Every task must:
- state its exact source/destination ownership boundary;
- record assumptions and source commit SHA;
- preserve license, NOTICE, copyright and provenance;
- exclude secrets, private keys, credentials, production data and .env files;
- add deterministic tests and negative cases appropriate to risk;
- run the relevant format/lint/build/test/security/license gates;
- attach machine-readable evidence;
- update the repository register and architecture docs;
- be independently verifiable without subjective human approval.

A checkbox is complete only when all acceptance criteria pass on the target commit. A code change without evidence is not complete.

---

## W0 — Inventory and source intelligence

### [ ] W0.01 — Exhaust all repository listings
**SPEC**
- Enumerate `bjo163/*` public repositories with paginated GitHub search.
- Enumerate all accessible private repositories and all `BRIDGE-ALL/*` repositories.
- Record search date, owner, visibility, default branch, archived flag and repository URL.
- Re-run the search at completion to detect repositories created/transferred during the audit.

**RULE**
- Never assume one search page is complete.
- Mark inaccessible/private/empty repositories explicitly.
- Do not treat an empty repository as an import candidate with inferred contents.

**ACCEPTANCE**
- Public and accessible-private inventory counts match the completed paginated search.
- Every result has a unique canonical owner/name key.
- The repository register has no unclassified *visibility/status* field, even if its domain remains unknown.

### [ ] W0.02 — Capture repository metadata
**SPEC**
For every inventory entry capture:
- default branch, visibility, archived status, size, description, primary languages;
- created/updated/pushed timestamps and latest commit SHA/date;
- latest release/tag and tag commit;
- license identifier and license-file path;
- open issues, open PRs, Actions workflows and latest CI result;
- submodules, Git LFS, release assets and deployment configuration where present.

**RULE**
- Record null/unknown when the API does not expose a value; never infer it.
- Distinguish repository update time from source-code activity.

**ACCEPTANCE**
- One machine-readable JSON/CSV snapshot is generated under `docs/ecosystem/inventory/`.
- The Markdown register links to the snapshot and explains missing fields.

### [ ] W0.03 — Recursive tree and documentation audit
**SPEC**
- Capture recursive default-branch trees for all priority candidates.
- Read README, architecture, product narrative, TODO, ADR, package manifests, Docker/Compose, CI, deployment, security, license and contribution docs.
- Identify apps, packages, crates, services, scripts, migrations, schemas, fixtures, assets, generated code and vendored upstream code.

**RULE**
- Directory presence is not implementation evidence.
- README claims must be cross-checked against source, tests and CI.
- Keep planning-only, parked, prototype and production claims separate.

**ACCEPTANCE**
- Every priority repository has a concise technical profile with source SHA.
- Every workspace/app/service has an owner and maturity label.

### [ ] W0.04 — Locate unresolved identities
**SPEC**
- Search source files and documentation for THE BATTERIES, ZILL, GODPROMISE, A-L-L-A-H, Empire of the Light, ZILL WATCH, and alternate spellings.
- Search across all accessible owners and repositories, not only exact repository names.
- Record exact path, source SHA, nearby context and the distinction between product, module, dataset, campaign and repository.

**RULE**
- Never assign a domain based on a name alone.
- Do not reproduce private or sensitive material in public documentation.

**ACCEPTANCE**
- Each name is classified as verified product, module, concept, alias, historical reference, unresolved, or not found.
- Each classification has source evidence or an explicit unknown status.

---

## W1 — Architecture and authority reconciliation

### [ ] W1.01 — ARSY implementation comparison
**SPEC**
Compare all Rust/Axum, Fastify, Go/PocketBase and other ARSY implementations:
- routes and API contracts;
- identity/auth/RBAC/tenant boundaries;
- data schemas, migrations and persistence;
- operations/jobs/scheduler/workflows;
- integrations, agents and remote execution;
- observability, audit and security controls;
- tests, CI, release/deployment and operational evidence;
- license and upstream derivation.

**RULE**
- Do not maintain multiple canonical ARSY Hubs.
- Reuse working domain behavior and contracts; do not blindly port.
- Preserve runtime compatibility during migration.

**ACCEPTANCE**
- An ADR selects one ARSY implementation and records rejected alternatives.
- Migration matrix maps every retained capability to a destination module and regression test.
- Exactly one API/backend/run-state authority is declared.

### [ ] W1.02 — MOONSHIPS implementation comparison
**SPEC**
Compare current MW Pier-derived Rust/Axum application with `bjo163/mwx-ships` Loco/SeaORM implementation:
- feature parity and unique capabilities;
- database schema and migration quality;
- remote execution and agent security;
- queue/scheduler reliability;
- UI/UX and deployment;
- license and upstream dependency provenance.

**RULE**
- Do not replace the current app because another README claims production-grade.
- No big-bang rewrite.
- Preserve existing Pier runtime compatibility identifiers.

**ACCEPTANCE**
- ADR chooses reuse/adapt/import/reject per capability.
- No duplicate infrastructure control plane remains in the target architecture.

### [ ] W1.03 — Platform capability ownership
**SPEC**
Set one owner for identity, policy, tenancy, ORM, storage, gateway, messaging, scheduler, event fabric, observability, UI, agent runtime, evidence and resource inventory.

**RULE**
- No two services own the same canonical database table, API namespace, identity authority, scheduler or event contract.
- Domain services own domain rules; ARSY owns cross-domain governance and coordination.

**ACCEPTANCE**
- Capability ownership matrix is committed.
- Every cross-domain call has a versioned contract and authorization boundary.

### [ ] W1.04 — Runtime and frontend boundary
**SPEC**
- Compare current MiniJinja UI with React/Next surfaces in source repos.
- Choose one primary shell and route-based module strategy.
- Identify where process isolation is genuinely required.

**RULE**
- One repository does not require one binary, but separate deployments require a documented technical reason.
- Do not create a new standalone product app or repository by default.
- Preserve existing user flows until replacement parity is tested.

**ACCEPTANCE**
- ADR defines the primary shell, frontend stack, shared components and route ownership.
- A prototype route proves the selected integration approach before bulk migration.

---

## W2 — Legal, security and provenance

### [ ] W2.01 — License and attribution audit
**SPEC**
- Detect license files, SPDX headers, NOTICE files, copyright, trademarks, upstream origin, dependencies and copied assets.
- Build an SBOM for each candidate.
- Check compatibility with MW's AGPL-3.0 distribution model.

**RULE**
- No import if license is missing, incompatible or unclear until resolved.
- Preserve upstream notices and source headers.
- Do not strip or rebrand third-party ownership.

**ACCEPTANCE**
- Every imported path has an approved license disposition.
- SBOM and attribution inventory are committed.

### [ ] W2.02 — Secrets and sensitive data audit
**SPEC**
- Scan git history and current trees for tokens, keys, credentials, private env files, customer data and personal data.
- Classify findings and remove/quarantine from import payload.
- Rotate exposed live credentials through the proper operator workflow; never print secrets.

**RULE**
- Never copy secrets into MW or audit logs.
- No secret values in issues, docs, test snapshots or CI artifacts.

**ACCEPTANCE**
- Clean secret scan on import commit and history strategy documented.
- Sensitive data exclusions are machine-tested.

### [ ] W2.03 — Dependency and supply-chain audit
**SPEC**
- Run language-specific dependency audit, license audit, malware/advisory checks, lockfile validation and build reproducibility checks.
- Identify abandoned or forked upstream packages.

**ACCEPTANCE**
- Risk report exists per import candidate.
- Critical/high unresolved advisories block import unless mitigated and documented.

---

## W3 — Import foundations and domain modules

### [ ] W3.01 — Establish import ledger and tooling
**SPEC**
- Add machine-readable source-to-destination manifest with source URL, exact SHA, license, imported paths, excluded paths, transformations and validation commands.
- Add automated check for duplicate source IDs and missing provenance.

**ACCEPTANCE**
- CI fails when an imported component lacks provenance metadata.

### [ ] W3.02 — Import shared platform foundations
Import in dependency order only after W1/W2:
1. shared contracts and schema registry;
2. identity and tenancy;
3. data/persistence adapters and migrations;
4. event/audit/evidence contracts;
5. operation engine and worker grants;
6. gateway, storage, observability and design system.

### [ ] W3.03 — Import MOONWITNESS domain
**SPEC**
- Import verified corpus, source lineage, claim/evidence models, witness workflows and UI routes.
- Preserve separation of text, translation, metadata, observation, claim, interpretation and commentary.
- Keep sensitive scripture datasets immutable and source-attributed.

**ACCEPTANCE**
- Corpus validation and provenance tests pass.
- No unverified claim is promoted to fact.

### [ ] W3.04 — Import ROCKSOUL domain
**SPEC**
- Import bounded cognition, explicit memory, evaluation and model adapter contracts.
- Keep model access behind policy-scoped interfaces.
- Keep shell execution, filesystem mutation and tool use disabled unless separately sandboxed and authorized.

**ACCEPTANCE**
- Capability and permission tests prove deny-by-default behavior.
- Model-generated plans cannot authorize themselves.

### [ ] W3.05 — Import AQEL, MOONSKY, MTS and other specialized domains
**SPEC**
- Import each domain as an isolated crate/module with versioned contracts.
- Preserve compiler determinism, messaging boundaries, time semantics and source data.
- Validate claims and behavior independently.

**ACCEPTANCE**
- Each domain builds/tests independently and through the MW workspace.
- No domain becomes a second global control plane.

### [ ] W3.06 — Import THE BATTERIES and ZILL only after identification
**SPEC**
- Locate source repositories, docs, code or owner-approved definitions.
- Identify their actual product/domain scope and dependency graph.
- Record unresolved concepts without inventing features.

**ACCEPTANCE**
- A source-backed domain brief exists before any implementation import.

---

## W4 — Unified human workspace

### [ ] W4.01 — One navigation and identity shell
- One sign-in/session/identity flow.
- One navigation model with role-aware modules.
- One design token and component owner.
- Domain route modules preserve product identity.
- No direct privileged database access from frontend.
- Demo/sample/unavailable/production states are explicit.

### [ ] W4.02 — Integrate MOONSHIPS operational workspace
- Preserve existing deployments, containers, databases, servers, domains, TLS, backups and agent flows.
- Expose cross-domain links through contracts rather than duplicate data.
- Preserve current product tagline and brand book unless a new narrative ADR explicitly changes it.

### [ ] W4.03 — Integrate knowledge and cognitive workspaces
- Add evidence search, corpus navigation, source tracing and research workflows.
- Add ROCKSOUL task/memory/evaluation surfaces only after permission and privacy contracts pass.
- Show provenance, confidence and status labels.

---

## W5 — Verification and release

### [ ] W5.01 — Workspace CI matrix
- Rust fmt/clippy/test/build.
- Frontend lint/typecheck/build/e2e for retained UI modules.
- Schema/migration compatibility.
- License/SBOM and secret scanning.
- Container build and clean-install smoke.
- Integration contracts and API compatibility.

### [ ] W5.02 — Reliability and security drills
- SQLite backup/restore and migration rollback.
- Durable operation recovery, fencing, idempotency and duplicate delivery.
- Agent credential expiry/revocation and isolation.
- Tenant-boundary negative tests.
- Gateway TLS and trust verification.
- Audit/event replay and retention tests.

### [ ] W5.03 — Operational readiness
- Deployment profiles for local/single-node and tested multi-node.
- Managed secrets, TLS/ingress, monitoring/alerting, backup/restore, rollback and runbooks.
- Production status remains NO-GO until live target-environment evidence is recorded.

---

## W6 — Legacy repository retirement

### [ ] W6.01 — Import completion review
For each source repository confirm exact source SHA, destination SHA, path map, license, tests, CI, security, provenance and rollback.

### [ ] W6.02 — Deprecation handoff
Where permitted, add a pointer README linking to MW and recording the import status and retained history.

### [ ] W6.03 — Archive eligibility
A repository may be marked archive-eligible only after every retained capability is imported or explicitly rejected, and the source repository's unique data/history is preserved.

**Forbidden:** automatic deletion, force-push, history rewrite, secret publication, or archive before verified acceptance.

## 2026-11-10 follow-on execution

- CI for the preceding PR head `6e69754eca2d8a1e0d2f31dde5c63e394bd2c3cd` completed successfully (GitHub Actions run 42655941865). This validates that commit's configured workflow only; it is not a production, migration, security or integration acceptance.
- Repository metadata API snapshot now covers 21 of 162 unique repositories visible in the combined search baseline: 5 initial samples plus 16 priority repos. Remaining 141 metadata records and repository history gaps are still open.
- ARSY parity matrix added. It separates MW/Pier's current project RBAC from MOONWITNESS ARSY's single-operator token and signed deployment-operation controls, and from the legacy A-L-L-A-H.COM gateway lineage.
- Source review confirms MW's AuthUser has no tenant/org field; its project membership/RBAC is not a general tenant hierarchy. MOONWITNESS ARSY currently maps deployment tenant_id to owner_id and uses one configured operator principal. These are distinct, concrete migration gaps.
- Next implementation gate is typed ARSY contracts and deterministic project/owner-to-tenant mapping. No runtime auth, schema, database or API behavior changed in this pass.
## 2026-11-10 ARSY contracts implementation increment

- Added isolated workspace crate `crates/arsy-contracts` with transport-neutral serde models for principals, tenant scope, resources, authorization requests/decisions, capability grants, two-person approvals, operations and audit events.
- Added structural validation and six unit tests. The contract requires explicit tenant scope, rejects self-approval, requires an idempotency key of at least eight characters, pins stable enum serialization and rejects unknown scope fields.
- This is additive and does not modify existing authentication/session semantics, API routes, database schema, authorization decisions or production data. It does not claim signature verification or policy enforcement.
- CI/build result for the new head is pending verification. If it fails, fix before expanding the integration surface.
