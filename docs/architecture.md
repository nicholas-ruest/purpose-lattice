# Architecture

## Bounded contexts

1. Covenant and plan ownership.
2. Columnar analysis and statistical assurance.
3. Intelligence recall and durable evidence.
4. Policy, identity and application orchestration.
5. API and product delivery.

## Workspace map

| pl-types | contracts and value objects | Called by pl-errors | ADR-0001, DDD-01 | Typed behavior + unit/integration acceptance |
| pl-errors | domain invariants | Called by pl-config | ADR-0002, DDD-02 | Typed behavior + unit/integration acceptance |
| pl-config | research mechanism | Called by pl-covenant | ADR-0003, DDD-03 | Typed behavior + unit/integration acceptance |
| pl-covenant | enterprise/Ruvnet adapter | Called by pl-plan | ADR-0004, DDD-04 | Typed behavior + unit/integration acceptance |
| pl-plan | governance and middleware | Called by pl-calibration | ADR-0005, DDD-05 | Typed behavior + unit/integration acceptance |
| pl-calibration | orchestration/delivery | Called by pl-taint | ADR-0006, DDD-06 | Typed behavior + unit/integration acceptance |
| pl-taint | contracts and value objects | Called by pl-retention | ADR-0007, DDD-07 | Typed behavior + unit/integration acceptance |
| pl-retention | domain invariants | Called by pl-risk | ADR-0008, DDD-08 | Typed behavior + unit/integration acceptance |
| pl-risk | research mechanism | Called by pl-embedding | ADR-0009, DDD-09 | Typed behavior + unit/integration acceptance |
| pl-embedding | enterprise/Ruvnet adapter | Called by pl-ruvector | ADR-0010, DDD-10 | Typed behavior + unit/integration acceptance |
| pl-ruvector | governance and middleware | Called by pl-recall | ADR-0011, DDD-11 | Typed behavior + unit/integration acceptance |
| pl-recall | orchestration/delivery | Called by pl-arrow | ADR-0012, DDD-12 | Typed behavior + unit/integration acceptance |
| pl-arrow | contracts and value objects | Called by pl-policy | ADR-0013, DDD-01 | Typed behavior + unit/integration acceptance |
| pl-policy | domain invariants | Called by pl-audit | ADR-0014, DDD-02 | Typed behavior + unit/integration acceptance |
| pl-audit | research mechanism | Called by pl-rvf | ADR-0015, DDD-03 | Typed behavior + unit/integration acceptance |
| pl-rvf | enterprise/Ruvnet adapter | Called by pl-repository | ADR-0016, DDD-04 | Typed behavior + unit/integration acceptance |
| pl-repository | governance and middleware | Called by pl-auth | ADR-0017, DDD-05 | Typed behavior + unit/integration acceptance |
| pl-auth | orchestration/delivery | Called by pl-middleware | ADR-0018, DDD-06 | Typed behavior + unit/integration acceptance |
| pl-middleware | contracts and value objects | Called by pl-orchestrator | ADR-0019, DDD-07 | Typed behavior + unit/integration acceptance |
| pl-orchestrator | domain invariants | Called by pl-api | ADR-0020, DDD-08 | Typed behavior + unit/integration acceptance |
| pl-api | research mechanism | Called by pl-cli / acceptance | ADR-0021, DDD-09 | Typed behavior + unit/integration acceptance |
| pl-seed | enterprise/Ruvnet adapter | Called by pl-cli / acceptance | ADR-0022, DDD-10 | Typed behavior + unit/integration acceptance |
| pl-evaluation | governance and middleware | Called by pl-cli / acceptance | ADR-0023, DDD-11 | Typed behavior + unit/integration acceptance |
| pl-cli | orchestration/delivery | Called by pl-cli / acceptance | ADR-0024, DDD-12 | Typed behavior + unit/integration acceptance |

Each row is verified through Cargo metadata and the live orchestrator call graph; arbitrary file splitting does not count.
