use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set, TransactionTrait,
};
use serde::Deserialize;
use uuid::Uuid;

use rustok_core::{DomainEvent, Error as RusToKError, EventBus};

use crate::entities::{price, product, product_option, product_translation, product_variant};

#[derive(Debug, Deserialize)]
pub struct CreateProductInput {
    pub status: Option<String>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub translations: Vec<ProductTranslationInput>,
    pub options: Vec<ProductOptionInput>,
    pub variants: Vec<CreateVariantInput>,
}

#[derive(Debug, Deserialize)]
pub struct ProductTranslationInput {
    pub locale: String,
    pub title: String,
    pub handle: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProductOptionInput {
    pub name: String,
    pub values: Vec<String>,
    pub position: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateVariantInput {
    pub sku: Option<String>,
    pub inventory_quantity: i32,
    pub price: Decimal,
    pub currency: String,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
}

pub struct ProductService {
    db: DatabaseConnection,
    event_bus: EventBus,
}

impl ProductService {
    pub fn new(db: DatabaseConnection, event_bus: EventBus) -> Self {
        Self { db, event_bus }
    }

    pub async fn create_product(
        &self,
        tenant_id: Uuid,
        actor_id: Uuid,
        input: CreateProductInput,
    ) -> Result<Uuid, RusToKError> {
        let product_id = rustok_core::generate_id();
        let now = Utc::now().into();

        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| RusToKError::Database(error.to_string()))?;

        product::ActiveModel {
            id: Set(product_id),
            tenant_id: Set(tenant_id),
            status: Set(input.status.unwrap_or_else(|| "draft".to_string())),
            vendor: Set(input.vendor),
            product_type: Set(input.product_type),
            metadata: Set(serde_json::json!({})),
            created_at: Set(now),
            updated_at: Set(now),
            published_at: NotSet,
        }
        .insert(&txn)
        .await
        .map_err(|error| RusToKError::Database(error.to_string()))?;

        for translation in input.translations {
            let handle = translation
                .handle
                .unwrap_or_else(|| slug::slugify(&translation.title));

            product_translation::ActiveModel {
                id: Set(rustok_core::generate_id()),
                product_id: Set(product_id),
                locale: Set(translation.locale),
                title: Set(translation.title),
                handle: Set(handle),
                description: Set(translation.description),
                meta_title: NotSet,
                meta_description: NotSet,
            }
            .insert(&txn)
            .await
            .map_err(|error| RusToKError::Database(error.to_string()))?;
        }

        for option in input.options {
            product_option::ActiveModel {
                id: Set(rustok_core::generate_id()),
                product_id: Set(product_id),
                name: Set(option.name),
                position: Set(option.position),
                values: Set(
                    serde_json::to_value(option.values)
                        .map_err(|error| RusToKError::Unknown(error.to_string()))?,
                ),
            }
            .insert(&txn)
            .await
            .map_err(|error| RusToKError::Database(error.to_string()))?;
        }

        for variant in input.variants {
            let variant_id = rustok_core::generate_id();

            product_variant::ActiveModel {
                id: Set(variant_id),
                product_id: Set(product_id),
                tenant_id: Set(tenant_id),
                sku: Set(variant.sku),
                barcode: NotSet,
                inventory_policy: Set("deny".to_string()),
                inventory_quantity: Set(variant.inventory_quantity),
                weight: NotSet,
                weight_unit: NotSet,
                option1: Set(variant.option1),
                option2: Set(variant.option2),
                option3: Set(variant.option3),
                position: Set(0),
                created_at: Set(now),
                updated_at: Set(now),
            }
            .insert(&txn)
            .await
            .map_err(|error| RusToKError::Database(error.to_string()))?;

            price::ActiveModel {
                id: Set(rustok_core::generate_id()),
                variant_id: Set(variant_id),
                currency_code: Set(variant.currency),
                amount: Set(variant.price),
                compare_at_amount: NotSet,
            }
            .insert(&txn)
            .await
            .map_err(|error| RusToKError::Database(error.to_string()))?;
        }

        txn.commit()
            .await
            .map_err(|error| RusToKError::Database(error.to_string()))?;

        self.event_bus.publish(
            tenant_id,
            Some(actor_id),
            DomainEvent::ProductCreated { product_id },
        )?;

        Ok(product_id)
    }
}
