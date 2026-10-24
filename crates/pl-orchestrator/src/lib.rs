use pl_audit::audit;
use pl_covenant::DataCovenant;
use pl_errors::{LatticeError, Result};
use pl_plan::ProcessingPlan;
use pl_repository::Repository;
use pl_ruvector::PurposeIndex;
use pl_types::{DatasetId, EvaluationResult, Sensitivity, StoredRecord};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterCovenant {
    pub dataset_id: String,
    pub name: String,
    pub owner: String,
    pub allowed_purposes: Vec<String>,
    pub sensitive_fields: Vec<String>,
    pub allowed_regions: Vec<String>,
    pub retention_days: u32,
    pub sensitivity: Sensitivity,
}
#[derive(Clone)]
pub struct LatticeService {
    repository: Repository,
    index: Arc<Mutex<PurposeIndex>>,
}
impl LatticeService {
    pub fn new(repository: Repository) -> Result<Self> {
        let mut index = PurposeIndex::new()?;
        for r in repository.records()? {
            if let StoredRecord::Plan {
                dataset_id,
                purpose,
                selected_fields,
                region,
                retention_days,
                ..
            } = r
            {
                let p = ProcessingPlan {
                    dataset_id: dataset_id.to_string(),
                    purpose,
                    selected_fields,
                    region,
                    retention_days,
                    operation: "read".into(),
                };
                index.insert(dataset_id.as_str(), pl_embedding::embed(&p), "historical")?;
            }
        }
        Ok(Self {
            repository,
            index: Arc::new(Mutex::new(index)),
        })
    }
    pub fn register(&self, c: RegisterCovenant, correlation: &str) -> Result<StoredRecord> {
        let id = DatasetId::new(c.dataset_id).map_err(LatticeError::Invalid)?;
        if self.repository.covenant(&id).is_ok() {
            return Err(LatticeError::Conflict("covenant already exists".into()));
        }
        let aggregate = DataCovenant::register(
            id.clone(),
            c.name,
            c.owner,
            c.allowed_purposes,
            c.sensitive_fields,
            c.allowed_regions,
            c.retention_days,
            c.sensitivity,
        )?;
        let payload =
            serde_json::to_vec(&aggregate).map_err(|e| LatticeError::Invalid(e.to_string()))?;
        self.repository.append(&audit(
            id.clone(),
            "covenant.registered",
            correlation,
            &"0".repeat(64),
            &payload,
        )?)?;
        let r = StoredRecord::Covenant {
            dataset_id: id,
            name: aggregate.name,
            owner: aggregate.owner,
            allowed_purposes: aggregate.allowed_purposes,
            sensitive_fields: aggregate.sensitive_fields,
            allowed_regions: aggregate.allowed_regions,
            retention_days: aggregate.retention_days,
            receipt: "pending".into(),
        };
        let receipt = self.repository.append(&r)?;
        Ok(match r {
            StoredRecord::Covenant {
                dataset_id,
                name,
                owner,
                allowed_purposes,
                sensitive_fields,
                allowed_regions,
                retention_days,
                ..
            } => StoredRecord::Covenant {
                dataset_id,
                name,
                owner,
                allowed_purposes,
                sensitive_fields,
                allowed_regions,
                retention_days,
                receipt,
            },
            _ => unreachable!(),
        })
    }
    pub fn list(&self) -> Result<Vec<StoredRecord>> {
        self.repository.covenants()
    }
    pub fn evaluate(&self, p: ProcessingPlan, correlation: &str) -> Result<EvaluationResult> {
        let id = p.validate()?;
        let c = self.load(&id)?;
        let arrow = pl_arrow::analyze(&p, &c.sensitive_fields)?;
        let embedding = pl_embedding::embed(&p);
        let analogues = {
            let i = self
                .index
                .lock()
                .map_err(|_| LatticeError::Dependency("index lock poisoned".into()))?;
            pl_recall::recall(&i, &embedding)?
        };
        let allowed_similarity = if c
            .allowed_purposes
            .iter()
            .any(|x| x.eq_ignore_ascii_case(&p.purpose))
        {
            0.08
        } else {
            0.84
        };
        let mut calibration = vec![0.07, 0.12, 0.18, 0.23, allowed_similarity];
        calibration.extend(analogues.iter().take(2).map(|a| a.distance.min(1.0)));
        let drift_upper = pl_calibration::conformal_upper(&calibration, 0.2)?;
        let minimization = pl_taint::minimization_ratio(&p.selected_fields, &c.sensitive_fields);
        let excess = pl_retention::excess_ratio(p.retention_days, c.retention_days)?;
        let policy = pl_policy::enforce(&c, &p, &arrow, drift_upper);
        let policy_ok = policy.is_ok();
        let (reasons, policy_error) = match policy {
            Ok(r) => (r, None),
            Err(e) => (vec![e.to_string()], Some(e)),
        };
        let (risk_score, decision) = pl_risk::compose(drift_upper, minimization, excess, policy_ok);
        if let Some(e) = policy_error {
            return Err(e);
        }
        let plan = StoredRecord::Plan {
            dataset_id: id.clone(),
            purpose: p.purpose.clone(),
            selected_fields: p.selected_fields.clone(),
            region: p.region.clone(),
            retention_days: p.retention_days,
            receipt: "pending".into(),
        };
        let receipt = self.repository.append(&plan)?;
        let result = EvaluationResult {
            dataset_id: id.clone(),
            decision,
            risk_score,
            drift_upper,
            minimization_ratio: minimization,
            selected_columns: arrow.selected_columns,
            sensitive_columns: arrow.sensitive_columns,
            reasons,
            analogues,
            receipt,
            correlation_id: correlation.into(),
        };
        self.repository.append(&StoredRecord::Evaluation {
            result: result.clone(),
        })?;
        self.index
            .lock()
            .map_err(|_| LatticeError::Dependency("index lock poisoned".into()))?
            .insert(id.as_str(), embedding, &format!("{:?}", result.decision))?;
        Ok(result)
    }
    pub fn audit(&self, id: &str) -> Result<Vec<StoredRecord>> {
        let id = DatasetId::new(id).map_err(LatticeError::Invalid)?;
        Ok(self
            .repository
            .records()?
            .into_iter()
            .filter(|r| match r {
                StoredRecord::Covenant { dataset_id, .. }
                | StoredRecord::Plan { dataset_id, .. }
                | StoredRecord::Audit { dataset_id, .. } => dataset_id == &id,
                StoredRecord::Evaluation { result } => result.dataset_id == id,
            })
            .collect())
    }
    fn load(&self, id: &DatasetId) -> Result<DataCovenant> {
        match self.repository.covenant(id)? {
            StoredRecord::Covenant {
                dataset_id,
                name,
                owner,
                allowed_purposes,
                sensitive_fields,
                allowed_regions,
                retention_days,
                ..
            } => DataCovenant::register(
                dataset_id,
                name,
                owner,
                allowed_purposes,
                sensitive_fields,
                allowed_regions,
                retention_days,
                Sensitivity::Confidential,
            ),
            _ => Err(LatticeError::NotFound(id.to_string())),
        }
    }
}
