# Research and pinned ingredients

| Ingredient | Revision | License | Executed boundary | Ablation |
|---|---|---|---|---|
| RuVector Core 2.4.0 | RuVector `46d816ddc4c2a8a06cd3613534a79b1d557d858b` | MIT | `VectorDB::insert/search` in `pl-ruvector` | no analogue decisions |
| RVF Runtime 0.3.2 | same RuVector revision | MIT OR Apache-2.0 | `RvfStore::ingest_batch/open_readonly/read_all_vectors` | no restart-durable evidence |
| Apache Arrow Rust 60.0.0 | `b391e5af7dff9f6909af45c01f7dddb68ba08c0f` | Apache-2.0 AND MIT | `RecordBatch::try_new` in `pl-arrow` | no column-level evidence |
| Split conformal upper bound | finite-sample quantile mechanism | research mechanism | `pl-calibration` changes decision risk | point drift only |
| Information-flow lattice | monotone sensitivity propagation | research mechanism | `pl-taint` changes minimization/sensitivity | no transformation-aware control |

The conformal mechanism is intentionally simple and deterministic for the prototype: its finite-sample quantile enters the policy and risk path. It is not presented as production calibration. Apache Arrow is a real executed data-plane boundary, not a logo or inspiration.
