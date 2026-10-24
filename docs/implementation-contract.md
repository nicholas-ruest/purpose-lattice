# Implementation contract

Every crate is a workspace member with a typed API and a real caller. Contracts/types flow inward; adapters flow outward; only the orchestrator composes decisions. The API may not write around the repository. Policy denial occurs before plan/evaluation append. RVF is the only product persistence boundary.

The first vertical slice is browser safe-plan evaluation. It must execute session middleware, plan validation, covenant lookup, Arrow analysis, embedding, RuVector recall, conformal/taint/retention logic, policy, RVF append and rendered output. Journey two changes inputs to prove a no-effect denial. Journey three reconstructs state from RVF.
