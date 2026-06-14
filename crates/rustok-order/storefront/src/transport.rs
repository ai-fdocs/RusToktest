use leptos::prelude::*;
use leptos_graphql::{GraphqlHttpError, GraphqlRequest, execute as execute_graphql};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

use crate::core::{StorefrontCheckoutAdjustment, StorefrontCheckoutCompletion};

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
pub struct CompleteCheckoutRequest {
    pub cart_id: String,
}

pub fn build_complete_checkout_request(cart_id: String) -> CompleteCheckoutRequest {
    CompleteCheckoutRequest {
        cart_id: normalize_required(cart_id),
    }
}

fn normalize_required(value: String) -> String {
    value.trim().to_string()
}

const COMPLETE_STOREFRONT_CHECKOUT_MUTATION: &str = "mutation CompleteStorefrontCheckout($input: CompleteStorefrontCheckoutInput!) { completeStorefrontCheckout(input: $input) { order { id status currencyCode shippingTotal adjustmentTotal totalAmount adjustments { id lineItemId sourceType sourceId amount currencyCode metadata } } paymentCollection { id status currencyCode } fulfillments { id } context { locale currencyCode } } }";

#[derive(Debug, Deserialize)]
struct CompleteStorefrontCheckoutResponse {
    #[serde(rename = "completeStorefrontCheckout")]
    completion: GraphqlCheckoutCompletion,
}

#[derive(Debug, Serialize)]
struct CompleteStorefrontCheckoutVariables {
    input: CompleteStorefrontCheckoutInput,
}

