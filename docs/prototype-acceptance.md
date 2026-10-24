# Prototype acceptance

| Journey | UI control | API/middleware | Application/crates | Upstream/persistence | Evidence |
|---|---|---|---|---|---|
| Allowed plan | Evaluate safe plan | session, correlation, idempotency, validation | covenant → Arrow → calibration/taint/retention → policy → orchestrator | RuVector insert/search and RVF append | Playwright journey 1, screenshot, correlation trace |
| Deny and recover | Try forbidden export, then safe plan | authenticated request and typed 403 | Arrow identifies sensitive export; policy stops append | RVF count unchanged, corrected plan appended | Playwright journey 2 and Rust no-effect test |
| Refresh/restart audit | Audit navigation | authenticated GET | repository rehydrates service | RVF open_readonly/read_all_vectors | Playwright journey 3, mobile screenshot, restart receipt |
