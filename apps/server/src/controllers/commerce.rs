use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json,
};
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::extractors::{auth::CurrentUser, tenant::CurrentTenant};
use rustok_commerce::dto::{
    AdjustInventoryInput, CreateProductInput, PriceInput, UpdateProductInput,
};
use rustok_commerce::{CatalogService, CommerceError, InventoryService, PricingService};
use rustok_core::{Action, EventBus, Permission, Rbac, Resource};

#[derive(Debug, Deserialize)]
pub struct SetInventoryRequest {
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct SetVariantPricesRequest {
    pub prices: Vec<PriceInput>,
}

fn ensure_permission(user: &CurrentUser, permission: Permission) -> Result<()> {
    if !Rbac::has_permission(&user.user.role, &permission) {
        return Err(Error::Unauthorized("Permission denied".into()));
    }
    Ok(())
}

fn map_commerce_error(error: CommerceError) -> Error {
    match error {
        CommerceError::ProductNotFound(_) | CommerceError::VariantNotFound(_) => {
            Error::BadRequest(error.to_string())
        }
        CommerceError::DuplicateHandle { .. }
        | CommerceError::DuplicateSku(_)
        | CommerceError::InvalidPrice(_)
        | CommerceError::InsufficientInventory { .. }
        | CommerceError::InvalidOptionCombination
        | CommerceError::Validation(_)
        | CommerceError::NoVariants
        | CommerceError::CannotDeletePublished => Error::BadRequest(error.to_string()),
        CommerceError::Database(err) => Error::BadRequest(err.to_string()),
    }
}

async fn create_product(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Json(input): Json<CreateProductInput>,
) -> Result<Response> {
    ensure_permission(&current_user, Permission::PRODUCTS_CREATE)?;
    let service = CatalogService::new(ctx.db.clone(), EventBus::default());
    let product = service
        .create_product(tenant.id, current_user.user.id, input)
        .await
        .map_err(map_commerce_error)?;
    format::json(product)
}

async fn get_product(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Path(product_id): Path<uuid::Uuid>,
) -> Result<Response> {
    ensure_permission(&current_user, Permission::PRODUCTS_READ)?;
    let service = CatalogService::new(ctx.db.clone(), EventBus::default());
    let product = service
        .get_product(tenant.id, product_id)
        .await
        .map_err(map_commerce_error)?;
    format::json(product)
}

async fn update_product(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Path(product_id): Path<uuid::Uuid>,
    Json(input): Json<UpdateProductInput>,
) -> Result<Response> {
    ensure_permission(&current_user, Permission::PRODUCTS_UPDATE)?;
    let service = CatalogService::new(ctx.db.clone(), EventBus::default());
    let product = service
        .update_product(tenant.id, current_user.user.id, product_id, input)
        .await
        .map_err(map_commerce_error)?;
    format::json(product)
}

async fn publish_product(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Path(product_id): Path<uuid::Uuid>,
) -> Result<Response> {
    ensure_permission(&current_user, Permission::PRODUCTS_UPDATE)?;
    let service = CatalogService::new(ctx.db.clone(), EventBus::default());
    let product = service
        .publish_product(tenant.id, current_user.user.id, product_id)
        .await
        .map_err(map_commerce_error)?;
    format::json(product)
}

async fn unpublish_product(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Path(product_id): Path<uuid::Uuid>,
) -> Result<Response> {
    ensure_permission(&current_user, Permission::PRODUCTS_UPDATE)?;
    let service = CatalogService::new(ctx.db.clone(), EventBus::default());
    let product = service
        .unpublish_product(tenant.id, current_user.user.id, product_id)
        .await
        .map_err(map_commerce_error)?;
    format::json(product)
}

async fn delete_product(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Path(product_id): Path<uuid::Uuid>,
) -> Result<Response> {
    ensure_permission(&current_user, Permission::PRODUCTS_DELETE)?;
    let service = CatalogService::new(ctx.db.clone(), EventBus::default());
    service
        .delete_product(tenant.id, current_user.user.id, product_id)
        .await
        .map_err(map_commerce_error)?;
    format::json(serde_json::json!({ "status": "deleted" }))
}

async fn adjust_inventory(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Json(input): Json<AdjustInventoryInput>,
) -> Result<Response> {
    ensure_permission(
        &current_user,
        Permission::new(Resource::Inventory, Action::Update),
    )?;
    let service = InventoryService::new(ctx.db.clone(), EventBus::default());
    let quantity = service
        .adjust_inventory(tenant.id, current_user.user.id, input)
        .await
        .map_err(map_commerce_error)?;
    format::json(serde_json::json!({ "inventory_quantity": quantity }))
}

async fn set_inventory(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Path(variant_id): Path<uuid::Uuid>,
    Json(input): Json<SetInventoryRequest>,
) -> Result<Response> {
    ensure_permission(
        &current_user,
        Permission::new(Resource::Inventory, Action::Update),
    )?;
    let service = InventoryService::new(ctx.db.clone(), EventBus::default());
    let quantity = service
        .set_inventory(tenant.id, current_user.user.id, variant_id, input.quantity)
        .await
        .map_err(map_commerce_error)?;
    format::json(serde_json::json!({ "inventory_quantity": quantity }))
}

async fn set_variant_prices(
    State(ctx): State<AppContext>,
    CurrentTenant(tenant): CurrentTenant,
    CurrentUser(current_user): CurrentUser,
    Path(variant_id): Path<uuid::Uuid>,
    Json(input): Json<SetVariantPricesRequest>,
) -> Result<Response> {
    ensure_permission(&current_user, Permission::PRODUCTS_UPDATE)?;
    let service = PricingService::new(ctx.db.clone(), EventBus::default());
    service
        .set_prices(tenant.id, current_user.user.id, variant_id, input.prices)
        .await
        .map_err(map_commerce_error)?;
    format::json(serde_json::json!({ "status": "updated" }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/commerce")
        .add("/products", post(create_product))
        .add("/products/:id", get(get_product).put(update_product).delete(delete_product))
        .add("/products/:id/publish", post(publish_product))
        .add("/products/:id/unpublish", post(unpublish_product))
        .add("/inventory/adjust", post(adjust_inventory))
        .add("/variants/:id/inventory", post(set_inventory))
        .add("/variants/:id/prices", post(set_variant_prices))
}
