# Architecture

Purpose Lattice is organized around five interacting bounded contexts: covenant ownership, processing-plan analysis, statistical and information-flow assurance, intelligence/evidence persistence, and authenticated product delivery. The browser path terminates in one orchestrator; adapters cannot append a plan until domain policy returns an allow decision.

## Runtime topology

The React application talks only to the versioned Axum API. Axum establishes the principal, correlation ID, and idempotency boundary before invoking `LatticeService`. The service validates a `ProcessingPlan`, loads its `DataCovenant`, constructs Apache Arrow evidence, computes minimization/retention/taint and conformal drift signals, retrieves RuVector analogues, and enforces the policy. Allowed plans and evaluations are appended through `Repository` to RVF; denied plans return before either append. On process reconstruction, the repository reopens RVF and rehydrates the RuVector purpose index from durable plan records.

## Workspace map

| Crate | Bounded context and responsibility | Typed public API | Actual callers / dependency edges | ADR / DDD | Tests and acceptance |
| --- | --- | --- | --- | --- | --- |
| `pl-types` | Shared kernel: validated dataset identity, decisions, sensitivity, analogues, results, and tagged records | `DatasetId`, `Decision`, `Sensitivity`, `EvaluationResult`, `StoredRecord` | Used across every domain, adapter, application, and delivery crate | ADR-0003; DDD-01 | Dataset-ID validation and serialization exercised through all integration paths |
| `pl-errors` | Shared kernel: explicit domain and delivery failures | `LatticeError`, `Problem`, `Result<T>` | All fallible crates; translated to HTTP problems by `pl-api` | ADR-0001; DDD-01 | Typed invalid, unauthorized, policy-denied, dependency, and persistence paths |
| `pl-config` | Runtime configuration and namespace isolation | `RuntimeConfig::from_env`, `namespace_dir` | `pl-api`, `pl-cli` | ADR-0005; DDD-11 | Defaults, safe namespace paths, executor test namespace |
| `pl-covenant` | Covenant context: register and preserve allowed purposes, regions, fields, retention, and sensitivity | `DataCovenant::register` | `pl-policy`, `pl-orchestrator` | ADR-0002; DDD-02 | Rejects invalid covenant state; seed and HTTP registration acceptance |
| `pl-plan` | Plan context: validated proposed processing workload | `ProcessingPlan::validate` | `pl-arrow`, `pl-embedding`, `pl-policy`, `pl-orchestrator`, `pl-api` | ADR-0004; DDD-03 | Required fields, operation allowlist, 1–128 selected fields |
| `pl-calibration` | Assurance context: finite-sample semantic-drift upper bound | `conformal_upper` | `pl-orchestrator` | ADR-0009; DDD-06 | Conservative finite-sample quantile test and ablation |
| `pl-taint` | Assurance context: information-flow sensitivity lattice | `join`, `flow_allowed` | `pl-policy`, `pl-orchestrator` | ADR-0010; DDD-06 | Restricted information cannot become public through aggregation |
| `pl-retention` | Assurance context: retention-envelope comparison | `excess_ratio` | `pl-policy`, `pl-orchestrator` | ADR-0012; DDD-06 | Boundary and excess calculations; unbounded request rejection |
| `pl-risk` | Assurance context: compose calibrated drift, minimization, retention, and taint risk | `compose` | `pl-orchestrator` | ADR-0013; DDD-06 | Deterministic score bounds and combined benchmark |
| `pl-embedding` | Intelligence context: deterministic plan embeddings for reproducible analogue recall | `embed` | `pl-ruvector`, `pl-orchestrator` | ADR-0007; DDD-04 | Stable fixed-dimension vectors across replay |
| `pl-ruvector` | Ruvnet adapter: real RuVector Core HNSW index | `PurposeIndex::new`, `insert`, `search` | `pl-recall`, `pl-orchestrator` | ADR-0008; DDD-04 | Executed insert/search test; without-recall ablation |
| `pl-recall` | Intelligence context: translate vector neighbours into governed analogues | `recall` | `pl-orchestrator` | ADR-0008; DDD-04 | Distance/decision mapping exercised in evaluation journey |
| `pl-arrow` | Analysis context: real Apache Arrow selected-column evidence | `analyse` returning `ColumnEvidence` | `pl-policy`, `pl-orchestrator` | ADR-0006; DDD-05 | Real `RecordBatch` counts selected and sensitive columns |
| `pl-policy` | Policy context: enforce purpose, region, retention, sensitive export, drift, and minimization before effects | `evaluate` | `pl-orchestrator` | ADR-0014; DDD-07 | Denial reasons and no-forbidden-append integration test |
| `pl-audit` | Evidence context: hash-linked audit event construction | `audit` | `pl-orchestrator` | ADR-0015; DDD-08 | Correlation and prior/payload hashes retained in RVF |
| `pl-rvf` | Ruvnet adapter: encode, append, reopen, and read RVF vectors | `RvfArchive::append`, `read_all` | `pl-repository` | ADR-0016; DDD-08 | Real RVF round-trip and process-restart evidence |
| `pl-repository` | Evidence context: sole persistence abstraction and synchronized append boundary | `Repository::open`, `append`, `records`, `covenant`, `covenants` | `pl-orchestrator`, `pl-api`, `pl-cli`, `pl-seed`, `pl-evaluation` | ADR-0016/0017; DDD-08 | Durable lookup, append count, and restart reconstruction |
| `pl-auth` | Identity context: prototype cookie authentication and role authorization | `Principal`, `authenticate`, `require` | `pl-api` | ADR-0018; DDD-09 | Missing/invalid session and steward-only registration paths |
| `pl-middleware` | Delivery context: correlation and idempotency validation | `validate_correlation`, `validate_idempotency` | `pl-api` | ADR-0019; DDD-09 | Malformed headers fail before application effects |
| `pl-orchestrator` | Application context: single governed use-case coordinator and effect ordering | `LatticeService`, `RegisterCovenant` | `pl-api`, `pl-cli`, `pl-seed`, `pl-evaluation` | ADR-0020; DDD-10 | Full-stack allowed/denied test and rehydration after restart |
| `pl-api` | Delivery context: versioned Axum routes, health/readiness, CORS, tracing, request IDs, graceful shutdown | `router`, `serve` | Executed by `pl-cli`; called by browser and contract tests | ADR-0020; DDD-11 | Live API exercised by three Playwright journeys |
| `pl-seed` | Delivery context: deterministic synthetic covenant through the real application interface | `seed` | `pl-api`, `pl-cli`, `pl-evaluation` | ADR-0005; DDD-11 | Idempotent seed, reset/isolation, visible UI fixture label |
| `pl-evaluation` | Evaluation context: reproducible combined benchmark and ingredient ablations | `benchmark`, `ablations` | `pl-cli`, CI evidence generation | ADR-0024; DDD-11 | Combined, no-conformal, no-Arrow, Ruvnet-only, enterprise-only variants |
| `pl-cli` | Executable composition root for serve, seed, demo, benchmark, and ablation | `purpose-lattice` subcommands | Human/operator and CI entrypoint | ADR-0024; DDD-11 | Locked end-to-end verifier invokes every relevant subcommand |

## Effect-order invariant

`validate → load covenant → Arrow evidence → assurance signals → RuVector recall → policy → RVF append` is the only write path. A policy error exits before the plan and evaluation append, which is asserted both by the Rust composed test and the live browser denial journey. Audit and covenant registration are append-only. No crate owns a second database.

## Deployment boundary

The service is stateless apart from the authorized RVF boundary. The local prototype uses a namespaced RVF file; Cloud Run configuration requires an IAM-scoped identity plus a durable authorized RVF service because container filesystems are ephemeral. The UI never receives persistence credentials. Production authentication must replace `dev-session` with external IAP/OIDC.
