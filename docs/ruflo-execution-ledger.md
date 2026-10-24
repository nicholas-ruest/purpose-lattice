# Ruflo execution ledger

Ruflo is coordination and provenance only. It never substitutes for source, tests, browser evidence, review or publication authority. The build uses one integration writer. No feedback-loop coding agent, automatic lesson approval, or policy promotion is authorized.

| Tool | Version / command | Result | Evidence meaning |
| --- | --- | --- | --- |
| Ruflo | `3.47.1`; `ruflo analyze boundaries crates --format json` | exit 0, but Rust graph reported zero nodes | Tool limitation; not counted as architecture proof |
| Ruflo | `ruflo analyze deps --security --format json` in `frontend/` | exit 0; 14 direct/dev dependencies summarized; embedded npm audit warned | Provenance receipt only; `npm audit`/locked build and Cargo deny are the dependency gates |
| MetaHarness / Darwin / Flywheel | availability probe | no task-relevant executable exposed on this executor | Not executed and not claimed; prior unrelated `.metaharness` directories were not reused |

Raw Ruflo outputs are retained in `evidence/prototype/`. Semantic architecture review uses Cargo manifests, actual caller paths, unit/integration tests and browser evidence.
