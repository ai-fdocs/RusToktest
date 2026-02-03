use rustok_core::Result;

#[derive(Debug, Default)]
pub struct LibraryMode;

impl LibraryMode {
    pub async fn start(&self) -> Result<()> {
        Ok(())
    }
}
