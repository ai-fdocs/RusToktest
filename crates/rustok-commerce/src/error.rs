use thiserror::Error;

pub type Result<T> = std::result::Result<T, CommerceError>;

#[derive(Debug, Error)]
pub enum CommerceError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Product is already published")]
    AlreadyPublished,

    #[error("Invalid product state: {0}")]
    InvalidState(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error(transparent)]
    Core(#[from] rustok_core::Error),
}
