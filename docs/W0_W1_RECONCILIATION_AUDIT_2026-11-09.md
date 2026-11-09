# MW W0/W1 Reconciliation — 2026-11-09

**Canonical repository:** `BRIDGE-ALLAH/MW`  
**Audit branch:** `audit/w0-w1-reconciliation-2026-11-09`  
**Baseline HEAD inspected:** `7db7fa32e879154a437ed1be4b8e85e361305ec1`  
**Audit date:** 2026-11-09  
**Status:** refreshed inventory evidence + provisional architecture reconciliation; not a migration approval.

## 1. Executive result

The current repository is correctly positioned as the canonical MW workspace, but the wider ecosystem is not yet consolidated in code. This pass refreshed the accessible GitHub inventory and inspected representative source READMEs. It did **not** copy, delete, archive, or modify any source repository.

Verified inventory snapshot:

| Scope | Results | What this means |
|---|---:|---|
| Public repositories indexed by GitHub search for `user:bjo163` | 142 (100 on page 1, 42 on page 2) | Public search baseline; not a guarantee of every historical/transferred/deleted repository |
| Separately accessible repository visible to the connected account under `bjo163` | 1: `arah-saas-development` | Private repository metadata visible; README content was not accessible in this pass |
| Visible `BRIDGE-ALLAH` organization repositories | 20 | Organization search returned 20 repositories |
| Total repositories in connected-account listing | 21 | 20 organization repositories plus the one separately accessible `bjo163` repository |

The earlier count of 142 public `bjo163` repositories is confirmed. The prior register should continue to label this as a visibility/search-index baseline, not an exhaustive history of every repository ever owned by the account.

## 2. Source evidence captured in this pass

README snapshots were read from the following source repositories. SHAs are the README blob SHAs at the time of this audit.

| Source | README blob SHA | Verified description / consequence |
|---|---|---|
| `bjo163/moonwitnessxv` | `df30c29080a61fd9465cc68a5a2933c6fc1c3cf5` | MoonWitness XV says ARSY is its existing Fastify + MoonQL Hub; it explicitly distinguishes accepted architecture from production deployment evidence. LUMEN implementation remains planned; ECHOLOG has bounded implementation with gates open; ORBIT accepted/planned. |
| `bjo163/A-L-L-A-H.COM` | `2c0fe5fab04eb5152a9a5a74b3d56d47735abc74` | Unified MoonWitness / Empire ecosystem source; README describes Fastify/TypeScript ARSY, Sentinel RADIUS, GATE, Dock, Headquarters, MoonWitness ID, WORLD OBSERVATORY, MOONSKY and HARBOR. MIT is claimed in README badge; license still needs file/dependency verification. |
| `bjo163/rocksoul` | `343f23436e86dbef527b41d5e24bb1f104bc95a4` | Rust-first intelligence runtime; README describes a seed native CLI/REPL and explicitly frames AGI as research direction, not a current capability claim. Candidate for bounded cognitive runtime, not global policy authority. |
| `bjo163/rocksoul-mftl` | `a70c4e39c835e1f9cd3941356e71d4232c164590` | Provenance-first narrative/belief corpus with STORY × EVENT × PERSON and related domains. Keep evidence lineage and research limitations intact. |
| `bjo163/moonships` | `557543e96de85cffd10b1988a25b1f1c7e15c937` | A distinct Windows-first Tauri 2 + Rust + React/TypeScript workbench. README says initial scaffold only; operational modules remain planned. Do not confuse it with the current MOONSHIPS infrastructure control plane. |
| `bjo163/mwx-ships` | `6a53ecd9889be70371a5ef507f4efe534bf6c319` | Alternate MOONSHIPS PaaS using Rust/Loco/SeaORM, with SQLite-first design. It is a competing implementation lineage, not automatically a replacement for Pier-derived MW. |
| `bjo163/moonsky` | `778829a77f50fccc809f34c35aea235ce56b5e07` | MOONSKY extracted messaging service with its own PostgreSQL schema and local operator identity. It says it is not a Hub and does not own ecosystem identity. This local operator model needs a deliberate federation/SSO decision before integration. |
| `bjo163/AQEL` | `a4ca2e0a40246d6f5ebf8c70e8a2dc9a49428f8c` | Arabic-first language project, pre-release v0.1 direction; separates language surface from semantic core. Treat as a language/compiler domain, not a platform control plane. |
| `bjo163/antikythera-time` | `ce4ffcf737fb0ddd0e4582fe2fa72132310086b2` | M-Time temporal interoperability research prototype; software-only scope, explicit separation of astronomy, calendar criteria, observation and authority. v1 readiness remains blocked on independent reproduction/external review. |
| `bjo163/moonwitness-II-q4q` | `4e2613a04f77f42806b8ee6b42bbd6882965f2b0` | Q4Q Quran evidence explorer with a closed v1 corpus boundary; canonical text, structural metadata and deterministic evidence are separated from claims/interpretations. Supabase is stated runtime source of truth. |
| `BRIDGE-ALLAH/MW-GATEWAY` | `cb2fbac572d6f725c477e60af7d54584c1f92f74` | Agentgateway-derived LLM/MCP/A2A connectivity and policy gateway; README claims Apache-2.0. Treat as bounded connectivity/data-plane candidate, pending upstream diff and security review. |
| `BRIDGE-ALLAH/MW-UI` | `a2e6770668bd29ef404804250b70155fd55f6545` | React/Next/Tailwind/shadcn interface foundation; README says ecosystem-specific design-system integration is not complete. Candidate design system, not a proven replacement shell. |
| `BRIDGE-ALLAH/MW-ORM` | `affd39d058b65e40ed386a219c2d7abd169e867f` | MoonWitness ORM describes modular Rust data layer intended to underpin MW-BAAS. Candidate only; current MW is SQLite/rusqlite-based. |
| `BRIDGE-ALLAH/MW-ID` | `ddf19d8d53a702fa63ce4cb3f422f91b1f15a0fb` | README remains upstream Kanidm identity-platform documentation. Repository name does not establish an MW-native identity implementation. |
| `BRIDGE-ALLAH/MW-STORAGE` | `f17e6cdd0b6c8e8f9ba4f858d70fb66a4de7fc59` | README is RustFS-derived object storage. Treat as an external infrastructure service/fork candidate, not an internal persistence replacement. |
| `BRIDGE-ALLAH/EMPIRE-OF-LIGHT` | `730645050f7ff07b4abe5b3171cbec2f32c82f88` | README explicitly declares Rust-first ARSY and Fastify as migration input. This conflicts with the Fastify-canonical decisions in MoonWitness XV and A-L-L-A-H.COM; architecture must be resolved from source parity, not README preference. |

