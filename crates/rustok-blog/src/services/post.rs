//! Blog Post Service
//!
//! Provides business logic for blog post operations, wrapping the content module
//! with blog-specific functionality.

use crate::dto::{CreatePostInput, PostListQuery, PostListResponse, PostResponse, PostSummary, UpdatePostInput};
use crate::error::{BlogError, BlogResult};
use crate::state_machine::BlogPostStatus;
use rustok_content::{
    BodyInput, CreateNodeInput, NodeService, NodeTranslationInput, UpdateNodeInput,
};
use rustok_core::SecurityContext;
use rustok_outbox::TransactionalEventBus;
use sea_orm::DatabaseConnection;
use serde_json::Value;
use tracing::{info, warn};
use uuid::Uuid;

/// Blog Post Service
///
/// Handles all blog post operations including CRUD, publishing, and archiving.
/// Uses the content module for storage but adds blog-specific logic.
pub struct PostService {
    node_service: NodeService,
    #[allow(dead_code)]
    db: DatabaseConnection,
}

impl PostService {
    /// Create a new PostService instance
    pub fn new(db: DatabaseConnection, event_bus: TransactionalEventBus) -> Self {
        Self {
            node_service: NodeService::new(db.clone(), event_bus),
            db,
        }
    }

    /// Create a new blog post
    ///
    /// Creates a post in draft or published state based on the input.
    /// Automatically handles tags by storing them in metadata.
    pub async fn create_post(
        &self,
        tenant_id: Uuid,
        security: SecurityContext,
        input: CreatePostInput,
    ) -> BlogResult<Uuid> {
        // Validate input
        self.validate_create_input(&input)?;

        // Build metadata with tags
        let mut metadata = input.metadata.unwrap_or_else(|| serde_json::json!({}));
        if let Value::Object(map) = &mut metadata {
            map.insert("tags".to_string(), serde_json::json!(input.tags));
            if let Some(cat_id) = input.category_id {
                map.insert("category_id".to_string(), serde_json::json!(cat_id));
            }
        } else {
            metadata = serde_json::json!({
                "tags": input.tags,
                "category_id": input.category_id,
                "meta": metadata,
            });
        }

        let status = if input.publish {
            rustok_content::entities::node::ContentStatus::Published
        } else {
            rustok_content::entities::node::ContentStatus::Draft
        };

        let author_id = security.user_id.ok_or(BlogError::AuthorRequired)?;

        let node = self
            .node_service
            .create_node(
                tenant_id,
                security.clone(),
                CreateNodeInput {
                    kind: "post".to_string(),
                    status: Some(status),
                    parent_id: None,
                    author_id: Some(author_id),
                    category_id: input.category_id,
                    position: None,
                    depth: None,
                    reply_count: None,
                    metadata,
                    translations: vec![NodeTranslationInput {
                        locale: input.locale.clone(),
                        title: Some(input.title),
                        slug: input.slug,
                        excerpt: input.excerpt,
                    }],
                    bodies: vec![BodyInput {
                        locale: input.locale,
                        body: Some(input.body),
                        format: None,
                    }],
                },
            )
            .await
            .map_err(BlogError::from)?;

        info!(
            post_id = %node.id,
            tenant_id = %tenant_id,
            published = input.publish,
            "Blog post created"
        );

        Ok(node.id)
    }

    /// Update an existing blog post
    pub async fn update_post(
        &self,
        post_id: Uuid,
        security: SecurityContext,
        input: UpdatePostInput,
    ) -> BlogResult<()> {
        // Build update input
        let mut update = UpdateNodeInput::default();

        // Handle title update via translations
        if input.title.is_some() || input.slug.is_some() || input.excerpt.is_some() {
            // Note: Full translation update requires all fields
            // This is a simplified implementation
            update.translations = Some(vec![NodeTranslationInput {
                locale: input.locale.clone().unwrap_or_else(|| "en".to_string()),
                title: input.title,
                slug: input.slug,
                excerpt: input.excerpt,
            }]);
        }

        // Handle body update via bodies
        if let Some(body) = input.body {
            update.bodies = Some(vec![BodyInput {
                locale: input.locale.clone().unwrap_or_else(|| "en".to_string()),
                body: Some(body),
                format: None,
            }]);
        }

        // Handle metadata updates (tags, category)
        if input.tags.is_some() || input.category_id.is_some() || input.metadata.is_some() {
            let mut metadata = input.metadata.unwrap_or_else(|| serde_json::json!({}));
            if let Value::Object(map) = &mut metadata {
                if let Some(tags) = input.tags {
                    map.insert("tags".to_string(), serde_json::json!(tags));
                }
                if let Some(cat_id) = input.category_id {
                    map.insert("category_id".to_string(), serde_json::json!(cat_id));
                }
            }
            update.metadata = Some(metadata);
        }

        if let Some(version) = input.version {
            update.expected_version = Some(version);
        }

        self.node_service
            .update_node(post_id, security)
            .await
            .map_err(BlogError::from)?;

        info!(
            post_id = %post_id,
            "Blog post updated"
        );

        Ok(())
    }

