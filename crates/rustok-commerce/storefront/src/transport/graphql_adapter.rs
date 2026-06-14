use crate::api::{self, ApiError};
use crate::core::{FetchCommerceRequest, SelectShippingOptionRequest};
use crate::model::StorefrontCommerceData;

pub async fn fetch_storefront_commerce(
    request: FetchCommerceRequest,
) -> Result<StorefrontCommerceData, ApiError> {
    api::fetch_storefront_commerce_graphql(request.selected_cart_id, request.locale).await
}

#[allow(dead_code)]
pub async fn select_storefront_shipping_option(
    request: SelectShippingOptionRequest,
) -> Result<(), ApiError> {
    api::select_storefront_shipping_option_graphql(request.owner_request).await
}
