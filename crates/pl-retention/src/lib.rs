use pl_errors::{LatticeError, Result};
pub fn excess_ratio(requested: u32, allowed: u32) -> Result<f64> {
    if requested == 0 || allowed == 0 {
        return Err(LatticeError::Invalid("retention must be positive".into()));
    }
    Ok(if requested <= allowed {
        0.0
    } else {
        f64::from(requested - allowed) / f64::from(allowed)
    })
}
