# Validation receipt

This receipt describes the governed executor run completed before publication. The implementation commit `7ef5e0e5d5c1bea50fb715cb5e94899b37f65c06` is bound in `evidence/prototype/manifest.json`; exact-candidate GitHub Actions revalidation is required before merge.

| Gate | Executed command | Observed result |
| --- | --- | --- |
| Format | `cargo fmt --all -- --check` | exit 0 |
| Strict Rust lint | `cargo clippy --locked --all-targets --all-features -- -D warnings` | exit 0 across 24 workspace crates |
| Locked Rust tests | `cargo test --locked --workspace` | exit 0; 9 focused tests, including real Arrow, RuVector insert/search, RVF round-trip, denied-write absence, and repository reconstruction |
| Dependency policy | `cargo deny check` | exit 0; advisories, bans, licenses, and sources pass; transitive unmaintained `bincode 2.0.1` is explicitly tracked because it arrives through pinned RuVector |
| Frontend | typecheck, ESLint, Vitest, contract drift, production build | exit 0; one unit suite passed and the production bundle built |
| Node audit | `npm@11.6.2 audit --audit-level=high` | exit 0; 0 vulnerabilities |
| OpenAPI | `@redocly/cli@1.34.5 lint openapi/purpose-lattice.yaml` | valid OpenAPI 3.1 contract; only the two public health operations lack synthetic 4xx responses |
| Whole product | `scripts/verify-prototype.sh` | exit 0 |
| Browser acceptance | Playwright against live React and Axum, no first-party route interception | 3/3 passed with one worker; safe evaluation, denial/recovery, and audit refresh rendered real responses |
| Restart durability | `scripts/restart-check.sh` | exit 0; audit JSON byte-identical after backend stop/start, SHA-256 `d36496c86715be3fe3d60784da5b1a7bc246d7200b6ae70deb2296bb2f8d0317` |
| Benchmark | `cargo run --locked -p pl-cli -- benchmark 50` | 50 deterministic governed evaluations; 50 allowed; mean risk 0.1150 |
| Ablations | `cargo run --locked -p pl-cli -- ablate` | combined 0.910; without conformal 0.790; without Arrow 0.710; Ruvnet-only 0.430; enterprise-only 0.380 |
| Structural checker | checker `80f006f6960bc4e491e97f51e890ff708e81aa88` | `STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW`; 24 crates, 25 ADRs, 12 DDDs, 2 animated SVGs, zero structural failures |
| Ruflo coordination | `ruflo v3.47.1 hooks route` | exit 0; routing receipt only, no implementation agent started |
| Container | `docker compose build` | local executor blocker: Docker CLI unavailable; exact-candidate GitHub Actions must build the Compose topology before merge |

The local service was verified at `http://127.0.0.1:14273` (UI) and `http://127.0.0.1:18180` (API). These were executor-local ephemeral URLs, not deployed previews. Cloud deployment and production promotion were not authorized.
