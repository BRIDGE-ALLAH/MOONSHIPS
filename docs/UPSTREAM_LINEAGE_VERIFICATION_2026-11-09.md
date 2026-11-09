# Upstream Lineage Verification — 2026-11-09

## Method and limitation

This review compares GitHub repository metadata (`fork`, `parent`, default branch, declared license) with the current root README/Cargo identity. A false GitHub `fork` flag does not prove the code is original; it only means GitHub does not currently represent that repository as a fork. README badges, package metadata, upstream URLs and source headers are treated as self-declared lineage evidence, not a complete commit-by-commit diff.

## Verified repository metadata

| Repository | GitHub fork flag / parent | Default branch | GitHub license metadata | Current disposition |
|---|---|---|---|---|
| `MW-ORM` | `true`, parent `SeaQL/sea-orm` | `master` | Apache-2.0 | Confirmed GitHub fork; compare divergence and notices before selective use |
| `MW-GATEWAY` | `false`, no parent field | `main` | Apache-2.0 | README explicitly links upstream `agentgateway/agentgateway`; self-declared upstream lineage, not a GitHub fork |
| `MW-STORAGE` | `false`, no parent field | `main` | Apache-2.0 | README still prominently identifies RustFS and links upstream `rustfs/rustfs`; do not infer a complete branded fork diff |
| `MW-ID` | `false`, no parent field | `master` | MPL-2.0 | README begins with Kanidm identity and describes Kanidm features; treat as Kanidm-derived until provenance is reconciled |
| `MW-UI` | `false`, no parent field | `main` | MIT | README identifies Moonwitness UI and says it currently builds on React, Next.js, Tailwind and shadcn/ui; not a GitHub fork |
| `MW-IDE` | `false`, no parent field | `main` | Apache-2.0 | README identifies MOONWITNESS IDE; previous source inspection found Lapce-derived crate/binary identifiers. Commit-level ancestry still required |
| `MW-BAAS` | `false`, no parent field | `master` | Apache-2.0 | README remains the unmodified-looking Loco framework welcome page and links Loco docs/crates. No bespoke BAAS product boundary verified |
| `EMPIRE-OF-LIGHT` | `false`, no parent field | `main` | Apache-2.0 | Root Cargo still declares Temps Contributors, `MIT OR Apache-2.0`, and Temps workspace crate names. Treat as Temps-derived platform, not an isolated ARSY library |

All metadata above was fetched from GitHub on 2026-11-09; it is a snapshot and can change.

## Consequences for consolidation

1. A GitHub fork flag and a source-level upstream claim are different facts. Record both separately.
2. Keep `MW-ORM`, `MW-STORAGE`, `MW-ID`, `MW-UI`, `MW-IDE`, `MW-BAAS`, `MW-GATEWAY`, and `EMPIRE-OF-LIGHT` as separate candidate source trees until exact commit ancestry, third-party notices, dependency licenses, security posture and build reproducibility are checked.
3. `MW-BAAS` is not currently approved as an independent product import. Its root README still describes Loco; first establish whether the repo contains a real bespoke application beyond framework source.
4. `MW-ID` may be evaluated as an identity-provider capability, but it must not replace ARSY authorization/policy without an explicit identity and authorization ADR.
5. `MW-STORAGE` should be evaluated as a separately operated S3-compatible storage service, not embedded as the MW primary database.
6. `MW-UI` and `MW-IDE` are substantial products; neither is a small component dependency. Keep the current MiniJinja UI until an ADR and migration tests justify a change.
7. `EMPIRE-OF-LIGHT` is a platform lineage with extensive Temps-derived code and notices; do not wholesale import or strip attribution.

## Remaining verification

- Compare exact upstream commit bases and local divergence for every candidate where upstream ancestry is claimed.
- Generate per-repository third-party inventory, NOTICE obligations and license scan.
- Verify build/test workflows at pinned SHAs.
- Confirm no candidate is copied into MW until its domain owner and integration boundary are explicit.

No source repository was modified, archived or deleted by this review.
