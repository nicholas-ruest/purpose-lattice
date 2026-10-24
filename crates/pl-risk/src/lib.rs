use pl_types::Decision;
pub fn compose(
    drift: f64,
    minimization: f64,
    retention_excess: f64,
    policy_ok: bool,
) -> (f64, Decision) {
    let score = (0.5 * drift + 0.3 * (1.0 - minimization) + 0.2 * retention_excess.min(1.0))
        .clamp(0.0, 1.0);
    let decision = if !policy_ok || score >= 0.72 {
        Decision::Deny
    } else if score >= 0.45 {
        Decision::Review
    } else {
        Decision::Allow
    };
    (score, decision)
}
