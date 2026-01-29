use async_graphql::{InputObject, Json, SimpleObject};
use rust_decimal::Decimal;
use rustok_commerce::dto;
use rustok_commerce::CommerceError;
use std::str::FromStr;
use uuid::Uuid;

#[derive(SimpleObject, Clone)]
pub struct Price {
    pub currency_code: String,
    pub amount: String,
    pub compare_at_amount: Option<String>,
    pub on_sale: bool,
}

impl From<&dto::PriceResponse> for Price {
    fn from(price: &dto::PriceResponse) -> Self {
        Self {
            currency_code: price.currency_code.clone(),
            amount: price.amount.to_string(),
            compare_at_amount: price.compare_at_amount.map(|value| value.to_string()),
            on_sale: price.on_sale,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct Variant {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub title: String,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub prices: Vec<Price>,
    pub inventory_quantity: i32,
    pub inventory_policy: String,
    pub in_stock: bool,
    pub weight: Option<String>,
    pub weight_unit: Option<String>,
    pub position: i32,
}

impl From<&dto::VariantResponse> for Variant {
    fn from(variant: &dto::VariantResponse) -> Self {
        Self {
            id: variant.id,
            product_id: variant.product_id,
            sku: variant.sku.clone(),
            barcode: variant.barcode.clone(),
            title: variant.title.clone(),
            option1: variant.option1.clone(),
            option2: variant.option2.clone(),
            option3: variant.option3.clone(),
            prices: variant.prices.iter().map(Price::from).collect(),
            inventory_quantity: variant.inventory_quantity,
            inventory_policy: variant.inventory_policy.clone(),
            in_stock: variant.in_stock,
            weight: variant.weight.map(|value| value.to_string()),
            weight_unit: variant.weight_unit.clone(),
            position: variant.position,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct ProductTranslation {
    pub locale: String,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

impl From<&dto::ProductTranslationResponse> for ProductTranslation {
    fn from(translation: &dto::ProductTranslationResponse) -> Self {
        Self {
            locale: translation.locale.clone(),
            title: translation.title.clone(),
            handle: translation.handle.clone(),
            description: translation.description.clone(),
            meta_title: translation.meta_title.clone(),
            meta_description: translation.meta_description.clone(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct ProductOption {
    pub id: Uuid,
    pub name: String,
    pub values: Vec<String>,
    pub position: i32,
}

impl From<&dto::ProductOptionResponse> for ProductOption {
    fn from(option: &dto::ProductOptionResponse) -> Self {
        Self {
            id: option.id,
            name: option.name.clone(),
            values: option.values.clone(),
            position: option.position,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct ProductImage {
    pub id: Uuid,
    pub media_id: Uuid,
    pub url: String,
    pub alt_text: Option<String>,
    pub position: i32,
}

impl From<&dto::ProductImageResponse> for ProductImage {
    fn from(image: &dto::ProductImageResponse) -> Self {
        Self {
            id: image.id,
            media_id: image.media_id,
            url: image.url.clone(),
            alt_text: image.alt_text.clone(),
            position: image.position,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub status: String,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub metadata: Json<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: Option<String>,
    pub translations: Vec<ProductTranslation>,
    pub options: Vec<ProductOption>,
    pub variants: Vec<Variant>,
    pub images: Vec<ProductImage>,
}

impl From<dto::ProductResponse> for Product {
    fn from(product: dto::ProductResponse) -> Self {
        Self {
            id: product.id,
            tenant_id: product.tenant_id,
            status: product.status,
            vendor: product.vendor,
            product_type: product.product_type,
            metadata: Json(product.metadata),
            created_at: product.created_at.to_rfc3339(),
            updated_at: product.updated_at.to_rfc3339(),
            published_at: product.published_at.map(|value| value.to_rfc3339()),
            translations: product
                .translations
                .iter()
                .map(ProductTranslation::from)
                .collect(),
            options: product.options.iter().map(ProductOption::from).collect(),
            variants: product.variants.iter().map(Variant::from).collect(),
            images: product.images.iter().map(ProductImage::from).collect(),
        }
    }
}

#[derive(InputObject, Clone)]
pub struct ProductTranslationInput {
    pub locale: String,
    pub title: String,
    pub handle: Option<String>,
    pub description: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

impl From<ProductTranslationInput> for dto::ProductTranslationInput {
    fn from(input: ProductTranslationInput) -> Self {
        Self {
            locale: input.locale,
            title: input.title,
            handle: input.handle,
            description: input.description,
            meta_title: input.meta_title,
            meta_description: input.meta_description,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct ProductOptionInput {
    pub name: String,
    pub values: Vec<String>,
}

impl From<ProductOptionInput> for dto::ProductOptionInput {
    fn from(input: ProductOptionInput) -> Self {
        Self {
            name: input.name,
            values: input.values,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct PriceInput {
    pub currency_code: String,
    pub amount: String,
    pub compare_at_amount: Option<String>,
}

impl TryFrom<PriceInput> for dto::PriceInput {
    type Error = String;

    fn try_from(input: PriceInput) -> Result<Self, Self::Error> {
        let amount = parse_decimal(&input.amount)?;
        let compare_at_amount = match input.compare_at_amount {
            Some(value) => Some(parse_decimal(&value)?),
            None => None,
        };

        Ok(Self {
            currency_code: input.currency_code,
            amount,
            compare_at_amount,
        })
    }
}

#[derive(InputObject, Clone)]
pub struct CreateVariantInput {
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub prices: Vec<PriceInput>,
    #[graphql(default)]
    pub inventory_quantity: i32,
    pub inventory_policy: Option<String>,
    pub weight: Option<String>,
    pub weight_unit: Option<String>,
}

impl TryFrom<CreateVariantInput> for dto::CreateVariantInput {
    type Error = String;

    fn try_from(input: CreateVariantInput) -> Result<Self, Self::Error> {
        let prices = input
            .prices
            .into_iter()
            .map(dto::PriceInput::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let weight = match input.weight {
            Some(value) => Some(parse_decimal(&value)?),
            None => None,
        };

        Ok(Self {
            sku: input.sku,
            barcode: input.barcode,
            option1: input.option1,
            option2: input.option2,
            option3: input.option3,
            prices,
            inventory_quantity: input.inventory_quantity,
            inventory_policy: input
                .inventory_policy
                .unwrap_or_else(|| "deny".to_string()),
            weight,
            weight_unit: input.weight_unit,
        })
    }
}

#[derive(InputObject, Clone)]
pub struct CreateProductInput {
    pub translations: Vec<ProductTranslationInput>,
    #[graphql(default)]
    pub options: Vec<ProductOptionInput>,
    pub variants: Vec<CreateVariantInput>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub metadata: Option<Json<serde_json::Value>>,
    #[graphql(default)]
    pub publish: bool,
}

impl TryFrom<CreateProductInput> for dto::CreateProductInput {
    type Error = String;

    fn try_from(input: CreateProductInput) -> Result<Self, Self::Error> {
        let variants = input
            .variants
            .into_iter()
            .map(dto::CreateVariantInput::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            translations: input
                .translations
                .into_iter()
                .map(dto::ProductTranslationInput::from)
                .collect(),
            options: input
                .options
                .into_iter()
                .map(dto::ProductOptionInput::from)
                .collect(),
            variants,
            vendor: input.vendor,
            product_type: input.product_type,
            metadata: input
                .metadata
                .map(|value| value.0)
                .unwrap_or_else(|| serde_json::json!({})),
            publish: input.publish,
        })
    }
}

#[derive(InputObject, Clone)]
pub struct UpdateProductInput {
    pub translations: Option<Vec<ProductTranslationInput>>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub metadata: Option<Json<serde_json::Value>>,
    pub status: Option<String>,
}

impl From<UpdateProductInput> for dto::UpdateProductInput {
    fn from(input: UpdateProductInput) -> Self {
        Self {
            translations: input
                .translations
                .map(|translations| translations.into_iter().map(Into::into).collect()),
            vendor: input.vendor,
            product_type: input.product_type,
            metadata: input.metadata.map(|value| value.0),
            status: input.status,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct AdjustInventoryInput {
    pub variant_id: Uuid,
    pub adjustment: i32,
    pub reason: Option<String>,
}

impl From<AdjustInventoryInput> for dto::AdjustInventoryInput {
    fn from(input: AdjustInventoryInput) -> Self {
        Self {
            variant_id: input.variant_id,
            adjustment: input.adjustment,
            reason: input.reason,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct SetVariantPricesInput {
    pub variant_id: Uuid,
    pub prices: Vec<PriceInput>,
}

impl TryFrom<SetVariantPricesInput> for (Uuid, Vec<dto::PriceInput>) {
    type Error = String;

    fn try_from(input: SetVariantPricesInput) -> Result<Self, Self::Error> {
        let prices = input
            .prices
            .into_iter()
            .map(dto::PriceInput::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok((input.variant_id, prices))
    }
}

fn parse_decimal(value: &str) -> Result<Decimal, String> {
    Decimal::from_str(value).map_err(|_| format!("Invalid decimal value: {value}"))
}

pub fn map_commerce_error(error: CommerceError) -> async_graphql::FieldError {
    match error {
        CommerceError::ProductNotFound(_) => {
            async_graphql::FieldError::new("Product not found")
        }
        CommerceError::VariantNotFound(_) => {
            async_graphql::FieldError::new("Variant not found")
        }
        CommerceError::DuplicateHandle { .. }
        | CommerceError::DuplicateSku(_)
        | CommerceError::InvalidPrice(_)
        | CommerceError::InsufficientInventory { .. }
        | CommerceError::InvalidOptionCombination
        | CommerceError::Validation(_)
        | CommerceError::NoVariants
        | CommerceError::CannotDeletePublished => {
            async_graphql::FieldError::new(error.to_string())
        }
        CommerceError::Database(err) => {
            async_graphql::FieldError::new(err.to_string())
        }
    }
}
