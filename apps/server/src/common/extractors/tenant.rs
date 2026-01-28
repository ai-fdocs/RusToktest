use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use rustok_core::context::TenantContext;

pub struct CurrentTenant(pub TenantContext);

#[async_trait]
impl<S> FromRequestParts<S> for CurrentTenant
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(tenant) = parts.extensions.get::<TenantContext>() {
            Ok(CurrentTenant(tenant.clone()))
        } else {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Tenant context is missing. Is tenant middleware enabled?",
            ))
        }
    }
}
