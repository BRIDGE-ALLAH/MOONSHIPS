# MW Ecosystem Consolidation Audit

**Status:** Active audit — baseline established, import decisions not yet complete  
**Canonical repository:** `BRIDGE-ALLAH/MW`  
**Audit scope:** `bjo163/*` public repositories discoverable through GitHub repository search, accessible private repositories, and `BRIDGE-ALLAH/*` repositories  
**Baseline date:** 2026-11-09

## 1. Executive decision

The user has designated **`BRIDGE-ALLAH/MW` as the only long-term working repository** for the complete ecosystem. All future active development, documentation, architecture decisions, product work, and operational planning should converge here.

This is a **single-repository / modular-monorepo strategy**, not a requirement to collapse every capability into one executable or one undifferentiated application.

- One canonical repository: `BRIDGE-ALLAH/MW`.
- One ecosystem source of truth: this repository.
- One highest technical control-plane authority: **ARSY**.
- Multiple bounded domains and product experiences may live inside MW.
- Legacy repositories become migration sources, read-only historical references, or archived only after verified import and retention checks.
- No source repository is to be deleted, overwritten, or archived merely because it has been inventoried.

The repository was renamed from MOONSHIPS to MW. The product identity **MOONSHIPS** remains valid as the infrastructure operations product/domain inside MW.

## 2. Non-negotiable consolidation rules

1. **Single place to work:** all new active work is performed in MW. No new standalone repository is created for an ecosystem product without a documented exception.
2. **Preserve provenance:** every imported component records its source repository, source commit/tag, upstream license, copyright notices, and local modifications.
3. **No blind copy:** import only after source inventory, dependency review, secret scan, license review, build/test assessment, and overlap analysis.
4. **No duplicate authorities:** one owner per domain, API, database schema, identity boundary, scheduler, event stream, and public contract.
5. **Modular monolith first:** prefer Rust crates/modules and explicit application/domain boundaries. Separate processes only for security isolation, independent lifecycle/scaling, or hard runtime constraints.
6. **Truthful maturity:** distinguish implemented, partial, prototype, planned, parked, legacy, and unknown. A README claim or directory is not proof of production readiness.
7. **No destructive cleanup before proof:** retain source repositories until imported code builds, tests, licenses, history/provenance, and operational behavior are verified.
8. **No secret migration:** secrets, production .env files, credentials, private keys, and live customer data must never be copied into Git history or audit artifacts.
9. **License compatibility:** preserve all upstream licenses, NOTICE files, attribution, trademarks, and source headers. Do not merge incompatible code until legal compatibility is established.
10. **Runtime compatibility:** preserve existing MOONSHIPS/Pier paths, systemd units, environment names, database formats, APIs, and agent protocols until a separately tested migration is approved by deterministic policy and rollback tests.

## 3. Target ecosystem model

The following is the proposed logical architecture, not a claim that all components already exist.

```text
MW — ONE CANONICAL REPOSITORY / UNIVERSE OS
│
├── ARSY — highest technical control plane
│   ├── identity, authentication, authorization and tenancy
│   ├── policy, trust, delegation and governance
│   ├── system awareness, inventory and resource graph
│   ├── orchestration, durable operations and workflow coordination
│   ├── event contracts, audit, evidence and accountability
│   └── integration contracts for all internal domains
│
├── PLATFORM FOUNDATION
│   ├── identity / access / organizations
│   ├── data / ORM / migrations / storage
│   ├── API / gateway / messaging / event fabric
│   ├── observability / security / backup / release
│   └── agent / worker / automation runtime
│
├── PRODUCT & DOMAIN EXPERIENCES
│   ├── MOONSHIPS — cloud, servers, containers and infrastructure operations
│   ├── MOONWITNESS — evidence, knowledge, research and witness corpus
│   ├── ROCKSOUL — cognitive runtime, AI tooling and creative intelligence
│   ├── THE BATTERIES — capability/energy/power domain (meaning and source TBD)
│   ├── ZILL — domain/product identity (meaning and source TBD)
│   ├── AQEL — Arabic-first programming language and semantic execution
│   ├── MOONSKY — messaging and channels
│   ├── MTS / ANTIKYTHERA — celestial time and scheduling research
│   ├── E-ISP / HARBOR — connectivity, customer and service operations
│   └── other verified projects from the repository register
│
├── HUMAN INTERACTION
│   ├── one primary MW shell/navigation
│   ├── shared identity and design system
│   ├── role-aware workspaces and product modules
│   └── truthful demo / unavailable / production states
│
└── GOVERNANCE & KNOWLEDGE
    ├── canonical narrative and brand registry
    ├── architecture decisions (ADRs)
    ├── product capability registry
    ├── repository import ledger
    └── automated quality, security, license and provenance gates
```

