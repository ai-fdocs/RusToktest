use async_trait::async_trait;

use rustok_core::Result;

use crate::backend::IggyBackend;
use crate::config::IggyConfig;

#[derive(Debug, Default)]
pub struct RemoteBackend;

#[async_trait]
impl IggyBackend for RemoteBackend {
    async fn connect(&self, _config: &IggyConfig) -> Result<()> {
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
