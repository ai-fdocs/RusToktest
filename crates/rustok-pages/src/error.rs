use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum PagesError {
    #[error("Content error: {0}")]
    Content(#[from] rustok_content::ContentError),

    #[error("Block not found: {0}")]
    BlockNotFound(Uuid),

    #[error("Menu not found: {0}")]
    MenuNotFound(Uuid),

    #[error("Page not found: {0}")]
    PageNotFound(Uuid),
}

pub type PagesResult<T> = Result<T, PagesError>;