### Access limitation

`bjo163/arah-saas-development` appears in the connected-account repository listing, but fetching its README returned 404. Do not infer that it is empty or public. Its contents remain inaccessible/unverified from this connection.

## 3. W1 — architecture conflict reconciliation

### 3.1 ARSY / backend authority

**Conflict confirmed.**

- MoonWitness XV and A-L-L-A-H.COM describe Fastify + MoonQL as the existing ARSY Hub.
- EMPIRE-OF-LIGHT says Rust-first ARSY is canonical and Fastify is migration input.
- Current MW is a real Rust/Axum infrastructure operations application, but its current source is Pier-derived; that alone does not prove it already implements all ARSY governance responsibilities.
- The small `BRIDGE-ALLAH/ARSY` repository is a prototype and must not be mistaken for the complete platform.

**Provisional target recommendation:** one ARSY domain authority hosted in the MW ecosystem, with Rust as the intended long-term backend direction. Fastify is a migration/reference source until route, data, identity, tenancy, job, audit, integration and test parity is demonstrated. Do not run competing global hubs. Do not claim the Rust implementation is complete yet.

**Required before ADR approval:** a source-level parity matrix of Fastify ARSY vs Rust/EMPIRE capabilities vs current MW, including schemas/migrations, route contracts, auth/RBAC, tenancy, scheduler/run-state, integrations, security, deployment and regression evidence.

#### Source-level Fastify ARSY security evidence (migration blocker)

A deeper read of `bjo163/A-L-L-A-H.COM` at source commit `bf8681c714b9983861664a05f0c6d9df752f7549` confirms that the LLM gateway is not yet safe to import wholesale:

- `packages/arsy-plugin-llm-gateway/src/index.ts` blob `9587a31bf1407368e43d539ee0f4714a6b2dea2e`: `GET /v1/models` uses unscoped raw SQL for models/routes; `POST /v1/embeddings` invokes upstream providers before a visible tenant check; `GET /v1/mcp/sse` creates anonymous session IDs; `GET /v1/mcp/audit` returns the global in-memory tool log; `GET /v1/gateway/providers` selects provider rows without a tenant predicate.
- `packages/arsy-plugin-llm-gateway/src/agent/loop.ts` blob `c57945b34893211fc06f860aa3fc88d9b4a50f4d`: agent execution calls `resolveRoute('dx-default')` without passing `session.company_id`, despite the session storing that tenant ID.
- `packages/arsy-plugin-llm-gateway/src/router.ts` blob `bf2d018bef0b422949404ee6a88363500ecf7a7d`: router query paths support tenant-scoped visible sets when a company ID is supplied, so missing tenant context at callers is material.
- `.plans/20261105-llm-gateway-tenant-isolation/spec.md` is marked `proposed`; its paired task file is also `proposed`. The source tree contains a FINAL_REPORT describing eight bugs and a remediation plan, but the presence of that report is not proof the fixes were merged or verified.
- `apps/arsy/CONTRACT.md` has an empty `routes: []` declaration and `last_verified: 2026-02-10`; it is not a reliable current route inventory.