### Authority boundary

ARSY is the highest **technical** control plane inside the software ecosystem. It does not represent divine authority and does not replace human accountability. The terms A-L-L-A-H, GODPROMISE, Empire of the Light, and religious references must be handled respectfully as narrative, inspiration, or project identity; software must never claim divine status, divine endorsement, or theological authority.

The exact public brand hierarchy is intentionally not frozen by this technical audit. The repository will preserve the original narratives and source context until a dedicated narrative synthesis reconciles them.

## 4. Confirmed findings from first-pass source inspection

### 4.1 Current canonical repository: MW

The renamed repository is still structurally the MOONSHIPS Rust application at this baseline:
- Rust workspace members: `pier-core`, `pier-agent`, `pier-net-helper`, and `pier-tests`.
- Web/API foundation: Rust, Axum, Tokio, MiniJinja.
- Main domain persistence: SQLite via rusqlite and embedded ordered migrations.
- Existing product identity and brand documentation still says MOONSHIPS.
- Root README installation and migration examples still reference `BRIDGE-ALLAH/MOONSHIPS`.
- Root Cargo metadata still declares `https://github.com/BRIDGE-ALLAH/MOONSHIPS`.
- `TODO.md` is still titled “MOONSHIPS — Consolidated Engineering Backlog”.
- This rename therefore requires a controlled documentation/metadata alignment pass. Do not rename runtime compatibility identifiers such as `pier`, `PIER_*`, `/opt/pier`, or systemd units as part of the GitHub repository rename.

### 4.2 Major architecture conflict: multiple ARSY authorities

At least three materially different ARSY implementations/narratives were found:
- `BRIDGE-ALLAH/ARSY`: Rust/Axum layered service; README describes in-memory repositories and a basic API skeleton.
- `BRIDGE-ALLAH/A-R-S-Y` and `bjo163/a-r-s-y`: Go/PocketBase-derived deployment control plane in early development.
- `MOONWITNESS` / `moonwitnessxv`: Fastify ARSY described as the current Hub/backend authority.
- `EMPIRE-OF-LIGHT`: explicitly specifies Rust-first ARSY and says Fastify is migration input, not a parallel production backend.

**Recommendation:** designate one ARSY implementation inside MW after a code-level comparison. The existing MW Rust/Axum foundation and the Rust-first direction in EMPIRE-OF-LIGHT are the strongest strategic fit. Treat Fastify and Go implementations as source inputs for requirements, data contracts, domain behavior, tests, and migration candidates—not as parallel long-term authorities. This recommendation remains provisional until their code, licenses, tests, and deployment requirements are compared.

### 4.3 MOONWITNESS source landscape

- `BRIDGE-ALLAH/MOONWITNESS` is an evidence-first knowledge/intelligence ecosystem with several Next.js app workspaces, packages, services, product registry, and explicit prototype/production-readiness caveats.
- `bjo163/moonwitnessxv` is a large TypeScript/pnpm monorepo with detailed architecture, milestones, ARSY/Fastify Hub, ORACLE, LUMEN, ECHOLOG, ORBIT, app workspaces, research, and extensive task/evidence documentation.
- `bjo163/moonwitness-II` is a story/witness platform with canonical content datasets and a web app.
- `bjo163/moonwitness-II-q4q` is a Quran scripture evidence explorer with explicit separation of canonical text, deterministic observations, evidence, claims, and interpretation.
- `bjo163/moonwitness-x` describes a Moonwitness Time System / Antikythera celestial time architecture.
- `bjo163/rocksoul-mftl` owns a provenance-first narrative/belief intelligence corpus and explicitly separates STORY, EVENT, PERSON, TEXT, and LAW domains.
- `bjo163/moonwitness-corpus`, `moonwitness-reborn`, `moonwitness-xiv`, `monwitness-xii`, `moonwitness-xii-season-global-championships`, and other variants require individual review before any import decision.

### 4.4 ROCKSOUL

`bjo163/rocksoul` is a Rust-first intelligence runtime. Its README explicitly distinguishes aspiration from implemented capability and currently documents a bounded CLI/REPL, model connectivity, local identity/profile, bounded cognition, explicit local memory, and evaluation. It explicitly excludes unrestricted shell execution, autonomous filesystem mutation, automatic code execution, multi-agent orchestration, and automatic self-modification in the current release.

`ROCKSOUL-WX`, `ROCKSOUL-EX`, `rocksoul-mftl`, `rocksoul-assets`, `rocksoul-community`, `rocksoul-mind`, `rocksoul-platform`, `rocksoul-legend`, `rocksoul-superhero`, `rocksoul-correlation`, `rocksoul-web`, `rocksoul-ui`, `rocksoul-aws`, and related projects must be treated as separate bounded candidates, not merged by name alone.

