use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{CommerceError, Result};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProductStatus {
    Draft,
    Published,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: ProductStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(tenant_id: Uuid, title: String, slug: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: rustok_core::generate_id(),
            tenant_id,
            title,
            slug,
            description,
            status: ProductStatus::Draft,
            published_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn publish(&mut self) -> Result<()> {
        match self.status {
            ProductStatus::Draft => {
                let now = Utc::now();
                self.status = ProductStatus::Published;
                self.published_at = Some(now);
                self.updated_at = now;
                Ok(())
            }
            ProductStatus::Published => Err(CommerceError::AlreadyPublished),
        }
    }
}
