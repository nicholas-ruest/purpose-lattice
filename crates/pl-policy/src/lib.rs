use pl_arrow::ArrowAnalysis;
use pl_covenant::DataCovenant;
use pl_errors::{LatticeError, Result};
use pl_plan::ProcessingPlan;
pub fn enforce(
    c: &DataCovenant,
    p: &ProcessingPlan,
    a: &ArrowAnalysis,
    drift_upper: f64,
) -> Result<Vec<String>> {
    let mut reasons = vec![];
    if !c
        .allowed_purposes
        .iter()
        .any(|x| normalize(x) == normalize(&p.purpose))
    {
        reasons.push("purpose_not_allowed".into())
    }
    if !c
        .allowed_regions
        .iter()
        .any(|x| x.eq_ignore_ascii_case(&p.region))
    {
        reasons.push("region_not_allowed".into())
    }
    if p.retention_days > c.retention_days {
        reasons.push("retention_exceeded".into())
    }
    if p.operation == "export" && a.sensitive_columns > 0 {
        reasons.push("sensitive_export".into())
    }
    if drift_upper > 0.72 {
        reasons.push("semantic_drift_upper_bound".into())
    }
    if reasons.is_empty() {
        Ok(reasons)
    } else {
        Err(LatticeError::PolicyDenied(reasons.join(",")))
    }
}
fn normalize(x: &str) -> String {
    x.to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