### 4.5 Foundation, gateway, identity, data and UI overlap

- `bjo163/ecosystem-foundation` declares itself LEGACY/SUPERSEDED and points to `bjo163/universe-foundation` as canonical. Verify that referenced repository and contract before adopting it.
- `MW-ORM`, `MW-BAAS`, `MW-ID`, `MW-STORAGE`, `MW-GATEWAY`, `MW-UI`, `MW-IDE`, and `MW-BOILERPLATE` exist as separate repositories in BRIDGE-ALLAH. Their source and licenses need inspection before choosing import boundaries.
- `MOONWITNESS-GATEWAY` and `MW-GATEWAY` are both present; overlap must be resolved.
- `ROCKSOUL-WX` also has AI gateway scope; distinguish model routing/AI runtime from general API ingress, auth, and internal service gateway.
- `MW-UI` may overlap with UI/design packages in MOONWITNESS and ROCKSOUL. Select one shared design-system owner inside MW.

### 4.6 Other significant candidates

- `bjo163/AQEL`: Arabic-first programming language with language-neutral semantic core; currently pre-release.
- `bjo163/mwx-ships`: a separate Rust/Loco/SeaORM MOONSHIPS implementation with SQLite-first architecture. This is a major code-level overlap with the current MW Rust/Axum/Pier-derived application and must be compared, not blindly merged.
- `bjo163/arah-saas-development`: private repository visible to the connected GitHub access; small Next.js application and security/setup docs. Include in private-source audit.
- `bjo163/arsy-base`: repository currently empty at the accessible default branch.
- Search did not find a repository named `thebatteries` or `ZILL` under the queried public `bjo163` repository set. These remain unresolved names and may exist in another owner, branch, project directory, or non-public source. Do not invent their scope.

## 5. Initial consolidation decisions

| Capability | Proposed canonical owner in MW | Import policy |
|---|---|---|
| Highest control plane | ARSY | Select one implementation after comparative audit |
| Infrastructure operations | MOONSHIPS domain | Preserve current MW implementation; compare `mwx-ships` before importing |
| Evidence and knowledge | MOONWITNESS domain | Import domain models, corpus, provenance, and verified workflows incrementally |
| Cognitive runtime | ROCKSOUL domain | Preserve bounded capability and explicit permission model |
| Deterministic computation | ORACLE domain/service | Import only after contract, isolation, persistence and recovery verification |
| Agent runtime | LUMEN / worker domain | Import as bounded workers under ARSY-issued policy and scoped credentials |
| Scripture evidence | Q4Q / scripture evidence domain | Preserve exact source text, canonical corpus boundaries, evidence and interpretation separation |
| Narrative/belief intelligence | MFTL domain | Preserve STORY/EVENT/PERSON/TEXT/LAW ownership boundaries |
| Celestial time | MTS / Antikythera domain | Research candidate; independently validate astronomical/time claims and testability |
| Language/compiler | AQEL domain | Separate compiler crate/workspace inside MW after license and toolchain review |
| Messaging | MOONSKY domain | Import contracts and bounded messaging implementation; no duplicate Hub |
| Connectivity/ISP | E-ISP/HARBOR domain | Import only if current product/business need and operational evidence are confirmed |
| Identity/data/storage/gateway/UI | Platform foundation | One canonical implementation each; no duplicate services |

## 6. Migration method

Each candidate follows the same sequence:

1. Inventory source repository, default branch, last activity, tags/releases, open issues/PRs, directory map, dependencies, license, CI, secrets exposure, generated/vendor code, and deployment assumptions.
2. Classify each path as import, adapt, reference, archive, or reject.
3. Map capabilities and contracts against existing MW code to identify overlap.
4. Import into a bounded destination path/crate with source provenance and preserved legal notices.
5. Add tests/fixtures and update architecture registry.
6. Run build, lint, unit/integration/E2E, security, license, secret, and dependency checks.
7. Record exact source SHA, destination SHA, evidence, deviations, and rollback/revert instructions.
8. Only after successful verification, mark source repository as historical/archival candidate. Archival remains a separate decision and is not automatic.

## 7. Repository rename cleanup — immediate

The GitHub repository rename has not automatically changed embedded text. Audit and update only public repository references that should now point to MW:
- Root README clone/install/migration URLs.
- `Cargo.toml` repository metadata.
- Documentation links and badges that refer to the old repository URL.
- GitHub Actions workflow references and release URLs.
- Docker labels/image metadata and generated manifests, if present.
- Website links and installation instructions.

Do **not** bulk replace MOONSHIPS product name or Pier compatibility identifiers. Keep product identity distinct from repository identity.

## 8. Audit limitations and next evidence

