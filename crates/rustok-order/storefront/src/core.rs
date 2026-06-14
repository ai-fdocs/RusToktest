use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderCheckoutResultData {
    pub order_id: String,
    pub order_status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderCheckoutResultLabels {
    pub badge: String,
    pub module_ownership: String,
    pub order_status_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderCheckoutResultViewModel {
    pub order_id: String,
    pub order_status_label: String,
    pub order_status: String,
    pub module_ownership: String,
}

pub fn build_order_checkout_result_view_model(
    data: OrderCheckoutResultData,
    labels: &OrderCheckoutResultLabels,
) -> OrderCheckoutResultViewModel {
    OrderCheckoutResultViewModel {
        order_id: data.order_id.trim().to_string(),
        order_status: data.order_status.trim().to_string(),
        order_status_label: labels.order_status_label.clone(),
        module_ownership: labels.module_ownership.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_order_checkout_result_identity_and_status() {
        let view_model = build_order_checkout_result_view_model(
            OrderCheckoutResultData {
                order_id: " order_1 ".into(),
                order_status: " completed ".into(),
            },
            &OrderCheckoutResultLabels {
                badge: "checkout result".into(),
                module_ownership: "Order details remain order-owned".into(),
                order_status_label: "Order status".into(),
            },
        );

        assert_eq!(view_model.order_id, "order_1");
        assert_eq!(view_model.order_status, "completed");
        assert_eq!(view_model.order_status_label, "Order status");
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderCheckoutActionLabels {
    pub pending: String,
    pub complete: String,
}

pub fn order_checkout_action_label(busy: bool, labels: &OrderCheckoutActionLabels) -> String {
    if busy {
        labels.pending.clone()
    } else {
        labels.complete.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct StorefrontCheckoutCompletion {
    pub order_id: String,
    pub order_status: String,
    pub currency_code: String,
    pub shipping_total: String,
    pub adjustment_total: String,
    pub total_amount: String,
    pub adjustments: Vec<StorefrontCheckoutAdjustment>,
    pub payment_collection_id: String,
    pub payment_collection_status: String,
    pub fulfillment_count: u64,
    pub context_locale: String,
    pub context_currency_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct StorefrontCheckoutAdjustment {
    pub id: String,
    pub line_item_id: Option<String>,
    pub source_type: String,
    pub source_id: Option<String>,
    pub scope: Option<String>,
    pub amount: String,
    pub currency_code: String,
    pub metadata: String,
}
