# Design-to-implementation traceability

| Requirement | Design | Planned source | Proof |
| --- | --- | --- | --- |
| register typed commitment | ADR-0002/DDD-02 | `pl-covenant`, `pl-orchestrator`, API create route, React form | J1 Playwright + RVF readback |
| calibrated risk horizon | ADR-0007/0008, DDD-05 | `pl-calibration`, `pl-taint`, `pl-retention`, `pl-risk` | frozen fixtures and ablations |
| analogue recall | ADR-0010, DDD-06 | `pl-embedding`, `pl-ruvector`, `pl-recall` | real upstream integration test |
| policy exceptions | ADR-0012/0013, DDD-07 | `pl-arrow`, `pl-policy` | J2 deny/correct/allow |
| restart durability | ADR-0015/0016, DDD-08 | `pl-audit`, `pl-rvf`, `pl-repository` | J3 process restart |
| product delivery | ADR-0018/0019/0020, DDD-10 | `pl-api`, middleware, React, typed client | contract + Playwright |
| governed learning | ADR-0025, DDD-12 | lesson map/outcome/feedback issue | issue URL and outcome receipt |
