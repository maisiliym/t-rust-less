use std::sync::Arc;

mod config;
mod error;
pub mod local;
mod remote;
mod secrets_provider;

#[cfg(unix)]
pub mod unix;

pub use self::config::{config_file, StoreConfig};
pub use self::error::*;

use crate::secrets_store::SecretsStore;

pub trait ClipboardControl {
  fn is_done(&self) -> ServiceResult<bool>;

  fn currently_providing(&self) -> ServiceResult<Option<String>>;

  fn destroy(&self) -> ServiceResult<()>;
}

pub trait TrustlessService {
  fn list_stores(&self) -> ServiceResult<Vec<String>>;

  fn set_store_config(&self, store_config: StoreConfig) -> ServiceResult<()>;

  fn get_store_config(&self, name: &str) -> ServiceResult<StoreConfig>;

  fn open_store(&self, name: &str) -> ServiceResult<Arc<dyn SecretsStore>>;

  fn get_default_store(&self) -> ServiceResult<Option<String>>;

  fn set_default_store(&self, name: &str) -> ServiceResult<()>;

  fn secret_to_clipboard(
    &self,
    store_name: &str,
    secret_id: &str,
    properties: &[&str],
    display_name: &str,
  ) -> ServiceResult<Arc<dyn ClipboardControl>>;
}

pub fn create_service() -> ServiceResult<Arc<dyn TrustlessService>> {
  #[cfg(unix)]
  {
    if let Some(remote) = self::unix::try_remote_service()? {
      return Ok(Arc::new(remote));
    }
  }
  Ok(Arc::new(self::local::LocalTrustlessService::new()?))
}
