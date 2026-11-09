# MOONSHIPS — Consolidated Engineering Backlog

> **Backlog revision:** 2  
> **Branch:** `rebrand/moonships-full-brand`  
> **Purpose:** single source of truth for engineering work, ordered by dependency and risk.  
> **Scope:** static-audit findings that still require verification, build/release hardening, security, reliability, then optional product expansion.  
> **Execution policy:** every task is designed to be implementable and verifiable by automation. No task requires a human code review, manual approval click, or subjective sign-off to satisfy its acceptance criteria. Risky actions must be guarded by deterministic, preconfigured machine policy and automated tests. This does **not** authorize unattended production deployment, destructive changes to customer data, or changes to live servers.

## How to read and execute this backlog

### Priority
- **P0 — Build/release integrity:** release pipeline and clean-build blockers.
- **P1 — Security boundary:** authorization, isolation, webhook trust, privileged binary integrity, credential protection.
- **P2 — Reliability and operability:** process lifecycle, database scheduling, backup/restore, telemetry, test depth.
- **P3 — Product expansion:** deploy intelligence, security posture, fleet/GitOps, diagnostics, recovery, extensibility.

### Task status
All checkboxes begin unchecked. A task may be checked only when every acceptance criterion passes on the task's target commit and the evidence artifacts exist. “Code written” is not completion.

### Required task execution contract
For each task:
1. Re-read the affected code at the current branch HEAD; do not assume earlier audit notes are still accurate.
2. Implement only the ownership boundary stated in the task. If adjacent work is needed, create a separate task instead of expanding scope silently.
3. Add deterministic unit, integration, regression, and negative tests appropriate to the risk.
4. Run formatting, lint, build, and relevant workspace tests.
5. Produce machine-readable evidence (CI result, test report, benchmark, migration fixture result, or generated artifact).
6. Update docs and this checklist in the same change.
7. Do not mark complete if a required test is skipped, flaky, or only manually asserted.

### Global non-negotiable rules
- Preserve Pier runtime contracts during branding-only migration: `/opt/pier`, `/opt/pier/data`, `/opt/pier/.env`, Linux user/group `pier`, `pier.service`, helper/updater unit names, `/run/pier/net.sock`, `PIER_*` variables, database formats, and executable names until a separately tested namespace migration exists.
- No destructive schema migration without an additive compatibility phase, automated backup verification, and tested rollback path.
- Authorization is enforced server-side. Hidden UI controls are not security controls.
- Fail closed on missing identity, missing secret, malformed signature, unknown role, invalid resource binding, and failed integrity checks.
- Secrets never appear in logs, API error bodies, test snapshots, CI artifacts, or diagnostic bundles.
- Do not run production migrations, install scripts, release publishing, or destructive restore against a live customer host as part of backlog implementation.
- Preserve AGPL-3.0 and upstream copyright/attribution obligations.
- Do not introduce a new dependency if an existing maintained crate/tool already meets the requirement.

---

# Wave 0 — Build and release integrity (P0)

## [ ] P0-01 — Make Docker build resolve the complete Cargo workspace

**Ownership boundary:** Dockerfile, Docker build context, `.dockerignore`, Docker build CI test. This task does not redesign runtime packaging or change workspace membership.

### SPEC
- The repository workspace declares four members: `pier-core`, `pier-agent`, `pier-net-helper`, and `pier-tests`.
- The current Dockerfile's dependency-cache stage copies only the manifests for `pier-core` and `pier-agent`, while the workspace manifest references all members. Cargo workspace metadata resolution can fail before the real source copy because the other member manifests are absent.
- The root `templates/` directory exists in the repository tree; do **not** treat its COPY instruction as a missing-directory finding. Verify whether it is actually needed by the final image and retain it if required.
- A clean checkout must build without relying on untracked files, developer caches, or generated files left over from another build.

### RULE
- Do not remove a workspace member or edit workspace membership merely to make Docker build pass.
- Dependency caching must not change the source being compiled.
- Use `--locked` for Cargo dependency resolution.
- Never pass secrets through Docker build args or bake them into layers.
- Preserve the current runtime command, required embedded assets, volume semantics, and documented environment variables unless tests prove they are unused.

### DESIGN
1. Inspect all workspace manifests and build scripts to identify metadata required before dependency compilation.
2. Copy the root `Cargo.toml`, `Cargo.lock`, and every workspace member `Cargo.toml` into the dependency stage.
3. Create only the dummy source files required for Cargo dependency caching; ensure each workspace member has the minimum expected target files or use a build strategy that does not require dummy crate targets.
4. Copy real `crates/` and required asset/template directories after dependency caching.
5. Build the intended runtime binary with `cargo build --release --locked --package pier-core`.
6. Audit runtime stage for shared libraries, CA certificates, Docker CLI/Compose, user/group, permissions, and embedded assets.
7. Keep build context explicit and exclude `.git`, local `target/`, secrets, local data, and unrelated files using `.dockerignore`.

### ACCEPTANCE CRITERIA
- [ ] `docker build --no-cache` succeeds from a clean checkout.
- [ ] Normal cached `docker build` succeeds on a second run.
- [ ] Build uses the committed lockfile and does not update dependencies.
- [ ] Cargo resolves all four workspace members in the builder stage.
- [ ] Runtime image contains the intended `pier` executable and all required runtime assets.
- [ ] Container starts as the intended non-root user and the health endpoint responds in a smoke test.
- [ ] Docker/Compose integration expected by the application is available and permissions are tested.
- [ ] Image inspection finds no repository secrets, local `.env`, host data, or developer `target/` directory.
- [ ] CI stores build logs and image digest as evidence.

---

## [ ] P0-02 — Make release publishing depend on successful CI for the exact same commit

**Ownership boundary:** GitHub Actions workflows, branch ruleset configuration documentation, release test fixtures. This task does not change artifact signing implementation; see P1-04.

### SPEC
- Current release workflow triggers on pushes to `main` and builds/publishes artifacts without an explicit dependency on the CI workflow.
- The CI workflow currently runs shell syntax, formatting, workspace check, and only the `pier-core` binary test target.
- Release publishing must be impossible when required checks are missing, failed, cancelled, or belong to a different commit SHA.

### RULE
- Release artifacts may only be published for the exact SHA that passed the required CI jobs.
- A stale successful run must never authorize a newer commit.
- Failed or skipped required checks are not success.
- Keep least-privilege GitHub Actions permissions; only the publish job receives release-write permission.
- Do not use `continue-on-error` for mandatory checks.

