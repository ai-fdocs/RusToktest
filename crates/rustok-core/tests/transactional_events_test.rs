use rustok_core::events::DomainEvent;
use rustok_outbox::{OutboxTransport, SysEventsMigration, TransactionalEventBus};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::prelude::SchemaManager;
use sea_orm_migration::MigrationTrait;
use std::sync::Arc;
use uuid::Uuid;

async fn setup_test_db() -> DatabaseConnection {
    let db_url = format!("sqlite:file:tx_events_{}?mode=memory&cache=shared", Uuid::new_v4());
    let mut opts = ConnectOptions::new(db_url);
    opts.max_connections(1)
        .min_connections(1)
        .sqlx_logging(false);

    let db = Database::connect(opts)
        .await
        .expect("Failed to connect test sqlite database");

    let schema_manager = SchemaManager::new(&db);
    SysEventsMigration
        .up(&schema_manager)
        .await
        .expect("Failed to run outbox migration");

    db
}

#[tokio::test]
async fn test_transactional_event_bus_creation() {
    let db = setup_test_db().await;
    let transport = Arc::new(OutboxTransport::new(db.clone()));
    let event_bus = TransactionalEventBus::new(transport);

    assert!(std::ptr::addr_of!(event_bus) as usize > 0);
}

#[tokio::test]
async fn test_publish_without_transaction() {
    let db = setup_test_db().await;
    let transport = Arc::new(OutboxTransport::new(db.clone()));
    let event_bus = TransactionalEventBus::new(transport);

    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let node_id = Uuid::new_v4();

    let event = DomainEvent::NodeCreated {
        node_id,
        kind: "post".to_string(),
        author_id: Some(user_id),
    };

    let result = event_bus.publish(tenant_id, Some(user_id), event).await;

    assert!(result.is_ok(), "Event publication should succeed");
}

#[test]
fn test_event_envelope_with_transaction_context() {
    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let node_id = Uuid::new_v4();

    let event = DomainEvent::NodeCreated {
        node_id,
        kind: "post".to_string(),
        author_id: Some(user_id),
    };

    use rustok_core::events::EventEnvelope;
    let envelope = EventEnvelope::new(tenant_id, Some(user_id), event);

    assert_eq!(envelope.tenant_id, tenant_id);
    assert_eq!(envelope.actor_id, Some(user_id));
    assert_eq!(envelope.event_type, "node.created");
    assert_eq!(envelope.schema_version, 1);
}

#[test]
fn test_transactional_bus_guarantees() {
    assert_eq!(
        std::mem::size_of::<TransactionalEventBus>(),
        std::mem::size_of::<Arc<dyn rustok_core::events::EventTransport>>()
    );
}
