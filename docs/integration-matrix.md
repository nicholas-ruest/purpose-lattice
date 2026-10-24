# Integration matrix

| Ingredient | Pinned revision | Capability | Boundary/crate | Test | Ablation |
|---|---|---|---|---|---|
| RuVector | 46d816d | analogue recall | VectorDB / pl-ruvector | executed_insert_search | without recall |
| RVF | 46d816d | append-only persistence | RvfStore / pl-rvf | durable_rvf_round_trip | without durability |
| Apache Arrow | b391e5a | column-use batch | RecordBatch / pl-arrow | real_arrow_batch_counts_sensitive_columns | without Arrow |
| Conformal bound | specified algorithm | finite-sample drift upper | pl-calibration | conservative quantile | without conformal |
