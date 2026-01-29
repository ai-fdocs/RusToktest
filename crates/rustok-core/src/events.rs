use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::generate_id;

const DEFAULT_EVENT_BUS_CAPACITY: usize = 256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub event: DomainEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DomainEvent {
    // ============ Module Events ============
    ModuleEnabled {
        tenant_id: Uuid,
        module_slug: String,
    },
    ModuleDisabled {
        tenant_id: Uuid,
        module_slug: String,
    },

    // ============ Content Events ============
    NodeCreated {
        node_id: Uuid,
        kind: String,
        author_id: Option<Uuid>,
    },
    NodeUpdated {
        node_id: Uuid,
    },
    NodePublished {
        node_id: Uuid,
        kind: String,
    },
    NodeDeleted {
        node_id: Uuid,
        kind: String,
    },

    // ============ Commerce Events ============
    ProductCreated {
        product_id: Uuid,
    },
    ProductUpdated {
        product_id: Uuid,
    },
    ProductPublished {
        product_id: Uuid,
    },
    ProductDeleted {
        product_id: Uuid,
    },

    VariantCreated {
        variant_id: Uuid,
        product_id: Uuid,
    },
    VariantUpdated {
        variant_id: Uuid,
        product_id: Uuid,
    },

    InventoryUpdated {
        variant_id: Uuid,
        location_id: Uuid,
        old_quantity: i32,
        new_quantity: i32,
    },
    InventoryLow {
        variant_id: Uuid,
        product_id: Uuid,
        remaining: i32,
        threshold: i32,
    },

    OrderPlaced {
        order_id: Uuid,
        customer_id: Option<Uuid>,
        total: i64,
    },
    OrderStatusChanged {
        order_id: Uuid,
        old_status: String,
        new_status: String,
    },
    OrderCompleted {
        order_id: Uuid,
    },
    OrderCancelled {
        order_id: Uuid,
        reason: Option<String>,
    },

    // ============ User Events ============
    UserRegistered {
        user_id: Uuid,
        email: String,
    },
    UserLoggedIn {
        user_id: Uuid,
    },

    // ============ Tag Events ============
    TagAttached {
        tag_id: Uuid,
        target_type: String,
        target_id: Uuid,
    },
    TagDetached {
        tag_id: Uuid,
        target_type: String,
        target_id: Uuid,
    },

    // ============ Index Events ============
    ReindexRequested {
        target_type: String,
        target_id: Option<Uuid>,
    },
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    fn handles(&self, event: &DomainEvent) -> bool;

    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    async fn handle(&self, envelope: &EventEnvelope) -> crate::Result<()>;
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<EventEnvelope>,
    capacity: usize,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender, capacity }
    }

    pub fn with_default_capacity() -> Self {
        Self::new(DEFAULT_EVENT_BUS_CAPACITY)
    }

    pub fn publish(&self, tenant_id: Uuid, event: DomainEvent) -> crate::Result<()> {
        let envelope = EventEnvelope {
            id: generate_id(),
            tenant_id,
            occurred_at: Utc::now(),
            event,
        };

        if self.sender.receiver_count() == 0 {
            tracing::debug!(?envelope.event, "No event subscribers");
        }

        if let Err(error) = self.sender.send(envelope) {
            tracing::debug!(?error, "Event publish failed");
        }

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.sender.subscribe()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

pub struct EventDispatcher {
    bus: EventBus,
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl EventDispatcher {
    pub fn new(bus: EventBus) -> Self {
        Self {
            bus,
            handlers: Vec::new(),
        }
    }

    pub fn register(&mut self, handler: Arc<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub fn start(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut receiver = self.bus.subscribe();

            while let Ok(envelope) = receiver.recv().await {
                for handler in &self.handlers {
                    if handler.handles(&envelope.event) {
                        if let Err(error) = handler.handle(&envelope).await {
                            tracing::error!(
                                handler = handler.name(),
                                ?error,
                                ?envelope.event,
                                "Event handler error"
                            );
                        }
                    }
                }
            }
        })
    }
}
