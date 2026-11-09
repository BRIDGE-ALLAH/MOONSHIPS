# Ecosystem Migration Matrix — 2026-11-09

**Status:** Target mapping for planning; no source code is approved for import by this document.  
**Rule:** Source SHA + legal/security review + one owner + contract + tests before any code/data migration.

## 1. Target workspace layout

The following are proposed module boundaries, not existing directories. Do not create empty crates solely to make the tree resemble a diagram.

| Target boundary | Current/source owner | Migration mode | First useful increment | Required proof |
|---|---|---|---|---|
| Existing MW runtime (pier-core, pier-agent, pier-net-helper) | BRIDGE-ALLAH/MW | Retain; incremental refactor only | Preserve current infrastructure routes and operational compatibility | Existing build/tests, clean install, upgrade and rollback, agent compatibility |
| ARSY contracts and policy | MOONWITNESS ARSY + A-L-L-A-H.COM Fastify source; MW current auth/RBAC baseline | Re-implement/adapt into Rust, not wholesale copy | Typed principal, tenant, permission, policy-decision and audit-event contracts | Route-to-policy matrix; no-principal/tenant-A/tenant-B/platform-principal negative tests |
| ARSY operations engine | MOONWITNESS deployment operations/outbox; mwx-ships durable queue patterns | Selective adaptation | Durable operation ID, idempotency key, lease/fencing, retry, cancellation and dead-letter contract | Crash recovery, duplicate delivery, stale lease, cancellation and replay tests |
| MOONSHIPS infrastructure domain | Current MW + selected EMPIRE/mwx-ships capabilities | Retain current runtime; selectively adapt | Inventory and capability map for server/app/container/database/deployment flows | Existing user journey parity, scoped agent authorization, migration compatibility |
| MOONWITNESS evidence | MOONWITNESS / MoonWitness XV | Selective domain import | Source, observation, claim, evidence, uncertainty and review contracts | Provenance integrity, append-only audit, source-rights and tenant tests |
| RGBL exact corpus | bjo163/moonwitness-corpus | Preserve as domain owner; integrate by API/contract | Stable work/edition/passage/content identifiers and import/export contract | Text hash, edition separation, rights metadata, corpus validation |
| MFTL narrative | bjo163/rocksoul-mftl | Preserve as domain owner; integrate by API/contract | STORY/claim/source/evidence model and cross-domain references | No duplicate event/person/text owners; evidence and counterevidence tests |
| ROCKSOUL cognition | bjo163/rocksoul | Bounded crate/process after contract review | Model adapter, bounded planner, explicit memory and verifier interface | Model output cannot authorize; no implicit tool execution; memory consent/retention tests |
| MOONSKY messaging | bjo163/moonsky | Keep independent service initially; later federate | Machine-client contract and tenant/channel ownership mapping | OIDC/federation, webhook replay, credential rotation, WebSocket auth, break-glass |
| AQEL language | bjo163/AQEL | Research/spec import only | Versioned grammar/semantic spec and compiler conformance fixtures | Deterministic compile/parse tests, source attribution, language review |
| M-Time | bjo163/antikythera-time | Research module/service with strict semantics | Versioned temporal value, timescale, observer, uncertainty and profile contracts | Reference comparison, calibration/falsification, external independent reproduction |
| ZILL WATCH | MOONWITNESS HEADQUARTER module | Route/module adaptation after ARSY contracts | Read-only inspection contract for rights, quality and policy findings | Source-backed findings, reviewer audit, no synthetic/live confusion |
| THE BATTERIES | MOONWITNESS narrative/source catalog | Do not implement yet | Resolve originating story/source and intended referent | Owner-confirmed source evidence and domain brief |
| Gateway | MW-GATEWAY, ROCKSOUL-WX, legacy Fastify plugin | No selection yet | Gateway-neutral contract and threat model | Tenant isolation, SSRF/DNS rebinding, TLS, egress, secrets, rate-limit and protocol tests |
| Identity provider | MW-ID/Kanidm candidate | No selection yet | OIDC provider comparison and federation contract | Recovery, key rotation, MFA, SCIM/group mapping, license and operational review |
| Persistence adapter | MW-ORM/SeaORM candidate | Prototype only | One representative domain adapter alongside current rusqlite | SQLite/Postgres parity, tenant scoping, migrations, transactions, rollback |
| Object storage | MW-STORAGE/RustFS candidate | Separate service only | S3-compatible adapter contract and local test target | S3 conformance, IAM, encryption, retention, restore and failure tests |
| Shared UI system | MW-UI candidate + current MW MiniJinja | Selective token/component extraction | Design tokens and accessible primitives | License/dependency review, keyboard/screen-reader tests, visual regression |
| Native workbench | bjo163/moonships Tauri app | Optional client | Authenticated API client against MW/ARSY contracts | Tauri capability audit, native-side authorization, signed update and secure storage |

## 2. Dependency order

1. **Governance and provenance:** source registry, source SHA pinning, license/NOTICE/SBOM, secret exclusions, import manifest.
2. **Cross-domain contracts:** IDs, tenant context, error envelope, event envelope, audit/evidence references, API versioning.
3. **ARSY trust foundation:** principal, tenant, policy, delegation, service identity, authorization decision.
4. **ARSY operation lifecycle:** durable operations, idempotency, leases, retries, cancellation, recovery, audit/outbox.
5. **MOONSHIPS compatibility:** map current infrastructure routes/resources into the contracts without changing existing runtime semantics.
6. **Evidence and corpus domains:** MOONWITNESS, RGBL and MFTL adapters with immutable provenance and owner-specific stores.
7. **Specialist domains:** ROCKSOUL, MOONSKY, AQEL and M-Time, each behind versioned contracts.
8. **Gateway, storage and UI adapters:** only after security, upstream and legal review.
9. **Product integration:** one shell/navigation, truthful maturity states, role-aware route exposure.
10. **Retirement:** only after destination tests, restore, rollback, link migration and provenance retention.

## 3. Mandatory import manifest

Every imported component must record:
- source repository, exact commit/tag and original path;
- destination path and domain owner;
- upstream project, fork/base commit and license identifier;
- retained LICENSE/NOTICE/copyright/trademark files;
- local modifications and transformation script;
- excluded paths (secrets, env files, generated assets, private data);
- dependency/SBOM and advisory result;
- build, unit, integration, security and license test commands;
- destination commit, reviewer, rollback path and migration state.

## 4. Explicit no-go rules

- No Fastify LLM gateway import until tenant propagation and route-level access tests pass.
- No EMPIRE-OF-LIGHT wholesale import; preserve Temps lineage and select bounded capabilities.
- No switch from rusqlite/SQLite without migration, restore and rollback evidence.
- No wholesale Next.js rewrite of the MW shell.
- No MOONSKY account/schema merge before federation and operator recovery are proven.
- No automatic ROCKSOUL tool execution or model-controlled memory mutation.
- No ZILL WATCH production claim while its rollout register says prototype.
- No THE BATTERIES implementation before the originating referent is confirmed.
- No source repository archive/delete based on inventory status alone.
