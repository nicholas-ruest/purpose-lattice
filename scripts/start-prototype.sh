#!/usr/bin/env bash
set -euo pipefail
mkdir -p .run evidence/prototype
PL_BIND=127.0.0.1:18180 PL_DATA_DIR=data PL_NAMESPACE=prototype cargo run --locked -p pl-cli -- seed
PL_BIND=127.0.0.1:18180 PL_DATA_DIR=data PL_NAMESPACE=prototype cargo run --locked -p pl-cli -- serve >.run/api.log 2>&1 & echo $! >.run/api.pid
npm --prefix frontend run dev -- --host 127.0.0.1 --port 14273 >.run/ui.log 2>&1 & echo $! >.run/ui.pid
for _ in $(seq 1 60); do curl -fsS http://127.0.0.1:18180/readyz >/dev/null 2>&1 && curl -fsS http://127.0.0.1:14273 >/dev/null 2>&1 && exit 0; sleep 1; done
echo 'prototype readiness failed' >&2; exit 1
