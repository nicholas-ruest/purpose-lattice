use pl_errors::{LatticeError, Result};
use pl_types::DatasetId;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProcessingPlan {
    pub dataset_id: String,
    pub purpose: String,
    pub selected_fields: Vec<String>,
    pub region: String,
    pub retention_days: u32,
    pub operation: String,
}
impl ProcessingPlan {
    pub fn validate(&self) -> Result<DatasetId> {
        let id = DatasetId::new(&self.dataset_id).map_err(LatticeError::Invalid)?;
        if self.purpose.trim().len() < 4 {
            return Err(LatticeError::Invalid("purpose is required".into()));
        }
        if self.selected_fields.is_empty() || self.selected_fields.len() > 128 {
            return Err(LatticeError::Invalid("select 1-128 fields".into()));
        }
        if self.region.trim().is_empty() || self.retention_days == 0 {
            return Err(LatticeError::Invalid(
                "region and retention are required".into(),
            ));
        }
        if !["read", "aggregate", "train", "export"].contains(&self.operation.as_str()) {
            return Err(LatticeError::Invalid("unsupported operation".into()));
        }
        Ok(id)
    }
}
