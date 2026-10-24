# ADR-0012: Retention policy

Status: Accepted

## Context

Purpose Lattice must govern agent data use across browser, middleware, Rust application services, Apache Arrow analysis, RuVector recall and RVF durability. A file-count artifact or post-hoc report would not enforce the data covenant before effect. This decision addresses retention policy in that live path.

## Alternatives

We considered a UI-only warning, an in-memory rule, and delegating the concern to an external service. Those alternatives either lose restart evidence, bypass typed domain ownership, or make the required integration impossible to exercise deterministically.

## Decision

Compare requested lifetime with covenant maximum. The implementation is owned by the corresponding bounded-context crate and is called from `pl-orchestrator`; tests must prove both the permitted path and a material failure path.

## Consequences and validation

The boundary adds explicit types and one more failure mode, but it makes the decision observable and ablatable. Validation requires locked Rust tests, the real upstream interface where applicable, the browser journey, an RVF receipt, and source-bound evidence. Missing execution is a release blocker.
