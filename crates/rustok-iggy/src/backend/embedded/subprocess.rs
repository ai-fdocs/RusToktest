use rustok_core::Result;

#[derive(Debug, Default)]
pub struct SubprocessMode;

impl SubprocessMode {
    pub async fn start(&self) -> Result<()> {
        Ok(())
    }
}