**Security interpretation:** source handlers show missing tenant binding and unscoped data access. The full app-level Fastify registration/auth encapsulation has not yet been exhaustively traced, so this audit does not claim that every route is externally unauthenticated. The cross-tenant data-scope gaps and missing tenant propagation are sufficient to block importing this gateway code as-is. Require route-level negative tests (no principal, tenant A, tenant B, platform principal) and verify all parent hooks before making any production exposure claim.

**Disposition:** preserve Fastify ARSY as a high-value behavior/contract migration source, but quarantine the LLM gateway plugin from import until its security tasks are implemented and independently verified. Do not port these defects into Rust.

### 3.2 MOONSHIPS product boundary

There are at least three distinct codebases/experiences:
1. Current `BRIDGE-ALLAH/MW`: Pier-derived Rust/Axum/MiniJinja infrastructure control plane.
2. `bjo163/mwx-ships`: Loco/SeaORM Rust PaaS alternative.
3. `bjo163/moonships`: Tauri desktop developer/infrastructure workbench scaffold.

**Recommendation:** retain current MW as the infrastructure control-plane baseline until capability/security/operations comparison proves otherwise. Treat mwx-ships as a selective capability source, not a wholesale replacement. Treat the Tauri workbench as a separate optional desktop client/product surface, not a competing server control plane.

### 3.3 Frontend

Current MW uses server-rendered Axum/MiniJinja. MW-UI is React/Next-based and states that its MW-specific design integration is unfinished. MoonWitness XV/A-L-L-A-H.COM also contain React/Next application surfaces.

**Recommendation:** keep the existing MW shell as the operational baseline while defining a route-based unified workspace. Reuse MW-UI design tokens/components only after license, build, accessibility, auth and route integration checks. Do not perform a big-bang rewrite or create separate apps by default.

### 3.4 Identity and service boundaries

- MW-ID is a Kanidm-derived identity-provider candidate, not a bespoke MW identity implementation.
- MOONSKY has its own operator identity and PostgreSQL schema; it explicitly does not claim to be a Hub.
- ARSY should own ecosystem authorization policy, tenancy mapping and delegated trust; a specialist IdP may authenticate identities.
- MOONSKY's local operator model must be reconciled with central SSO without moving ownership of the `moonsky` data schema into ARSY.

### 3.5 Data and storage

- Current MW uses SQLite/rusqlite and embedded migrations.
- MW-ORM is SeaORM-derived and intended for MW-BAAS; not adopted by MW.
- MW-STORAGE is RustFS-derived S3-compatible storage candidate.
- Q4Q uses Supabase as its declared runtime source of truth; corpus migration must preserve canonical text and evidence hashes/lineage.

**Recommendation:** do not standardize all domains on a new ORM/database during consolidation. Establish data ownership and versioned APIs first; adopt shared adapters only after representative prototypes and migration/rollback tests.

### 3.6 Domain ownership candidates

| Domain | Candidate owner | Initial status | Boundary |
|---|---|---|---|
| Infrastructure operations | MOONSHIPS | Existing MW implementation | Hosts, apps, containers, DB services, domains, TLS, backups and agents |
| Ecosystem governance/control plane | ARSY | Target architecture; implementation parity incomplete | Identity integration, authorization, tenancy, trust, policy, orchestration, audit |
| Evidence / knowledge | MOONWITNESS | Multiple source repositories | Source lineage, corpus, observations, claims and uncertainty |
| Cognitive runtime | ROCKSOUL | Rust seed runtime exists | Bounded cognition, memory and capability runtime; no global authorization |
| Narrative corpus | MFTL / RGBL / AWS | Source repository exists | Story/event/person/text/law evidence and research provenance |
| Messaging | MOONSKY | Extracted service README describes bounded service | Owns messaging schema/transports; integrate identity through contract |
| Language/compiler | AQEL | Pre-release | Arabic-first language surface and deterministic semantic core |
| Temporal systems | M-Time / Antikythera | Research prototype | Temporal computation and explainable calendar/astronomy layers |
| Gateway | MW-GATEWAY and ROCKSOUL-WX | Competing candidates | Compare protocol data plane vs workload planning/policy facade |
| Object storage | MW-STORAGE | RustFS-derived candidate | S3 object storage service; not canonical relational DB |
| Identity provider | MW-ID | Kanidm-derived candidate | Authentication/identity provider only after provenance and ops review |
| THE BATTERIES | Unknown | Unresolved | No technical scope assigned without source evidence |
| ZILL / ZILL WATCH | Partially referenced | Unresolved | MoonWitness XV references ZILL WATCH as a module; product/domain ownership remains unverified |

