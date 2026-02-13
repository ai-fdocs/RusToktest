use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rustok_content::state_machine::{ContentState, ContentTransition};
use rustok_commerce::state_machine::{OrderState, OrderTransition};
use uuid::Uuid;

fn bench_content_state_transitions(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("content_state_transitions");
    
    // Benchmark: Draft -> Review transition
    group.bench_function("draft_to_review", |b| {
        b.iter(|| {
            let state = ContentState::Draft {
                tenant_id,
                created_by: user_id,
            };
            let transition = ContentTransition::SubmitForReview {
                submitted_by: user_id,
            };
            black_box(state.apply(black_box(transition)))
        })
    });
    
    // Benchmark: Review -> Published transition
    group.bench_function("review_to_published", |b| {
        b.iter(|| {
            let state = ContentState::Review {
                tenant_id,
                submitted_by: user_id,
            };
            let transition = ContentTransition::Publish {
                published_by: user_id,
            };
            black_box(state.apply(black_box(transition)))
        })
    });
    
    // Benchmark: Published -> Archived transition
    group.bench_function("published_to_archived", |b| {
        b.iter(|| {
            let state = ContentState::Published {
                tenant_id,
                published_by: user_id,
                published_at: chrono::Utc::now(),
            };
            let transition = ContentTransition::Archive {
                archived_by: user_id,
            };
            black_box(state.apply(black_box(transition)))
        })
    });
    
    // Benchmark: Invalid transition (should fail fast)
    group.bench_function("invalid_transition", |b| {
        b.iter(|| {
            let state = ContentState::Archived {
                tenant_id,
                archived_by: user_id,
                archived_at: chrono::Utc::now(),
            };
            let transition = ContentTransition::Publish {
                published_by: user_id,
            };
            black_box(state.apply(black_box(transition)))
        })
    });
    
    group.finish();
}

fn bench_order_state_transitions(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    let order_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("order_state_transitions");
    
    // Benchmark: Cart -> Pending transition
    group.bench_function("cart_to_pending", |b| {
        b.iter(|| {
            let state = OrderState::Cart {
                tenant_id,
                customer_id,
            };
            let transition = OrderTransition::Checkout {
                order_id,
                total_cents: 10000,
            };
            black_box(state.apply(black_box(transition)))
        })
    });
    
    // Benchmark: Pending -> Paid transition
    group.bench_function("pending_to_paid", |b| {
        b.iter(|| {
            let state = OrderState::Pending {
                tenant_id,
                customer_id,
                order_id,
                total_cents: 10000,
            };
            let transition = OrderTransition::MarkPaid {
                payment_id: Uuid::new_v4().to_string(),
            };
            black_box(state.apply(black_box(transition)))
        })
    });
    
    // Benchmark: Paid -> Shipped transition
    group.bench_function("paid_to_shipped", |b| {
        b.iter(|| {
            let state = OrderState::Paid {
                tenant_id,
                customer_id,
                order_id,
                total_cents: 10000,
                payment_id: Uuid::new_v4().to_string(),
            };
            let transition = OrderTransition::Ship {
                tracking_number: "TRACK123".to_string(),
            };
            black_box(state.apply(black_box(transition)))
        })
    });
    
    // Benchmark: State validation (idempotency check)
    group.bench_function("idempotency_check", |b| {
        b.iter(|| {
            let state = OrderState::Paid {
                tenant_id,
                customer_id,
                order_id,
                total_cents: 10000,
                payment_id: Uuid::new_v4().to_string(),
            };
            // Trying to pay again should fail
            let transition = OrderTransition::MarkPaid {
                payment_id: Uuid::new_v4().to_string(),
            };
            black_box(state.apply(black_box(transition)))
        })
    });
    
    group.finish();
}

fn bench_state_clone(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("state_clone");
    
    // Benchmark cloning content states of different sizes
    group.bench_function("clone_draft", |b| {
        let state = ContentState::Draft {
            tenant_id,
            created_by: user_id,
        };
        b.iter(|| {
            black_box(state.clone())
        })
    });
    
    group.bench_function("clone_published", |b| {
        let state = ContentState::Published {
            tenant_id,
            published_by: user_id,
            published_at: chrono::Utc::now(),
        };
        b.iter(|| {
            black_box(state.clone())
        })
    });
    
    group.finish();
}

criterion_group!(
    state_machine_benches,
    bench_content_state_transitions,
    bench_order_state_transitions,
    bench_state_clone
);
criterion_main!(state_machine_benches);
