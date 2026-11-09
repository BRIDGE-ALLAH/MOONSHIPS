# MW-GATEWAY Provenance and Fixture Review — 2026-11-09

## Scope

Source-level review of the public `BRIDGE-ALLAH/MW-GATEWAY` repository at the current default-branch snapshot. This is not a full dependency audit, binary/package audit, or penetration test.

## Repository provenance

- GitHub repository: `BRIDGE-ALLAH/MW-GATEWAY`
- Root license file: Apache License 2.0.
- Recursive tree contained 3,438 entries at review time and was not truncated.
- The project identifies as an agentgateway / HTTP / LLM / policy data-plane workspace. It is a substantial platform, not a small library suitable for copying wholesale.
- Any integration must preserve upstream notices, copyright, dependency provenance, and license obligations. The repository's Apache-2.0 license does not automatically relicense every bundled dependency or fixture.

## G-01 — Testbox contains a certificate and private key

The tree contains:
- `controller/hack/testbox/dummy-idp.cert` (1,964 bytes)
- `controller/hack/testbox/dummy-idp.key` (3,272 bytes)

The certificate PEM identifies a dummy IDP subject (`dummy-idp.default, O=kgateway.dev`) and a validity window from 2025-12-12 through 2035-12-10. The adjacent file is a PEM-encoded private key.

**Assessment:** path and certificate naming strongly indicate a test fixture, but this alone does not prove that it is synthetic, unused by release workflows, or excluded from container/package artifacts. It is not evidence by itself of a production credential leak.

**Required before import or release use**
1. Trace all references to the fixture and establish its generation/ownership.
2. Verify the certificate's public key matches the checked-in private key and determine issuer/subject/SAN/validity with an independent certificate parser.
3. Prove the fixture is restricted to tests and local testbox tooling; ensure no production chart, image, release archive, example deployment, or runtime default includes it.
4. Add secret scanning with a narrow, documented test-fixture exception; do not blanket-ignore PEM private keys.
5. Rotate/remove it if it is not demonstrably synthetic or if any non-test use is found.
6. Ensure CI and release packaging tests assert the fixture paths are absent from deliverables.

## Integration decision

**Quarantine; no source import approved.**

MW-GATEWAY remains a strong candidate for a separately operated data-plane/gateway boundary, with ARSY retaining policy authority and the gateway enforcing and reporting decisions. Before a decision, compare its protocol, policy, identity, tenancy, secrets, observability, resource and deployment interfaces against the current MW runtime and ARSY.

No files in MW-GATEWAY were modified. No repository was archived or deleted.
