#[cfg(target_arch = "wasm32")]
use leptos::web_sys;
use leptos_graphql::{execute as execute_graphql, GraphqlHttpError, GraphqlRequest};
use serde::{Deserialize, Serialize};

use crate::model::{InventoryAdminBootstrap, InventoryProductDetail, InventoryProductList};

pub type InventoryTransportError = GraphqlHttpError;

pub trait InventoryReadTransport {
    async fn fetch_bootstrap(
        &self,
        token: Option<String>,
        tenant_slug: Option<String>,
    ) -> Result<InventoryAdminBootstrap, InventoryTransportError>;

    async fn fetch_products(
        &self,
        request: InventoryProductsRequest,
    ) -> Result<InventoryProductList, InventoryTransportError>;

    async fn fetch_product(
        &self,
        request: InventoryProductRequest,
    ) -> Result<Option<InventoryProductDetail>, InventoryTransportError>;
}

#[derive(Clone, Debug)]
pub struct InventoryProductsRequest {
    pub token: Option<String>,
    pub tenant_slug: Option<String>,
    pub tenant_id: String,
    pub locale: Option<String>,
    pub search: Option<String>,
    pub status: Option<String>,
}

#[derive(Clone, Debug)]
pub struct InventoryProductRequest {
    pub token: Option<String>,
    pub tenant_slug: Option<String>,
    pub tenant_id: String,
    pub id: String,
    pub locale: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CommerceGraphqlInventoryReadAdapter;

const BOOTSTRAP_QUERY: &str = "query InventoryAdminBootstrap { currentTenant { id slug name } }";
const PRODUCTS_QUERY: &str = "query InventoryAdminProducts($tenantId: UUID!, $locale: String, $filter: ProductsFilter) { products(tenantId: $tenantId, locale: $locale, filter: $filter) { total page perPage hasNext items { id status title handle vendor productType shippingProfileSlug tags createdAt publishedAt } } }";
const PRODUCT_QUERY: &str = "query InventoryAdminProduct($tenantId: UUID!, $id: UUID!, $locale: String) { product(tenantId: $tenantId, id: $id, locale: $locale) { id status vendor productType shippingProfileSlug createdAt updatedAt publishedAt translations { locale title handle description } variants { id sku barcode shippingProfileSlug title option1 option2 option3 inventoryQuantity inventoryPolicy inStock prices { currencyCode amount compareAtAmount onSale } } } }";

#[derive(Debug, Deserialize)]
struct BootstrapResponse {
    #[serde(rename = "currentTenant")]
    current_tenant: crate::model::CurrentTenant,
}

#[derive(Debug, Deserialize)]
struct ProductsResponse {
    products: InventoryProductList,
}

#[derive(Debug, Deserialize)]
struct ProductResponse {
    product: Option<InventoryProductDetail>,
}

#[derive(Debug, Serialize)]
struct TenantScopedVariables<T> {
    #[serde(rename = "tenantId")]
    tenant_id: String,
    #[serde(flatten)]
    extra: T,
}

#[derive(Debug, Serialize)]
struct ProductsVariables {
    locale: Option<String>,
    filter: ProductsFilter,
}

#[derive(Debug, Serialize)]
struct ProductVariables {
    id: String,
    locale: Option<String>,
}

#[derive(Debug, Serialize)]
struct ProductsFilter {
    status: Option<String>,
    vendor: Option<String>,
    search: Option<String>,
    page: Option<u64>,
    #[serde(rename = "perPage")]
    per_page: Option<u64>,
}

fn graphql_url() -> String {
    if let Some(url) = option_env!("RUSTOK_GRAPHQL_URL") {
        return url.to_string();
    }

    #[cfg(target_arch = "wasm32")]
    {
        let origin = web_sys::window()
            .and_then(|window| window.location().origin().ok())
            .unwrap_or_else(|| "http://localhost:5150".to_string());
        format!("{origin}/api/graphql")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let base =
            std::env::var("RUSTOK_API_URL").unwrap_or_else(|_| "http://localhost:5150".to_string());
        format!("{base}/api/graphql")
    }
}

pub(crate) fn normalize_optional_trimmed(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

pub(crate) fn normalize_locale_filter(locale: Option<String>) -> Option<String> {
    normalize_optional_trimmed(locale)
}

pub(crate) fn normalize_search_filter(search: Option<String>) -> Option<String> {
    normalize_optional_trimmed(search)
}

pub(crate) fn normalize_status_filter(status: Option<String>) -> Option<String> {
    normalize_optional_trimmed(status).map(|value| value.to_ascii_uppercase())
}

async fn request<V, T>(
    query: &str,
    variables: Option<V>,
    token: Option<String>,
    tenant_slug: Option<String>,
) -> Result<T, InventoryTransportError>
where
    V: Serialize,
    T: for<'de> Deserialize<'de>,
{
    execute_graphql(
        &graphql_url(),
        GraphqlRequest::new(query, variables),
        token,
        tenant_slug,
        None,
    )
    .await
}

impl InventoryReadTransport for CommerceGraphqlInventoryReadAdapter {
    async fn fetch_bootstrap(
        &self,
        token: Option<String>,
        tenant_slug: Option<String>,
    ) -> Result<InventoryAdminBootstrap, InventoryTransportError> {
        let response: BootstrapResponse = request::<serde_json::Value, BootstrapResponse>(
            BOOTSTRAP_QUERY,
            None,
            token,
            tenant_slug,
        )
        .await?;
        Ok(InventoryAdminBootstrap {
            current_tenant: response.current_tenant,
        })
    }

    async fn fetch_products(
        &self,
        request_data: InventoryProductsRequest,
    ) -> Result<InventoryProductList, InventoryTransportError> {
        let response: ProductsResponse = request(
            PRODUCTS_QUERY,
            Some(TenantScopedVariables {
                tenant_id: request_data.tenant_id,
                extra: ProductsVariables {
                    locale: normalize_locale_filter(request_data.locale),
                    filter: ProductsFilter {
                        status: normalize_status_filter(request_data.status),
                        vendor: None,
                        search: normalize_search_filter(request_data.search),
                        page: Some(1),
                        per_page: Some(24),
                    },
                },
            }),
            request_data.token,
            request_data.tenant_slug,
        )
        .await?;
        Ok(response.products)
    }

    async fn fetch_product(
        &self,
        request_data: InventoryProductRequest,
    ) -> Result<Option<InventoryProductDetail>, InventoryTransportError> {
        let response: ProductResponse = request(
            PRODUCT_QUERY,
            Some(TenantScopedVariables {
                tenant_id: request_data.tenant_id,
                extra: ProductVariables {
                    id: request_data.id,
                    locale: normalize_locale_filter(request_data.locale),
                },
            }),
            request_data.token,
            request_data.tenant_slug,
        )
        .await?;
        Ok(response.product)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        normalize_locale_filter, normalize_optional_trimmed, normalize_search_filter,
        normalize_status_filter, PRODUCTS_QUERY, PRODUCT_QUERY,
    };

    #[test]
    fn normalize_optional_trimmed_keeps_non_blank_and_drops_blank_values() {
        assert_eq!(
            normalize_optional_trimmed(Some("  value  ".to_string())),
            Some("value".to_string())
        );
        assert_eq!(normalize_optional_trimmed(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_trimmed(None), None);
    }

    #[test]
    fn normalize_locale_filter_trims_and_drops_blank_values() {
        assert_eq!(
            normalize_locale_filter(Some("  de-DE  ".to_string())),
            Some("de-DE".to_string())
        );
        assert_eq!(normalize_locale_filter(Some("   ".to_string())), None);
        assert_eq!(normalize_locale_filter(None), None);
    }

    #[test]
    fn normalize_search_filter_trims_and_drops_blank_values() {
        assert_eq!(
            normalize_search_filter(Some("  winter jacket  ".to_string())),
            Some("winter jacket".to_string())
        );
        assert_eq!(normalize_search_filter(Some("   ".to_string())), None);
        assert_eq!(normalize_search_filter(None), None);
    }

    #[test]
    fn normalize_status_filter_trims_and_uppercases_values() {
        assert_eq!(
            normalize_status_filter(Some(" active ".to_string())),
            Some("ACTIVE".to_string())
        );
    }

    #[test]
    fn normalize_status_filter_drops_blank_values() {
        assert_eq!(normalize_status_filter(Some("   ".to_string())), None);
        assert_eq!(normalize_status_filter(None), None);
    }

    #[test]
    fn transitional_graphql_adapter_keeps_inventory_read_model_fields() {
        for expected in [
            "inventoryQuantity",
            "inventoryPolicy",
            "inStock",
            "variants",
            "translations",
            "shippingProfileSlug",
        ] {
            assert!(
                PRODUCT_QUERY.contains(expected),
                "product query must keep inventory read model field {expected}"
            );
        }

        for expected in ["total", "page", "perPage", "hasNext", "items"] {
            assert!(
                PRODUCTS_QUERY.contains(expected),
                "products query must keep inventory feed field {expected}"
            );
        }
    }
}
