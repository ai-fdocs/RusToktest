use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::context::{AuthContext, TenantContext};
use crate::graphql::types::{Tenant, TenantModule, User};
use crate::models::{tenant_modules, users};

#[derive(Default)]
pub struct RootQuery;

#[Object]
impl RootQuery {
    async fn health(&self) -> &str {
        "GraphQL is working!"
    }

    async fn api_version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    async fn current_tenant(&self, ctx: &Context<'_>) -> Result<Tenant> {
        let tenant = ctx.data::<TenantContext>()?;
        Ok(Tenant {
            id: tenant.id,
            name: tenant.name.clone(),
            slug: tenant.slug.clone(),
        })
    }

    async fn enabled_modules(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;
        let tenant = ctx.data::<TenantContext>()?;
        let modules = tenant_modules::Entity::find_enabled(&app_ctx.db, tenant.id)
            .await
            .map_err(|err| err.to_string())?;

        Ok(modules)
    }

    async fn tenant_modules(&self, ctx: &Context<'_>) -> Result<Vec<TenantModule>> {
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;
        let tenant = ctx.data::<TenantContext>()?;
        let modules = tenant_modules::Entity::find()
            .filter(tenant_modules::Column::TenantId.eq(tenant.id))
            .all(&app_ctx.db)
            .await
            .map_err(|err| err.to_string())?;

        Ok(modules
            .into_iter()
            .map(|module| TenantModule {
                module_slug: module.module_slug,
                enabled: module.enabled,
                settings: module.settings.to_string(),
            })
            .collect())
    }

    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let auth = match ctx.data_opt::<AuthContext>() {
            Some(auth) => auth,
            None => return Ok(None),
        };
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;
        let tenant = ctx.data::<TenantContext>()?;

        let user = users::Entity::find()
            .filter(users::Column::Id.eq(auth.user_id))
            .filter(users::Column::TenantId.eq(tenant.id))
            .one(&app_ctx.db)
            .await
            .map_err(|err| err.to_string())?;

        Ok(user.as_ref().map(User::from))
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let auth = ctx.data::<AuthContext>().map_err(|_| "Unauthorized")?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !rustok_core::Rbac::has_permission(&auth.role, &rustok_core::Permission::USERS_LIST) {
            return Err("Permission denied: users:list required".into());
        }

        let users = users::Entity::find()
            .filter(users::Column::TenantId.eq(tenant.id))
            .all(&app_ctx.db)
            .await
            .map_err(|err| err.to_string())?;

        Ok(users.iter().map(User::from).collect())
    }
}
