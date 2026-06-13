use leptos::prelude::*;

use crate::core::{
    build_order_checkout_result_view_model, OrderCheckoutResultData, OrderCheckoutResultLabels,
};

#[component]
pub fn OrderCheckoutResultCard(
    result: OrderCheckoutResultData,
    labels: OrderCheckoutResultLabels,
) -> impl IntoView {
    let view_model = build_order_checkout_result_view_model(result, &labels);

    view! {
        <article class="mt-6 rounded-2xl border border-primary/30 bg-primary/5 p-5">
            <div class="text-xs font-medium uppercase tracking-[0.18em] text-primary">
                {labels.badge}
            </div>
            <h4 class="mt-2 text-base font-semibold text-card-foreground">{view_model.order_id}</h4>
            <p class="mt-2 text-sm text-muted-foreground">
                {view_model.module_ownership}
            </p>
            <div class="mt-4 grid gap-3 md:grid-cols-2">
                <article class="rounded-2xl border border-border bg-card p-4">
                    <div class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                        {view_model.order_status_label}
                    </div>
                    <div class="mt-2 text-lg font-semibold text-card-foreground break-all">
                        {view_model.order_status}
                    </div>
                </article>
            </div>
        </article>
    }
}