The first pass confirms the public repository search returned 142 `bjo163` repositories across two result pages, plus one separately accessible private repository. The BRIDGE-ALLAH organization listing returned 20 repositories at the time of inspection. This is a search-visible inventory, not a guarantee that all private, transferred, deleted, or inaccessible repositories have been enumerated.

The next audit pass must:
- Capture complete metadata and README for all 142 public repositories.
- Inspect recursive file trees for all high-relevance candidates.
- Read source manifests and license files before any import.
- Inspect git activity, tags/releases, issues, PRs, and CI status.
- Locate missing/unresolved `THEBATTERIES`, `ZILL`, and related project names through other owners, references, and source docs.
- Verify the canonical Universe Foundation repository referenced by the legacy foundation README.
- Produce an import ledger with explicit per-repository status and exact source commit.

**No repository has been deleted, archived, or bulk-copied as a result of this audit.**


## 9. Further narrative and domain-source findings

### 9.1 A-L-L-A-H and Empire of the Light are not one simple product label

The source landscape contains at least two distinct but related design baselines:
- `bjo163/empire-of-the-light` describes A-L-L-A-H as **“Accountable Life, Learning & Action Hub”**, explicitly a stylized project acronym and not a claim of divine authority. It describes a human-governed, source-grounded knowledge and mission system. Its current README labels the baseline proposed and not production-certified.
- `BRIDGE-ALLAH/EMPIRE-OF-LIGHT` describes A-L-L-A-H.COM as the public ecosystem brand, Empire of Light as the vision, and a Rust-first ARSY platform in one monorepo. It explicitly treats Fastify as migration input rather than a parallel production backend.
- `bjo163/A-L-L-A-H.COM` is a large Next.js/Fastify monorepo source. Its code and documentation must be compared with the Rust-first EMPIRE baseline and current MW before import.

These identities should be preserved as source narratives until a final brand/naming ADR defines parent identity, public home, product identity and technical authority separately.

### 9.2 THE BATTERIES

The term appears in the title of the MoonWitness XV master vision document alongside Empire of the Light, A-L-L-A-H, GOD-PROMISE, ROCKSOUL, Corpus and Witness. This proves it is part of the historic narrative vocabulary, but does **not** establish its technical function, implementation, or repository ownership. No exact public repository match was found in the `bjo163` search snapshot. Keep it as an unresolved domain identity pending source discovery.

### 9.3 ZILL

The MOONWITNESS master README lists **ZILL WATCH** among HEADQUARTER modules. No exact `ZILL` repository was found in the public `bjo163` search snapshot. The current evidence supports treating ZILL WATCH as a historical/product module name, not a standalone product or backend. Its exact scope and source implementation remain to be inspected.

### 9.4 Evidence and narrative corpus boundaries

The ROCKSOUL research family has intentionally separated domain repositories:
- `rocksoul-mftl`: STORY / narrative / belief intelligence.
- `rocksoul-legend`: EVENT / historical and event intelligence.
- `moonwitness-corpus`: TEXT / scripture and revelation-reference intelligence (the README calls the domain RGBL).
- `rocksoul-superhero`: PERSON / human agency (to verify in its own source).
- `rocksoul-aws`: LAW / legal analysis (to verify in its own source).
- `rocksoul-assets`: visual system and delivery asset source of truth.
- `rocksoul-crayon`: shared operational console (referenced by multiple source READMEs; needs its own repo audit).

Their source docs insist on provenance-first records, uncertainty, scoped claims, and explicit separation between textual presence and universal authority. Preserve these domain boundaries during consolidation; do not merge them into one undifferentiated “AI knowledge” table.

### 9.5 MOONSKY messaging boundary

`bjo163/moonsky` describes an independently deployed messaging console with its own PostgreSQL schema and operator identity, explicitly not an ARSY Hub and not a shared ecosystem identity authority. The MoonWitness XV source says D18 messaging moved out of XV into MOONSKY. The target MW design should integrate MOONSKY through a bounded messaging contract; decide whether to retain its process boundary or adapt it into an internal module after reviewing security, transport dependencies and operational isolation. Do not introduce a second global Hub or identity authority.

### 9.6 ROCKSOUL Assets and domain ownership

`bjo163/rocksoul-assets` declares itself the visual source of truth for MoonWitness × Rocksoul and documents a completed asset release. This is a high-value import candidate for the unified MW design system and brand registry, subject to license and path audit. Do not duplicate its asset catalog into multiple app-specific copies.

### 9.7 Current decision remains provisional

The source documents are not consistent about ARSY: MoonWitness XV calls Fastify/MoonQL its single Hub; EMPIRE-OF-LIGHT specifies Rust-first ARSY; current MW is Rust/Axum. The consolidation must choose one technical authority in MW and port required behavior by contract. No source README should silently overrule the canonical MW ADR once ratified.
