use pl_errors::{LatticeError, Result};
use pl_rvf::RvfArchive;
use pl_types::{DatasetId, StoredRecord};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
#[derive(Clone)]
pub struct Repository {
    path: PathBuf,
    lock: Arc<Mutex<()>>,
}
impl Repository {
    pub fn open(dir: &Path) -> Result<Self> {
        fs::create_dir_all(dir).map_err(|e| LatticeError::Persistence(e.to_string()))?;
        Ok(Self {
            path: dir.join("purpose-lattice.rvf"),
            lock: Arc::new(Mutex::new(())),
        })
    }
    pub fn append(&self, record: &StoredRecord) -> Result<String> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| LatticeError::Persistence("archive lock poisoned".into()))?;
        let bytes =
            serde_json::to_vec(record).map_err(|e| LatticeError::Persistence(e.to_string()))?;
        let id = self.records_unlocked()?.len() as u64 + 1;
        RvfArchive::append(&self.path, id, &bytes)?;
        Ok(hex::encode(Sha256::digest(bytes)))
    }
    fn records_unlocked(&self) -> Result<Vec<StoredRecord>> {
        RvfArchive::read_all(&self.path)?
            .into_iter()
            .map(|(_, b)| {
                serde_json::from_slice(&b).map_err(|e| LatticeError::Persistence(e.to_string()))
            })
            .collect()
    }
    pub fn records(&self) -> Result<Vec<StoredRecord>> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| LatticeError::Persistence("archive lock poisoned".into()))?;
        self.records_unlocked()
    }
    pub fn covenant(&self, id: &DatasetId) -> Result<StoredRecord> {
        self.records()?
            .into_iter()
            .rev()
            .find(|r| matches!(r,StoredRecord::Covenant{dataset_id,..}if dataset_id==id))
            .ok_or_else(|| LatticeError::NotFound(id.to_string()))
    }
    pub fn covenants(&self) -> Result<Vec<StoredRecord>> {
        Ok(self
            .records()?
            .into_iter()
            .filter(|r| matches!(r, StoredRecord::Covenant { .. }))
            .collect())
    }
}
