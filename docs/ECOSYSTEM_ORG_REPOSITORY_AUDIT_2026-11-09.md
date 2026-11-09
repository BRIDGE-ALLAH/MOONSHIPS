# BRIDGE-ALLAH Organization Repository Audit — 2026-11-09

## Scope and confidence

This is a first-pass source audit of all 20 repositories returned by the connected GitHub account for owner `BRIDGE-ALLAH` on 2026-11-09. It combines repository metadata and root README inspection where available. It is not yet a complete source-code, dependency, license, security, branch, release, or commit-history audit. Empty repositories and missing README files are explicitly marked. No repository has been imported, archived, deleted, or modified by this audit.

The connected account currently exposes one repository under `bjo163`: `bjo163/arah-saas-development` (private, default branch `main`, size 231 KB as reported by GitHub). This does not establish visibility into the other public/private repositories found in the earlier 142-result search snapshot.

## Organization inventory and first-pass disposition

| Repository | Default branch | Visibility | Size (KB) | README/source observation | Initial disposition |
|---|---:|---:|---:|---|---|
| `MW` | main | public | 4,033 | MOONSHIPS/Pier Rust + Axum + MiniJinja infra control plane; canonical target repo | **Canonical host**; preserve runtime and compatibility |
| `MW-GATEWAY` | main | public | 69,259 | Agentgateway-derived AI/LLM, MCP, A2A connectivity and policy gateway; README declares gateway is not application hub or identity authority | **Import candidate as bounded gateway domain**, subject to upstream license, fork delta, security, build and deploy audit |
| `MW-UI` | main | public | 71,467 | React/Next/Tailwind/shadcn UI foundation; evidence-first component direction; MIT README claim; roadmap includes unfinished integration | **UI source candidate**, compare with existing MW MiniJinja UI and MOONWITNESS apps before choosing frontend strategy |
| `MW-IDE` | main | public | 28,624 | Lapce-derived native Rust/Floem/wgpu engineering IDE; README distinguishes inherited upstream features from planned MoonWitness features | **Separate product candidate**, not core runtime dependency; preserve Lapce notices and license |
| `MW-ORM` | master | public | 20,902 | SeaORM 2.0-derived Rust ORM/module-runtime direction; upstream crate names remain; target architecture not all implemented | **Data-layer candidate**, not yet approved as MW primary ORM; audit exact SeaORM license, fork delta, APIs, compatibility and prototype against current rusqlite |
| `MW-BAAS` | master | public | 16,974 | README remains the upstream Loco framework README, not MW-BAAS product documentation | **Unclassified / likely unbranded upstream mirror**; do not integrate until provenance, fork delta, license and intended ownership are established |
| `MW-ID` | master | public | 110,974 | README remains upstream Kanidm identity platform documentation | **Identity infrastructure candidate / upstream mirror**; do not treat as MW-owned identity implementation; audit Kanidm license, changes, operational footprint and federation contract |
| `MW-STORAGE` | main | public | 125,816 | RustFS-derived distributed object storage; README states S3-compatible scope and inherited upstream foundation | **Storage infrastructure candidate / upstream fork**; audit RustFS license, fork delta, security, deployment and compatibility before integration |
| `EMPIRE-OF-LIGHT` | main | public | 353,702 | Temps-derived Rust workspace and deployment assets; declares Rust-first ARSY; Fastify A-L-L-A-H.COM as migration source only | **High-priority architecture/source candidate**; reconcile against MW's Pier/Axum baseline and MOONWITNESS Fastify ARSY before import |
| `MW-BOILERPLATE` | main | public | 335 | Moonbase Dioxus/Rust static-web starter; no backend, database or auth; README says UI compile/render and deployment verification pending | **Reusable scaffold candidate**, not production frontend; avoid introducing another default UI without ADR |
| `AGILITY` | main | public | 247,989 | README is GPT4Free/g4f project text, GPLv3 indicated; no MW-specific purpose established from README | **Unrelated/unclear source; quarantine from MW** pending ownership and legal review |
| `ARSY` | main | public | 36 | Small Rust/Axum/Tokio layered service; in-memory user repository; README says license not chosen | **Early prototype/reference only**; not a production identity or control plane |
| `A-R-S-Y` | main | public | 55,369 | Go + PocketBase deployment control-plane fork; early MVP, SSH/Docker Compose scope; README explicitly says not production-ready | **Separate experimental implementation; do not merge with canonical ARSY** without explicit architecture decision |
| `wtagent` | main | public | 4,018 | Node CLI agent controlling browser-based AI websites and local file/command operations | **Independent developer-tool candidate**; high-risk permissions and provider terms/privacy review required |
| `MOONWITNESS` | main | public | 4,763 | Large Next/Fastify/Python monorepo; README says ARSY Fastify is canonical backend there, ORACLE is bounded Python service; several apps/prototypes; production NO-GO | **High-priority domain/source audit**; substantial architecture conflict with Rust-first EMPIRE and current MW. Do not import wholesale |
| `MOONWITNESS-GATEWAY` | main | public | 0 | Empty repository; GitHub Contents API reports no files | **Empty placeholder**; inspect branches/history before disposition |
| `ROCKSOUL-EX` | main | public | 213 | Local-first VS Code extension; README reports version 0.3.0 and mission-scoped patches with explicit approval and proof | **Strong standalone product candidate**; inspect source/security/release and integrate as bounded developer-experience module or package |
| `ROCKSOUL-WX` | main | public | 172 | Rust/Axum outcome-driven AI gateway; policy-first provider routing, receipts, explicit workload trust boundaries | **Strong bounded service candidate**; compare responsibilities with MW-GATEWAY before selecting or composing gateways |
| `FS-BOILERPLATE` | main | public | 0 | Empty repository; no README content | **Empty placeholder**; inspect branches/history before disposition |
| `BOILERPLATE-INTELEGENT-ENTERPRISE-FULLSTACK` | master | public | 298,879 | Bun/Turborepo/Fastify/Next boilerplate; README reports Phase 0; includes multiple ORM/DB concepts and optional Rust/Loco server | **Reference/legacy boilerplate only**; not a canonical platform; audit provenance and security before any reuse |

