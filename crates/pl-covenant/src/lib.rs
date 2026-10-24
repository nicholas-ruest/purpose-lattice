use pl_errors::{LatticeError, Result};
use pl_types::{DatasetId, Sensitivity};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataCovenant {
    pub dataset_id: DatasetId,
    pub name: String,
    pub owner: String,
    pub allowed_purposes: Vec<String>,
    pub sensitive_fields: Vec<String>,
    pub allowed_regions: Vec<String>,
    pub retention_days: u32,
    pub sensitivity: Sensitivity,
}
impl DataCovenant {
    #[allow(clippy::too_many_arguments)]
    pub fn register(
        dataset_id: DatasetId,
        name: String,
        owner: String,
        allowed_purposes: Vec<String>,
        sensitive_fields: Vec<String>,
        allowed_regions: Vec<String>,
        retention_days: u32,
        sensitivity: Sensitivity,
    ) -> Result<Self> {
        if name.trim().len() < 3 || owner.trim().len() < 3 {
            return Err(LatticeError::Invalid("name and owner are required".into()));
        }
        if allowed_purposes.is_empty() || allowed_regions.is_empty() {
            return Err(LatticeError::Invalid(
                "at least one purpose and region are required".into(),
            ));
        }
        if retention_days == 0 || retention_days > 3650 {
            return Err(LatticeError::Invalid(
                "retention must be 1-3650 days".into(),
            ));
        }
        Ok(Self {
            dataset_id,
            name,
            owner,
            allowed_purposes,
            sensitive_fields,
            allowed_regions,
            retention_days,
            sensitivity,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_unbounded_retention() {
        assert!(DataCovenant::register(
            DatasetId::new("orders").unwrap(),
            "Orders".into(),
            "data-team".into(),
            vec!["fraud detection".into()],
            vec![],
            vec!["ca".into()],
            0,
            Sensitivity::Internal
        )
        .is_err())
    }
}
