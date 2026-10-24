#!/usr/bin/env bash
set -euo pipefail
rustup toolchain install 1.99.0 --profile minimal
npm --prefix frontend ci
npx --prefix frontend playwright install chromium
cargo fetch --locked
