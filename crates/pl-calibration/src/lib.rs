use pl_errors::{LatticeError, Result};
pub fn conformal_upper(scores: &[f64], alpha: f64) -> Result<f64> {
    if scores.is_empty()
        || !(0.0..1.0).contains(&alpha)
        || scores
            .iter()
            .any(|x| !x.is_finite() || *x < 0.0 || *x > 1.0)
    {
        return Err(LatticeError::Invalid(
            "invalid conformal calibration set".into(),
        ));
    }
    let mut s = scores.to_vec();
    s.sort_by(f64::total_cmp);
    let rank = (((s.len() + 1) as f64 * (1.0 - alpha)).ceil() as usize).clamp(1, s.len());
    Ok(s[rank - 1])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finite_sample_upper_is_conservative() {
        assert_eq!(conformal_upper(&[0.1, 0.2, 0.4, 0.3], 0.25).unwrap(), 0.4)
    }
}