#[derive(Debug, Serialize)]
struct CompleteStorefrontCheckoutInput {
    #[serde(rename = "cartId")]
    cart_id: Uuid,
    #[serde(rename = "createFulfillment")]
    create_fulfillment: bool,
    metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GraphqlCheckoutCompletion {
    order: GraphqlOrderSummary,
    #[serde(rename = "paymentCollection")]
    payment_collection: GraphqlCheckoutCompletionPaymentCollection,
    fulfillments: Vec<GraphqlFulfillmentSummary>,
    context: GraphqlStoreContext,
}

#[derive(Debug, Deserialize)]
struct GraphqlOrderSummary {
    id: String,
    status: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
    #[serde(rename = "shippingTotal")]
    shipping_total: String,
    #[serde(rename = "adjustmentTotal")]
    adjustment_total: String,
    #[serde(rename = "totalAmount")]
    total_amount: String,
    adjustments: Vec<GraphqlCheckoutAdjustment>,
}

#[derive(Debug, Deserialize)]
struct GraphqlCheckoutAdjustment {
    id: String,
    #[serde(rename = "lineItemId")]
    line_item_id: Option<String>,
    #[serde(rename = "sourceType")]
    source_type: String,
    #[serde(rename = "sourceId")]
    source_id: Option<String>,
    amount: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
    metadata: String,
}

#[derive(Debug, Deserialize)]
struct GraphqlCheckoutCompletionPaymentCollection {
    id: String,
    status: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
}

#[derive(Debug, Deserialize)]
struct GraphqlFulfillmentSummary {}

#[derive(Debug, Deserialize)]
struct GraphqlStoreContext {
    locale: String,
    #[serde(rename = "currencyCode")]
    currency_code: Option<String>,
}

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

fn parse_adjustment_scope(metadata: &str) -> Option<String> {
    serde_json::from_str::<Value>(metadata)
        .ok()
        .and_then(|value| {
            value
                .get("scope")
                .and_then(Value::as_str)
                .map(str::to_string)
        })
}

fn map_graphql_checkout_completion(
    value: GraphqlCheckoutCompletion,
) -> StorefrontCheckoutCompletion {
    let adjustments = value
        .order
        .adjustments
        .into_iter()
        .map(|adjustment| StorefrontCheckoutAdjustment {
            id: adjustment.id,
            line_item_id: adjustment.line_item_id,
            source_type: adjustment.source_type,
            source_id: adjustment.source_id,
            scope: parse_adjustment_scope(&adjustment.metadata),
            amount: adjustment.amount,
            currency_code: adjustment.currency_code,
            metadata: adjustment.metadata,
        })
        .collect::<Vec<_>>();
    StorefrontCheckoutCompletion {
        order_id: value.order.id,
        order_status: value.order.status,
        currency_code: value.order.currency_code,
        shipping_total: value.order.shipping_total,
        adjustment_total: value.order.adjustment_total,
        total_amount: value.order.total_amount,
        adjustments,
        payment_collection_id: value.payment_collection.id,
        payment_collection_status: value.payment_collection.status,
        fulfillment_count: value.fulfillments.len() as u64,
        context_locale: value.context.locale,
        context_currency_code: value
            .context
            .currency_code
            .or(Some(value.payment_collection.currency_code)),
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
fn map_native_checkout_completion(
    value: rustok_commerce::CompleteCheckoutResponse,
) -> StorefrontCheckoutCompletion {
    let adjustments = value
        .order
        .adjustments
        .into_iter()
        .map(|adjustment| StorefrontCheckoutAdjustment {
            id: adjustment.id.to_string(),
            line_item_id: adjustment.line_item_id.map(|value| value.to_string()),
            source_type: adjustment.source_type,
            source_id: adjustment.source_id,
            scope: adjustment
                .metadata
                .get("scope")
                .and_then(Value::as_str)
                .map(str::to_string),
            amount: adjustment.amount.normalize().to_string(),
            currency_code: adjustment.currency_code,
            metadata: adjustment.metadata.to_string(),
        })
        .collect::<Vec<_>>();
    StorefrontCheckoutCompletion {
        order_id: value.order.id.to_string(),
        order_status: value.order.status,
        currency_code: value.order.currency_code,
        shipping_total: value.order.shipping_total.normalize().to_string(),
        adjustment_total: value.order.adjustment_total.normalize().to_string(),
        total_amount: value.order.total_amount.normalize().to_string(),
        adjustments,
        payment_collection_id: value.payment_collection.id.to_string(),
        payment_collection_status: value.payment_collection.status,
        fulfillment_count: value.fulfillments.len() as u64,
        context_locale: value.context.locale,
        context_currency_code: value.context.currency_code,
    }
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

pub async fn complete_storefront_checkout(
    request: CompleteCheckoutRequest,
) -> Result<StorefrontCheckoutCompletion, ApiError> {
    match complete_storefront_checkout_server(request.cart_id.clone()).await {
        Ok(completion) => Ok(completion),
        Err(error) if should_fallback_to_graphql(&error) => {
            complete_storefront_checkout_graphql(request.cart_id).await
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

pub async fn complete_storefront_checkout_graphql(
    cart_id: String,
) -> Result<StorefrontCheckoutCompletion, ApiError> {
    let Some((_, parsed_cart_id)) = parse_cart_id(Some(cart_id))? else {
        return Err(ApiError::Validation(
            "cart_id must not be empty".to_string(),
        ));
    };

    let response: CompleteStorefrontCheckoutResponse = request(
        COMPLETE_STOREFRONT_CHECKOUT_MUTATION,
        CompleteStorefrontCheckoutVariables {
            input: CompleteStorefrontCheckoutInput {
                cart_id: parsed_cart_id,
                create_fulfillment: true,
                metadata: None,
            },
        },
    )
    .await?;

    Ok(map_graphql_checkout_completion(response.completion))
}

#[server(prefix = "/api/fn", endpoint = "order/complete-checkout")]
pub async fn complete_storefront_checkout_server(
    cart_id: String,
) -> Result<StorefrontCheckoutCompletion, ServerFnError> {
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
            resolve_storefront_customer_id(app_ctx.db.clone(), tenant.id, auth.0.clone()).await?;
        ensure_storefront_cart_access(&cart, storefront_customer_id)?;
        let _ = reprice_storefront_cart_line_items(
            &app_ctx,
            tenant.id,
            &cart_service,
            cart,
            Some(&request_context),
        )
        .await?;
        let actor_id = auth.0.map(|auth| auth.user_id).unwrap_or_else(Uuid::nil);

        let response = rustok_commerce::CheckoutService::new(
            app_ctx.db.clone(),
            rustok_api::loco::transactional_event_bus_from_context(&app_ctx),
        )
        .complete_checkout(
            tenant.id,
            actor_id,
            rustok_commerce::CompleteCheckoutInput {
                cart_id: parsed_cart_id,
                shipping_option_id: None,
                shipping_selections: None,
                region_id: None,
                country_code: None,
                locale: None,
                create_fulfillment: true,
                metadata: json!({}),
            },
        )
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

        Ok(map_native_checkout_completion(response))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = cart_id;
        Err(ServerFnError::new(
            "order/complete-checkout requires the `ssr` feature",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_request_trims_cart_id() {
        let request = build_complete_checkout_request(" cart-1 ".into());
        assert_eq!(request.cart_id, "cart-1");
    }
}