### DESIGN
1. Define one canonical set of required check names and keep them stable.
2. Configure repository branch ruleset/branch protection to require PR and required CI checks before merge.
3. Make release workflow wait for CI completion using a SHA-bound workflow dependency (for example `workflow_run` with explicit branch and head-SHA equality checks) or an equivalent mechanism that cannot race.
4. Split release into build/test, artifact preparation, and publish jobs.
5. Pass the verified commit SHA and artifact digests between jobs; do not rebuild from a mutable branch name after validation.
6. Restrict release-write permission to the final publish job.
7. Add workflow tests or a script that validates the release gate's branch/SHA/conclusion logic.

### ACCEPTANCE CRITERIA
- [ ] Simulated failed CI results in zero release/tag/asset publication.
- [ ] Missing or cancelled CI results in zero publication.
- [ ] A successful CI run for SHA A cannot authorize release of SHA B.
- [ ] A successful CI run for the exact release SHA allows the publish job to proceed.
- [ ] PR checks are required by repository ruleset; configuration is machine-readable or captured in an automated verification report.
- [ ] Build job has read-only repository permissions.
- [ ] Only publish job has the minimum release-write permission.
- [ ] Release metadata includes source SHA, toolchain, artifact SHA-256, and build timestamp.
- [ ] A test validates both positive and negative gate cases.

---

# Wave 1 — Security boundaries (P1)

## [ ] P1-01 — Enforce Global Admin authorization on global security settings

**Ownership boundary:** `crates/pier-core/src/api/security.rs`, route registration/policy in `crates/pier-core/src/api/mod.rs`, authorization tests. Do not implement the entire audit-log platform here.

### SPEC
- The `/security/settings` update handler changes global security policy, including `security.require_password_on_delete`.
- Static audit found the handler did not receive an authenticated user/role context and the route was not registered behind the global-admin guard.
- Only Global Admin/Owner may read or mutate global security policy, according to the product's established role model.

### RULE
- Unauthenticated callers receive 401; authenticated users without global-admin capability receive 403.
- Project-scoped Admin/Editor/Viewer cannot elevate themselves by changing project membership or request payload.
- Missing user context, unknown role, role lookup failure, or policy-service failure must deny the mutation.
- Do not trust role names supplied by the client.
- Preserve existing setting values on upgrade; do not silently reset a customer's security preference.

### DESIGN
1. Reuse the canonical server-side authorization helper; do not create a parallel role parser.
2. Add authenticated principal/context to the handler.
3. Place the route behind the global-admin router guard or use an equivalent explicit capability check.
4. Validate the setting key and value against a typed allowlist.
5. Emit a structured security-setting event through the audit event interface (the generic audit pipeline is expanded in P2-05).
6. Return a stable error shape with no internal role/secret data.
7. Keep the UI permission state consistent with the API, but rely on API enforcement.

### ACCEPTANCE CRITERIA
- [ ] Global Admin/Owner can update the allowed setting.
- [ ] Project Admin, Editor, Viewer, and ordinary authenticated users receive 403.
- [ ] Unauthenticated requests receive 401.
- [ ] Unknown role and role lookup failure deny the update.
- [ ] Client-supplied actor/role fields have no effect.
- [ ] Invalid setting key/type is rejected without a database write.
- [ ] Successful update emits one audit event; denied attempts cannot mutate state.
- [ ] Regression tests verify the previous default and existing stored values survive upgrade.
- [ ] API docs describe the capability and error contract.

---

## [ ] P1-02 — Isolate DB Browser credentials and bind queries to the selected resource

**Ownership boundary:** `crates/pier-core/src/api/db_browser.rs`, DB credential provisioning/mapping, SQL authorization tests. Do not redesign general database backup credentials here; see P1-05.

### SPEC
- Static audit found DB Browser target resolution can use PostgreSQL superuser/root credentials sourced from container environment and accepts a client-supplied database name.
- A project Editor with SQL execution capability must not gain cluster-wide access to unrelated databases merely because the application knows a root credential.
- The server must derive the allowed database target from the selected service/resource identity.

### RULE
- Client cannot submit or override host, port, username, password, connection string, or arbitrary database target.
- Project membership does not imply database superuser permission.
- Viewer is read-only; Editor gets only the explicitly granted database-level operations; database administration is a separate capability.
- Reject unknown resource IDs, mismatched project IDs, and database names outside the server-side allowlist before opening a DB connection.
- Do not expose connection errors containing passwords, DSNs, or internal host details.

### DESIGN
1. Create a server-side mapping: resource ID → project ID, engine, service/container identity, permitted database/schema names, credential reference, and capability set.
2. Provision per-database/service credentials with least-privilege grants; do not use root/postgres for ordinary query execution.
3. Resolve all connection parameters from trusted server-side metadata.
4. Reject or remove the client-supplied `database` field; if compatibility requires it, validate exact membership in the resource allowlist.
5. Use read-only transaction/session mode for Viewer and enforce write/DDL capabilities for Editor/Admin.
6. Add credential rotation/migration for existing DB resources, with legacy credential detection and a fail-safe remediation state.
7. Emit resource-scoped audit events without storing raw SQL by default.

### ACCEPTANCE CRITERIA
- [ ] Editor on database A cannot enumerate, read, or mutate database B on the same DB server.
- [ ] Arbitrary database names and resource/project mismatches are rejected before connection.
- [ ] Viewer cannot execute INSERT/UPDATE/DELETE/DDL or change session permissions to bypass read-only mode.
- [ ] Normal query path does not use root/postgres credentials.
- [ ] PostgreSQL and MySQL/MariaDB integration tests cover isolation and permission boundaries.
- [ ] Existing resources are migrated to scoped credentials or marked blocked with a deterministic remediation message.
- [ ] API responses and logs contain no credential/DSN values.
- [ ] Audit events identify actor, project, resource, operation class, and outcome.

---

## [ ] P1-03 — Verify every webhook event before parsing side effects

**Ownership boundary:** `crates/pier-core/src/api/webhooks.rs`, webhook secret storage/rotation contract, webhook tests. Do not implement general deployment rollback behavior here.

### SPEC
- Static audit found webhook processing paths where an unset secret could be treated as eligible/trusted and installation-event state changes could occur before the signature verification used by push handling.
- All provider events capable of changing installation mapping, source metadata, deployment state, or triggering deploy must be authenticated before side effects.

### RULE
- Missing/empty secret means reject; it never means unsigned trusted mode.
- Verify the raw request body before deserialization or side effects.
- Use provider-specific signature headers/algorithms and constant-time comparison.
- Unknown provider, malformed signature, unsupported event, oversized body, and replayed delivery ID must fail safely.
- Do not log raw body, secret, signature, or authorization headers.
- Secret rotation must not silently create a period where requests are accepted unsigned.