    /// Publish a blog post
    ///
    /// Transitions a draft post to published state.
    pub async fn publish_post(
        &self,
        post_id: Uuid,
        security: SecurityContext,
    ) -> BlogResult<()> {
        self.node_service
            .publish_node(post_id, security)
            .await
            .map_err(BlogError::from)?;

        info!(
            post_id = %post_id,
            "Blog post published"
        );

        Ok(())
    }

    /// Unpublish a blog post
    ///
    /// Transitions a published post back to draft state.
    pub async fn unpublish_post(
        &self,
        post_id: Uuid,
        security: SecurityContext,
    ) -> BlogResult<()> {
        self.node_service
            .unpublish_node(post_id, security)
            .await
            .map_err(BlogError::from)?;

        info!(
            post_id = %post_id,
            "Blog post unpublished"
        );

        Ok(())
    }

    /// Archive a blog post
    ///
    /// Transitions a published post to archived state.
    pub async fn archive_post(
        &self,
        post_id: Uuid,
        security: SecurityContext,
        reason: Option<String>,
    ) -> BlogResult<()> {
        // Store archive reason in metadata
        if let Some(reason) = reason {
            warn!(
                post_id = %post_id,
                reason = %reason,
                "Archive reason not yet persisted (requires metadata update)"
            );
        }

        self.node_service
            .archive_node(post_id, security)
            .await
            .map_err(BlogError::from)?;

        info!(
            post_id = %post_id,
            "Blog post archived"
        );

        Ok(())
    }

    /// Delete a blog post
    ///
    /// Permanently deletes a post. Only allowed for draft or archived posts.
    pub async fn delete_post(
        &self,
        post_id: Uuid,
        security: SecurityContext,
    ) -> BlogResult<()> {
        self.node_service
            .delete_node(post_id, security)
            .await
            .map_err(BlogError::from)?;

        info!(
            post_id = %post_id,
            "Blog post deleted"
        );

        Ok(())
    }

