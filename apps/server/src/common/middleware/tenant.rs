use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use loco_rs::prelude::*;
use rustok_core::context::TenantContext;

use crate::models::tenants;

/// Resolve the tenant for each request.
pub async fn resolve(
    State(ctx): State<AppContext>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    if should_skip_tenant(req.uri().path()) {
        return Ok(next.run(req).await);
    }

    let tenant_model = if let Some(id_str) = header_value(&req, "x-tenant-id") {
        match uuid::Uuid::parse_str(id_str) {
            Ok(uuid) => tenants::Entity::find_by_id(&ctx.db, uuid)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid Tenant ID format".into(),
                ));
            }
        }
    } else if let Some(slug) = header_value(&req, "x-tenant-slug") {
        tenants::Entity::find_by_slug(&ctx.db, slug)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    } else if let Some(domain) = host_domain(&req) {
        tenants::Entity::find_by_domain(&ctx.db, &domain)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "X-Tenant-ID or X-Tenant-Slug header required".into(),
        ));
    };

    if let Some(tenant) = tenant_model {
        if !tenant.is_enabled() {
            return Err((StatusCode::FORBIDDEN, "Tenant is disabled".into()));
        }

        let tenant_ctx = TenantContext::new(tenant.id, tenant.name, tenant.slug);
        req.extensions_mut().insert(tenant_ctx);
        Ok(next.run(req).await)
    } else {
        Err((StatusCode::NOT_FOUND, "Tenant not found".into()))
    }
}

fn should_skip_tenant(path: &str) -> bool {
    path.starts_with("/health")
}

fn header_value<'a>(req: &'a Request<axum::body::Body>, name: &str) -> Option<&'a str> {
    req.headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn host_domain(req: &Request<axum::body::Body>) -> Option<String> {
    let host = req.headers().get(header::HOST)?.to_str().ok()?;
    let trimmed = host.trim();
    if trimmed.is_empty() {
        return None;
    }
    let host_only = trimmed.split(':').next().unwrap_or(trimmed);
    if host_only.is_empty() {
        return None;
    }
    Some(host_only.to_string())
}