### DESIGN
1. Normalize provider identity and event metadata from trusted route/config.
2. Read bounded raw body bytes and validate content length.
3. Resolve secret for the target installation/service.
4. Verify provider signature against raw bytes using constant-time comparison.
5. Validate delivery ID/timestamp and apply idempotency/replay protection.
6. Only after successful verification, deserialize and route event to business logic.
7. Store webhook secrets encrypted; support generate, rotate, test, revoke, and legacy-secret remediation.
8. Keep signature verification as a shared middleware/service so no event handler can bypass it.

### ACCEPTANCE CRITERIA
- [ ] Missing secret, missing signature, malformed signature, and invalid signature are rejected before any state change.
- [ ] Valid GitHub and GitLab signatures are accepted for supported events.
- [ ] Installation events cannot mutate installation mappings without valid signature.
- [ ] Replayed delivery IDs are idempotently ignored/rejected.
- [ ] Oversized and malformed payloads are rejected within bounded resource use.
- [ ] Tests cover all supported event types, not only push.
- [ ] Legacy services with no secret enter a documented blocked/remediation state.
- [ ] No raw secrets/signatures/payloads appear in logs or error responses.

---

## [ ] P1-04 — Authenticate integrity and provenance of privileged helper downloads

**Ownership boundary:** helper release assets, `scripts/install.sh` helper fallback, bootstrap/updater helper installation. Do not own the general CI release gate (P0-02).

### SPEC
- The network helper runs with elevated host privileges and performs host networking operations.
- Static audit found an `install.sh` fallback path that downloads the helper without checksum validation, while other download paths have different integrity behavior.
- Every path that installs or replaces the privileged helper must enforce the same verification policy.

### RULE
- Never execute or install a downloaded helper before verification.
- Missing checksum/manifest/signature or verification tool failure is a hard failure.
- Do not trust an unauthenticated checksum fetched from the same mutable location as the binary as the sole provenance control.
- Never overwrite the currently working helper before the new artifact is verified.
- Do not silently fall back to an unverified binary.

### DESIGN
1. Define one canonical helper artifact naming/version/architecture manifest.
2. Generate SHA-256 checksums and signed provenance/attestation in the release pipeline.
3. Pin the expected signer identity/issuer or trusted public key in the installer.
4. Download binary and metadata into a root-owned temporary directory with restrictive permissions.
5. Verify checksum, signature/provenance, architecture, executable format, and expected version.
6. Install atomically only after all checks pass; retain previous helper for rollback.
7. Use one shared verification function across bootstrap, installer, updater, and repair paths.
8. Record verified digest and source release in an install metadata file.

### ACCEPTANCE CRITERIA
- [ ] All helper download/install code paths call the same verifier.
- [ ] Checksum mismatch, signature mismatch, missing metadata, wrong architecture, and invalid binary all fail closed.
- [ ] Existing working helper remains intact on failed update.
- [ ] Valid signed artifact installs atomically with correct owner/mode.
- [ ] Automated shell tests cover network errors, truncated downloads, missing files, bad signatures, and rollback.
- [ ] Release artifacts expose digest and provenance evidence.
- [ ] Installer output reports verified version/digest without exposing secrets.

---

## [ ] P1-05 — Encrypt backup database credentials at rest in SQLite

**Ownership boundary:** backup credential persistence and key management, including migration. DB Browser runtime credentials are owned by P1-02.

### SPEC
- Static audit found backup `DbCredential` passwords documented/stored as plaintext in the `database_credentials` SQLite table.
- Scheduled backup must still work after service restart without requiring an operator to re-enter credentials for every run.

### RULE
- No hard-coded master key in source, container image, or default config.
- Never log plaintext credential or include it in API output.
- A missing/wrong key must produce an explicit blocked/error state, never a blank/default password.
- Preserve the old record until ciphertext decrypt verification succeeds.
- Key rotation must preserve decryptability of existing backups/configuration.

### DESIGN
1. Define versioned encrypted record format: algorithm, nonce, ciphertext, key ID, format version.
2. Use envelope encryption with master key supplied by protected host secret file or supported secret manager.
3. Set strict owner/mode for local key material; never store key beside the SQLite DB with equal permissions.
4. Implement read/write abstraction so backup code never sees plaintext outside a short-lived in-memory scope.
5. Migrate legacy plaintext rows transactionally: read → encrypt → write → decrypt-verify → remove plaintext.
6. Provide a machine-readable migration status and deterministic recovery instructions.
7. Support key rotation with old/new key IDs and automated re-encryption verification.

### ACCEPTANCE CRITERIA
- [ ] New credentials are encrypted before SQLite persistence.
- [ ] Migration converts every legacy row and verifies decryption before deleting plaintext.
- [ ] SQLite dump and backup metadata do not reveal plaintext passwords.
- [ ] Backup succeeds after process restart with configured key material.
- [ ] Missing/wrong key blocks the affected backup job with an actionable error.
- [ ] Key rotation test decrypts all old records and verifies new writes use the active key.
- [ ] Logs, traces, API responses, and error snapshots contain no credential plaintext.
- [ ] Migration is idempotent and has a tested rollback/restore fixture.

## [ ] P1-06 — Add an automated public-brand consistency gate

**Ownership boundary:** branding inventory, public copy/assets/metadata, and a CI check that distinguishes public product identity from compatibility identifiers. Do not rename Pier runtime interfaces in this task.

### SPEC
- Public-facing product identity must consistently read MOONSHIPS across repository metadata, README/install/migration docs, dashboard title and auth surfaces, support/update links, locale-visible copy, favicon/logo/OG artwork, installer output, and release download URLs.
- Some Pier strings are intentional runtime compatibility identifiers (crate/binary names, systemd units, filesystem paths, environment variables, helper/socket names, and existing storage keys). They must be allowlisted and must not be blindly replaced.
- Current branch has MOONSHIPS logo/icon/OG assets and updates key dashboard/docs/installer surfaces, but a complete automated sweep is needed before declaring the rebrand closed.

### RULE
- A public user-facing brand reference may not accidentally regress to Pier.
- Runtime compatibility identifiers may remain only when explicitly listed in a versioned allowlist with a reason and owner.
- Do not rename runtime identifiers as a cosmetic cleanup.
- Do not change browser storage keys without a dual-read/migration strategy that preserves existing preferences.
- All locale files must preserve the same key set and placeholder structure.

### DESIGN
1. Create a machine-readable branding inventory with approved public brand string, canonical repository URL, support URL, release URL, logo/icon paths, and explicit legacy compatibility allowlist.
2. Add a CI script that scans public-facing files and reports unapproved old-brand references with file/line and category.
3. Scan README/docs, package metadata, template-visible text, metadata tags, locale values, installer/bootstrap output and URLs, service display descriptions, and asset alt/title text.
4. Exclude source identifiers and runtime contracts only through narrow exact-path/pattern allowlist entries; do not broadly exclude all Rust or shell files.
5. Add locale parity checks for keys, interpolation placeholders, and required product name.
6. Validate all brand assets exist, are referenced by the intended surfaces, and are valid SVG/XML.
7. Generate a report listing remaining Pier references grouped into intentional runtime compatibility vs public-facing regression.

