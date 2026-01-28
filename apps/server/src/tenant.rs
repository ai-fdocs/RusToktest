use serde::Serialize;
use uuid::Uuid;

use crate::models::_entities::tenants::Model;

#[derive(Clone, Debug, Serialize)]
pub struct TenantContext {
    pub tenant: Model,
}

impl TenantContext {
    pub fn new(tenant: Model) -> Self {
        Self { tenant }
    }

    pub fn id(&self) -> Uuid {
        self.tenant.id
    }

    pub fn slug(&self) -> &str {
        &self.tenant.slug
    }

    pub fn is_active(&self) -> bool {
        self.tenant.is_active
    }
}
