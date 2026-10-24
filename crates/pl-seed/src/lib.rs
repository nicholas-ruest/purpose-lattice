use pl_errors::Result;
use pl_orchestrator::{LatticeService, RegisterCovenant};
use pl_repository::Repository;
use pl_types::Sensitivity;
pub fn seed(repo: Repository) -> Result<()> {
    let service = LatticeService::new(repo)?;
    if service.list()?.is_empty() {
        service.register(
            RegisterCovenant {
                dataset_id: "support-cases".into(),
                name: "Support case corpus".into(),
                owner: "customer-trust".into(),
                allowed_purposes: vec!["service quality analysis".into(), "fraud detection".into()],
                sensitive_fields: vec!["email".into(), "message".into()],
                allowed_regions: vec!["ca".into(), "uk".into()],
                retention_days: 30,
                sensitivity: Sensitivity::Confidential,
            },
            "seed-purpose-lattice",
        )?;
    }
    Ok(())
}
