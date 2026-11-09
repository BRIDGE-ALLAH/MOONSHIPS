# Federation Security Review — 2026-11-09

## Scope and disposition

This is a source-level review of the current `main` snapshot. It is not a penetration test and does not establish that every route, deployment, or production environment is affected. Findings below are integration blockers until resolved and regression-tested.

Reviewed files:
- `crates/pier-core/src/federation/client.rs`
- `crates/pier-core/src/federation/write_client.rs`
- `crates/pier-core/src/api/federation.rs`
- `crates/pier-core/src/auth/federation.rs`
- `crates/pier-core/src/auth/rbac/policy.rs`

## F-01 — Federation HTTP clients disable TLS certificate validation

**Severity: Critical for credential confidentiality; P0 integration blocker**

Both federation HTTP client factories explicitly call `danger_accept_invalid_certs(true)`:
- `federation/client.rs`: `client()`, currently used for read-sync with `X-Pier-Peer-Token`.
- `federation/write_client.rs`: `client()`, currently used for remote stack mutations with `X-Pier-Federation`.

The comments justify this as support for self-signed certificates. That rationale does not protect the bearer-like credentials: an active network attacker able to intercept or redirect the connection can impersonate the peer and capture the token, or alter responses/operations. Upgrading `http://` to `https://` does not provide server authentication while certificate validation is disabled.

**Required remediation**
1. Restore normal certificate and hostname validation by default.
2. Add an explicit per-peer trust model for private/self-signed deployments: operator-installed CA roots or a pinned certificate/public key with rotation and revocation procedures. Never make insecure validation a silent global default.
3. Ensure the certificate identity matches the configured DNS name or IP SAN; do not disable hostname checks to accommodate mesh IPs.
4. Keep separate read and write scopes, add bounded timeouts, and redact authorization headers/tokens from all diagnostics.
5. Add tests proving invalid, expired, wrong-host and untrusted certificates fail; trusted custom CA/pin succeeds; rotation/revocation is enforced.

**Do not** simply remove the flag without implementing the documented private-certificate path: that would break existing self-signed peer installations. No production code change is made in this review.

## F-02 — Long-lived federation credential is returned to browser and placed in WebSocket query string

**Severity: High; P0 integration blocker**

`api/federation.rs::peer_stack_logs_ws_info` returns both `ws_url` and the plaintext `peer.token` to the browser. The comments state that the browser appends it as `?token=...`.

`auth/federation.rs::require_federation` accepts that query-string token as a fallback for requests protected by the federation middleware. Query-string credentials can be retained in browser history, reverse-proxy/access logs, tracing systems, telemetry, error reports, and copied URLs. The credential is a long-lived federation token, not a narrowly scoped one-time WebSocket credential.

**Required remediation**
1. Do not expose the stored long-lived federation token to browser code.
2. Introduce a short-lived, single-use, stack-scoped WebSocket ticket minted by the primary after checking the user's session and project/resource permission.
3. Bind the ticket to peer, stack, audience, expiry, and nonce; consume it atomically once and reject replay.
4. Prefer a WebSocket subprotocol or a dedicated handshake exchange for the ticket. If a query parameter remains unavoidable, ensure it is short-lived and redact it from every proxy, application, and telemetry log.
5. Remove general long-lived token query fallback after a compatibility window; keep server-to-server authentication in the dedicated header.
6. Add tests for expiry, replay, wrong peer/stack, revoked session, unauthorized user, and token redaction.

## F-03 — RBAC route coverage is explicitly incremental

**Severity: High; P0 before multi-domain integration**

`auth/rbac/policy.rs` states that project-scoped permission variants are wired incrementally. The central `can()` function has project View/Edit/Admin checks and peer-specific rules, but the presence of enum variants does not prove every handler invokes the appropriate guard.

**Required remediation**
1. Build a machine-readable route-to-permission matrix from Axum route registration and handler guards.
2. Fail closed for mutating routes without an explicit permission mapping.
3. Add negative tests for unauthenticated, viewer, editor, project-admin, global-admin, owner, peer, and federation-token principals as applicable.
4. Verify tenant/resource ownership in every query and mutation; a route-level role check alone is insufficient.

## F-04 — Peer endpoint URL and network boundary review remains open

**Severity: High; unresolved**

The peer clients derive outbound destinations from persisted peer URL/host/port and mesh-address fields. This review has not yet verified all creation/update guards, DNS resolution behavior, redirect policy, private/link-local/loopback restrictions, or whether an administrator-controlled URL can be influenced by lower-privilege users.

**Required remediation**
1. Trace every write path for peer URL, host, port, and mesh IP.
2. Restrict configuration to authorized Owner/Admin principals and validate URL scheme, host, port, and address class.
3. Disable cross-origin redirects for credential-bearing requests, or revalidate destination and strip credentials on redirect.
4. Resolve and validate DNS results at connection time to mitigate DNS rebinding; explicitly define allowed private mesh ranges.
5. Add SSRF regression tests for loopback, link-local, cloud metadata addresses, IPv4/IPv6 alternate encodings, DNS rebinding, and redirects.

## F-05 — Federation ownership model is a useful boundary, but requires route-wide proof

The agent API checks stack ownership against the current federation token ID in `assert_owned`. This is a useful isolation mechanism for resources created by a primary. It does not replace session/RBAC checks on the primary, nor prove that every peer-facing route uses the ownership check.

## Integration gate

No code from other repositories may be imported into the federation/auth/network surface until F-01 through F-04 have remediation plans, tests, and a route/tenant coverage report. Existing behavior and self-hosted compatibility must be preserved through explicit trust configuration and migration tests.

## Review limits

- Source inspection only; no live penetration test was performed.
- No production configuration, logs, network traffic, or secret stores were accessed.
- No TLS behavior, route coverage, DNS/SSRF behavior, or WebSocket replay behavior is declared fixed.
- No source repositories were modified, archived, or deleted.
