# Domain Source Audit — RGBL, MFTL, MOONSKY and MOONSHIPS variants

Audit date: 2026-11-09. Source review is based on repository README files and selected documentation; implementation-level review and exact commit pinning remain pending.

## 1. RGBL — canonical text / scripture / sacred-literature corpus

Source: `bjo163/moonwitness-corpus` (README identifies the project as RGBL / Scripture & Revelation Reference Intelligence).

### Verified scope claims from source documentation

- Provenance-first, multi-tradition text and sacred-literature corpus.
- Separates source artifact, work, expression/edition, passage/content, assertion, evidence and provenance.
- Maintains exact text before downstream assessment or interpretation.
- Explicitly states that textual presence is not universal authority, translation is not source text, textual parallel is not identity, and assessment is not a universal verdict.
- Claims canonical ownership of `mw:*` corpus identity for works, passages, content, resources, assertions, evidence, provenance and corpus assessments.
- Interoperability map keeps STORY (MFTL), EVENT (LEGEND), PERSON (SUPERHERO), TEXT (RGBL), LAW (AWS), DESIGN (assets) and CONSOLE (Crayon) as distinct domains.
- README references `rocksoul-crayon` and `rocksoul-assets` as separate consumers/providers; those repositories are not present in the visible BRIDGE-ALLAH org inventory.

### MW disposition

**Preserve as a bounded canonical corpus domain**, not as an all-purpose knowledge authority. Import requires:
- exact schema and migration inventory;
- stable identifiers and URI compatibility;
- rights and licensing metadata;
- source artifact retention and content hashing;
- edition/translation separation;
- claim scope and evidence provenance;
- import/export contract and test fixtures.

No religious authority, doctrinal conclusion or supernatural verdict is inferred from corpus inclusion. Preserve original text, edition, translation, metadata, assertion and interpretation as separate records.

## 2. MFTL — STORY / narrative and belief intelligence

Source: `bjo163/rocksoul-mftl`.

### Verified scope claims from source documentation

- Provenance-first narrative and belief intelligence corpus for mythology, folklore, supernatural claims, historical ritual traditions, information integrity, deviation and comparative analysis.
- Distinguishes story/narrative from belief, ritual, empirical evidence and historical fact.
- Preserves sources, claims, context, counterevidence and uncertainty.
- Documents canonical separation from EVENT (LEGEND), PERSON (SUPERHERO), exact TEXT (RGBL), LAW (AWS), visual design (assets) and operator console (Crayon).
- MFTL documentation says `@rocksoul/ui` is its shared React implementation; it does not intend to fork the shared design system.

### MW disposition

**Preserve as a bounded STORY domain**. It should not absorb exact corpus text, canonical historical event records, people/agency identity or legal conclusions. Cross-domain links should use stable identifiers and provenance-aware references, not duplicated records or shared mutable tables.

## 3. MOONSKY — messaging domain and service boundary

Source: `bjo163/moonsky`.

### Verified scope claims from source documentation

- Standalone operator console for messaging channels.
- Own PostgreSQL schema (`moonsky`), own operator identity and independent deployment.
- Originated as SKYLINK communication surface inside MoonWitness XV; extracted as an independent repository.
- Telegram webhook ingestion verifies provider secret token; outbound Telegram send uses grammY.
- WhatsApp pairing is optional and depends on configured private auth directory.
- Provides operator REST API and WebSocket event stream.
- Own operator login uses scrypt password hashes, SHA-256 session digests, 12-hour sessions, login throttling and bootstrap administrator.
- Service credential is scoped to machine-initiated sends and channel-registration handshake; README says it cannot read messaging data or manage operator sessions.
- README documents a local bootstrap password for local Compose and requires `MOONSKY_BOOTSTRAP_PASSWORD` for non-local deployment.

### Architectural conflict and MW disposition

MOONSKY currently owns its own operator identity and does not depend on ARSY to sign in. That is an explicit source design, not a minor technical detail. It conflicts with the MW target of one ecosystem identity/control-plane authority.

**Preserve MOONSKY as a bounded messaging domain and schema owner**, but do not automatically preserve a second global user/identity authority. Before integration, decide:
- ARSY OIDC/session integration versus a deliberately isolated operator identity for emergency administration;
- machine-to-machine credential scopes and rotation;
- tenant and channel-account ownership mapping;
- webhook secret rotation and replay controls;
- WhatsApp session storage, filesystem permissions and backup/restore;
- outbound send authorization, rate limits, idempotency and audit;
- WebSocket authorization and event-level access filtering;
- migration of operator identities and session invalidation strategy.

The README's local bootstrap password is documented as local-only; verify Compose network exposure and ensure production mode fails closed without an explicit strong bootstrap secret. Do not publish or reuse the example credential in production.

## 4. MOONSHIPS implementations are three different products/sources

### Current `BRIDGE-ALLAH/MW`

- Pier-derived Rust/Axum/MiniJinja Linux self-hosted infrastructure control plane.
- Maintains Pier runtime identifiers and existing compatibility paths.
- This is the current canonical repository and infrastructure product implementation.

### `bjo163/mwx-ships`

- Rust + Loco.rs + SeaORM + Docker + Traefik mini-PaaS.
- SQLite-first, with optional PostgreSQL for multi-worker scale.
- README describes remote execution over SSH, no host Docker socket mounted into control plane, strict SSH host-key checking, durable SQLite queue, RBAC/scoped tokens/audit and rollback.
- It is a distinct, substantial alternative infrastructure implementation, not a simple UI branch of current MW.

**Disposition:** high-priority capability comparison candidate. Compare deployment lifecycle, queue durability, authorization, tenant model, remote command boundary, backups, rollback, host agents, schema, migrations and test coverage. Import only missing, verified capabilities with source commit and license provenance; do not merge both runtimes wholesale.

### `bjo163/moonships`

- Windows-first Tauri 2 + Rust + React + TypeScript/Vite workbench scaffold.
- README explicitly says initial desktop scaffold; most operational tools, persistence and integrations remain planned.

**Disposition:** separate specialist desktop product candidate, not a replacement for the Linux MOONSHIPS control plane and not a reason to introduce Tauri into the MW server runtime.

## 5. Consolidation principle for these domains

One repository does not mean one process, one database schema, one UI framework or one undifferentiated domain model.

Target:
- one canonical source repository (MW);
- one authoritative identity/policy/governance control plane (ARSY);
- bounded domain ownership for infra, messaging, text corpus, story, event, person and legal research;
- stable contracts and explicit data ownership;
- specialist process boundaries only when isolation, scaling, provider constraints or lifecycle justify them;
- preserve provenance and source history through imports.

A domain may keep a separate schema or deployable process inside the monorepo when justified, but it must not create a competing ecosystem-wide identity, authorization, audit or orchestration authority.

## 6. Next source verification

1. Capture exact source HEAD SHAs and LICENSE/NOTICE files for all five repositories.
2. Inspect RGBL and MFTL schema definitions, migrations, seed/corpus records, API contracts, tests, rights metadata and import/export behavior.
3. Inspect MOONSKY auth/session implementation, migrations, webhook verification, WebSocket auth, secrets, Compose exposure and production bootstrap behavior.
4. Compare `mwx-ships` source with current MW for features, data model, worker semantics, security and license.
5. Record candidate import paths in MW only after provenance and compatibility gates pass.
