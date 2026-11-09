# Priority Source-Tree Audit — 2026-11-09

**Target:** BRIDGE-ALLAH/MW  
**Audit branch:** audit/w0-w1-reconciliation-2026-11-09  
**Method:** GitHub recursive default-branch trees plus selected architecture, security, manifest, migration and feature-rollout documents.  
**Limit:** This is a priority source-tree audit, not a complete line-by-line review, build, penetration test, license opinion or deployment verification.

## 1. Snapshot and source-tree evidence

All listed recursive trees returned without truncation. Tree SHAs identify the snapshot reviewed; they are not claims that every file was semantically audited.

| Source | Default-branch tree SHA | Entries | Observed structure / disposition |
|---|---|---:|---|
| bjo163/moonwitnessxv | 1b5c1b717b68db8ca7c06fa3dc5cc392edc9257d | 2,364 | Large TypeScript monorepo with apps, packages, CI, ADRs, security contracts and migration plans. ARSY/Fastify is a migration reference; many app surfaces are explicitly parked, scaffolded or gated. |
| bjo163/A-L-L-A-H.COM | bf8681c714b9983861664a05f0c6d9df752f7549 | 1,465 | Fastify/TypeScript ARSY source, LLM gateway plugin and numerous quarantined/proposed plans. Do not equate proposed remediation with merged fixes. |
| bjo163/rocksoul | 6da18d7e2144136e02d7a7ff67577b49112b59f0 | 67 | Small Rust workspace with CI, architecture, cognitive-core, evaluation and security/sandbox plans. Bounded cognition candidate, not global authority. |
| bjo163/rocksoul-mftl | 1519de291267eaa90960bcb04f7ac94e325d9d35 | 127 | Provenance-first narrative corpus with data model, integrity, research and public/operator boundaries. Keep STORY separate from exact corpus, event and person ownership. |
| bjo163/moonships | bb4c5f6ea2b137dd42d9274f87d1c4604582c7a3 | 240 | Tauri 2 + Rust + React/Vite desktop workbench; architecture and security model are proposed baseline; package scripts include tests/typecheck/build. Separate client experience, not current server shell. |
| bjo163/mwx-ships | f5c4ef8694bf13e1c46c8eb6469da2d71312caec | 287 | Rust/Loco/SeaORM PaaS with explicit SQLite-first ADRs, Docker/SSH deployment and JWT/localStorage auth. Its authorization doc is single-admin MVP; not suitable as-is for ecosystem multi-tenancy. |
| bjo163/moonsky | 1f9884bcffc9daa2fa69595278bf8b41f2c0e0b2 | 80 | Standalone messaging service, own PostgreSQL migrations/schema, operator identity, Telegram/WhatsApp adapters and CI. Its accepted ADR explicitly rejects ARSY dependency for operator login. |
| bjo163/AQEL | 80e57c2441339597ab7ae69f674ef168f2e671d9 | 3 | Very small pre-release language/spec surface; do not represent as a production compiler/runtime. |
| bjo163/antikythera-time | 7cfd4a585617795ad231bbca5628fa943159bdea | 212 | Multi-crate Rust temporal research project with extensive calibration, reference, compatibility and history-guard workflows. Charter excludes redefining SI/UTC/TAI or claiming standards authority; v1 needs independent reproduction and review. |
| BRIDGE-ALLAH/MW-GATEWAY | 8b2075ffebca7c7e043f2fe06ffd0dc6fec37200 | 2,963 | Large agentgateway-derived Rust/Go/gRPC gateway tree. Treat as a separate upstream-derived candidate, not a drop-in small library. |
| BRIDGE-ALLAH/MW-UI | 0916123ceebb59b78ceafe0bebe9f3d23585899d | 6,413 | Large React/Next/Tailwind/shadcn UI framework tree. Design-system source candidate; not an already-integrated MW component package. |
| BRIDGE-ALLAH/MW-ORM | cfeeba8eb4a3e19f5ff42afc7bb07ff9c9674ba0 | 1,651 | SeaORM-derived workspace/fork. Candidate persistence adapter; current MW runtime remains rusqlite/SQLite. |
| BRIDGE-ALLAH/MW-ID | 8b5382f2037bcaf2bf84cb67e7e6353d73de173a | 1,264 | Kanidm-derived identity platform source and docs. Repo name does not prove bespoke MW identity logic. |
| BRIDGE-ALLAH/MW-STORAGE | 38468dd43cbf7b5135724350915c83f3f8043f97 | 3,372 | RustFS-derived object storage platform. Evaluate as separate S3-compatible service, not as relational database or embedded filesystem replacement. |
| BRIDGE-ALLAH/EMPIRE-OF-LIGHT | 2f1e92cd91f22372875409266cd7e2ddcd96c553 | 7,690 | Large Temps-derived platform workspace with Rust services, React frontend, tests and operational workflows. It is not a small standalone ARSY implementation. |