### ACCEPTANCE CRITERIA
- [x] CI branding scan passes on the candidate branch (run 42592051587, commit f533eac8b3de20bd6e500636acec64e404bc1f99).
- [x] Every remaining Pier match is either removed from public copy or documented in the path-scoped compatibility allowlist with reason and owner.
- [ ] No public title, header, support link, installer banner, README hero, OG metadata, or release URL points to the old product identity.
- [ ] Legacy runtime identifiers remain unchanged and are covered by compatibility tests.
- [x] Locale files have identical 2,610 keys and placeholder signatures across English, Spanish, and Simplified Chinese.
- [ ] Logo, icon, and OG assets validate and render in a build smoke test.
- [x] The report is deterministic and includes zero unclassified matches.
- [ ] The check runs on every pull request and cannot be skipped for a release candidate.

---

---

# Wave 2 — Runtime reliability and operability (P2)

## [ ] P2-01 — Cancel shell tasks by process group, including descendants

**Ownership boundary:** `crates/pier-agent/src/shell.rs`, process lifecycle helper, agent shell tests. Do not change general deployment job cancellation semantics outside the shell task boundary.

### SPEC
- Static audit found cancellation/timeout sends SIGTERM to the Bash PID only.
- Child processes spawned by Bash may outlive cancellation, keep pipes open, consume resources, or continue side effects.
- A task cancellation must terminate all processes launched in that task's isolated process group.

### RULE
- Never signal a global process name or process group not created by the task.
- Graceful termination precedes force kill.
- Timeout and user cancellation must share the same cleanup path.
- Always reap child processes and close stdout/stderr readers.
- Preserve exit status semantics for normally completed commands.

### DESIGN
1. Spawn each shell task in its own process group/session using platform-supported process APIs.
2. Store process-group identity with the task handle.
3. On cancel/timeout send SIGTERM to the group, wait a bounded grace period, then SIGKILL the group if still alive.
4. Await child exit and stream-reader completion; clean up task state even on error.
5. Add platform-specific implementation behind a narrow abstraction if non-Unix platforms are supported.

### ACCEPTANCE CRITERIA
- [ ] Test shell script spawns nested children; cancellation terminates every descendant.
- [ ] Timeout terminates every descendant and leaves no orphan.
- [ ] Graceful command completion preserves exit code/stdout/stderr.
- [ ] Process outside the task's group remains alive.
- [ ] Repeated cancel/timeout races do not panic, deadlock, or leak handles.
- [ ] Platform limitations are explicit and CI covers every supported target.

---

## [ ] P2-02 — Execute full workspace quality gates in CI

**Ownership boundary:** CI workflow and test harness invocation. Release dependency/gating is owned by P0-02.

### SPEC
- Current CI checks workspace compilation but explicitly runs tests only for the `pier-core` binary target.
- Every workspace member and test target must be executed, including agent, network helper, and test harness crates.
- Security-critical behavior added by P1 tasks must have integration coverage in CI.

### RULE
- Keep `--locked` on Cargo check/test/clippy.
- A skipped test suite must fail or be explicitly represented as a separate non-required platform job with a tracked reason.
- Do not run host-privileged integration tests directly on a shared runner.
- Do not suppress flaky tests; fix determinism or isolate them with an explicit owner and expiry.

### DESIGN
1. Run `cargo fmt --all -- --check`.
2. Run `cargo check --workspace --locked`.
3. Run `cargo clippy --workspace --all-targets --locked -- -D warnings` after resolving current baseline warnings.
4. Run `cargo test --workspace --locked`.
5. Add isolated container-backed integration suites for webhook signatures, DB isolation, credential encryption/migration, backup/restore, and process cancellation.
6. Add shell static analysis and script tests for installer/bootstrap/helper verification.
7. Add Docker build smoke test from P0-01.
8. Publish test reports and preserve failure logs with secret redaction.

### ACCEPTANCE CRITERIA
- [ ] CI enumerates and tests every workspace member and test target.
- [ ] A deliberately failing test makes the required PR check fail.
- [ ] No required test target is silently omitted.
- [ ] Security integration tests run in isolated fixtures/containers.
- [ ] Shell tests cover supported installers and failure paths.
- [ ] Docker clean build is part of required CI.
- [ ] CI artifacts contain test reports but no secrets.
- [ ] CI duration and flaky-test rate are measured; stability is not traded for hidden skips.

---

## [ ] P2-03 — Move blocking SQLite work off async executor workers

**Ownership boundary:** SQLite connection ownership/access layer and its call sites. Do not change schema, authorization, or business semantics as part of this refactor.

### SPEC
- Shared state currently uses a synchronous SQLite connection behind a mutex and async handlers acquire that lock.
- Blocking queries and lock contention can block Tokio workers and serialize unrelated requests.
- Database concurrency must be bounded and transaction behavior preserved.

### RULE
- Do not hold a DB mutex while awaiting network I/O or unrelated filesystem work.
- Respect rusqlite connection thread-safety/ownership constraints.
- Preserve transaction boundaries, busy timeout, WAL behavior, and isolation semantics unless a separately tested change is approved by machine policy.
- Avoid unbounded blocking task creation.

### DESIGN
1. Inventory all connection lock call sites and classify read/write/transaction duration.
2. Add tracing for lock wait, query duration, busy errors, and queue depth before changing architecture.
3. Choose a single ownership model: dedicated DB worker/actor, bounded blocking executor with safe connection ownership, or a supported pool.
4. Route all SQLite operations through the chosen abstraction; prevent direct mutex access from handlers.
5. Bound queue size and define overload/backpressure behavior.
6. Keep migrations and backup operations serialized against conflicting writes.

### ACCEPTANCE CRITERIA
- [ ] No blocking SQLite call runs directly on a Tokio async worker.
- [ ] No DB lock is held across `await`.
- [ ] Concurrency load test shows no starvation/deadlock and bounded queue growth.
- [ ] Existing transaction and rollback tests pass unchanged.
- [ ] Migration/backup cannot race with incompatible writes.
- [ ] Benchmark compares baseline and new design for representative read/write workloads.
- [ ] Metrics expose DB queue wait, query latency, and busy errors.

---

## [ ] P2-04 — Verify backup artifacts and run isolated restore drills

**Ownership boundary:** backup artifact format/verification, scheduled restore-test runner, recovery status UI/API. Credential encryption itself is owned by P1-05.

