use uuid::Uuid;

use rustok_core::{DomainEvent, EventBus};

use crate::entities::Product;
use crate::error::{CommerceError, Result};

#[derive(Debug, Clone)]
pub struct CreateProductInput {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProductService {
    event_bus: EventBus,
}

impl ProductService {
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }

    pub fn create_product(
        &self,
        tenant_id: Uuid,
        actor_id: Option<Uuid>,
        input: CreateProductInput,
    ) -> Result<Product> {
        let title = input.title.trim();
        if title.is_empty() {
            return Err(CommerceError::Validation("Title is required".to_string()));
        }

        let slug = input.slug.trim();
        if slug.is_empty() {
            return Err(CommerceError::Validation("Slug is required".to_string()));
        }

        let product = Product::new(
            tenant_id,
            title.to_string(),
            slug.to_string(),
            input.description,
        );

        self.event_bus.publish(
            tenant_id,
            actor_id,
            DomainEvent::ProductCreated {
                product_id: product.id,
            },
        )?;

        Ok(product)
    }

    pub fn publish_product(
        &self,
        tenant_id: Uuid,
        actor_id: Option<Uuid>,
        product: &mut Product,
    ) -> Result<()> {
        if product.tenant_id != tenant_id {
            return Err(CommerceError::InvalidState(
                "Tenant mismatch for product".to_string(),
            ));
        }

        product.publish()?;

        self.event_bus.publish(
            tenant_id,
            actor_id,
            DomainEvent::ProductPublished {
                product_id: product.id,
            },
        )?;

        Ok(())
    }
}