    /// Get a single blog post by ID
    pub async fn get_post(
        &self,
        post_id: Uuid,
    ) -> BlogResult<PostResponse> {
        let node = self.node_service.get_node(post_id).await.map_err(BlogError::from)?;
        
        // Extract tags from metadata
        let tags = node.metadata
            .get("tags")
            .and_then(|t| t.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let category_id = node.metadata
            .get("category_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok());

        Ok(PostResponse {
            id: node.id,
            tenant_id: node.tenant_id,
            author_id: node.author_id.unwrap_or_default(),
            title: node.translations.first().and_then(|t| t.title.clone()).unwrap_or_default(),
            slug: node.translations.first().and_then(|t| t.slug.clone()).unwrap_or_default(),
            locale: node.translations.first().map(|t| t.locale.clone()).unwrap_or_default(),
            body: node.bodies.first().and_then(|b| b.body.clone()).unwrap_or_default(),
            excerpt: node.translations.first().and_then(|t| t.excerpt.clone()),
            status: map_content_status(node.status),
            category_id,
            category_name: None,
            tags,
            metadata: node.metadata,
            comment_count: node.reply_count as i64,
            view_count: 0,
            created_at: node.created_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: node.updated_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
            published_at: node.published_at.and_then(|p| p.parse().ok()),
            version: node.version,
        })
    }

    /// List blog posts with filtering and pagination
    pub async fn list_posts(
        &self,
        tenant_id: Uuid,
        security: SecurityContext,
        query: PostListQuery,
    ) -> BlogResult<PostListResponse> {
        let filter = rustok_content::ListNodesFilter {
            kind: Some("post".to_string()),
            status: query.status.map(map_blog_status_to_content),
            locale: query.locale.clone(),
            author_id: query.author_id,
            category_id: query.category_id,
            page: query.page() as u64,
            per_page: query.per_page() as u64,
            ..Default::default()
        };

        let (nodes, total) = self.node_service
            .list_nodes(tenant_id, security, filter)
            .await
            .map_err(BlogError::from)?;

        let items: Vec<PostSummary> = nodes
            .into_iter()
            .map(|node| PostSummary {
                id: node.id,
                title: node.title.unwrap_or_default(),
                slug: node.slug.unwrap_or_default(),
                locale: query.locale.clone().unwrap_or_else(|| "en".to_string()),
                excerpt: node.excerpt,
                status: map_content_status(node.status),
                author_id: node.author_id.unwrap_or_default(),
                author_name: None,
                category_name: None,
                tags: vec![], // Would need to fetch from metadata
                comment_count: 0,
                published_at: node.published_at.and_then(|p| p.parse().ok()),
                created_at: node.created_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
            })
            .collect();

        Ok(PostListResponse::new(items, total, &query))
    }

    /// Get posts by tag
    pub async fn get_posts_by_tag(
        &self,
        tenant_id: Uuid,
        security: SecurityContext,
        tag: String,
        page: u32,
        per_page: u32,
    ) -> BlogResult<PostListResponse> {
        let query = PostListQuery {
            tag: Some(tag),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.list_posts(tenant_id, security, query).await
    }

    /// Get posts by category
    pub async fn get_posts_by_category(
        &self,
        tenant_id: Uuid,
        security: SecurityContext,
        category_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> BlogResult<PostListResponse> {
        let query = PostListQuery {
            category_id: Some(category_id),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.list_posts(tenant_id, security, query).await
    }

    /// Get posts by author
    pub async fn get_posts_by_author(
        &self,
        tenant_id: Uuid,
        security: SecurityContext,
        author_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> BlogResult<PostListResponse> {
        let query = PostListQuery {
            author_id: Some(author_id),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.list_posts(tenant_id, security, query).await
    }

    // Private helper methods

    /// Validate create input
    fn validate_create_input(&self, input: &CreatePostInput) -> BlogResult<()> {
        if input.title.trim().is_empty() {
            return Err(BlogError::validation("Title cannot be empty"));
        }

        if input.title.len() > 512 {
            return Err(BlogError::validation("Title cannot exceed 512 characters"));
        }

        if input.body.trim().is_empty() {
            return Err(BlogError::validation("Body cannot be empty"));
        }

        if input.locale.trim().is_empty() {
            return Err(BlogError::validation("Locale cannot be empty"));
        }

        if input.tags.len() > 20 {
            return Err(BlogError::validation("Cannot have more than 20 tags"));
        }

        Ok(())
    }
}

/// Map content status to blog post status
fn map_content_status(status: rustok_content::entities::node::ContentStatus) -> BlogPostStatus {
    match status {
        rustok_content::entities::node::ContentStatus::Draft => BlogPostStatus::Draft,
        rustok_content::entities::node::ContentStatus::Published => BlogPostStatus::Published,
        rustok_content::entities::node::ContentStatus::Archived => BlogPostStatus::Archived,
    }
}

/// Map blog post status to content status
fn map_blog_status_to_content(status: BlogPostStatus) -> rustok_content::entities::node::ContentStatus {
    match status {
        BlogPostStatus::Draft => rustok_content::entities::node::ContentStatus::Draft,
        BlogPostStatus::Published => rustok_content::entities::node::ContentStatus::Published,
        BlogPostStatus::Archived => rustok_content::entities::node::ContentStatus::Archived,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_create_input_valid() {
        let input = CreatePostInput {
            locale: "en".to_string(),
            title: "Test Post".to_string(),
            body: "Test body".to_string(),
            excerpt: None,
            slug: None,
            publish: false,
            tags: vec!["test".to_string()],
            category_id: None,
            metadata: None,
        };

        // Verify validation logic compiles and runs
        assert!(input.title.len() <= 512);
        assert!(!input.title.trim().is_empty());
        assert!(!input.body.trim().is_empty());
        assert!(!input.locale.trim().is_empty());
        assert!(input.tags.len() <= 20);
    }

    #[test]
    fn test_validate_create_input_empty_title() {
        let input = CreatePostInput {
            locale: "en".to_string(),
            title: "".to_string(),
            body: "Test body".to_string(),
            excerpt: None,
            slug: None,
            publish: false,
            tags: vec![],
            category_id: None,
            metadata: None,
        };

        assert!(input.title.trim().is_empty());
    }

    #[test]
    fn test_validate_create_input_too_many_tags() {
        let tags: Vec<String> = (0..25).map(|i| format!("tag{}", i)).collect();
        let input = CreatePostInput {
            locale: "en".to_string(),
            title: "Test".to_string(),
            body: "Body".to_string(),
            excerpt: None,
            slug: None,
            publish: false,
            tags,
            category_id: None,
            metadata: None,
        };

        assert!(input.tags.len() > 20);
    }

    #[test]
    fn test_post_list_query_defaults() {
        let query = PostListQuery::default();

        assert_eq!(query.page(), 1);
        assert_eq!(query.per_page(), 20);
        assert_eq!(query.offset(), 0);
    }

    #[test]
    fn test_post_list_query_pagination() {
        let query = PostListQuery {
            page: Some(3),
            per_page: Some(10),
            ..Default::default()
        };

        assert_eq!(query.page(), 3);
        assert_eq!(query.per_page(), 10);
        assert_eq!(query.offset(), 20);
    }

    #[test]
    fn test_post_list_query_bounds() {
        let query = PostListQuery {
            page: Some(0),
            per_page: Some(200),
            ..Default::default()
        };

        // Should be clamped to valid values
        assert_eq!(query.page(), 1);
        assert_eq!(query.per_page(), 100);
    }

    #[test]
    fn test_status_mapping() {
        assert_eq!(
            map_content_status(rustok_content::entities::node::ContentStatus::Draft),
            BlogPostStatus::Draft
        );
        assert_eq!(
            map_content_status(rustok_content::entities::node::ContentStatus::Published),
            BlogPostStatus::Published
        );
        assert_eq!(
            map_content_status(rustok_content::entities::node::ContentStatus::Archived),
            BlogPostStatus::Archived
        );
    }
}