### SPEC
- A backup marked successful must be complete, readable, and integrity-checked.
- The system must periodically prove that backup artifacts can be restored in an isolated disposable environment.
- Restore rehearsal must not mutate production resources.

### RULE
- Never restore test data into production database, volume, host, or network.
- Do not mark a backup verified merely because a file exists.
- Retention cleanup must never delete the last known-good verified backup for a protected resource.
- Temporary test resources must be cleaned even on failure/cancellation.
- Do not emit backup contents or credentials to logs.

### DESIGN
1. Define a versioned manifest containing backup ID, resource identity, engine/version, created time, format version, component list, sizes, and checksums.
2. Validate checksums and archive readability immediately after backup.
3. Schedule restore drills in isolated containers/namespaces with no route to production networks.
4. Restore representative PostgreSQL/MySQL/MariaDB fixtures and validate schema, row counts/markers, indexes, and application-level invariants.
5. Destroy temporary resources in a guaranteed cleanup/finally path.
6. Store verification result, duration, environment version, and sanitized error reason.
7. Expose last successful backup and last successful restore drill separately.

### ACCEPTANCE CRITERIA
- [ ] Corrupt/truncated/missing backup components are detected.
- [ ] Restore drills succeed for each supported database engine.
- [ ] Restore test cannot reach or overwrite production targets (verified by network and target isolation tests).
- [ ] Temporary containers, volumes, and networks are removed after both success and failure.
- [ ] Retention preserves configured minimum known-good recovery points.
- [ ] UI/API distinguishes created, integrity-checked, restore-verified, failed, and expired.
- [ ] Alert fires on backup failure, integrity failure, or restore-drill SLA breach.
- [ ] Recovery report is machine-readable and contains no secrets.

---

## [ ] P2-05 — Implement one reusable append-only security audit event pipeline

**Ownership boundary:** audit event schema, persistence, query/export API, redaction, retention, and consumers. Task-specific authorization remains in P1-01/P1-02/P1-03.

### SPEC
- Security-sensitive actions need consistent traceability across authentication, role changes, security settings, DB Browser access, deployment, webhook configuration, credential rotation, backup/restore, and privileged operations.
- Existing ad hoc logs are not a substitute for a structured audit event store.
- This task provides the common event pipeline; it must not duplicate authorization logic in each feature.

### RULE
- Audit records are append-only through the application interface.
- Ordinary users cannot edit/delete audit events.
- Never store passwords, tokens, private keys, webhook signatures, raw credential values, or raw SQL parameters.
- Audit storage failure for a high-risk action must follow a documented fail-closed policy; do not silently lose the event.
- Audit API access is itself permission-checked and auditable.

### DESIGN
1. Define versioned event schema: event ID, timestamp UTC, actor ID/type, action, target type/ID, project/tenant scope, outcome, source/request ID, and redacted metadata.
2. Provide typed event constructors so features cannot invent inconsistent field names.
3. Persist events transactionally where practical with the action; use an outbox pattern where cross-resource atomicity is impossible.
4. Add tamper-evidence via chained digest or equivalent integrity mechanism, with periodic verification.
5. Add query filters, pagination, export, retention, and optional syslog/SIEM forwarding.
6. Add automated redaction validation at the event boundary.
7. Integrate existing security settings, DB Browser, webhook, deployment, backup, and credential events without changing their authorization decisions.

### ACCEPTANCE CRITERIA
- [ ] All listed action families emit normalized events with actor, target, action, outcome, and timestamp.
- [ ] Failed and denied attempts are represented without leaking request secrets.
- [ ] Application API cannot update/delete existing audit records.
- [ ] Tamper-evidence verification detects modified/deleted records in test fixtures.
- [ ] Query/export permissions are enforced server-side.
- [ ] Retention and export are deterministic and covered by tests.
- [ ] Redaction tests inject dummy passwords/tokens/private keys and prove they are absent.
- [ ] Audit pipeline outage behavior is tested for each critical action class.

---

## [ ] P2-06 — Add scoped, expiring, revocable API tokens

**Ownership boundary:** API token model, storage, authentication middleware, management endpoints/UI. Do not change existing browser session authentication.

### SPEC
- Automation/integrations need credentials narrower than a human admin session.
- Tokens must be scoped by capability and resource, have expiry, and be revocable independently.
- Token plaintext is displayed once at creation; only a secure verifier/hash is persisted.

### RULE
- No implicit global-admin scope.
- Default scope is empty/minimum and expiry is bounded.
- Revoked/expired tokens fail immediately.
- Token owner cannot grant a scope they do not possess.
- Token authentication must not bypass tenant/project/resource authorization.

### DESIGN
1. Define token metadata: ID, owner, hashed secret, scope list, resource allowlist, created/expiry/last-used/revoked timestamps.
2. Use a cryptographically secure random token and a suitable one-way verifier; compare safely.
3. Add per-request scope and resource checks after token authentication.
4. Add create/list/revoke/rotate endpoints and UI.
5. Add rate limits and last-used metadata without storing plaintext token.
6. Support migration-free coexistence with session auth.

### ACCEPTANCE CRITERIA
- [ ] Token cannot invoke endpoint outside its scope.
- [ ] Token cannot access resource outside its allowlist.
- [ ] Expired and revoked tokens fail immediately.
- [ ] Token plaintext is returned only once and never appears in DB/logs.
- [ ] Owner cannot create higher privilege than their own capability.
- [ ] Session login and existing API behavior remain regression-tested.
- [ ] Token creation, scope change, rotation, and revocation emit audit events.

---

# Wave 3 — Product expansion (P3)

> These are independent product initiatives. Each must be delivered behind a feature flag and may not block completion of P0–P2. “No human approval required” means rollout decisions are driven by explicit machine policies and automated acceptance tests; production activation remains outside this backlog.

## [ ] P3-01 — Health-gated deployment with automatic rollback

**Ownership boundary:** deployment state machine, health probe execution, rollback orchestration. Do not implement GitOps reconciliation or canary traffic shaping here.

### SPEC
- A deployment is not successful merely because the container started.
- New release must pass configurable readiness/health criteria before being marked healthy.
- When a new release fails the configured gate, system automatically restores the last known-good deployment when rollback is safe.

### RULE
- Rollback is allowed only when a known-good image/config snapshot exists and compatibility policy says rollback is safe.
- If a migration is marked irreversible or incompatible, stop promotion and enter a blocked state rather than attempting destructive rollback.
- All thresholds are explicit configuration; no hidden defaults that can cause an infinite wait.
- No manual approval state is part of the state machine.

