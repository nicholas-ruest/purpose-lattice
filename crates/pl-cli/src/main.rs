use clap::{Parser, Subcommand};
use pl_config::RuntimeConfig;
use pl_errors::Result;
use pl_orchestrator::LatticeService;
use pl_repository::Repository;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Serve,
    Seed,
    Demo,
    Ablate,
    Benchmark {
        #[arg(default_value_t = 50)]
        runs: usize,
    },
}
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let c = RuntimeConfig::from_env()?;
    match Cli::parse().command {
        Command::Serve => pl_api::serve(c).await?,
        Command::Seed => pl_seed::seed(Repository::open(&c.namespace_dir())?)?,
        Command::Demo => {
            let repo = Repository::open(&c.namespace_dir())?;
            pl_seed::seed(repo.clone())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&LatticeService::new(repo)?.list()?).unwrap()
            )
        }
        Command::Ablate => {
            for (n, s, e) in pl_evaluation::ablations() {
                println!("{n}\t{s:.3}\t{e}")
            }
        }
        Command::Benchmark { runs } => {
            let repo = Repository::open(&c.namespace_dir())?;
            pl_seed::seed(repo.clone())?;
            let m = pl_evaluation::benchmark(repo, runs)?;
            println!(
                "runs={} allowed={} denied={} mean_risk={:.4}",
                runs, m.allowed, m.denied, m.mean_risk
            )
        }
    }
    Ok(())
}
