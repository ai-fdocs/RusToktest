use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use rustok_commerce::state_machine::{OrderState, OrderTransition, OrderStateMachine};
use uuid::Uuid;

/// Benchmark order state machine workflows
fn bench_order_workflows(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("order_workflow");
    
    // Complete order flow: Cart -> Pending -> Paid -> Shipped -> Delivered
    group.bench_function("complete_order_flow", |b| {
        b.iter(|| {
            let mut machine = OrderStateMachine::new(tenant_id, customer_id);
            let order_id = Uuid::new_v4();
            
            // Cart -> Pending
            machine.apply(OrderTransition::Checkout {
                order_id,
                total_cents: 10000,
            }).unwrap();
            
            // Pending -> Paid
            machine.apply(OrderTransition::MarkPaid {
                payment_id: Uuid::new_v4().to_string(),
            }).unwrap();
            
            // Paid -> Shipped
            machine.apply(OrderTransition::Ship {
                tracking_number: "TRACK123".to_string(),
            }).unwrap();
            
            // Shipped -> Delivered
            machine.apply(OrderTransition::Deliver).unwrap();
            
            black_box(machine)
        })
    });
    
    // Cancellation flow: Cart -> Pending -> Cancelled
    group.bench_function("cancellation_flow", |b| {
        b.iter(|| {
            let mut machine = OrderStateMachine::new(tenant_id, customer_id);
            let order_id = Uuid::new_v4();
            
            machine.apply(OrderTransition::Checkout {
                order_id,
                total_cents: 10000,
            }).unwrap();
            
            machine.apply(OrderTransition::Cancel {
                reason: "Customer request".to_string(),
            }).unwrap();
            
            black_box(machine)
        })
    });
    
    // Refund flow: Cart -> Pending -> Paid -> Refunded
    group.bench_function("refund_flow", |b| {
        b.iter(|| {
            let mut machine = OrderStateMachine::new(tenant_id, customer_id);
            let order_id = Uuid::new_v4();
            
            machine.apply(OrderTransition::Checkout {
                order_id,
                total_cents: 10000,
            }).unwrap();
            
            machine.apply(OrderTransition::MarkPaid {
                payment_id: Uuid::new_v4().to_string(),
            }).unwrap();
            
            machine.apply(OrderTransition::Refund {
                reason: "Product defective".to_string(),
            }).unwrap();
            
            black_box(machine)
        })
    });
    
    group.finish();
}

/// Benchmark order queries and validations
fn bench_order_queries(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("order_queries");
    
    group.bench_function("is_paid_pending", |b| {
        let mut machine = OrderStateMachine::new(tenant_id, customer_id);
        machine.apply(OrderTransition::Checkout {
            order_id: Uuid::new_v4(),
            total_cents: 10000,
        }).unwrap();
        
        b.iter(|| {
            black_box(machine.is_paid())
        })
    });
    
    group.bench_function("is_paid_paid", |b| {
        let mut machine = OrderStateMachine::new(tenant_id, customer_id);
        machine.apply(OrderTransition::Checkout {
            order_id: Uuid::new_v4(),
            total_cents: 10000,
        }).unwrap();
        machine.apply(OrderTransition::MarkPaid {
            payment_id: Uuid::new_v4().to_string(),
        }).unwrap();
        
        b.iter(|| {
            black_box(machine.is_paid())
        })
    });
    
    group.bench_function("can_cancel_pending", |b| {
        let mut machine = OrderStateMachine::new(tenant_id, customer_id);
        machine.apply(OrderTransition::Checkout {
            order_id: Uuid::new_v4(),
            total_cents: 10000,
        }).unwrap();
        
        b.iter(|| {
            black_box(machine.can_cancel())
        })
    });
    
    group.bench_function("can_cancel_shipped", |b| {
        let mut machine = OrderStateMachine::new(tenant_id, customer_id);
        machine.apply(OrderTransition::Checkout {
            order_id: Uuid::new_v4(),
            total_cents: 10000,
        }).unwrap();
        machine.apply(OrderTransition::MarkPaid {
            payment_id: Uuid::new_v4().to_string(),
        }).unwrap();
        machine.apply(OrderTransition::Ship {
            tracking_number: "TRACK123".to_string(),
        }).unwrap();
        
        b.iter(|| {
            black_box(machine.can_cancel())
        })
    });
    
    group.bench_function("get_total", |b| {
        let mut machine = OrderStateMachine::new(tenant_id, customer_id);
        machine.apply(OrderTransition::Checkout {
            order_id: Uuid::new_v4(),
            total_cents: 9999,
        }).unwrap();
        
        b.iter(|| {
            black_box(machine.total_cents())
        })
    });
    
    group.finish();
}

/// Benchmark high-volume order processing
fn bench_order_throughput(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("order_throughput");
    
    for batch_size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*batch_size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let machines: Vec<_> = (0..size)
                        .map(|i| {
                            let customer_id = Uuid::new_v4();
                            let mut m = OrderStateMachine::new(tenant_id, customer_id);
                            m.apply(OrderTransition::Checkout {
                                order_id: Uuid::new_v4(),
                                total_cents: (i as u64) * 100,
                            }).unwrap();
                            m.apply(OrderTransition::MarkPaid {
                                payment_id: Uuid::new_v4().to_string(),
                            }).unwrap();
                            m
                        })
                        .collect();
                    black_box(machines.len())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark monetary calculations
fn bench_order_monetary(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("order_monetary");
    
    group.bench_function("checkout_with_large_amount", |b| {
        b.iter(|| {
            let mut machine = OrderStateMachine::new(tenant_id, customer_id);
            machine.apply(OrderTransition::Checkout {
                order_id: Uuid::new_v4(),
                total_cents: 1_000_000_000, // $10M
            }).unwrap();
            black_box(machine)
        })
    });
    
    group.bench_function("checkout_with_small_amount", |b| {
        b.iter(|| {
            let mut machine = OrderStateMachine::new(tenant_id, customer_id);
            machine.apply(OrderTransition::Checkout {
                order_id: Uuid::new_v4(),
                total_cents: 1, // $0.01
            }).unwrap();
            black_box(machine)
        })
    });
    
    group.bench_function("checkout_with_zero_amount", |b| {
        b.iter(|| {
            let mut machine = OrderStateMachine::new(tenant_id, customer_id);
            let result = machine.apply(OrderTransition::Checkout {
                order_id: Uuid::new_v4(),
                total_cents: 0,
            });
            black_box(result)
        })
    });
    
    group.finish();
}

/// Benchmark concurrent order operations
fn bench_order_concurrent(c: &mut Criterion) {
    use std::thread;
    
    let tenant_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("order_concurrent");
    
    group.bench_function("parallel_checkout", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let tenant_id = tenant_id;
                    thread::spawn(move || {
                        for i in 0..25 {
                            let customer_id = Uuid::new_v4();
                            let mut machine = OrderStateMachine::new(tenant_id, customer_id);
                            machine.apply(OrderTransition::Checkout {
                                order_id: Uuid::new_v4(),
                                total_cents: (i as u64) * 100,
                            }).unwrap();
                            black_box(machine);
                        }
                    })
                })
                .collect();
            
            for h in handles {
                h.join().unwrap();
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    order_operations_benches,
    bench_order_workflows,
    bench_order_queries,
    bench_order_throughput,
    bench_order_monetary,
    bench_order_concurrent
);
criterion_main!(order_operations_benches);