### DESIGN
1. Model deployment states: queued, preparing, deploying, probing, healthy, promoted, rollback-in-progress, rolled-back, blocked, failed.
2. Support HTTP/TCP/container health checks with startup grace, interval, timeout, consecutive failure count, and total deadline.
3. Persist previous image digest, config revision, deployment ID, and rollback eligibility.
4. Automatically rollback on threshold breach when safe; otherwise freeze traffic and emit actionable alert.
5. Record state transitions and probe evidence.
6. Add deterministic failure injection for crash loop, probe timeout, dependency unavailable, and rollback failure.

### ACCEPTANCE CRITERIA
- [ ] Healthy deployment promotes only after all configured probes pass.
- [ ] Failed probes stop promotion and trigger safe automatic rollback.
- [ ] Irreversible migration policy blocks unsafe rollback and records the reason.
- [ ] No deployment can remain indefinitely in a transitional state.
- [ ] Every state transition is auditable.
- [ ] Test suite covers success, crash loop, timeout, rollback success, and rollback failure.
- [ ] Feature can be disabled without changing legacy deployment behavior.

---

## [ ] P3-02 — Add canary/blue-green rollout using deterministic traffic policy

**Ownership boundary:** traffic splitting and staged rollout only. Health gate and rollback mechanics are provided by P3-01.

### SPEC
- Multi-instance services should support progressive exposure of a new version to a controlled percentage of traffic.
- Rollout progression is driven by configured health/error/latency thresholds.

### RULE
- Traffic weights must sum to 100% and be validated before rollout.
- If metrics are missing or stale, rollout pauses/fails closed; it must not infer success.
- Do not apply canary to single-instance services or unsupported routing providers.
- No human approval action is required; configured policy determines progression.

### DESIGN
1. Define rollout plan schema: strategy, steps, traffic weights, observation window, metric thresholds, timeout, and rollback action.
2. Implement blue-green target registration and atomic route switch.
3. Implement canary traffic weights for supported proxy providers.
4. Evaluate health and error/latency metrics at each stage.
5. Feed failure decisions to P3-01 rollback state machine.
6. Keep legacy deployment path unchanged when feature is disabled.

### ACCEPTANCE CRITERIA
- [ ] Invalid weights and unsupported services are rejected before changes.
- [ ] Traffic weights at each stage match configured plan.
- [ ] Missing/stale telemetry blocks progression.
- [ ] Threshold breach triggers automated rollback.
- [ ] Route switch is atomic or has a tested bounded inconsistency window.
- [ ] Tests cover each rollout step, timeout, telemetry loss, and rollback.
- [ ] Feature flag off preserves existing behavior.

---

## [ ] P3-03 — Build Security Center with explainable findings and SBOM

**Ownership boundary:** scanner orchestration, finding model, SBOM storage/display. Do not duplicate secret redaction or audit event infrastructure.

### SPEC
- Provide a unified view of image vulnerabilities, dependency CVEs, exposed ports, risky runtime configuration, image provenance, and repository secret-pattern findings.
- Every finding must be actionable and tied to a concrete artifact/service.

### RULE
- A numeric score must never be represented as a guarantee of security.
- No secret value is transmitted to an external scanner.
- Findings cannot be silently suppressed; accepted-risk entries require reason, owner identity, expiry, and audit event.
- Scanner failure must be distinguishable from zero findings.

### DESIGN
1. Generate SBOM for each immutable image digest/build.
2. Define normalized finding schema: scanner, rule ID, severity, evidence reference, affected artifact, first/last seen, remediation, status.
3. Run scanners as isolated jobs with bounded CPU/memory/time and no unnecessary secret access.
4. Correlate findings with service, host, deployment, and image digest.
5. Add trend view, filters, machine-readable export, and expiring risk exceptions.
6. Show scan coverage and stale/missing scan state.

### ACCEPTANCE CRITERIA
- [ ] SBOM is linked to immutable image digest and can be exported.
- [ ] Finding has severity, evidence, remediation, affected asset, and scan timestamp.
- [ ] Scanner error is not displayed as a clean scan.
- [ ] Risk exception has owner, reason, expiry, and audit event.
- [ ] Secret scanning fixtures prove no raw secret leaves the platform.
- [ ] Repeated scans deduplicate stable findings and preserve history.
- [ ] Feature flag off has no impact on deployment.

---

## [ ] P3-04 — Add multi-server Fleet Manager and safe agent lifecycle

**Ownership boundary:** host registry, enrollment, heartbeat, inventory, agent upgrade orchestration. Does not own application deployment strategy.

### SPEC
- Operators need a consistent view of multiple hosts, agent health/version/capabilities, capacity, labels, and maintenance state.
- Enrollment and agent upgrade must be secure, observable, and idempotent.

### RULE
- Host identity is unique and cryptographically bound to enrollment.
- Enrollment tokens are one-time, short-lived, and scoped.
- Offline/stale agents are never reported as successfully updated.
- Mass operations use bounded batches and stop automatically on failure thresholds.

### DESIGN
1. Add host registry with stable ID, identity key, labels, region/environment, agent version, capabilities, heartbeat, and last-seen.
2. Implement one-time enrollment and revoke/rotate flow.
3. Add health states: online, degraded, stale, offline, revoked.
4. Add maintenance/drain mode and rolling agent upgrade with batch size and failure threshold.
5. Add per-host action results and retry/backoff.
6. Reuse P2-05 audit events.

### ACCEPTANCE CRITERIA
- [ ] Duplicate or forged host identity is rejected.
- [ ] Expired/reused enrollment token is rejected.
- [ ] Offline/stale hosts are visibly distinct from healthy hosts.
- [ ] Upgrade proceeds in bounded batches and halts on configured failure threshold.
- [ ] Per-host result is persisted and retry is idempotent.
- [ ] Revoke prevents future authenticated agent actions.
- [ ] Host enrollment, upgrade, maintenance, and revoke emit audit events.

---

## [ ] P3-05 — Implement GitOps manifests and configuration drift detection

**Ownership boundary:** desired-state schema, Git source sync, drift diff/reconcile. Does not own secret manager or traffic rollout internals.

### SPEC
- Service configuration should be represented as versioned desired state in Git.
- Platform compares desired state with live state and reports drift.
- Reconciliation must be safe, idempotent, and policy-driven.

### RULE
- Secret values are never committed or printed in diffs; manifests contain secret references only.
- Invalid manifests never partially apply.
- Reconcile loop has rate limits, retry bounds, and stale-source detection.
- No manual approve button is required; preconfigured environment policy determines whether a diff is auto-applied, blocked, or reverted.

### DESIGN
1. Define versioned schema for services, image/source, environment references, ports, volumes, networks, healthchecks, deploy policy, and dependencies.
2. Validate schema and semantic constraints before any mutation.
3. Fetch Git desired state using least-privilege deploy key/token.
4. Compute redacted diff between desired and live state.
5. Apply only allowed change classes according to environment policy.
6. Track sync status: in-sync, drifted, reconciling, blocked, failed, stale-source.
7. Support rollback to a previous Git commit using the normal deployment/backup contracts.

