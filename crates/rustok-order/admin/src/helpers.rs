use leptos::prelude::*;

use crate::model::OrderDetailEnvelope;

#[allow(clippy::too_many_arguments)]
pub async fn handle_action_result(
    result: Result<(), crate::transport::ApiError>,
    token_value: Option<String>,
    tenant_value: Option<String>,
    tenant_id: String,
    order_id: String,
    action_error_label: String,
    load_order_error_label: String,
    order_not_found_label: String,
    set_refresh_nonce: WriteSignal<u64>,
    set_busy: WriteSignal<bool>,
    set_error: WriteSignal<Option<String>>,
    set_selected_id: WriteSignal<Option<String>>,
    set_selected: WriteSignal<Option<OrderDetailEnvelope>>,
    set_payment_id: WriteSignal<String>,
    set_payment_method: WriteSignal<String>,
    set_tracking_number: WriteSignal<String>,
    set_carrier: WriteSignal<String>,
    set_delivered_signature: WriteSignal<String>,
    set_cancel_reason: WriteSignal<String>,
) {
    match result {
        Ok(()) => {
            match crate::transport::fetch_order_detail(
                token_value,
                tenant_value,
                tenant_id,
                order_id,
            )
            .await
            {
                Ok(Some(detail)) => {
                    apply_order_detail(
                        &detail,
                        set_selected_id,
                        set_selected,
                        set_payment_id,
                        set_payment_method,
                        set_tracking_number,
                        set_carrier,
                        set_delivered_signature,
                        set_cancel_reason,
                    );
                    set_refresh_nonce.update(|value| *value += 1);
                }
                Ok(None) => {
                    clear_order_detail(
                        set_selected_id,
                        set_selected,
                        set_payment_id,
                        set_payment_method,
                        set_tracking_number,
                        set_carrier,
                        set_delivered_signature,
                        set_cancel_reason,
                    );
                    set_error.set(Some(order_not_found_label));
                }
                Err(err) => {
                    clear_order_detail(
                        set_selected_id,
                        set_selected,
                        set_payment_id,
                        set_payment_method,
                        set_tracking_number,
                        set_carrier,
                        set_delivered_signature,
                        set_cancel_reason,
                    );
                    set_error.set(Some(format!("{load_order_error_label}: {err}")));
                }
            }
        }
        Err(err) => set_error.set(Some(format!("{action_error_label}: {err}"))),
    }

    set_busy.set(false);
}

#[allow(clippy::too_many_arguments)]
pub fn apply_order_detail(
    detail: &OrderDetailEnvelope,
    set_selected_id: WriteSignal<Option<String>>,
    set_selected: WriteSignal<Option<OrderDetailEnvelope>>,
    set_payment_id: WriteSignal<String>,
    set_payment_method: WriteSignal<String>,
    set_tracking_number: WriteSignal<String>,
    set_carrier: WriteSignal<String>,
    set_delivered_signature: WriteSignal<String>,
    set_cancel_reason: WriteSignal<String>,
) {
    set_selected_id.set(Some(detail.order.id.clone()));
    set_selected.set(Some(detail.clone()));
    set_payment_id.set(detail.order.payment_id.clone().unwrap_or_default());
    set_payment_method.set(
        detail
            .order
            .payment_method
            .clone()
            .unwrap_or_else(|| "manual".to_string()),
    );
    set_tracking_number.set(
        detail
            .order
            .tracking_number
            .clone()
            .or_else(|| {
                detail
                    .fulfillment
                    .as_ref()
                    .and_then(|item| item.tracking_number.clone())
            })
            .unwrap_or_default(),
    );
    set_carrier.set(
        detail
            .order
            .carrier
            .clone()
            .or_else(|| {
                detail
                    .fulfillment
                    .as_ref()
                    .and_then(|item| item.carrier.clone())
            })
            .unwrap_or_else(|| "manual".to_string()),
    );
    set_delivered_signature.set(detail.order.delivered_signature.clone().unwrap_or_default());
    set_cancel_reason.set(
        detail
            .order
            .cancellation_reason
            .clone()
            .or_else(|| {
                detail
                    .fulfillment
                    .as_ref()
                    .and_then(|item| item.cancellation_reason.clone())
            })
            .unwrap_or_default(),
    );
}

#[allow(clippy::too_many_arguments)]
pub fn clear_order_detail(
    set_selected_id: WriteSignal<Option<String>>,
    set_selected: WriteSignal<Option<OrderDetailEnvelope>>,
    set_payment_id: WriteSignal<String>,
    set_payment_method: WriteSignal<String>,
    set_tracking_number: WriteSignal<String>,
    set_carrier: WriteSignal<String>,
    set_delivered_signature: WriteSignal<String>,
    set_cancel_reason: WriteSignal<String>,
) {
    set_selected_id.set(None);
    set_selected.set(None);
    set_payment_id.set(String::new());
    set_payment_method.set("manual".to_string());
    set_tracking_number.set(String::new());
    set_carrier.set("manual".to_string());
    set_delivered_signature.set(String::new());
    set_cancel_reason.set(String::new());
}
