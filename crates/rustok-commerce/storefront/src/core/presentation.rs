use crate::i18n::t;
use crate::model::{StorefrontCheckoutPaymentCollection, StorefrontCommerceData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommerceStorefrontShellViewModel {
    pub badge: String,
    pub title: String,
    pub subtitle: String,
    pub load_error: String,
    pub action_error: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommerceStorefrontContextViewModel {
    pub effective_locale: String,
    pub tenant: String,
    pub tenant_default_locale: String,
    pub channel: String,
    pub channel_resolution_source: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommercePaymentCollectionViewModel {
    pub collection_id: String,
    pub status: String,
}

pub fn build_storefront_shell_view_model(locale: Option<&str>) -> CommerceStorefrontShellViewModel {
    CommerceStorefrontShellViewModel {
        badge: t(locale, "commerce.badge", "commerce"),
        title: t(locale, "commerce.title", "Commerce orchestration hub"),
        subtitle: t(
            locale,
            "commerce.subtitle",
            "Catalog, pricing, regions, and cart line-item handling now live in module-owned storefront packages. Commerce remains the aggregate storefront handoff for checkout context and cross-domain flow.",
        ),
        load_error: t(
            locale,
            "commerce.error.load",
            "Failed to load commerce storefront data",
        ),
        action_error: t(
            locale,
            "commerce.error.action",
            "Failed to update aggregate checkout state",
        ),
    }
}

pub fn build_storefront_context_view_model(
    data: StorefrontCommerceData,
    locale: Option<&str>,
) -> CommerceStorefrontContextViewModel {
    let empty_value = t(locale, "commerce.context.empty", "not resolved");

    CommerceStorefrontContextViewModel {
        effective_locale: data.effective_locale,
        tenant: data
            .tenant_slug
            .unwrap_or_else(|| t(locale, "commerce.context.tenantMissing", "host tenant")),
        tenant_default_locale: data.tenant_default_locale,
        channel: data.channel_slug.unwrap_or_else(|| empty_value.clone()),
        channel_resolution_source: data
            .channel_resolution_source
            .unwrap_or_else(|| empty_value.clone()),
    }
}

pub fn build_payment_collection_view_model(
    payment_collection: Option<StorefrontCheckoutPaymentCollection>,
    locale: Option<&str>,
) -> CommercePaymentCollectionViewModel {
    let (collection_id, status) = payment_collection
        .map(|collection| (collection.id, collection.status))
        .unwrap_or_else(|| {
            (
                t(locale, "commerce.payment.emptyId", "not attached"),
                t(locale, "commerce.payment.emptyStatus", "pending"),
            )
        });

    CommercePaymentCollectionViewModel {
        collection_id,
        status,
    }
}

pub fn error_with_context(context: &str, error: &str) -> String {
    format!("{context}: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn storefront_data(
        tenant_slug: Option<&str>,
        channel_slug: Option<&str>,
        channel_resolution_source: Option<&str>,
    ) -> StorefrontCommerceData {
        StorefrontCommerceData {
            effective_locale: "ru".to_string(),
            tenant_slug: tenant_slug.map(str::to_string),
            tenant_default_locale: "en".to_string(),
            channel_slug: channel_slug.map(str::to_string),
            channel_resolution_source: channel_resolution_source.map(str::to_string),
            selected_cart_id: None,
            checkout: None,
        }
    }

    #[test]
    fn context_view_model_preserves_resolved_context() {
        let view_model = build_storefront_context_view_model(
            storefront_data(Some("main"), Some("web"), Some("domain")),
            Some("en"),
        );

        assert_eq!(view_model.effective_locale, "ru");
        assert_eq!(view_model.tenant, "main");
        assert_eq!(view_model.tenant_default_locale, "en");
        assert_eq!(view_model.channel, "web");
        assert_eq!(view_model.channel_resolution_source, "domain");
    }

    #[test]
    fn context_view_model_applies_missing_context_fallbacks() {
        let view_model =
            build_storefront_context_view_model(storefront_data(None, None, None), Some("en"));

        assert_eq!(view_model.tenant, "host tenant");
        assert_eq!(view_model.channel, "not resolved");
        assert_eq!(view_model.channel_resolution_source, "not resolved");
    }

    #[test]
    fn payment_collection_view_model_preserves_resolved_collection() {
        let view_model = build_payment_collection_view_model(
            Some(StorefrontCheckoutPaymentCollection {
                id: "paycol_1".to_string(),
                status: "authorized".to_string(),
            }),
            Some("en"),
        );

        assert_eq!(view_model.collection_id, "paycol_1");
        assert_eq!(view_model.status, "authorized");
    }

    #[test]
    fn payment_collection_view_model_applies_empty_fallbacks() {
        let view_model = build_payment_collection_view_model(None, Some("en"));

        assert_eq!(view_model.collection_id, "not attached");
        assert_eq!(view_model.status, "pending");
    }
}