### ACCEPTANCE CRITERIA
- [ ] Invalid schema or incompatible version is rejected before mutation.
- [ ] Drift identifies exact changed fields without showing secret values.
- [ ] Auto-reconcile only applies changes allowed by stored policy.
- [ ] Repeated reconcile is idempotent.
- [ ] Stale Git source and API failures do not overwrite live state.
- [ ] Rollback to previous commit is testable and auditable.
- [ ] Secret scanning proves no plaintext secret is present in manifest/diff/log.

---

## [ ] P3-06 — Add resource intelligence and capacity forecasting

**Ownership boundary:** metrics collection, retention/downsampling, dashboards, threshold alerts, forecast. Does not own host enrollment (P3-04).

### SPEC
- Provide per-host and per-service CPU, memory, disk, network, restart, and capacity history.
- Forecasts must communicate uncertainty and data coverage.

### RULE
- Metrics must have timestamp, source, sampling interval, and quality/coverage metadata.
- Forecast is an estimate, not a guarantee.
- Sampling must not materially degrade agent or server performance.
- Missing telemetry must be shown as missing, never as zero.

### DESIGN
1. Define versioned metric schema and bounded sampling cadence.
2. Aggregate/downsample metrics and apply retention tiers.
3. Add CPU/RAM/disk/network dashboards by host/service/time range.
4. Add configurable thresholds and alert deduplication.
5. Start with transparent trend-based forecasting and confidence intervals before complex models.
6. Measure agent/server overhead and tune collection budget.

### ACCEPTANCE CRITERIA
- [ ] Dashboard filters by host, service, and time range.
- [ ] Missing/stale telemetry is visibly distinct from zero usage.
- [ ] Disk pressure and capacity alerts are covered by deterministic tests.
- [ ] Forecast shows horizon, data coverage, and uncertainty.
- [ ] Retention/downsampling policy is configurable and tested.
- [ ] Collection overhead stays within a documented CPU/memory budget.

---

## [ ] P3-07 — Build Disaster Recovery Wizard with automated preflight and verification

**Ownership boundary:** recovery orchestration UI/API and recovery report. Backup artifact validation and restore drills are owned by P2-04.

### SPEC
- Guide an operator through selecting a verified backup, validating target compatibility/capacity, restoring, and verifying service recovery.
- Provide a rehearsal mode that restores into an isolated environment.

### RULE
- The wizard must not silently overwrite a target with incompatible data.
- Production restore is not part of automated backlog tests; tests use isolated disposable targets.
- Preflight failure blocks execution deterministically.
- No human approval step is needed in the task's automated test flow; configured policy and explicit execution context determine permitted targets.

### DESIGN
1. Flow: select backup → verify manifest/checksum → resolve target → compatibility preflight → capacity preflight → execute in isolated test or authorized target → post-restore verification → report.
2. Show data age, components, schema version, target identity, and expected overwrite scope.
3. Reuse restore engine from P2-04.
4. Generate machine-readable recovery report and audit events.
5. Add failure recovery and cleanup for interrupted restore.

### ACCEPTANCE CRITERIA
- [ ] Corrupt backup, incompatible version, insufficient capacity, and unknown target block before mutation.
- [ ] Rehearsal restores only into isolated temporary resources.
- [ ] Post-restore verification checks schema and application-level markers.
- [ ] Interrupted restore leaves target in a deterministic recoverable state.
- [ ] Report includes source backup, target, duration, verification result, and sanitized errors.
- [ ] All restore attempts are audited.

---

## [ ] P3-08 — Add redacted one-click diagnostics bundle

**Ownership boundary:** diagnostic collection allowlist, redaction, archive generation, UI/API. Do not add general-purpose file browser access.

### SPEC
- Produce a support bundle containing version/build metadata, sanitized config, service status, redacted logs, agent health, and relevant system metrics.
- Bundle generation must be bounded and safe to share.

### RULE
- Redaction runs before archive creation.
- No raw database dumps, persistent volume data, private keys, tokens, passwords, or unrestricted environment files by default.
- User can inspect included categories and redaction summary before export.
- Bundle size and collection time are bounded.

### DESIGN
1. Define strict allowlist of source files/commands and category toggles.
2. Add centralized redaction patterns for credentials, DSNs, tokens, private keys, cookies, and configurable personal data.
3. Generate manifest with included files, collection timestamps, redaction counts, and checksum.
4. Limit archive size and command execution time.
5. Add cancellation and cleanup for partial bundles.
6. Add test fixtures with synthetic secrets for every supported pattern.

### ACCEPTANCE CRITERIA
- [ ] Synthetic secrets are absent from generated bundle and manifest.
- [ ] Disallowed files/directories cannot be included through user path input.
- [ ] Raw DB/volume data is excluded by default and cannot be included through traversal.
- [ ] Bundle size/time limits are enforced.
- [ ] Cancellation removes partial temporary artifacts.
- [ ] Manifest and checksum verify after generation.

---

## [ ] P3-09 — Add WebAuthn/passkeys as phishing-resistant authentication

**Ownership boundary:** WebAuthn credential registration/authentication, recovery factors, policy enforcement. Do not redesign existing password/session authentication.

### SPEC
- Support passkeys for admin and user accounts with multiple authenticators and safe recovery.
- Enforce correct origin and relying-party identity.

### RULE
- Origin/RP ID mismatch always fails.
- Recovery cannot bypass account security policy.
- Credential enrollment, revoke, and recovery events are audited.
- Existing login remains available according to configured migration policy.

### DESIGN
1. Implement WebAuthn registration and assertion using a maintained library.
2. Store credential ID, public key, counter/backup state, transports, and timestamps; never store private authenticator secrets.
3. Support multiple authenticators, revoke, and recovery codes.
4. Add policy by role/organization with machine-enforced enforcement.
5. Add rate limits and replay/counter validation.

### ACCEPTANCE CRITERIA
- [ ] Supported browser/platform registration and login tests pass.
- [ ] Invalid origin, RP ID, challenge, signature, and replay are rejected.
- [ ] Revoke immediately prevents future authentication using that credential.
- [ ] Recovery codes are one-time and stored as secure verifiers.
- [ ] Policy enforcement is server-side and auditable.
- [ ] Existing login regression suite remains green.

---

## [ ] P3-10 — Define a capability-scoped integration/plugin contract

**Ownership boundary:** event/webhook integration contract and capability manifest. Arbitrary executable plugin runtime is explicitly out of scope until a separate threat model exists.

### SPEC
- Provide safe extension points for notifications, monitoring, deployment events, and secret-provider adapters.
- Integrations must declare the exact capabilities they need.

