# Ubiquitous language

## Purpose

Covenant, processing plan, purpose drift, minimization, taint, decision and receipt. This bounded-context view prevents persistence, user-interface and upstream library types from becoming the domain language.

## Model and invariants

The aggregate accepts validated value objects only. Commands are rejected before an event is appended when purpose, geography, minimization, retention or authority violates the covenant. Allowed decisions produce immutable records carrying dataset identity, correlation and receipt.

## Events and ownership

The context emits typed registration, plan-evaluated and audit-recorded facts. `pl-orchestrator` owns the process; domain crates own rules; adapters translate Apache Arrow, RuVector and RVF failures into explicit application errors.

## Ports, tests and acceptance

Public Rust functions are the inbound ports. Repository and upstream adapters are outbound ports. Unit tests cover invariants; the acceptance suite proves UI → API → middleware → application → upstream/persistence → visible result, including denial with no forbidden durable effect and restart retrieval.


## Context

Purpose Lattice uses this vocabulary to align policy, analysis, persistence, and delivery decisions around one governed processing plan.
