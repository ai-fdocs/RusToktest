mod graphql_adapter;
mod native_server_adapter;

use crate::api::ApiError;
use crate::core::{FetchCommerceRequest, SelectShippingOptionRequest};
use crate::model::StorefrontCommerceData;

pub async fn fetch_storefront_commerce(
    request: FetchCommerceRequest,
) -> Result<StorefrontCommerceData, ApiError> {
    match native_server_adapter::fetch_storefront_commerce(request.clone()).await {
        Ok(data) => Ok(data),
        Err(error) if should_fallback_to_graphql(&error) => {
            graphql_adapter::fetch_storefront_commerce(request).await
        }
        Err(error) => Err(error),
    }
}

#[allow(dead_code)]
pub async fn select_storefront_shipping_option(
    request: SelectShippingOptionRequest,
) -> Result<(), ApiError> {
    match native_server_adapter::select_storefront_shipping_option(request.clone()).await {
        Ok(()) => Ok(()),
        Err(error) if should_fallback_to_graphql(&error) => {
            graphql_adapter::select_storefront_shipping_option(request).await
        }
        Err(error) => Err(error),
    }
}

fn should_fallback_to_graphql(error: &ApiError) -> bool {
    match error {
        ApiError::ServerFn(server_error) => {
            server_error.contains("MissingServer")
                || server_error.contains("missing server")
                || server_error.contains("not available on this target")
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_missing_server_errors_can_fallback_to_graphql() {
        assert!(should_fallback_to_graphql(&ApiError::ServerFn(
            "MissingServerFunction".into()
        )));
        assert!(should_fallback_to_graphql(&ApiError::ServerFn(
            "server function is not available on this target".into()
        )));
    }

    #[test]
    fn validation_and_graphql_errors_do_not_trigger_fetch_fallback() {
        assert!(!should_fallback_to_graphql(&ApiError::Validation(
            "cart_id must be a valid UUID".into()
        )));
        assert!(!should_fallback_to_graphql(&ApiError::Graphql(
            "network unavailable".into()
        )));
    }
}
