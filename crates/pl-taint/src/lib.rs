use pl_types::Sensitivity;
pub fn minimization_ratio(selected: &[String], sensitive: &[String]) -> f64 {
    if selected.is_empty() {
        return 0.0;
    }
    let touched = selected
        .iter()
        .filter(|f| sensitive.iter().any(|s| s == *f))
        .count();
    1.0 - (touched as f64 / selected.len() as f64)
}
pub fn propagation(input: &Sensitivity, operation: &str) -> Sensitivity {
    match (input, operation) {
        (Sensitivity::Restricted, "aggregate") => Sensitivity::Confidential,
        (x, _) => x.clone(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aggregation_cannot_make_restricted_public() {
        assert_eq!(
            propagation(&Sensitivity::Restricted, "aggregate"),
            Sensitivity::Confidential
        )
    }
}