## 4. Import and archive decision rules

No repository is approved for wholesale import or archival by this pass.

Before importing any source:
1. Pin source commit SHA and capture recursive tree.
2. Verify LICENSE, NOTICE, source headers, dependencies, assets, trademarks and upstream ancestry.
3. Run secret and sensitive-data scans on current tree and history strategy.
4. Map unique capability to one MW owner and identify overlapping implementations.
5. Exclude credentials, private keys, production data, `.env`, and generated artifacts unless explicitly safe and reproducible.
6. Add source-to-destination provenance ledger and deterministic tests.
7. Build/test in an isolated branch; retain source repository until behavior and provenance are verified.

## 5. W0/W1 task status after this pass

| Task | Status | Evidence / remaining gate |
|---|---|---|
| W0.01 repository search inventory | **Partially verified** | 142 public bjo163 search results; 20 BRIDGE-ALLAH repos; one separate accessible bjo163 private repo. Need pagination recheck and historical/transferred repo caveat. |
| W0.02 repository metadata snapshot | **Open** | This pass did not yet produce full machine-readable metadata for every repo (latest SHA, releases, license, workflows, issues, LFS/submodules). |
| W0.03 recursive source/documentation audit | **Open** | Representative README checks only; priority source trees and implementation/tests remain to inspect. |
| W0.04 unresolved identities | **Open** | THE BATTERIES and ZILL need code/documentation searches across all accessible repositories, not only repository-name search. |

#### W0.04 exact-term search update
Code-search queries were run against `bjo163/moonwitnessxv`, `bjo163/A-L-L-A-H.COM`, `bjo163/rocksoul`, `bjo163/rocksoul-mftl`, `bjo163/moonwitness-corpus`, and `bjo163/moonwitness-reborn`:
- `THE BATTERIES` / `THEBATTERIES`: no relevant project hit. One A-L-L-A-H.COM match was an operating-system PowerShell flag `-DontStopIfGoingOnBatteries`, a false positive.
- `ZILL WATCH` / `ZILL`: no relevant exact project hit in MoonWitness XV or MoonWitness Reborn. Some corpus hits were substring matches inside unrelated transliterations such as `Al-Mudzill`, not the ZILL project identity.
- Existing audit evidence that MoonWitness XV references ZILL WATCH as a module remains a historical lead, not enough to assign a current product owner or technical scope.

**Disposition:** both identities remain unresolved. Search-index absence is not proof of nonexistence; continue path-level and historical commit searches before deciding whether either is a product, module, campaign, dataset, or retired name.
| W1.01 ARSY implementation comparison | **Open — blocker** | README-level conflict confirmed; route/schema/security parity audit required. |
| W1.02 MOONSHIPS implementation comparison | **Open** | Pier/Axum vs Loco/SeaORM vs Tauri workbench source comparison required. |
| W1.03 platform capability ownership | **Provisional** | Candidate matrix in this document; needs ADR and API/schema ownership map. |
| W1.04 runtime/frontend boundary | **Provisional** | Keep MiniJinja shell baseline; prototype React route integration only after route/auth inventory. |

## 6. Next execution wave

1. Generate complete JSON/CSV metadata snapshot for all accessible `bjo163` public results and all visible `BRIDGE-ALL` repositories.
2. Run recursive tree + source SHA capture for the highest-priority sources: `A-L-L-A-H.COM`, `moonwitnessxv`, `EMPIRE-OF-LIGHT`, `rocksoul`, `mwx-ships`, `MOONWITNESS`, `MW-GATEWAY`, `ROCKSOUL-WX`, `moonsky`, `AQEL`, `antikythera-time`.
3. Build ARSY route/schema/security parity matrix before selecting the final implementation boundary.
4. Draft ADR-0002 (ARSY) and ADR-0003 (Pier vs Temps vs Loco capability selection) only after source evidence is complete.
5. Keep all source repositories intact until import and operational parity are proven.

**No production deployment, customer data migration, destructive repository cleanup, or source-repository archival is authorized by this audit.**
