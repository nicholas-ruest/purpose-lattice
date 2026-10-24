use pl_errors::LatticeError;
use pl_orchestrator::{LatticeService, RegisterCovenant};
use pl_plan::ProcessingPlan;
use pl_repository::Repository;
use pl_types::{Decision, Sensitivity};

#[test]
fn allowed_and_denied_plans_preserve_auditable_effects() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repository = Repository::open(dir.path()).expect("repository");
    let service = LatticeService::new(repository.clone()).expect("service");

    service
        .register(
            RegisterCovenant {
                dataset_id: "orders-us".into(),
                name: "US order analytics".into(),
                owner: "commerce-data".into(),
                allowed_purposes: vec!["fraud-analysis".into()],
                sensitive_fields: vec!["email".into()],
                allowed_regions: vec!["us-central1".into()],
                retention_days: 30,
                sensitivity: Sensitivity::Confidential,
            },
            "test-register",
        )
        .expect("register covenant");

    let allowed = service
        .evaluate(
            ProcessingPlan {
                dataset_id: "orders-us".into(),
                purpose: "fraud-analysis".into(),
                selected_fields: vec!["order_id".into(), "amount".into()],
                region: "us-central1".into(),
                retention_days: 7,
                operation: "aggregate".into(),
            },
            "test-allow",
        )
        .expect("allowed evaluation");
    assert_eq!(allowed.decision, Decision::Allow);

    let before_denial = repository.records().expect("records").len();
    let denied = service
        .evaluate(
            ProcessingPlan {
                dataset_id: "orders-us".into(),
                purpose: "marketing-export".into(),
                selected_fields: vec!["email".into()],
                region: "europe-west1".into(),
                retention_days: 365,
                operation: "export".into(),
            },
            "test-deny",
        )
        .expect_err("policy denial must stop before persistence");
    assert!(matches!(denied, LatticeError::PolicyDenied(_)));
    assert_eq!(repository.records().expect("records").len(), before_denial);

    drop(service);
    let reopened = LatticeService::new(repository).expect("reopen service");
    let audit = reopened.audit("orders-us").expect("audit after restart");
    assert!(audit.len() >= 4, "append-only audit should survive restart");
}