## 2. Canonical MW runtime baseline

The current MW workspace manifest declares four crates: pier-core, pier-agent, pier-net-helper and pier-tests. It is Rust 2021, version 1.5.3, AGPL-3.0, with Axum 0.8.8, rusqlite 0.38 (bundled SQLite), MiniJinja 2.19 and embedded static assets. SQLx Postgres/MySQL and Redis are present for the optional database-browser capability; their presence does not make them the primary application datastore.

**Disposition:** retain this working runtime as the infrastructure control plane baseline. Preserve Pier-compatible data paths, services, helper sockets, environment variables and agent protocols until a tested migration proves compatibility. Do not rename internal runtime identifiers as part of a visual brand change.

## 3. Material source findings

### 3.1 MOONSHIPS variants are different artifacts

- Current MW is the Linux self-hosted infrastructure operations server and existing operational baseline.
- mwx-ships is an alternate Loco/SeaORM PaaS. Its accepted ADRs describe SQLite-first deployment state and durable queue; however its authorization model is explicitly single-admin MVP, and its documented frontend stores JWT in localStorage. Reuse selected patterns only after security adaptation and parity tests.
- moonships is a Windows-first Tauri desktop workbench with React 19/Vite. Its architecture explicitly calls for one desktop process and modular Rust, with Tauri native-side authorization. Keep it as an optional operator client/workbench, not a replacement for the MW server or its main browser shell.

### 3.2 MOONSKY owns its current operator identity

MOONSKY ADR-0001 is accepted: standalone deployment, own PostgreSQL schema, own operator identity, no ARSY session probe, and a scoped machine service key. This is a real source decision, not merely an old README statement.

**Target integration:** shared ARSY-backed ecosystem SSO is the desired experience, but it requires a separately reviewed OIDC/federation ADR, account mapping, migration and rollback. Preserve a narrowly scoped break-glass path for service recovery; never silently merge the MOONSKY schema or credentials into another domain.

### 3.3 ROCKSOUL is bounded cognition, not an autonomous system authority

The current ROCKSOUL cognitive-core contract says planning and reflection are model calls, conversation history is session-local, and reflection is not independent verification. It explicitly says no shell executor, arbitrary function tools, plugin marketplace or autonomous filesystem mutation is enabled. Persistent memory is explicit and user-controlled; automatic ingestion/injection is disabled.

**Target integration:** keep ROCKSOUL behind scoped ARSY grants and deterministic verifier contracts. Model output cannot authorize itself or be treated as evidence.

### 3.4 MFTL and RGBL must remain separate owners

MFTL's data model says it stores what a source says before what an assessor concludes. MFTL owns narrative comparison and downstream assessment; RGBL owns exact corpus text, work/passage identity and source provenance. LEGEND owns canonical event records; SUPERHERO owns canonical person/transmission records. These are cross-domain references, not permission to duplicate mutable canonical registries.

### 3.5 M-Time is a bounded temporal protocol/research domain

