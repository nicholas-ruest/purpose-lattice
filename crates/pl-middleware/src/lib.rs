use pl_errors::{LatticeError, Result};
pub fn validate_idempotency(v: Option<&str>) -> Result<&str> {
    let v = v.ok_or_else(|| LatticeError::Invalid("idempotency-key required".into()))?;
    if v.len() < 8 || v.len() > 128 {
        return Err(LatticeError::Invalid("invalid idempotency-key".into()));
    }
    Ok(v)
}
pub fn validate_correlation(v: Option<&str>) -> Result<String> {
    let v = v.unwrap_or("generated");
    if v.len() > 128 {
        return Err(LatticeError::Invalid("correlation id too long".into()));
    }
    Ok(v.into())
}
