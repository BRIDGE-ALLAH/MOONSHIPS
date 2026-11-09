# Deep Source Audit — Platform, ARSY, Gateway, Data and UI

Audit date: 2026-11-09. This is a source-tree and selected-file audit at the exact default-branch commits listed below. It is not a complete security review and no builds/tests were executed in this pass.

## 1. Pinned source snapshots

| Repository | Default branch | HEAD SHA | Last commit date (UTC) | GitHub license metadata | Tree entries |
|---|---|---|---|---|---:|
| `BRIDGE-ALLAH/MW` | main | `4706e20fc95c78a95129b117ae187511ce0fb98c` | 2026-11-09 12:48 | AGPL-3.0 | 473 |
| `BRIDGE-ALLAH/EMPIRE-OF-LIGHT` | main | `2f1e92cd91f22372875409266cd7e2ddcd96c553` | 2026-11-06 04:33 | Apache-2.0 | 8,878 |
| `BRIDGE-ALLAH/MOONWITNESS` | main | `8f9a5de38c39ea82388178effefead06af4e2c8b` | 2026-11-09 10:47 | not declared in repository metadata | 975 |
| `BRIDGE-ALLAH/MW-GATEWAY` | main | `8b2075ffebca7c7e043f2fe06ffd0dc6fec37200` | 2026-11-06 00:08 | Apache-2.0 | 3,438 |
| `BRIDGE-ALLAH/ROCKSOUL-WX` | main | `d1b0217cf1a1071b2db378acf396069e511f44ec` | 2026-11-08 20:11 | no license selected (README) | 68 |
| `BRIDGE-ALLAH/MW-ORM` | master | `cfeeba8eb4a3e19f5ff42afc7bb07ff9c9674ba0` | 2026-11-06 00:03 | Apache-2.0 in repository metadata | 2,071 |
| `BRIDGE-ALLAH/MW-ID` | master | `8b5382f2037bcaf2bf84cb67e7e6353d73de173a` | 2026-11-05 03:26 | MPL-2.0 in repository metadata | 1,476 |
| `BRIDGE-ALLAH/MW-STORAGE` | main | `38468dd43cbf7b5135724350915c83f3f8043f97` | 2026-11-06 02:06 | Apache-2.0 | 4,019 |
| `BRIDGE-ALLAH/MW-UI` | main | `0916123ceebb59b78ceafe0bebe9f3d23585899d` | 2026-11-06 01:58 | MIT | 7,124 |
| `BRIDGE-ALLAH/MW-BAAS` | master | `3ddc279740ebf0516403ca8f7028ce481d616a02` | 2026-11-05 09:33 | Apache-2.0 in repository metadata | 1,476 |
| `BRIDGE-ALLAH/MW-IDE` | main | `b4776e46cb2d8016fe25f5389d577a7e895e734b` | 2026-11-08 04:24 | Apache-2.0 | 444 |

Tree counts are GitHub recursive-tree entries at the pinned default branch, not counts of source files or lines. All trees above were returned without truncation.

## 2. MW current implementation baseline

Pinned source: `BRIDGE-ALLAH/MW@4706e20fc95c78a95129b117ae187511ce0fb98c`.

Observed code structure includes:
- `crates/pier-core/src/auth`: password, cookie/session, API tokens, server tokens, setup tokens, federation, rotation, TOTP, audit, middleware and RBAC.
- `crates/pier-core/src/db/schema.rs`: embedded ordered SQLite migrations and `_migrations` version table.
- `crates/pier-core/src/auth/rbac/policy.rs`: central `can(user, permission, conn)` function with global roles, project memberships and peer-authenticated permission handling.
- `crates/pier-core/src/auth/federation.rs`: federation token prefix `pier_fed_`, random 24-byte entropy, hash-only storage, dedicated header and middleware restricted to the agent federation surface.

Important implementation caveat:
- RBAC comments explicitly state that project-scoped permission variants are wired incrementally and the enum currently carries vocabulary beyond the guards already shipped. Do not interpret the full Permission enum as proof every route enforces every project permission.
- Peer-authenticated requests are Admin-equivalent for a bounded list of resource operations, but not user-management or federation permissions. Route-level call-site coverage must be tested.
- The migration runner records ordered integer versions and applies pending SQL sequentially. Any import must preserve existing database upgrade semantics and must not renumber historical migrations.
- This pass did not re-run the previous observation about TLS certificate validation in all federation HTTP clients; that remains a high-priority explicit security check before any federation integration.

Disposition: retain MW as the runtime and infrastructure baseline. Add new domains through explicit modules/contracts and tested migrations; do not replace current auth/database/session mechanisms wholesale.

## 3. MOONWITNESS ARSY: implementation maturity and useful migration assets

Pinned source: `BRIDGE-ALLAH/MOONWITNESS@8f9a5de38c39ea82388178effefead06af4e2c8b`.

