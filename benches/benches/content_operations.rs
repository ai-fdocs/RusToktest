use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use rustok_content::state_machine::{ContentState, ContentTransition, ContentStateMachine};
use uuid::Uuid;

/// Benchmark content state machine workflow
fn bench_content_workflow(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("content_workflow");
    
    // Full workflow: Draft -> Review -> Published
    group.bench_function("full_publish_workflow", |b| {
        b.iter(|| {
            let mut machine = ContentStateMachine::new(tenant_id, user_id);
            
            // Draft -> Review
            machine.apply(ContentTransition::SubmitForReview {
                submitted_by: user_id,
            }).unwrap();
            
            // Review -> Published
            machine.apply(ContentTransition::Publish {
                published_by: user_id,
            }).unwrap();
            
            black_box(machine)
        })
    });
    
    // Rejection workflow: Draft -> Review -> Draft
    group.bench_function("rejection_workflow", |b| {
        b.iter(|| {
            let mut machine = ContentStateMachine::new(tenant_id, user_id);
            
            // Draft -> Review
            machine.apply(ContentTransition::SubmitForReview {
                submitted_by: user_id,
            }).unwrap();
            
            // Review -> Draft (rejection)
            machine.apply(ContentTransition::Reject {
                rejected_by: user_id,
                reason: "Needs changes".to_string(),
            }).unwrap();
            
            black_box(machine)
        })
    });
    
    // Archive workflow: Draft -> Review -> Published -> Archived
    group.bench_function("archive_workflow", |b| {
        b.iter(|| {
            let mut machine = ContentStateMachine::new(tenant_id, user_id);
            
            machine.apply(ContentTransition::SubmitForReview {
                submitted_by: user_id,
            }).unwrap();
            
            machine.apply(ContentTransition::Publish {
                published_by: user_id,
            }).unwrap();
            
            machine.apply(ContentTransition::Archive {
                archived_by: user_id,
            }).unwrap();
            
            black_box(machine)
        })
    });
    
    group.finish();
}

/// Benchmark state queries
fn bench_content_queries(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("content_queries");
    
    group.bench_function("is_editable_draft", |b| {
        let machine = ContentStateMachine::new(tenant_id, user_id);
        b.iter(|| {
            black_box(machine.can_edit())
        })
    });
    
    group.bench_function("is_editable_published", |b| {
        let mut machine = ContentStateMachine::new(tenant_id, user_id);
        machine.apply(ContentTransition::SubmitForReview {
            submitted_by: user_id,
        }).unwrap();
        machine.apply(ContentTransition::Publish {
            published_by: user_id,
        }).unwrap();
        
        b.iter(|| {
            black_box(machine.can_edit())
        })
    });
    
    group.bench_function("get_history", |b| {
        let mut machine = ContentStateMachine::new(tenant_id, user_id);
        machine.apply(ContentTransition::SubmitForReview {
            submitted_by: user_id,
        }).unwrap();
        machine.apply(ContentTransition::Publish {
            published_by: user_id,
        }).unwrap();
        
        b.iter(|| {
            black_box(machine.history().len())
        })
    });
    
    group.finish();
}

/// Benchmark batch operations
fn bench_content_batch(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("content_batch");
    
    for batch_size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*batch_size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let machines: Vec<_> = (0..size)
                        .map(|_| {
                            let mut m = ContentStateMachine::new(tenant_id, user_id);
                            m.apply(ContentTransition::SubmitForReview {
                                submitted_by: user_id,
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

/// Benchmark state serialization
fn bench_content_serialization(c: &mut Criterion) {
    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    let mut group = c.benchmark_group("content_serialization");
    
    group.bench_function("serialize_draft", |b| {
        let machine = ContentStateMachine::new(tenant_id, user_id);
        b.iter(|| {
            let json = serde_json::to_string(&machine).unwrap();
            black_box(json.len())
        })
    });
    
    group.bench_function("serialize_with_history", |b| {
        let mut machine = ContentStateMachine::new(tenant_id, user_id);
        machine.apply(ContentTransition::SubmitForReview {
            submitted_by: user_id,
        }).unwrap();
        machine.apply(ContentTransition::Publish {
            published_by: user_id,
        }).unwrap();
        
        b.iter(|| {
            let json = serde_json::to_string(&machine).unwrap();
            black_box(json.len())
        })
    });
    
    group.bench_function("deserialize", |b| {
        let mut machine = ContentStateMachine::new(tenant_id, user_id);
        machine.apply(ContentTransition::SubmitForReview {
            submitted_by: user_id,
        }).unwrap();
        let json = serde_json::to_string(&machine).unwrap();
        
        b.iter(|| {
            let deserialized: ContentStateMachine = serde_json::from_str(&json).unwrap();
            black_box(deserialized)
        })
    });
    
    group.finish();
}

criterion_group!(
    content_operations_benches,
    bench_content_workflow,
    bench_content_queries,
    bench_content_batch,
    bench_content_serialization
);
criterion_main!(content_operations_benches);
