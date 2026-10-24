#!/usr/bin/env bash
set -euo pipefail
for f in .run/ui.pid .run/api.pid; do if [[ -f "$f" ]]; then kill "$(cat "$f")" 2>/dev/null || true; fi; done
