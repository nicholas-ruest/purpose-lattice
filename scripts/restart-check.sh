#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"
trap './scripts/stop-prototype.sh' EXIT

./scripts/stop-prototype.sh
find data/prototype -mindepth 1 -delete 2>/dev/null || true
./scripts/start-prototype.sh

cookie=.run/restart-cookie.txt
curl -fsS -c "$cookie" -H 'content-type: application/json' \
  -d '{"role":"analyst"}' http://127.0.0.1:18180/api/v1/session >/dev/null
curl -fsS -b "$cookie" -H 'content-type: application/json' \
  -H 'x-correlation-id: restart-before' -H 'idempotency-key: restart-check-1' \
  -d '{"dataset_id":"support-cases","purpose":"service quality analysis","selected_fields":["case_id"],"region":"ca","retention_days":7,"operation":"aggregate"}' \
  http://127.0.0.1:18180/api/v1/covenants/support-cases/evaluate \
  > evidence/prototype/restart-evaluation.json
curl -fsS -b "$cookie" \
  http://127.0.0.1:18180/api/v1/covenants/support-cases/audit \
  > .run/audit-before.json

./scripts/stop-prototype.sh
sleep 1
./scripts/start-prototype.sh
curl -fsS -c "$cookie" -H 'content-type: application/json' \
  -d '{"role":"analyst"}' http://127.0.0.1:18180/api/v1/session >/dev/null
curl -fsS -b "$cookie" \
  http://127.0.0.1:18180/api/v1/covenants/support-cases/audit \
  > .run/audit-after.json

cmp .run/audit-before.json .run/audit-after.json
sha256sum .run/audit-before.json | tee evidence/prototype/restart-audit.sha256