### RULE
- No plugin inherits global-admin rights by default.
- No arbitrary code execution in the initial SDK.
- Network/secret access must be explicit and scoped.
- Event delivery is versioned, bounded, idempotent, and revocable.

### DESIGN
1. Define plugin manifest with ID, version, compatibility range, requested scopes, event subscriptions, and endpoint metadata.
2. Define versioned event schema and signature verification.
3. Add capability evaluation and per-plugin secret references.
4. Implement bounded delivery with retries, backoff, idempotency key, timeout, and dead-letter status.
5. Add enable/disable/revoke and audit events.
6. Reserve executable sandbox runtime for a later separately scoped project.

### ACCEPTANCE CRITERIA
- [ ] Plugin with no capability cannot access protected resources.
- [ ] Scope escalation is rejected.
- [ ] Event delivery is idempotent and bounded.
- [ ] Failed endpoint does not block core deployment/API work.
- [ ] Disable/revoke prevents subsequent deliveries.
- [ ] Manifest compatibility is validated before activation.
- [ ] No arbitrary plugin code executes in this phase.

---

# Cross-task dependency map

| Task | Depends on | Must not duplicate |
|---|---|---|
| P0-01 Docker workspace build | Current Cargo workspace inventory | Runtime refactor |
| P0-02 Release gate | CI check names from P2-02 | Helper signature generation (P1-04) |
| P1-01 Global security RBAC | Existing auth/role policy | Generic audit store (P2-05); emit through event interface |
| P1-02 DB Browser isolation | Resource registry and DB provisioning | Backup credential storage (P1-05) |
| P1-03 Webhook verification | Provider config/secret store | Deployment rollback (P3-01) |
| P1-04 Helper integrity | Release artifact metadata | CI gate itself (P0-02) |
| P1-05 Backup credential encryption | Host secret/key source | DB Browser runtime credential model (P1-02) |
| P2-01 Process tree cancellation | Agent task lifecycle | Deployment state machine |
| P2-02 Full workspace CI | Workspace member inventory | Release dependency (P0-02) |
| P2-03 SQLite execution model | Existing DB call-site inventory | Schema/business logic changes |
| P2-04 Backup verification/restore drills | Backup format and encrypted credential access | DR Wizard UI (P3-07) |
| P2-05 Audit pipeline | Event schema and persistence | Feature-specific authorization |
| P2-06 Scoped API tokens | Existing auth middleware/policy | Browser session redesign |
| P3-01 Health gate/rollback | Existing deployment lifecycle | Traffic shaping (P3-02) |
| P3-02 Canary/blue-green | P3-01 health/rollback interfaces | Health probe engine |
| P3-03 Security Center | Build/image metadata | General audit pipeline |
| P3-04 Fleet Manager | Agent identity/heartbeat | Application deployment policy |
| P3-05 GitOps | Manifest schema and deployment API | Traffic shaping internals |
| P3-06 Resource Intelligence | Agent metrics | Host enrollment |
| P3-07 DR Wizard | P2-04 restore/verification engine | Backup artifact implementation |
| P3-08 Diagnostics | Redaction utility | General-purpose filesystem access |
| P3-09 Passkeys | Existing authentication policy | Session architecture replacement |
| P3-10 Integration SDK | Event and capability model | Arbitrary executable plugin runtime |

# Automated release readiness gate

A release candidate is ready only when the following machine-verifiable conditions are true:

- [ ] P0-01 and P0-02 pass on the exact candidate commit SHA.
- [ ] P1-01 through P1-05 pass their security negative tests.
- [ ] P2-01 through P2-04 pass runtime and recovery tests.
- [ ] P2-05 audit pipeline integrity and redaction tests pass.
- [ ] P2-06 token scope/expiry/revocation tests pass if scoped tokens are enabled in the release.
- [ ] All required CI jobs complete successfully; no required job is skipped/cancelled.
- [ ] Docker image build and startup smoke tests pass.
- [ ] Migration fixture from the oldest supported Pier schema/config passes and is idempotent.
- [ ] Backup/restore fixture passes for every supported DB engine.
- [ ] No secret-scanning, dependency-audit, or artifact-integrity gate reports an unhandled critical finding.
- [ ] Release artifacts have source SHA, checksums, and provenance metadata.
- [ ] Release notes and migration/rollback documentation are generated and versioned.
- [ ] Production deployment is explicitly outside this automated release-readiness checklist.

# Definition of Done template for future tasks

Every new task must use this exact structure:

- [ ] **TASK-ID — Imperative title**
  - **SPEC:** current observed problem, exact scope, explicit non-goals, compatibility contract, expected behavior.
  - **RULE:** authorization, trust boundary, invariants, failure modes, privacy/security limits, resource/time limits.
  - **DESIGN:** affected files/modules, API/schema contract, data flow, migration, observability, cleanup, rollback.
  - **ACCEPTANCE CRITERIA:** binary/pass-fail criteria with expected inputs/outputs and negative cases.
  - **AUTOMATED TEST PLAN:** unit, integration, regression, migration, failure injection, platform matrix.
  - **AUTOMATED EVIDENCE:** CI run URL/SHA, test reports, benchmark, migration fixture output, artifact digest.
  - **ROLLBACK:** deterministic rollback trigger and procedure, including data compatibility.
  - **DEPENDENCIES:** exact task IDs; do not restate another task's implementation scope.

**Maintainer note:** Keep the backlog single-sourced. If a new finding overlaps an existing task, update that task's SPEC/RULE/DESIGN/ACCEPTANCE CRITERIA or add a clearly non-overlapping subtask; do not create duplicate tickets. P0/P1 precede feature work. All checks are automated and objective; no task is considered complete on a subjective human sign-off. No task authorizes production changes.


### Execution evidence and remaining external blocker

- Source/docs/UI/installer/locale/assets branding changes are implemented on `rebrand/moonships-full-brand`; Open Graph image references use an absolute raw-GitHub URL so social crawlers can resolve the image after merge.
- The branding gate, its unit tests, release-gate tests, shell syntax, Rust formatting, workspace check, and core binary tests passed on commit `f533eac8b3de20bd6e500636acec64e404bc1f99`.
- A clean Docker build was still running at the time of this update; its result must be checked on the final commit.
- **External repository metadata is not complete:** `GET /repos/BRIDGE-ALLAH/MOONSHIPS` reports `homepage=https://pier.team`. The connected GitHub integration exposes repository metadata as read-only and has no repository-settings write action. A repository administrator must clear that website field or replace it with the official MOONSHIPS website. The repository description should also be updated to begin with `MOONSHIPS —`.
- Branch-protection enforcement of the branding check cannot be verified with the current connection. Do not mark P1-06 complete until repository homepage/description and required-check enforcement are confirmed.
