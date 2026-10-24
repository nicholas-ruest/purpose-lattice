# Purpose Lattice contributor contract

This repository implements the 2026-10-22 Dream Machine project. The committed design checkpoint precedes implementation.

- One writer owns the worktree and shared manifests.
- Keep Rust/TypeScript/Markdown source files below 500 lines where practical.
- Validate identity, IDs, payloads, paths, durations and vector dimensions.
- Never commit credentials or `.env`; `.env.example` contains safe placeholders only.
- Core Memory is build guidance, not a runtime dependency.
- A Rust crate counts only with typed behavior, real callers or executable entrypoint, tests and an acceptance path.
- The browser suite may not intercept first-party API routes.
- Publication requires full-stack verification, pinned checker and semantic evidence review.