Its charter explicitly says it is not a universal absolute clock, does not redefine SI/UTC/TAI/IAU/IERS standards, and does not choose a theology or fiqh profile as scientifically true. Keep physical measurements, models, calendar criteria, observation and jurisdiction/authority decisions distinct. No v1/standard claim without the stated independent reproduction and external reviews.

### 3.6 ZILL WATCH is verified as a HEADQUARTER module; THE BATTERIES is not a verified product yet

Search in BRIDGE-ALLAH/MOONWITNESS at source commit 8f9a5de38c39ea82388178effefead06af4e2c8b found:
- apps/headquarter/src/app/zill-watch/page.tsx;
- apps/headquarter/src/shell/navigation.ts;
- apps/headquarter/FEATURE-ROLLOUT.md;
- docs/17-glossary.md and docs/32-universe-and-product-architecture-registry.md;
- packages/policy/src/index.ts with ZILL policy gates.

The source calls ZILL WATCH a quality, rights and policy inspection module inside HEADQUARTER, not a separate application. Its rollout register classifies it as **prototype**; the inspection contract, rights/policy enforcement and review audit are production gates. A changelog also records removal of synthetic inspection records. Do not market it as a live inspection service.

Search for THE BATTERIES in the same MOONWITNESS organization finds a narrative/source-catalog reference alongside IsRockFeel and fictional characters, not a verified technical product/repository. The intended referent and relationship are unresolved. Preserve the exact spelling in the narrative/source catalog; do not assign a platform capability or build a module from the name alone.

## 4. Import disposition

| Candidate | Disposition | Gate before any import |
|---|---|---|
| Current MW / Pier lineage | Retain as canonical runtime baseline | Compatibility and existing CI/security gates |
| MoonWitness XV / MOONWITNESS | Domain source; selectively map capabilities | Route/tenant/security parity, data ownership, license, exact-SHA regression |
| A-L-L-A-H.COM Fastify ARSY | Migration/reference source; gateway import blocked | Tenant-isolation fixes, route/auth trace, negative tests, full parity |
| EMPIRE-OF-LIGHT / Temps | Selective capability source only | Upstream diff, license/NOTICE, overlap and capability tests |
| ROCKSOUL | Bounded cognitive domain candidate | Explicit memory/privacy policy, verifier integration, sandbox and deny-by-default tests |
| MFTL / RGBL | Preserve separate evidence/narrative and exact-text owners | Stable IDs, source rights, corpus validation and import/export tests |
| MOONSKY | Preserve standalone messaging owner; plan federation | OIDC/service identity, tenant mapping, WebSocket authorization, webhook replay, credential rotation |
| AQEL | Research/spec domain | Compiler specification, deterministic conformance suite and license review |
| M-Time | Research/temporal domain | Independent reproduction, calibration, uncertainty and external review |
| MW-GATEWAY / ROCKSOUL-WX | Gateway candidates; no canonical selection yet | Upstream provenance, security review, auth/policy integration, protocol tests |
| MW-UI | Design-system source candidate | License/dependency audit, token extraction, accessibility and route integration |
| MW-ORM | Persistence adapter candidate | Representative-domain prototype, SQLite/Postgres migration and tenant tests |
| MW-ID | Identity-provider candidate only | Fork/provenance, operations, security, OIDC and recovery review |
| MW-STORAGE | Separate object-storage service candidate | S3 conformance, IAM, retention, encryption, backup/restore and recovery |
| moonships desktop | Optional native client | Threat model, Tauri capability review and native authorization tests |
| mwx-ships | Selective patterns only | Multi-tenant auth redesign, migration and deployment parity |

## 5. What this audit does not prove

- No clean builds, tests, container builds, deployment, restore drills or penetration tests were executed by this source-tree pass.
- Recursive tree presence does not prove production readiness.
- A README, ADR or feature route does not prove runtime deployment.
- Upstream ancestry and license compatibility require exact commit and file/dependency-level review.
- This priority set is not the entire 142-repository inventory.
- No source repository was modified, archived or deleted.
