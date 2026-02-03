use uuid::Uuid;

pub fn partition_key(tenant_id: Uuid) -> String {
    tenant_id.to_string()
}
