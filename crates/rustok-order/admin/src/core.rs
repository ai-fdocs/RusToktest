use crate::i18n::t;
use crate::model::{OrderDetail, OrderLineItem, OrderListItem};

pub const DEFAULT_ORDER_PAGE: u64 = 1;
pub const DEFAULT_ORDER_PER_PAGE: u64 = 24;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderListRequest {
    pub status: Option<String>,
    pub page: u64,
    pub per_page: u64,
}

pub fn text_or_none(value: impl AsRef<str>) -> Option<String> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub fn order_list_request(status: impl AsRef<str>) -> OrderListRequest {
    OrderListRequest {
        status: text_or_none(status),
        page: DEFAULT_ORDER_PAGE,
        per_page: DEFAULT_ORDER_PER_PAGE,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderMarkPaidCommand {
    pub payment_id: String,
    pub payment_method: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderShipCommand {
    pub tracking_number: String,
    pub carrier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderDeliverCommand {
    pub delivered_signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderCancelCommand {
    pub reason: Option<String>,
}

pub fn prepare_mark_paid_command(
    payment_id: impl AsRef<str>,
    payment_method: impl AsRef<str>,
    requirements_message: String,
) -> Result<OrderMarkPaidCommand, String> {
    let Some(payment_id) = text_or_none(payment_id) else {
        return Err(requirements_message);
    };
    let Some(payment_method) = text_or_none(payment_method) else {
        return Err(requirements_message);
    };

    Ok(OrderMarkPaidCommand {
        payment_id,
        payment_method,
    })
}

pub fn prepare_ship_order_command(
    tracking_number: impl AsRef<str>,
    carrier: impl AsRef<str>,
    requirements_message: String,
) -> Result<OrderShipCommand, String> {
    let Some(tracking_number) = text_or_none(tracking_number) else {
        return Err(requirements_message);
    };
    let Some(carrier) = text_or_none(carrier) else {
        return Err(requirements_message);
    };

    Ok(OrderShipCommand {
        tracking_number,
        carrier,
    })
}

pub fn prepare_deliver_order_command(delivered_signature: impl AsRef<str>) -> OrderDeliverCommand {
    OrderDeliverCommand {
        delivered_signature: text_or_none(delivered_signature),
    }
}

pub fn prepare_cancel_order_command(reason: impl AsRef<str>) -> OrderCancelCommand {
    OrderCancelCommand {
        reason: text_or_none(reason),
    }
}

pub fn localized_order_status(locale: Option<&str>, status: &str) -> String {
    match status {
        "pending" => t(locale, "order.status.pending", "Pending"),
        "confirmed" => t(locale, "order.status.confirmed", "Confirmed"),
        "paid" => t(locale, "order.status.paid", "Paid"),
        "shipped" => t(locale, "order.status.shipped", "Shipped"),
        "delivered" => t(locale, "order.status.delivered", "Delivered"),
        "cancelled" => t(locale, "order.status.cancelled", "Cancelled"),
        _ => status.to_string(),
    }
}

pub fn order_status_badge(status: &str) -> &'static str {
    match status {
        "delivered" => "border-emerald-200 bg-emerald-50 text-emerald-700",
        "paid" => "border-blue-200 bg-blue-50 text-blue-700",
        "shipped" => "border-amber-200 bg-amber-50 text-amber-700",
        "cancelled" => "border-rose-200 bg-rose-50 text-rose-700",
        _ => "border-slate-200 bg-slate-100 text-slate-700",
    }
}

pub fn summarize_order_lines(lines: &[OrderLineItem]) -> String {
    let preview = lines
        .iter()
        .take(2)
        .map(|line| format!("{} x{}", line.title, line.quantity))
        .collect::<Vec<_>>();
    if preview.is_empty() {
        "no line items".to_string()
    } else if lines.len() > 2 {
        format!("{} +{} more", preview.join(", "), lines.len() - 2)
    } else {
        preview.join(", ")
    }
}

pub fn format_order_caption(order: &OrderListItem) -> String {
    let mut parts = vec![format!("{} {}", order.total_amount, order.currency_code)];
    if let Some(customer_id) = order.customer_id.as_deref() {
        parts.push(format!("customer {}", short_order_id(customer_id)));
    }
    parts.push(format!("created {}", order.created_at));
    parts.join(" · ")
}

pub fn summarize_order_header(order: &OrderDetail) -> String {
    [
        Some(format!("{} {}", order.total_amount, order.currency_code)),
        order
            .payment_id
            .as_ref()
            .map(|payment_id| format!("payment {payment_id}")),
        order
            .tracking_number
            .as_ref()
            .map(|tracking| format!("tracking {tracking}")),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" · ")
}

pub fn summarize_order_timeline(order: &OrderDetail) -> String {
    let mut steps = vec![format!("created {}", order.created_at)];
    if let Some(value) = order.confirmed_at.as_deref() {
        steps.push(format!("confirmed {value}"));
    }
    if let Some(value) = order.paid_at.as_deref() {
        steps.push(format!("paid {value}"));
    }
    if let Some(value) = order.shipped_at.as_deref() {
        steps.push(format!("shipped {value}"));
    }
    if let Some(value) = order.delivered_at.as_deref() {
        steps.push(format!("delivered {value}"));
    }
    if let Some(value) = order.cancelled_at.as_deref() {
        steps.push(format!("cancelled {value}"));
    }
    steps.join(" · ")
}

pub fn action_hint(locale: Option<&str>, status: &str) -> String {
    match status {
        "confirmed" => t(
            locale,
            "order.actionHint.confirmed",
            "The next operational step is marking the order as paid.",
        ),
        "paid" => t(
            locale,
            "order.actionHint.paid",
            "The order is paid and ready for shipment.",
        ),
        "shipped" => t(
            locale,
            "order.actionHint.shipped",
            "The order is in transit and can be marked as delivered.",
        ),
        "delivered" => t(
            locale,
            "order.actionHint.delivered",
            "The order is complete; only inspection remains.",
        ),
        "cancelled" => t(
            locale,
            "order.actionHint.cancelled",
            "The order is cancelled; lifecycle buttons stay read-only.",
        ),
        _ => t(
            locale,
            "order.actionHint.pending",
            "This order is waiting for the next lifecycle event from checkout or operations.",
        ),
    }
}

pub fn short_order_id(value: &str) -> String {
    value.chars().take(8).collect()
}

pub fn text_or_dash(value: Option<&str>) -> String {
    value
        .filter(|item| !item.trim().is_empty())
        .unwrap_or("—")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_list_request_trims_status_and_uses_defaults() {
        let request = order_list_request(" paid ");

        assert_eq!(request.status.as_deref(), Some("paid"));
        assert_eq!(request.page, DEFAULT_ORDER_PAGE);
        assert_eq!(request.per_page, DEFAULT_ORDER_PER_PAGE);
    }

    #[test]
    fn blank_text_normalizes_to_none() {
        assert_eq!(text_or_none("  "), None);
    }

    #[test]
    fn mark_paid_command_trims_required_fields() {
        let command = prepare_mark_paid_command(" pay_1 ", " manual ", "required".to_string())
            .expect("valid mark-paid command");

        assert_eq!(command.payment_id, "pay_1");
        assert_eq!(command.payment_method, "manual");
    }

    #[test]
    fn mark_paid_command_rejects_missing_required_fields() {
        let error = prepare_mark_paid_command(" ", "manual", "Payment fields required".to_string())
            .expect_err("blank payment id must fail before transport");

        assert_eq!(error, "Payment fields required");
    }

    #[test]
    fn ship_order_command_trims_required_fields() {
        let command = prepare_ship_order_command(" track_1 ", " dhl ", "required".to_string())
            .expect("valid ship command");

        assert_eq!(command.tracking_number, "track_1");
        assert_eq!(command.carrier, "dhl");
    }

    #[test]
    fn ship_order_command_rejects_missing_required_fields() {
        let error =
            prepare_ship_order_command("track", " ", "Shipping fields required".to_string())
                .expect_err("blank carrier must fail before transport");

        assert_eq!(error, "Shipping fields required");
    }

    #[test]
    fn deliver_order_command_normalizes_optional_signature() {
        let command = prepare_deliver_order_command(" signed by Alex ");
        assert_eq!(
            command.delivered_signature.as_deref(),
            Some("signed by Alex")
        );

        let blank = prepare_deliver_order_command(" ");
        assert_eq!(blank.delivered_signature, None);
    }

    #[test]
    fn cancel_order_command_normalizes_optional_reason() {
        let command = prepare_cancel_order_command(" customer request ");
        assert_eq!(command.reason.as_deref(), Some("customer request"));

        let blank = prepare_cancel_order_command(" ");
        assert_eq!(blank.reason, None);
    }

    #[test]
    fn order_status_badge_maps_lifecycle_states() {
        assert!(order_status_badge("paid").contains("text-blue-700"));
        assert!(order_status_badge("cancelled").contains("text-rose-700"));
        assert!(order_status_badge("pending").contains("text-slate-700"));
    }

    #[test]
    fn text_or_dash_normalizes_blank_optional_display_values() {
        assert_eq!(text_or_dash(Some(" value ")), " value ");
        assert_eq!(text_or_dash(Some("   ")), "—");
        assert_eq!(text_or_dash(None), "—");
    }
}