Recursive tree shows a nontrivial ARSY package with:
- migrations `001` through `009`, covering evidence events, Docker connector, deployment operations, deployment authorization, outbox leases, product/assets/channels/listings, login attempts, payout lifecycle/FX and governance snapshots;
- unit and integration tests for deployment authorization, deployment operations, outbox, Docker connector, workload adapter, evidence, governance, login throttling, finance, assets, channels and listings;
- PostgreSQL-specific integration tests and migration helpers.

Selected migration evidence:
- `004_arsy_deployment_authorization.sql` defines composite owner-scoped foreign keys for operation, Docker server, capability grant and approval evidence. Approval records enforce `approver_id <> requested_actor_id` and operation uniqueness per owner. Capability grants, revocations, approvals and authorization decisions are guarded by append-only update/delete rejection triggers.
- `005_arsy_deployment_outbox_leases.sql` introduces lease fencing tokens, bounded attempts (1–25), terminal dead-letter timestamps and a partial index over available undelivered events. Its comments specify at-least-once delivery and stable outbox IDs as idempotency keys.
- Source app composition includes PostgreSQL pool lifecycle and route groups for Docker, deployments, evidence, governance, products, assets, channels/listings, inspections, financial operations and ORACLE integration.

These are useful behaviors and data-integrity patterns to preserve. But the whole Fastify/PostgreSQL application must not be imported as a second ARSY authority. The destination should be a set of bounded MW domains and contracts, with each data set assigned one owner.

### ARSY migration mapping (provisional)

| Source capability | Target MW destination | Migration rule |
|---|---|---|
| Operator identity/token | ARSY identity integration | Map to canonical identity; preserve machine credential separation and rotation |
| Capability grants/approvals | ARSY authorization domain | Port semantics, signature verification, tenant scoping and separation-of-duties tests |
| Deployment operations/outbox | MOONSHIPS operations domain | Preserve idempotency, lease fencing, retry and dead-letter semantics |
| Docker connector | MOONSHIPS provider adapter | Preserve read-only connector boundary and distinct mutation authorization |
| Evidence/governance | MOONWITNESS evidence domain / ARSY audit contracts | Preserve append-only guarantees, provenance, source and tenant scope |
| Product/assets/channels/listings | Product-specific domain modules | Do not force infrastructure semantics onto commercial/content entities |
| Finance/payout/FX | Dedicated finance bounded context | Keep separate from infrastructure resource tables and privileged operations |

No migration is approved by this table. It is a planning map requiring full route, schema and data ownership review.

## 4. EMPIRE-OF-LIGHT: platform lineage and import risk

Pinned source: `BRIDGE-ALLAH/EMPIRE-OF-LIGHT@2f1e92cd91f22372875409266cd7e2ddcd96c553`.

Root Cargo manifest confirms:
- workspace has a very large Temps-derived crate set, including auth/authz, database/migrations, queue, workload, deployment, Docker, backup, providers, networking, observability, analytics, AI gateway, MCP, memory, sandbox and agents;
- workspace package metadata still names `Temps Contributors`, version `0.1.0-beta.60`, `https://github.com/gotempsh/temps`, and `MIT OR Apache-2.0`;
- root GitHub repository metadata says Apache-2.0, while Cargo workspace declares dual MIT OR Apache-2.0. File-level notices and dependency licenses must be reconciled.

The 8,878-entry tree and broad workspace mean EMPIRE is not a small ARSY-only module. It is a second platform lineage. Importing it wholesale would create substantial duplication and build/dependency burden with MW's Pier baseline.

Disposition:
- Preserve EMPIRE as a high-value platform capability reference and source lineage.
- Build a capability-by-capability comparison (deployments, agents, queue, database, authz, networking, backup, providers, observability, AI/MCP).
- Import only bounded, missing capabilities after provenance, license and operational compatibility are verified.
- Do not rename inherited Temps crates or strip attribution as part of consolidation.

## 5. Gateway comparison: MW-GATEWAY versus ROCKSOUL-WX

### MW-GATEWAY

Pinned source: `BRIDGE-ALLAH/MW-GATEWAY@8b2075ffebca7c7e043f2fe06ffd0dc6fec37200`.
- Apache-2.0 license file exists; workspace contains agentgateway, HTTP, LLM, policy, CEL, xDS and protocol crates.
- Tree includes extensive controller and policy tests.
- Test harness tree includes `dummy-idp.cert` and `dummy-idp.key`. They appear in a testbox fixture path; still inspect and confirm the private key is synthetic, never reused, excluded from release images and not trusted outside tests. Do not treat mere presence as evidence of a production secret leak, but add secret-scanner validation.
- Multiple Git-pinned dependency patches are present in the workspace manifest. These need reproducible provenance and update policy.

### ROCKSOUL-WX

Pinned source: `BRIDGE-ALLAH/ROCKSOUL-WX@d1b0217cf1a1071b2db378acf396069e511f44ec`.
- Small Rust 2024 workspace with typed workload contracts, deterministic policy, planner, provider, router, Axum gateway and Dioxus command deck.
- Policy source explicitly fails closed for confidential/restricted workloads: no public cloud, no raw-data egress, requires local/private execution.
- README explicitly says it is single-operator, not multi-tenant; rate limits are in-memory per shared gateway credential; no distributed quotas; no automatic provider failover; max-cost is rejected because pre-execution reservation/settlement is absent; human approval is blocked until a workflow exists.
- README says no license has been selected.

