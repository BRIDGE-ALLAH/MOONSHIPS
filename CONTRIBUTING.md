# Contributing to MOONSHIPS

Thank you for your interest in contributing to MOONSHIPS.

## Project and license

MOONSHIPS is distributed under [AGPL-3.0](LICENSE). Review the license and existing copyright and attribution notices before submitting or redistributing changes.

This repository is a MOONSHIPS-branded derivative of the Pier codebase. The files prefixed `UPSTREAM-PIER-` are preserved upstream legal/reference texts; they are not MOONSHIPS contributor agreements, trademark policy, or a grant of rights from the upstream trademark owner. This repository does not currently require a separate MOONSHIPS CLA.

## How to contribute

1. Fork the repository.
2. Create a focused branch for your feature or fix.
3. Add deterministic tests for behavior changes.
4. Run `cargo fmt --all -- --check`.
5. Run `cargo check --workspace --locked`.
6. Run the relevant tests; CI runs the required pull-request checks.
7. Submit a pull request with a clear description, compatibility impact, and test evidence.

## Code and compatibility rules

- Preserve documented runtime compatibility contracts unless a separately tested migration is included.
- Do not remove or rename existing data paths, service units, environment variables, storage keys, or binary names as cosmetic cleanup.
- Do not commit secrets, production data, generated credentials, or local environment files.
- Keep changes scoped and document migrations and rollback behavior.

## Reporting issues

Use [GitHub Issues](https://github.com/BRIDGE-ALLAH/MW/issues). Include reproduction steps, the affected MOONSHIPS version/commit, OS, and Docker version.

## Security vulnerabilities

Do not report security vulnerabilities in public issues. Follow [SECURITY.md](SECURITY.md) and use the repository's private vulnerability reporting channel.
