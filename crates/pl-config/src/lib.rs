use pl_errors::{LatticeError, Result};
use std::{net::SocketAddr, path::PathBuf};
#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    pub bind: SocketAddr,
    pub data_dir: PathBuf,
    pub namespace: String,
    pub auth_mode: String,
}
impl RuntimeConfig {
    pub fn from_env() -> Result<Self> {
        let bind = std::env::var("PL_BIND")
            .unwrap_or_else(|_| "127.0.0.1:18180".into())
            .parse()
            .map_err(|e| LatticeError::Invalid(format!("PL_BIND: {e}")))?;
        let data_dir =
            PathBuf::from(std::env::var("PL_DATA_DIR").unwrap_or_else(|_| "data/prototype".into()));
        let namespace =
            std::env::var("PL_NAMESPACE").unwrap_or_else(|_| "purpose-lattice-prototype".into());
        let auth_mode = std::env::var("PL_AUTH_MODE").unwrap_or_else(|_| "dev-session".into());
        Ok(Self {
            bind,
            data_dir,
            namespace,
            auth_mode,
        })
    }
    pub fn namespace_dir(&self) -> PathBuf {
        self.data_dir.join(&self.namespace)
    }
}
