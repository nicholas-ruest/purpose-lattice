#!/usr/bin/env bash
set -euo pipefail
count=$(find crates -mindepth 2 -maxdepth 2 -name Cargo.toml | wc -l)
[[ "$count" -ge 20 ]]
! grep -RIE 'HORIZON_|commitment-horizon|Commitment Horizon|commitment_' --exclude=daily-build-ledger.md --exclude=check-workspace.sh --exclude-dir=.git --exclude-dir=target --exclude-dir=node_modules .
printf 'linked workspace crates: %s\n' "$count"
