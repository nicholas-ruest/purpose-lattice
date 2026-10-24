use chrono::Utc;
use pl_errors::Result;
use pl_types::{DatasetId, StoredRecord};
use sha2::{Digest, Sha256};
pub fn audit(
    id: DatasetId,
    event: &str,
    correlation: &str,
    previous: &str,
    payload: &[u8],
) -> Result<StoredRecord> {
    let payload_hash = hex::encode(Sha256::digest(payload));
    let receipt = hex::encode(Sha256::digest(format!(
        "{id}|{event}|{correlation}|{previous}|{payload_hash}"
    )));
    Ok(StoredRecord::Audit {
        dataset_id: id,
        event: event.into(),
        correlation_id: correlation.into(),
        previous_hash: previous.into(),
        payload_hash,
        recorded_at: Utc::now().to_rfc3339(),
        receipt,
    })
}
