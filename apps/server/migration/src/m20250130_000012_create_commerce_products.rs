use sea_orm_migration::prelude::*;

use super::m20250101_000001_create_tenants::Tenants;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Products::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Products::TenantId).uuid().not_null())
                    .col(
                        ColumnDef::new(Products::Status)
                            .string_len(32)
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(Products::Vendor).string_len(255))
                    .col(ColumnDef::new(Products::ProductType).string_len(255))
                    .col(
                        ColumnDef::new(Products::Metadata)
                            .json_binary()
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(Products::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Products::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Products::PublishedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Products::Table, Products::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductTranslations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductTranslations::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProductTranslations::ProductId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductTranslations::Locale)
                            .string_len(8)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductTranslations::Title)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductTranslations::Handle)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProductTranslations::Description).text())
                    .col(ColumnDef::new(ProductTranslations::MetaTitle).string_len(255))
                    .col(
                        ColumnDef::new(ProductTranslations::MetaDescription)
                            .string_len(500),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductTranslations::Table, ProductTranslations::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_prod_trans_uniq")
                    .table(ProductTranslations::Table)
                    .col(ProductTranslations::ProductId)
                    .col(ProductTranslations::Locale)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_prod_trans_handle")
                    .table(ProductTranslations::Table)
                    .col(ProductTranslations::Locale)
                    .col(ProductTranslations::Handle)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductOptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductOptions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductOptions::ProductId).uuid().not_null())
                    .col(
                        ColumnDef::new(ProductOptions::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ProductOptions::Name)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductOptions::Values)
                            .json_binary()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductOptions::Table, ProductOptions::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductVariants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductVariants::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductVariants::ProductId).uuid().not_null())
                    .col(ColumnDef::new(ProductVariants::TenantId).uuid().not_null())
                    .col(ColumnDef::new(ProductVariants::Sku).string_len(64))
                    .col(ColumnDef::new(ProductVariants::Barcode).string_len(64))
                    .col(ColumnDef::new(ProductVariants::Ean).string_len(64))
                    .col(ColumnDef::new(ProductVariants::Upc).string_len(64))
                    .col(
                        ColumnDef::new(ProductVariants::InventoryPolicy)
                            .string_len(32)
                            .not_null()
                            .default("deny"),
                    )
                    .col(
                        ColumnDef::new(ProductVariants::InventoryManagement)
                            .string_len(32)
                            .not_null()
                            .default("manual"),
                    )
                    .col(
                        ColumnDef::new(ProductVariants::InventoryQuantity)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(ProductVariants::Weight).decimal())
                    .col(
                        ColumnDef::new(ProductVariants::WeightUnit)
                            .string_len(8)
                            .default("kg"),
                    )
                    .col(ColumnDef::new(ProductVariants::Option1).string_len(100))
                    .col(ColumnDef::new(ProductVariants::Option2).string_len(100))
                    .col(ColumnDef::new(ProductVariants::Option3).string_len(100))
                    .col(
                        ColumnDef::new(ProductVariants::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ProductVariants::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ProductVariants::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductVariants::Table, ProductVariants::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductVariants::Table, ProductVariants::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(VariantTranslations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VariantTranslations::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(VariantTranslations::VariantId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(VariantTranslations::Locale)
                            .string_len(8)
                            .not_null(),
                    )
                    .col(ColumnDef::new(VariantTranslations::Title).string_len(255))
                    .foreign_key(
                        ForeignKey::create()
                            .from(VariantTranslations::Table, VariantTranslations::VariantId)
                            .to(ProductVariants::Table, ProductVariants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Prices::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Prices::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Prices::VariantId).uuid().not_null())
                    .col(
                        ColumnDef::new(Prices::CurrencyCode)
                            .string_len(3)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Prices::Amount).decimal().not_null())
                    .col(ColumnDef::new(Prices::CompareAtAmount).decimal())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Prices::Table, Prices::VariantId)
                            .to(ProductVariants::Table, ProductVariants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_prices_uniq")
                    .table(Prices::Table)
                    .col(Prices::VariantId)
                    .col(Prices::CurrencyCode)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductImages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductImages::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductImages::ProductId).uuid().not_null())
                    .col(ColumnDef::new(ProductImages::MediaId).uuid().not_null())
                    .col(
                        ColumnDef::new(ProductImages::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(ProductImages::AltText).string_len(255))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductImages::Table, ProductImages::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductImages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Prices::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(VariantTranslations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProductVariants::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProductOptions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProductTranslations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
    TenantId,
    Status,
    Vendor,
    ProductType,
    Metadata,
    CreatedAt,
    UpdatedAt,
    PublishedAt,
}

#[derive(Iden)]
enum ProductTranslations {
    Table,
    Id,
    ProductId,
    Locale,
    Title,
    Handle,
    Description,
    MetaTitle,
    MetaDescription,
}

#[derive(Iden)]
enum ProductOptions {
    Table,
    Id,
    ProductId,
    Position,
    Name,
    Values,
}

#[derive(Iden)]
enum ProductVariants {
    Table,
    Id,
    ProductId,
    TenantId,
    Sku,
    Barcode,
    Ean,
    Upc,
    InventoryPolicy,
    InventoryManagement,
    InventoryQuantity,
    Weight,
    WeightUnit,
    Option1,
    Option2,
    Option3,
    Position,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum VariantTranslations {
    Table,
    Id,
    VariantId,
    Locale,
    Title,
}

#[derive(Iden)]
enum Prices {
    Table,
    Id,
    VariantId,
    CurrencyCode,
    Amount,
    CompareAtAmount,
}

#[derive(Iden)]
enum ProductImages {
    Table,
    Id,
    ProductId,
    MediaId,
    Position,
    AltText,
}
