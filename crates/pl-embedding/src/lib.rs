use pl_plan::ProcessingPlan;
use sha2::{Digest, Sha256};
pub const DIMENSION: usize = 16;
pub fn embed(plan: &ProcessingPlan) -> Vec<f32> {
    let text = format!(
        "{}|{}|{}|{}",
        plan.purpose,
        plan.operation,
        plan.region,
        plan.selected_fields.join(",")
    );
    let digest = Sha256::digest(text.as_bytes());
    (0..DIMENSION)
        .map(|i| f32::from(digest[i]) / 255.0)
        .collect()
}
