pub mod embedded;
pub mod remote;

use async_trait::async_trait;

use rustok_core::Result;

use crate::config::IggyConfig;

#[async_trait]
pub trait IggyBackend: Send + Sync + 'static {
    async fn connect(&self, config: &IggyConfig) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;
}
