use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Information about the current tenant available during request handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantContext {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

impl TenantContext {
    pub fn new(id: Uuid, name: String, slug: String) -> Self {
        Self { id, name, slug }
    }
}
