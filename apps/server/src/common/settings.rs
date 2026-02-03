use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_TENANT_ID: Uuid = Uuid::from_u128(1);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RustokSettings {
    #[serde(default)]
    pub tenant: TenantSettings,
    #[serde(default)]
    pub features: FeatureSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenantSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_resolution")]
    pub resolution: String,
    #[serde(default = "default_header_name")]
    pub header_name: String,
    #[serde(default = "default_tenant_id")]
    pub default_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeatureSettings {
    #[serde(default = "default_true")]
    pub registration: bool,
    #[serde(default = "default_true")]
    pub search_indexing: bool,
}

impl Default for TenantSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            resolution: default_resolution(),
            header_name: default_header_name(),
            default_id: default_tenant_id(),
        }
    }
}

impl Default for FeatureSettings {
    fn default() -> Self {
        Self {
            registration: true,
            search_indexing: true,
        }
    }
}

impl RustokSettings {
    pub fn from_settings(settings: &Option<serde_json::Value>) -> Result<Self, serde_json::Error> {
        let root = settings
            .clone()
            .unwrap_or_else(|| serde_json::json!({}));
        let rustok = root.get("rustok").cloned().unwrap_or_else(|| serde_json::json!({}));
        serde_json::from_value(rustok)
    }
}

fn default_tenant_id() -> Uuid {
    DEFAULT_TENANT_ID
}

fn default_resolution() -> String {
    "host".to_string()
}

fn default_header_name() -> String {
    "X-Tenant-ID".to_string()
}

fn default_true() -> bool {
    true
}
