use arrow_array::{ArrayRef, BooleanArray, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use pl_errors::{LatticeError, Result};
use pl_plan::ProcessingPlan;
use std::sync::Arc;
#[derive(Clone, Debug, PartialEq)]
pub struct ArrowAnalysis {
    pub selected_columns: usize,
    pub sensitive_columns: usize,
    pub batch_rows: usize,
}
pub fn analyze(plan: &ProcessingPlan, sensitive: &[String]) -> Result<ArrowAnalysis> {
    let flags: Vec<bool> = plan
        .selected_fields
        .iter()
        .map(|f| sensitive.contains(f))
        .collect();
    let schema = Arc::new(Schema::new(vec![
        Field::new("field", DataType::Utf8, false),
        Field::new("sensitive", DataType::Boolean, false),
    ]));
    let cols: Vec<ArrayRef> = vec![
        Arc::new(StringArray::from(plan.selected_fields.clone())),
        Arc::new(BooleanArray::from(flags.clone())),
    ];
    let batch = RecordBatch::try_new(schema, cols)
        .map_err(|e| LatticeError::Dependency(format!("Apache Arrow batch: {e}")))?;
    Ok(ArrowAnalysis {
        selected_columns: batch.num_rows(),
        sensitive_columns: flags.into_iter().filter(|x| *x).count(),
        batch_rows: batch.num_rows(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_arrow_batch_counts_sensitive_columns() {
        let p = ProcessingPlan {
            dataset_id: "orders".into(),
            purpose: "fraud detection".into(),
            selected_fields: vec!["amount".into(), "email".into()],
            region: "ca".into(),
            retention_days: 30,
            operation: "aggregate".into(),
        };
        let a = analyze(&p, &["email".into()]).unwrap();
        assert_eq!((a.batch_rows, a.sensitive_columns), (2, 1))
    }
}
