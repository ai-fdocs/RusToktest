use async_graphql::{Context, FieldError, Object, Result};

use crate::context::{AuthContext, TenantContext};
use crate::graphql::commerce::{
    map_commerce_error, AdjustInventoryInput, CreateProductInput, Product as CommerceProduct,
    SetVariantPricesInput, UpdateProductInput,
};
use crate::graphql::errors::GraphQLError;
use crate::graphql::types::TenantModule;
use crate::models::_entities::tenant_modules::Entity as TenantModulesEntity;
use rustok_commerce::{CatalogService, InventoryService, PricingService};
use rustok_core::{Action, EventBus, ModuleContext, ModuleRegistry, Permission, Rbac, Resource};

#[derive(Default)]
pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn toggle_module(
        &self,
        ctx: &Context<'_>,
        module_slug: String,
        enabled: bool,
    ) -> Result<TenantModule> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;

        if !matches!(
            auth.role,
            rustok_core::UserRole::Admin | rustok_core::UserRole::SuperAdmin
        ) {
            return Err(<FieldError as GraphQLError>::permission_denied("Forbidden"));
        }

        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;
        let tenant = ctx.data::<TenantContext>()?;
        let registry = ctx.data::<ModuleRegistry>()?;

        if !registry.contains(&module_slug) {
            return Err(FieldError::new("Unknown module"));
        }
        let module = TenantModulesEntity::toggle(&app_ctx.db, tenant.id, &module_slug, enabled)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;

        if let Some(module_impl) = registry.get(&module_slug) {
            let module_ctx = ModuleContext {
                db: &app_ctx.db,
                tenant_id: tenant.id,
                config: &module.settings,
            };

            let hook_result = if enabled {
                module_impl.on_enable(module_ctx).await
            } else {
                module_impl.on_disable(module_ctx).await
            };

            if let Err(err) = hook_result {
                tracing::error!(
                    "Module hook failed for {} (enabled={}): {}",
                    module_slug,
                    enabled,
                    err
                );
            }
        }

        Ok(TenantModule {
            module_slug: module.module_slug,
            enabled: module.enabled,
            settings: module.settings.to_string(),
        })
    }

    async fn create_product(
        &self,
        ctx: &Context<'_>,
        input: CreateProductInput,
    ) -> Result<CommerceProduct> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !Rbac::has_permission(&auth.role, &Permission::PRODUCTS_CREATE) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: products:create required",
            ));
        }

        let input = input
            .try_into()
            .map_err(|err: String| FieldError::new(err))?;

        let service = CatalogService::new(app_ctx.db.clone(), EventBus::default());
        let product = service
            .create_product(tenant.id, auth.user_id, input)
            .await
            .map_err(map_commerce_error)?;

        Ok(product.into())
    }

    async fn update_product(
        &self,
        ctx: &Context<'_>,
        product_id: uuid::Uuid,
        input: UpdateProductInput,
    ) -> Result<CommerceProduct> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !Rbac::has_permission(&auth.role, &Permission::PRODUCTS_UPDATE) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: products:update required",
            ));
        }

        let input = input.into();
        let service = CatalogService::new(app_ctx.db.clone(), EventBus::default());
        let product = service
            .update_product(tenant.id, auth.user_id, product_id, input)
            .await
            .map_err(map_commerce_error)?;

        Ok(product.into())
    }

    async fn publish_product(
        &self,
        ctx: &Context<'_>,
        product_id: uuid::Uuid,
    ) -> Result<CommerceProduct> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !Rbac::has_permission(&auth.role, &Permission::PRODUCTS_UPDATE) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: products:update required",
            ));
        }

        let service = CatalogService::new(app_ctx.db.clone(), EventBus::default());
        let product = service
            .publish_product(tenant.id, auth.user_id, product_id)
            .await
            .map_err(map_commerce_error)?;

        Ok(product.into())
    }

    async fn unpublish_product(
        &self,
        ctx: &Context<'_>,
        product_id: uuid::Uuid,
    ) -> Result<CommerceProduct> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !Rbac::has_permission(&auth.role, &Permission::PRODUCTS_UPDATE) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: products:update required",
            ));
        }

        let service = CatalogService::new(app_ctx.db.clone(), EventBus::default());
        let product = service
            .unpublish_product(tenant.id, auth.user_id, product_id)
            .await
            .map_err(map_commerce_error)?;

        Ok(product.into())
    }

    async fn delete_product(&self, ctx: &Context<'_>, product_id: uuid::Uuid) -> Result<bool> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !Rbac::has_permission(&auth.role, &Permission::PRODUCTS_DELETE) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: products:delete required",
            ));
        }

        let service = CatalogService::new(app_ctx.db.clone(), EventBus::default());
        service
            .delete_product(tenant.id, auth.user_id, product_id)
            .await
            .map_err(map_commerce_error)?;

        Ok(true)
    }

    async fn adjust_inventory(
        &self,
        ctx: &Context<'_>,
        input: AdjustInventoryInput,
    ) -> Result<i32> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !Rbac::has_permission(
            &auth.role,
            &Permission::new(Resource::Inventory, Action::Update),
        ) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: inventory:update required",
            ));
        }

        let input = input.into();
        let service = InventoryService::new(app_ctx.db.clone(), EventBus::default());
        let quantity = service
            .adjust_inventory(tenant.id, auth.user_id, input)
            .await
            .map_err(map_commerce_error)?;

        Ok(quantity)
    }

    async fn set_variant_prices(
        &self,
        ctx: &Context<'_>,
        input: SetVariantPricesInput,
    ) -> Result<bool> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !Rbac::has_permission(&auth.role, &Permission::PRODUCTS_UPDATE) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: products:update required",
            ));
        }

        let (variant_id, prices) = input
            .try_into()
            .map_err(|err: String| FieldError::new(err))?;

        let service = PricingService::new(app_ctx.db.clone(), EventBus::default());
        service
            .set_prices(tenant.id, auth.user_id, variant_id, prices)
            .await
            .map_err(map_commerce_error)?;

        Ok(true)
    }
}
