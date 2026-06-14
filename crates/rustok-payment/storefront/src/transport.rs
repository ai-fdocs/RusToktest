use leptos::prelude::*;
use leptos_graphql::{GraphqlHttpError, GraphqlRequest, execute as execute_graphql};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

use crate::core::StorefrontCheckoutPaymentCollection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    Graphql(String),
    ServerFn(String),
    Validation(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Graphql(error) => write!(f, "{error}"),
            Self::ServerFn(error) => write!(f, "{error}"),
            Self::Validation(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<GraphqlHttpError> for ApiError {
    fn from(value: GraphqlHttpError) -> Self {
        Self::Graphql(value.to_string())
    }
}

impl From<ServerFnError> for ApiError {
    fn from(value: ServerFnError) -> Self {
        Self::ServerFn(value.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentCollectionCreateRequest {
    pub cart_id: String,
}

pub fn build_payment_collection_create_request(cart_id: String) -> PaymentCollectionCreateRequest {
    PaymentCollectionCreateRequest {
        cart_id: normalize_required(cart_id),
    }
}

fn normalize_required(value: String) -> String {
    value.trim().to_string()
}

const CREATE_STOREFRONT_PAYMENT_COLLECTION_MUTATION: &str = "mutation CreateStorefrontPaymentCollection($input: CreateStorefrontPaymentCollectionInput!) { createStorefrontPaymentCollection(input: $input) { id status currencyCode amount authorizedAmount capturedAmount orderId providerId createdAt updatedAt payments { id } } }";

#[derive(Debug, Deserialize)]
struct CreateStorefrontPaymentCollectionResponse {
    #[serde(rename = "createStorefrontPaymentCollection")]
    payment_collection: GraphqlPaymentCollection,
}

#[derive(Debug, Serialize)]
struct CreateStorefrontPaymentCollectionVariables {
    input: CreateStorefrontPaymentCollectionInput,
}

#[derive(Debug, Serialize)]
struct CreateStorefrontPaymentCollectionInput {
    #[serde(rename = "cartId")]
    cart_id: Uuid,
    metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GraphqlPaymentCollection {
    id: String,
    status: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
    amount: String,
    #[serde(rename = "authorizedAmount")]
    authorized_amount: String,
    #[serde(rename = "capturedAmount")]
    captured_amount: String,
    #[serde(rename = "orderId")]
    order_id: Option<String>,
    #[serde(rename = "providerId")]
    provider_id: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    payments: Vec<GraphqlPayment>,
}

#[derive(Debug, Deserialize)]
struct GraphqlPayment {}

fn configured_tenant_slug() -> Option<String> {
    [
        "RUSTOK_TENANT_SLUG",
        "NEXT_PUBLIC_TENANT_SLUG",
        "NEXT_PUBLIC_DEFAULT_TENANT_SLUG",
    ]
    .into_iter()
    .find_map(|key| {
        std::env::var(key).ok().and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
    })
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn normalize_cart_id(value: Option<String>) -> Option<String> {
    normalize_optional(value)
}

fn parse_cart_id(value: Option<String>) -> Result<Option<(String, Uuid)>, ApiError> {
    match normalize_cart_id(value) {
        Some(cart_id) => {
            let parsed = Uuid::parse_str(cart_id.as_str())
                .map_err(|_| ApiError::Validation("cart_id must be a valid UUID".to_string()))?;
            Ok(Some((cart_id, parsed)))
        }
        None => Ok(None),
    }
}

fn graphql_url() -> String {
    if let Ok(url) = std::env::var("RUSTOK_GRAPHQL_URL") {
        return url;
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

async fn request<V, T>(query: &str, variables: V) -> Result<T, ApiError>
where
    V: Serialize,
    T: for<'de> Deserialize<'de>,
{
    execute_graphql(
        &graphql_url(),
        GraphqlRequest::new(query, Some(variables)),
        None,
        configured_tenant_slug(),
        None,
    )
    .await
    .map_err(ApiError::from)
}

fn map_graphql_payment_collection(
    value: GraphqlPaymentCollection,
) -> StorefrontCheckoutPaymentCollection {
    StorefrontCheckoutPaymentCollection {
        id: value.id,
        status: value.status,
        currency_code: value.currency_code,
        amount: value.amount,
        authorized_amount: value.authorized_amount,
        captured_amount: value.captured_amount,
        order_id: value.order_id,
        provider_id: value.provider_id,
        payment_count: value.payments.len() as u64,
        created_at: value.created_at,
        updated_at: value.updated_at,
    }
}

#[cfg(feature = "ssr")]
async fn resolve_storefront_customer_id(
    db: sea_orm::DatabaseConnection,
    tenant_id: Uuid,
    auth: Option<rustok_api::AuthContext>,
) -> Result<Option<Uuid>, ServerFnError> {
    let Some(auth) = auth else {
        return Ok(None);
    };

    match rustok_customer::CustomerService::new(db)
        .get_customer_by_user(tenant_id, auth.user_id)
        .await
    {
        Ok(customer) => Ok(Some(customer.id)),
        Err(rustok_customer::CustomerError::CustomerByUserNotFound(_)) => Ok(None),
        Err(err) => Err(ServerFnError::new(err.to_string())),
    }
}

#[cfg(feature = "ssr")]
fn ensure_storefront_cart_access(
    cart: &rustok_commerce::CartResponse,
    storefront_customer_id: Option<Uuid>,
) -> Result<(), ServerFnError> {
    if let Some(owner_customer_id) = cart.customer_id {
        match storefront_customer_id {
            Some(customer_id) if customer_id == owner_customer_id => Ok(()),
            Some(_) => Err(ServerFnError::new(
                "Cart does not belong to the current storefront customer",
            )),
            None => Err(ServerFnError::new(
                "Authentication required to access this cart",
            )),
        }
    } else {
        Ok(())
    }
}

#[cfg(feature = "ssr")]
fn merge_metadata(current: Value, patch: Value) -> Value {
    match (current, patch) {
        (Value::Object(mut current), Value::Object(patch)) => {
            for (key, value) in patch {
                current.insert(key, value);
            }
            Value::Object(current)
        }
        (_, patch) => patch,
    }
}

#[cfg(feature = "ssr")]
fn cart_context_metadata(
    cart: &rustok_commerce::CartResponse,
    context: &rustok_commerce::StoreContextResponse,
) -> Value {
    json!({
        "cart_context": {
            "region_id": cart.region_id,
            "country_code": cart.country_code,
            "locale": context.locale,
            "currency_code": context.currency_code,
            "selected_shipping_option_id": cart.selected_shipping_option_id,
            "email": cart.email,
        }
    })
}

#[cfg(feature = "ssr")]
fn map_native_payment_collection(
    value: rustok_commerce::PaymentCollectionResponse,
) -> StorefrontCheckoutPaymentCollection {
    StorefrontCheckoutPaymentCollection {
        id: value.id.to_string(),
        status: value.status,
        currency_code: value.currency_code,
        amount: value.amount.normalize().to_string(),
        authorized_amount: value.authorized_amount.normalize().to_string(),
        captured_amount: value.captured_amount.normalize().to_string(),
        order_id: value.order_id.map(|value| value.to_string()),
        provider_id: value.provider_id,
        payment_count: value.payments.len() as u64,
        created_at: value.created_at.to_rfc3339(),
        updated_at: value.updated_at.to_rfc3339(),
    }
}

#[cfg(feature = "ssr")]
fn resolve_requested_locale(
    requested: Option<String>,
    request_context_locale: Option<&str>,
    tenant_default_locale: &str,
) -> String {
    normalize_optional(requested)
        .or_else(|| {
            request_context_locale.and_then(|value| normalize_optional(Some(value.to_string())))
        })
        .or_else(|| normalize_optional(Some(tenant_default_locale.to_string())))
        .unwrap_or_default()
}

#[cfg(feature = "ssr")]
fn normalize_public_channel_slug(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_ascii_lowercase())
}

#[cfg(feature = "ssr")]
async fn reprice_storefront_cart_line_items(
    app_ctx: &loco_rs::app::AppContext,
    tenant_id: Uuid,
    cart_service: &rustok_commerce::CartService,
    cart: rustok_cart::CartResponse,
    request_context: Option<&rustok_api::RequestContext>,
) -> Result<rustok_cart::CartResponse, ServerFnError> {
    if cart.line_items.is_empty() {
        return Ok(cart);
    }

    let pricing_service = rustok_commerce::PricingService::new(
        app_ctx.db.clone(),
        rustok_api::loco::transactional_event_bus_from_context(app_ctx),
    );
    let channel_id = cart
        .channel_id
        .or_else(|| request_context.and_then(|ctx| ctx.channel_id));
    let channel_slug = normalize_public_channel_slug(cart.channel_slug.as_deref()).or_else(|| {
        request_context.and_then(|ctx| normalize_public_channel_slug(ctx.channel_slug.as_deref()))
    });
    let mut updates = Vec::new();
    for line_item in &cart.line_items {
        let Some(variant_id) = line_item.variant_id else {
            continue;
        };
        let pricing_context = rustok_commerce::services::PriceResolutionContext {
            currency_code: cart.currency_code.to_ascii_uppercase(),
            region_id: cart.region_id,
            price_list_id: None,
            channel_id,
            channel_slug: channel_slug.clone(),
            quantity: Some(line_item.quantity),
        };
        let resolved_price = pricing_service
            .resolve_variant_price(tenant_id, variant_id, pricing_context)
            .await
            .map_err(|err| ServerFnError::new(err.to_string()))?
            .ok_or_else(|| {
                ServerFnError::new("Unable to resolve storefront price for cart line item")
            })?;
        updates.push(storefront_cart_pricing_update(
            line_item.id,
            line_item.quantity,
            &resolved_price,
        ));
    }

    if updates.is_empty() {
        Ok(cart)
    } else {
        cart_service
            .reprice_line_items(tenant_id, cart.id, updates)
            .await
            .map_err(|err| ServerFnError::new(err.to_string()))
    }
}

#[cfg(feature = "ssr")]
fn storefront_cart_pricing_update(
    line_item_id: Uuid,
    quantity: i32,
    resolved_price: &rustok_commerce::services::ResolvedPrice,
) -> rustok_cart::services::cart::CartLineItemPricingUpdate {
    let base_unit_price = resolved_price
        .compare_at_amount
        .filter(|compare_at| *compare_at > resolved_price.amount)
        .unwrap_or(resolved_price.amount);
    let pricing_adjustment = if base_unit_price > resolved_price.amount {
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "kind".to_string(),
            serde_json::Value::from(if resolved_price.price_list_id.is_some() {
                "price_list"
            } else {
                "sale"
            }),
        );
        metadata.insert(
            "base_amount".to_string(),
            serde_json::Value::from(base_unit_price.normalize().to_string()),
        );
        metadata.insert(
            "effective_amount".to_string(),
            serde_json::Value::from(resolved_price.amount.normalize().to_string()),
        );
        if let Some(compare_at_amount) = resolved_price.compare_at_amount {
            metadata.insert(
                "compare_at_amount".to_string(),
                serde_json::Value::from(compare_at_amount.normalize().to_string()),
            );
        }
        if let Some(discount_percent) = resolved_price.discount_percent {
            metadata.insert(
                "discount_percent".to_string(),
                serde_json::Value::from(discount_percent.normalize().to_string()),
            );
        }
        if let Some(price_list_id) = resolved_price.price_list_id {
            metadata.insert(
                "price_list_id".to_string(),
                serde_json::Value::from(price_list_id.to_string()),
            );
        }
        if let Some(channel_id) = resolved_price.channel_id {
            metadata.insert(
                "channel_id".to_string(),
                serde_json::Value::from(channel_id.to_string()),
            );
        }
        if let Some(channel_slug) = resolved_price.channel_slug.as_deref() {
            metadata.insert(
                "channel_slug".to_string(),
                serde_json::Value::from(channel_slug),
            );
        }

        Some(rustok_cart::services::cart::CartPricingAdjustmentUpdate {
            source_id: resolved_price.price_list_id.map(|value| value.to_string()),
            amount: (base_unit_price - resolved_price.amount)
                * rust_decimal::Decimal::from(quantity),
            metadata: serde_json::Value::Object(metadata),
        })
    } else {
        None
    };

    rustok_cart::services::cart::CartLineItemPricingUpdate {
        line_item_id,
        unit_price: base_unit_price,
        pricing_adjustment,
    }
}

pub async fn create_storefront_payment_collection(
    request: PaymentCollectionCreateRequest,
) -> Result<StorefrontCheckoutPaymentCollection, ApiError> {
    match create_storefront_payment_collection_server(request.cart_id.clone()).await {
        Ok(collection) => Ok(collection),
        Err(error) if should_fallback_to_graphql(&error) => {
            create_storefront_payment_collection_graphql(request.cart_id).await
        }
        Err(error) => Err(ApiError::from(error)),
    }
}

fn should_fallback_to_graphql(error: &ServerFnError) -> bool {
    let server_error = error.to_string();
    server_error.contains("MissingServer")
        || server_error.contains("missing server")
        || server_error.contains("not available on this target")
}

pub async fn create_storefront_payment_collection_graphql(
    cart_id: String,
) -> Result<StorefrontCheckoutPaymentCollection, ApiError> {
    let Some((_, parsed_cart_id)) = parse_cart_id(Some(cart_id))? else {
        return Err(ApiError::Validation(
            "cart_id must not be empty".to_string(),
        ));
    };

    let response: CreateStorefrontPaymentCollectionResponse = request(
        CREATE_STOREFRONT_PAYMENT_COLLECTION_MUTATION,
        CreateStorefrontPaymentCollectionVariables {
            input: CreateStorefrontPaymentCollectionInput {
                cart_id: parsed_cart_id,
                metadata: None,
            },
        },
    )
    .await?;

    Ok(map_graphql_payment_collection(response.payment_collection))
}

#[server(prefix = "/api/fn", endpoint = "payment/create-payment-collection")]
pub async fn create_storefront_payment_collection_server(
    cart_id: String,
) -> Result<StorefrontCheckoutPaymentCollection, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use leptos::prelude::expect_context;
        use loco_rs::app::AppContext;

        let app_ctx = expect_context::<AppContext>();
        let request_context = leptos_axum::extract::<rustok_api::RequestContext>()
            .await
            .map_err(ServerFnError::new)?;
        let tenant = leptos_axum::extract::<rustok_api::TenantContext>()
            .await
            .map_err(ServerFnError::new)?;
        let auth = leptos_axum::extract::<rustok_api::OptionalAuthContext>()
            .await
            .map_err(ServerFnError::new)?;
        let Some((_, parsed_cart_id)) =
            parse_cart_id(Some(cart_id.clone())).map_err(|err| ServerFnError::new(err.to_string()))?
        else {
            return Err(ServerFnError::new("cart_id must not be empty"));
        };

        let cart_service = rustok_commerce::CartService::new(app_ctx.db.clone());
        let cart = cart_service
            .get_cart(tenant.id, parsed_cart_id)
            .await
            .map_err(|err| ServerFnError::new(err.to_string()))?;
        let storefront_customer_id =
            resolve_storefront_customer_id(app_ctx.db.clone(), tenant.id, auth.0).await?;
        ensure_storefront_cart_access(&cart, storefront_customer_id)?;
        let cart = reprice_storefront_cart_line_items(
            &app_ctx,
            tenant.id,
            &cart_service,
            cart,
            Some(&request_context),
        )
        .await?;

        let service = rustok_commerce::PaymentService::new(app_ctx.db.clone());
        if let Some(existing) = service
            .find_reusable_collection_by_cart(tenant.id, cart.id)
            .await
            .map_err(|err| ServerFnError::new(err.to_string()))?
        {
            return Ok(map_native_payment_collection(existing));
        }

        let context = rustok_commerce::StoreContextService::new(app_ctx.db.clone())
            .resolve_context(
                tenant.id,
                rustok_commerce::ResolveStoreContextInput {
                    region_id: cart.region_id,
                    country_code: cart.country_code.clone(),
                    locale: Some(resolve_requested_locale(
                        cart.locale_code.clone(),
                        Some(request_context.locale.as_str()),
                        tenant.default_locale.as_str(),
                    )),
                    currency_code: Some(cart.currency_code.clone()),
                },
            )
            .await
            .map_err(|err| ServerFnError::new(err.to_string()))?;

        let collection = service
            .create_collection(
                tenant.id,
                rustok_commerce::CreatePaymentCollectionInput {
                    cart_id: Some(cart.id),
                    order_id: None,
                    customer_id: cart.customer_id,
                    currency_code: cart.currency_code.clone(),
                    amount: cart.total_amount,
                    metadata: merge_metadata(json!({}), cart_context_metadata(&cart, &context)),
                },
            )
            .await
            .map_err(|err| ServerFnError::new(err.to_string()))?;

        Ok(map_native_payment_collection(collection))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = cart_id;
        Err(ServerFnError::new(
            "payment/create-payment-collection requires the `ssr` feature",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_trims_cart_id() {
        let request = build_payment_collection_create_request(" cart-1 ".into());
        assert_eq!(request.cart_id, "cart-1");
    }
}