## Material architecture findings

### 1. There are multiple competing ARSY implementations

The visible sources now establish at least four distinct implementations/directions:

1. `MW`: existing production-oriented Rust/Axum/Pier infrastructure control plane.
2. `BRIDGE-ALLAH/ARSY`: small Rust/Axum/Tokio layered prototype with in-memory persistence.
3. `BRIDGE-ALLAH/A-R-S-Y`: Go/PocketBase-based deployment-control-plane experiment.
4. `BRIDGE-ALLAH/MOONWITNESS`: Fastify/Node ARSY backend, described in its README as that repository's canonical backend.
5. `BRIDGE-ALLAH/EMPIRE-OF-LIGHT`: declares Rust-first ARSY and treats Fastify as migration input.

The prior `bjo163/moonwitnessxv` source also described Fastify/MoonQL as a Hub. These claims conflict and must be reconciled through route, data model, identity, tenancy, policy, job, and integration inventories—not by choosing a language from README claims alone.

**Working recommendation (not yet an approved ADR):** use the existing Rust/Axum MW platform as the canonical runtime host; treat ARSY as the governance/control-plane domain implemented within that host unless a source-code comparison demonstrates a better extraction boundary. Keep the Go/PocketBase and Fastify versions as migration/reference sources. Preserve useful behavior and contracts; do not run multiple global authorities.

### 2. The gateway domain also has competing implementations

- `MW-GATEWAY` is an Agentgateway-derived LLM/MCP/A2A connectivity and policy gateway.
- `ROCKSOUL-WX` is a smaller Rust/Axum outcome-oriented AI workload gateway with privacy/egress policy, provider routing, acceptance checks and metadata receipts.

These are not automatically duplicates: one may be the protocol/data-plane gateway and the other a workload planner/policy facade. A contract and feature comparison is required before choosing one, composing them, or importing both.

### 3. Identity, ORM, storage, and UI are likely upstream forks—not native MW modules

- `MW-ID` currently identifies itself as Kanidm.
- `MW-ORM` identifies itself as a SeaORM-derived codebase.
- `MW-STORAGE` identifies itself as RustFS-derived.
- `MW-UI` identifies itself as a React/Next/shadcn foundation.
- `MW-BAAS` README is still Loco upstream.
- `MW-IDE` identifies Lapce as its current foundation.

Each needs exact upstream commit, license, local diff, security posture, build/test evidence and trademark/branding review. Repository name and logo do not transfer upstream ownership or make the fork a completed MW capability.

### 4. UI direction is not settled

Current MW is server-rendered Axum/MiniJinja. The organization also has React/Next Moonwitness UI, Dioxus Moonbase, and multiple app workspaces in MOONWITNESS/EMPIRE. Do not introduce a wholesale frontend rewrite. Compare capability, accessibility, deployment, authentication, API contracts, runtime footprint and maintenance cost; retain one default product shell and permit bounded specialist surfaces only where justified.

### 5. Licensing and provenance are release blockers

The first-pass README signals include AGPL-3.0 for current MW/Pier, Apache-2.0 for MW-GATEWAY, MIT for MW-UI, upstream licenses for Lapce, SeaORM, Kanidm, RustFS, Loco, PocketBase and GPT4Free, and an unspecified license in the small ARSY prototype. README badges/text are not legal verification. Before copying any source, inspect LICENSE/NOTICE/COPYING, dependency licenses, source commit ancestry, modifications, trademarks, assets and generated files. The small ARSY repository explicitly says no license has been chosen; do not copy it into a distributable product until resolved.

## Immediate audit queue

1. Record exact default-branch HEAD SHA and latest commit timestamp for all 20 organization repositories and all accessible priority `bjo163` repositories.
2. Inspect full trees and manifests for `MW-GATEWAY`, `ROCKSOUL-WX`, `MW-ORM`, `MW-ID`, `MW-STORAGE`, `MW-UI`, `MW-IDE`, `MW-BOILERPLATE`, `EMPIRE-OF-LIGHT`, `MOONWITNESS`, `ARSY`, and `A-R-S-Y`.
3. For every upstream-derived repository, establish exact upstream URL, base commit, fork delta, local changes, licenses and notices.
4. Compare ARSY routes/contracts/data/auth/tenancy/jobs/policy across the Rust, Go, and Fastify sources.
5. Compare MW-GATEWAY and ROCKSOUL-WX contracts and security boundaries.
6. Draft ADRs for canonical ARSY, frontend shell, gateway composition, identity provider integration, ORM and object storage only after source evidence is collected.
7. No source import, repository retirement, destructive migration, or production deployment until provenance, compatibility and rollback requirements are satisfied.

## Safety and decision rules

- Repository README statements are claims to verify, not proof of implementation or production readiness.
- Preserve source repositories and history.
- No bulk copy, subtree import, force-push, archive, deletion, or destructive DB migration in this audit phase.
- Keep credentials, private source content and secrets out of public documentation.
- A repository's inclusion in this inventory does not mean its contents are approved for redistribution.
- Keep the distinction between implemented, tested, deployed, planned, inherited upstream and unknown explicit in all future MW docs.
