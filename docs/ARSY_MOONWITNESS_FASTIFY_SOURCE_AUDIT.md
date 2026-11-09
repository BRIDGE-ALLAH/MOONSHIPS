# ARSY Source Implementation Notes — MOONWITNESS Fastify Candidate

Audit date: 2026-11-09  
Source: `BRIDGE-ALLAH/MOONWITNESS`, default branch `main`.  
Important: this is a targeted source read, not a complete security audit or test run.

## What is present in source

The ARSY package manifest identifies `@riotworks/arsy`, Fastify 5, PostgreSQL (`pg`), Zod 4, and shared workspace packages `@riotworks/arsy-core` and `@riotworks/contract`. It has scripts for type-check, lint, unit tests, integration tests and database migrations.

The application composition in `apps/arsy/src/app.ts` creates a Fastify app, PostgreSQL pool, Oracle client, operator authentication hook and registers these bounded route groups:
- Docker connector
- Deployment operations
- Evidence
- Governance
- Login attempts
- Opportunities
- Products
- Assets
- Channels
- Listings
- Inspections
- Financial operations and payout/FX
- Portfolio metrics
- ORACLE integration

This is materially more than a minimal HTTP skeleton. However, route registration and package presence do not prove each domain is complete, correctly authorized, or production-ready.

## Authentication model observed

`apps/arsy/src/auth/operator-token.ts`:
- Reads `ARSY_OPERATOR_TOKEN` and `ARSY_OPERATOR_ID` from server environment.
- Applies a Fastify pre-handler to paths beginning `/api/v1/`.
- Uses a constant-time token comparison for `x-arsy-operator-token`.
- Fails closed with HTTP 503 when operator credentials are not configured and HTTP 401 for invalid credentials.
- Establishes a principal with `kind: 'operator'` and a configured operator ID.
- Leaves non-`/api/v1/` routes outside this hook, including public health/liveness routes.

This appears to be a bounded single-operator/service credential boundary, not evidence of complete end-user identity, multi-tenant session management, federation or general RBAC. The actual policy/approval model must be read from the route-specific authorization modules and the broader capability contract.

## Deployment authorization signals

`apps/arsy/src/deployments/authorization.ts` contains a more sophisticated authorization boundary than the simple operator token alone:
- Operation context includes owner, actor, deployment, Docker server, action, resource, argument hash, tenant, purpose and session identifiers.
- Capability grants are represented with issuer, subject, tenant, purpose, session, actions, resources, argument hashes and expiry.
- Grant verification uses trusted public keys and Ed25519 signatures.
- Approval records include requester, approver, policy version, decision, reason and validity window.
- Policy providers are bounded by an AbortSignal and timeout handling.

These are meaningful implementation signals. They do not yet establish the completeness of the system-wide identity/tenant model; verify the call sites, key rotation, revocation, grant delegation, approval separation-of-duties, persistence constraints, replay resistance and tests.

## Docker connector boundary observed

`apps/arsy/src/docker/routes.ts`:
- Restricts local Docker socket configuration to `/var/run/docker.sock` or `/run/docker.sock`.
- For remote Docker, resolves the endpoint and validates/encrypts TLS credentials before use.
- Stores server ownership and connector audit events in PostgreSQL.
- Scopes server listing and lookup by configured operator owner ID.
- Exposes information, container listing, logs and stats in the inspected route sections.
- The MOONWITNESS README explicitly describes the authenticated Docker connector as deliberately read-only and bounded.

The read-only boundary is an important constraint to preserve. Deployment mutation is a distinct route group and has its own authorization machinery; do not infer that Docker connector read endpoints grant permission to start/stop/exec/deploy containers.

## Data and lifecycle

- `apps/arsy/src/db.ts` requires `DATABASE_URL`, creates a PostgreSQL pool with max 20 connections and 30-second idle timeout, and closes the pool on app shutdown.
- `apps/arsy/src/index.ts` validates host/port configuration, starts Fastify and registers graceful SIGTERM/SIGINT shutdown.
- `apps/arsy/src/config.ts` defaults to loopback host `127.0.0.1` and port `3001`.

The source uses PostgreSQL as an ARSY service store, whereas current MW's mainline uses SQLite/rusqlite. A migration decision must preserve domain ownership and define whether ARSY state is consolidated into MW's primary database or remains an isolated service datastore. Do not merge schemas without a data ownership and migration plan.

## Important documentation context

MOONWITNESS docs include:
- `docs/32-universe-and-product-architecture-registry.md`: distinguishes core product experiences, prototypes, targets and current workspace maturity.
- `docs/33-platform-capability-ownership-and-dependency-map.md`: documents one-owner-per-capability, ARSY as control-plane authority, read-only Docker connector, separate build/deploy and queue/business-state boundaries.
- `docs/adr/0002-docker-workload-deployment-ownership.md`: referenced as accepted in the capability map dated 2026-11-08.

This ownership model is useful migration input, but it must be reconciled with the target MW architecture and source code before ratification.

## Required next verification

1. Fetch exact source commit SHA and all migrations for ARSY and shared packages.
2. Enumerate every registered route and map it to contract, owner, tenant boundary, auth hook and tests.
3. Inspect every deployment authorization call site, policy provider, grant store, approval store and signature key lifecycle.
4. Inspect Docker remote endpoint DNS/IP validation and TLS credential handling for SSRF, rebinding, certificate validation, secret encryption and log leakage.
5. Inspect PostgreSQL migrations, constraints, indexes, transaction behavior and tenant scoping.
6. Inspect evidence/governance modules for provenance, audit immutability, authorization and retention.
7. Run source CI and tests from the exact pinned commit in a controlled environment before proposing any import.
8. Map each domain to an MW destination module and define data/API compatibility before migration.

## Provisional disposition

Keep MOONWITNESS Fastify ARSY as a high-value behavior and contract reference. Do not import the Fastify server as a second long-term backend authority. Extract and port only verified domain behavior that has no suitable Rust/MW owner, preserving route compatibility where required and retaining the original repository history.
