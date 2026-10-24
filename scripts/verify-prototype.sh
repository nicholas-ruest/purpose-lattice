#!/usr/bin/env bash
set -euo pipefail
trap './scripts/stop-prototype.sh' EXIT
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo deny check
npm --prefix frontend run typecheck
npm --prefix frontend run lint
npm --prefix frontend test
npm --prefix frontend run contract
npm --prefix frontend run build
npx -y npm@11.6.2 --prefix frontend audit --audit-level=high
find data/prototype -mindepth 1 -delete 2>/dev/null || true
./scripts/start-prototype.sh
npm --prefix frontend run e2e
./scripts/restart-check.sh
cargo run --locked -p pl-cli -- ablate | tee evidence/prototype/ablation.txt
cargo run --locked -p pl-cli -- benchmark 50 | tee evidence/prototype/benchmark.txt
