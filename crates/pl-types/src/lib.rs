use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DatasetId(String);
impl DatasetId {
    pub fn new(v: impl Into<String>) -> Result<Self, String> {
        let v = v.into();
        if v.len() < 3
            || v.len() > 64
            || !v
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            Err("dataset id must be 3-64 safe characters".into())
        } else {
            Ok(Self(v))
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for DatasetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Sensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Allow,
    Deny,
    Review,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analogue {
    pub dataset_id: String,
    pub distance: f64,
    pub decision: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EvaluationResult {
    pub dataset_id: DatasetId,
    pub decision: Decision,
    pub risk_score: f64,
    pub drift_upper: f64,
    pub minimization_ratio: f64,
    pub selected_columns: usize,
    pub sensitive_columns: usize,
    pub reasons: Vec<String>,
    pub analogues: Vec<Analogue>,
    pub receipt: String,
    pub correlation_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StoredRecord {
    Covenant {
        dataset_id: DatasetId,
        name: String,
        owner: String,
        allowed_purposes: Vec<String>,
        sensitive_fields: Vec<String>,
        allowed_regions: Vec<String>,
        retention_days: u32,
        receipt: String,
    },
    Plan {
        dataset_id: DatasetId,
        purpose: String,
        selected_fields: Vec<String>,
        region: String,
        retention_days: u32,
        receipt: String,
    },
    Evaluation {
        result: EvaluationResult,
    },
    Audit {
        dataset_id: DatasetId,
        event: String,
        correlation_id: String,
        previous_hash: String,
        payload_hash: String,
        recorded_at: String,
        receipt: String,
    },
}
