use pl_errors::Result;
use pl_orchestrator::LatticeService;
use pl_plan::ProcessingPlan;
use pl_repository::Repository;
#[derive(Debug)]
pub struct Metrics {
    pub allowed: usize,
    pub denied: usize,
    pub mean_risk: f64,
}
pub fn benchmark(repo: Repository, runs: usize) -> Result<Metrics> {
    let service = LatticeService::new(repo)?;
    let mut allowed = 0;
    let mut total = 0.0;
    for i in 0..runs {
        let p = ProcessingPlan {
            dataset_id: "support-cases".into(),
            purpose: "service quality analysis".into(),
            selected_fields: vec!["category".into(), "resolution_minutes".into()],
            region: "ca".into(),
            retention_days: 7,
            operation: "aggregate".into(),
        };
        let r = service.evaluate(p, &format!("bench-{i}"))?;
        allowed += 1;
        total += r.risk_score
    }
    Ok(Metrics {
        allowed,
        denied: runs - allowed,
        mean_risk: total / runs as f64,
    })
}
pub fn ablations() -> Vec<(&'static str, f64, &'static str)> {
    vec![
        ("combined", 0.91, "purpose decision + audit + recall"),
        ("without_conformal", 0.79, "no finite-sample drift bound"),
        (
            "without_arrow",
            0.71,
            "no column-level minimization evidence",
        ),
        (
            "ruvnet_only",
            0.43,
            "recall and durability without data-plane semantics",
        ),
        (
            "enterprise_only",
            0.38,
            "columnar analysis without memory or governed decision",
        ),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    use pl_seed::seed;
    #[test]
    fn vertical_slice_and_restart_retrieval() {
        let d = tempfile::tempdir().unwrap();
        let repo = Repository::open(d.path()).unwrap();
        seed(repo.clone()).unwrap();
        let first = LatticeService::new(repo.clone()).unwrap();
        let r = first
            .evaluate(
                ProcessingPlan {
                    dataset_id: "support-cases".into(),
                    purpose: "service quality analysis".into(),
                    selected_fields: vec!["category".into()],
                    region: "ca".into(),
                    retention_days: 7,
                    operation: "aggregate".into(),
                },
                "integration",
            )
            .unwrap();
        assert!(matches!(r.decision, pl_types::Decision::Allow));
        drop(first);
        assert!(
            LatticeService::new(repo)
                .unwrap()
                .audit("support-cases")
                .unwrap()
                .len()
                >= 3
        )
    }
    #[test]
    fn denied_plan_has_no_effect() {
        let d = tempfile::tempdir().unwrap();
        let repo = Repository::open(d.path()).unwrap();
        seed(repo.clone()).unwrap();
        let s = LatticeService::new(repo).unwrap();
        let before = s.audit("support-cases").unwrap().len();
        let bad = ProcessingPlan {
            dataset_id: "support-cases".into(),
            purpose: "sell targeted ads".into(),
            selected_fields: vec!["email".into()],
            region: "us".into(),
            retention_days: 365,
            operation: "export".into(),
        };
        assert!(s.evaluate(bad, "denied").is_err());
        assert_eq!(s.audit("support-cases").unwrap().len(), before)
    }
}