Disposition:
- MW-GATEWAY is the stronger candidate for protocol/data-plane connectivity after upstream license/security review.
- ROCKSOUL-WX is a valuable small policy/workload-contract reference, but must not be deployed as a multi-customer control plane or imported while licensing is unresolved.
- If both patterns are reused, define policy ownership once in ARSY; gateway enforces a signed/validated policy decision and reports evidence, rather than inventing a parallel identity authority.

## 6. ORM, identity, storage and UI: verified lineage

### MW-ORM
Pinned source: `MW-ORM@cfeeba8eb4a3e19f5ff42afc7bb07ff9c9674ba0`.
- Cargo package is still named `sea-orm`, repository points to SeaQL/SeaORM, version 2.0.4, license MIT OR Apache-2.0, Rust 1.94 requirement.
- Workspace adds a `moonwitness-module` member, but this does not establish MW-specific ORM ownership or production adoption.
- Evaluate in a bounded prototype against current SQLite/rusqlite before adoption; verify local diff, upstream commit and migration semantics.

### MW-ID
Pinned source: `MW-ID@8b5382f2037bcaf2bf84cb67e7e6353d73de173a`.
- GitHub metadata identifies MPL-2.0; tree and README indicate Kanidm identity platform.
- Root LICENSE file was not found at the inspected path. Legal provenance is therefore not fully verified even though metadata has a license value.
- Candidate for an identity provider, not a reason to move all authorization policy out of ARSY.

### MW-STORAGE
Pinned source: `MW-STORAGE@38468dd43cbf7b5135724350915c83f3f8043f97`.
- RustFS-derived object storage; Apache-2.0 license file includes RustFS copyright notice.
- Tree includes S3 compatibility, fault tolerance, healing, KMS, replication, upgrade, security and migration test workflows.
- Candidate as an independently operated object-storage service; not an embedded MW database or default requirement for a single-node install.

### MW-UI
Pinned source: `MW-UI@0916123ceebb59b78ceafe0bebe9f3d23585899d`.
- MIT metadata; app/v4 is Next 16.4.2 + React 19.2.3 + Tailwind 4 + shadcn/Base UI ecosystem with a broad AI UI/developer tooling dependency set.
- It is a substantial UI framework source, not just a small component library. Importing the whole application would impose a separate frontend runtime and dependency lifecycle.
- Prefer extracting reviewed design tokens, accessibility patterns and selected components only if a concrete MW need is demonstrated. Keep current MiniJinja shell until an ADR and migration tests justify change.

### MW-BAAS and MW-IDE
- MW-BAAS root README remains Loco upstream documentation; it is not currently verified as a bespoke MW backend-as-a-service product.
- MW-IDE tree is a Lapce-derived Rust/Floem editor with Apache-2.0 metadata and 444 entries. It is a specialist desktop product candidate, not a dependency of the server control plane.

## 7. Legal, security and operational blockers

1. ROCKSOUL-WX has no selected license in its README: no source import or redistribution.
2. MOONWITNESS repository metadata does not declare a license in the inspected API response: obtain explicit rights/provenance before copying.
3. MW-ID metadata says MPL-2.0 but the root LICENSE path was not found: resolve actual license file and notices before reuse.
4. EMPIRE has dual-license Cargo metadata and repository Apache-2.0 metadata, plus inherited Temps notices: verify file-level licensing and dependency notices.
5. MW-GATEWAY includes test IDP certificate/key fixtures: verify synthetic-only status and secret scanning.
6. MW current federation TLS validation remains a priority verification item before any cross-repository federation code is adopted.
7. MOONWITNESS deployment authorization and outbox designs are valuable but require full call-site and integration tests, not just schema inspection.
8. No source repo CI was run by this audit. Tree presence of tests/workflows is not a pass result.

## 8. Next required work

- Resolve legal rights for MOONWITNESS, ROCKSOUL-WX and MW-ID before source import.
- Pin upstream base commits and produce fork-diff summaries for MW-GATEWAY, MW-ORM, MW-ID, MW-STORAGE, MW-UI, MW-IDE, MW-BAAS and EMPIRE.
- Complete route/schema/authz/tenant parity matrix for MOONWITNESS ARSY and current MW.
- Inspect current MW federation clients for TLS validation, DNS rebinding/SSRF, credential scope, token revocation and WebSocket token handling.
- Inspect gateway dummy test key, CI secret scanning, release packaging and SBOM provenance.
- Run candidate CI/test suites on exact pinned SHAs in isolated builds before proposing integration.
- Draft ADRs only after evidence; do not ratify architecture based on README declarations alone.

## 9. Safety constraints

No production code imported, no migrations applied, no source repository modified, no deployment run, and no repository archived or deleted in this audit. Keep all source history and attribution intact.
