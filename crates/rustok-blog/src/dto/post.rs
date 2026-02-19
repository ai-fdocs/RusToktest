//! DTOs for Blog Post operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::state_machine::BlogPostStatus;

/// Input for creating a new blog post
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePostInput {
    /// Locale for the post (e.g., "en", "ru")
    pub locale: String,

    /// Post title
    #[schema(max_length = 512)]
    pub title: String,

    /// Post body content (markdown or HTML)
    pub body: String,

    /// Optional excerpt/summary
    #[schema(max_length = 1000)]
    pub excerpt: Option<String>,

    /// Optional URL slug (auto-generated if not provided)
    #[schema(max_length = 255)]
    pub slug: Option<String>,

    /// Whether to publish immediately
    pub publish: bool,

    /// Tags for the post
    #[schema(max_items = 20)]
    pub tags: Vec<String>,

    /// Optional category ID
    pub category_id: Option<Uuid>,

    /// Optional metadata (custom fields)
    pub metadata: Option<Value>,
}

/// Input for updating an existing blog post
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePostInput {
    /// Locale for the update (required for translation updates)
    pub locale: Option<String>,

    /// Updated title
    #[schema(max_length = 512)]
    pub title: Option<String>,

    /// Updated body content
    pub body: Option<String>,

    /// Updated excerpt
    #[schema(max_length = 1000)]
    pub excerpt: Option<String>,

    /// Updated slug
    #[schema(max_length = 255)]
    pub slug: Option<String>,

    /// Updated tags
    #[schema(max_items = 20)]
    pub tags: Option<Vec<String>>,

    /// Updated category
    pub category_id: Option<Uuid>,

    /// Updated metadata
    pub metadata: Option<Value>,

    /// Version for optimistic concurrency
    pub version: Option<i32>,
}

impl Default for UpdatePostInput {
    fn default() -> Self {
        Self {
            locale: None,
            title: None,
            body: None,
            excerpt: None,
            slug: None,
            tags: None,
            category_id: None,
            metadata: None,
            version: None,
        }
    }
}

/// Full blog post response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostResponse {
    /// Post ID
    pub id: Uuid,

    /// Tenant ID
    pub tenant_id: Uuid,

    /// Author ID
    pub author_id: Uuid,

    /// Post title
    pub title: String,

    /// URL slug
    pub slug: String,

    /// Locale
    pub locale: String,

    /// Post body
    pub body: String,

    /// Excerpt/summary
    pub excerpt: Option<String>,

    /// Post status
    pub status: BlogPostStatus,

    /// Category ID
    pub category_id: Option<Uuid>,

    /// Category name (if available)
    pub category_name: Option<String>,

    /// Tags
    pub tags: Vec<String>,

    /// Custom metadata
    pub metadata: Value,

    /// Comment count
    pub comment_count: i64,

    /// View count
    pub view_count: i64,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Publication timestamp (if published)
    pub published_at: Option<DateTime<Utc>>,

    /// Version for optimistic concurrency
    pub version: i32,
}

/// Summary of a blog post (for listings)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostSummary {
    /// Post ID
    pub id: Uuid,

    /// Post title
    pub title: String,

    /// URL slug
    pub slug: String,

    /// Locale
    pub locale: String,

    /// Excerpt/summary
    pub excerpt: Option<String>,

    /// Post status
    pub status: BlogPostStatus,

    /// Author ID
    pub author_id: Uuid,

    /// Author name (if available)
    pub author_name: Option<String>,

    /// Category name
    pub category_name: Option<String>,

    /// Tags
    pub tags: Vec<String>,

    /// Comment count
    pub comment_count: i64,

    /// Publication timestamp
    pub published_at: Option<DateTime<Utc>>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Query parameters for listing posts
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct PostListQuery {
    /// Filter by status
    pub status: Option<BlogPostStatus>,

    /// Filter by category
    pub category_id: Option<Uuid>,

    /// Filter by tag
    pub tag: Option<String>,

    /// Filter by author
    pub author_id: Option<Uuid>,

    /// Search query
    pub search: Option<String>,

    /// Locale filter
    pub locale: Option<String>,

    /// Page number (1-based)
    #[schema(default = 1)]
    pub page: Option<u32>,

    /// Page size
    #[schema(default = 20, maximum = 100)]
    pub per_page: Option<u32>,

    /// Sort field
    #[schema(default = "created_at")]
    pub sort_by: Option<String>,

    /// Sort direction
    #[schema(default = "desc")]
    pub sort_order: Option<String>,
}

impl PostListQuery {
    /// Get page number (default: 1)
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    /// Get per_page (default: 20, max: 100)
    pub fn per_page(&self) -> u32 {
        self.per_page.unwrap_or(20).min(100).max(1)
    }

    /// Get offset for pagination
    pub fn offset(&self) -> u64 {
        (self.page() - 1) as u64 * self.per_page() as u64
    }
}

/// Paginated list of posts
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostListResponse {
    /// Posts
    pub items: Vec<PostSummary>,

    /// Total count
    pub total: u64,

    /// Current page
    pub page: u32,

    /// Items per page
    pub per_page: u32,

    /// Total pages
    pub total_pages: u32,
}

impl PostListResponse {
    /// Create a new paginated response
    pub fn new(items: Vec<PostSummary>, total: u64, query: &PostListQuery) -> Self {
        let per_page = query.per_page();
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;

        Self {
            items,
            total,
            page: query.page(),
            per_page,
            total_pages,
        }
    }
}
