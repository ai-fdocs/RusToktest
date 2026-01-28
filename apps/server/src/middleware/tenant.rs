use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use loco_rs::{
    app::AppContext,
    controller::middleware::MiddlewareLayer,
    Result,
};
use sea_orm::DbErr;
use tower::{Layer, Service};

use crate::{
    models::{self, tenants},
    tenant::TenantContext,
};

#[derive(Clone, Debug, Default)]
pub struct TenantMiddleware;

impl TenantMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl MiddlewareLayer for TenantMiddleware {
    fn name(&self) -> &'static str {
        "tenant"
    }

    fn config(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(serde_json::json!({ "enable": true }))
    }

    fn apply(
        &self,
        app: axum::Router<AppContext>,
    ) -> Result<axum::Router<AppContext>> {
        Ok(app.layer(TenantLayer))
    }
}

#[derive(Clone, Debug)]
struct TenantLayer;

impl<S> Layer<S> for TenantLayer {
    type Service = TenantService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TenantService { inner }
    }
}

#[derive(Clone, Debug)]
struct TenantService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for TenantService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let ctx = req.extensions().get::<AppContext>().cloned();

        Box::pin(async move {
            if should_skip_tenant(&req) {
                return inner.call(req).await;
            }

            if let Some(ctx) = ctx {
                let tenant_result = resolve_tenant(&ctx, &req).await;
                match tenant_result {
                    Ok(Some(tenant)) => {
                        req.extensions_mut().insert(TenantContext::new(tenant));
                    }
                    Ok(None) => {
                        return Ok(response_with_status(
                            StatusCode::BAD_REQUEST,
                            "Tenant not found",
                        ));
                    }
                    Err(error) => {
                        tracing::error!(error = ?error, "failed to resolve tenant");
                        return Ok(response_with_status(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Tenant lookup failed",
                        ));
                    }
                }
            }

            inner.call(req).await
        })
    }
}

async fn resolve_tenant(
    ctx: &AppContext,
    req: &Request<Body>,
) -> Result<Option<models::_entities::tenants::Model>, DbErr> {
    if let Some(slug) = tenant_slug_from_headers(req) {
        return tenants::Entity::find_by_slug(&ctx.db, &slug).await;
    }

    if let Some(domain) = tenant_domain_from_request(req) {
        return tenants::Entity::find_by_domain(&ctx.db, &domain).await;
    }

    Ok(None)
}

fn should_skip_tenant(req: &Request<Body>) -> bool {
    req.uri().path().starts_with("/health")
}

fn tenant_slug_from_headers(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get("x-tenant")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
}

fn tenant_domain_from_request(req: &Request<Body>) -> Option<String> {
    let host = req
        .headers()
        .get(axum::http::header::HOST)
        .and_then(|value| value.to_str().ok())?;

    let trimmed = host.trim();
    if trimmed.is_empty() {
        return None;
    }

    let host_only = trimmed
        .split(':')
        .next()
        .unwrap_or(trimmed)
        .to_string();

    if host_only.is_empty() {
        None
    } else {
        Some(host_only)
    }
}

fn response_with_status(status: StatusCode, message: &str) -> Response {
    let payload = serde_json::json!({ "error": message });
    let body = serde_json::to_vec(&payload).unwrap_or_default();
    Response::builder()
        .status(status)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}
