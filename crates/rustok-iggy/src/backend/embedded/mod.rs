use async_trait::async_trait;

use rustok_core::Result;

use crate::backend::IggyBackend;
use crate::config::IggyConfig;

pub mod library;
pub mod subprocess;

#[derive(Debug, Default)]
pub struct EmbeddedBackend;

#[async_trait]
impl IggyBackend for EmbeddedBackend {
    async fn connect(&self, _config: &IggyConfig) -> Result<()> {
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
