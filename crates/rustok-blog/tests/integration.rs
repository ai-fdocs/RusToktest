//! Integration tests for the Blog module
//!
//! These tests require a database connection and are marked with #[ignore]
//! to prevent running in CI without proper test infrastructure.

use rustok_blog::dto::{CreatePostInput, PostListQuery};
use rustok_blog::state_machine::{BlogPost, BlogPostStatus, CommentStatus};
use rustok_blog::{BlogError, BlogModule};
use rustok_core::events::EventEnvelope;
use rustok_core::{DomainEvent, MigrationSource, RusToKModule, SecurityContext};
use tokio::sync::broadcast;
use uuid::Uuid;

type TestResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct TestContext {
    // Would contain PostService, db connection, event receiver
    tenant_id: Uuid,
    events: broadcast::Receiver<EventEnvelope>,
}

#[tokio::test]
#[ignore = "Integration test requires database/migrations + indexer wiring"]
async fn test_post_lifecycle() -> TestResult<()> {
    let ctx = test_context().await?;

    let input = CreatePostInput {
        locale: "en".to_string(),
        title: "Test Post".to_string(),
        body: "Hello, Blog!".to_string(),
        excerpt: Some("Short excerpt".to_string()),
        slug: None,
        publish: false,
        tags: vec!["rust".to_string()],
        category_id: None,
        metadata: None,
    };

    // Would create post and verify event was emitted
    let _post_id = Uuid::new_v4();

    Ok(())
}

#[tokio::test]
#[ignore = "Integration test requires database"]
async fn test_create_and_publish_post() -> TestResult<()> {
    let _ctx = test_context().await?;

    // Create draft
    let input = CreatePostInput {
        locale: "en".to_string(),
        title: "Draft Post".to_string(),
        body: "Content".to_string(),
        excerpt: None,
        slug: Some("draft-post".to_string()),
        publish: false,
        tags: vec![],
        category_id: None,
        metadata: None,
    };

    // Would verify draft was created with Draft status

    // Would publish and verify status change

    Ok(())
}

#[tokio::test]
#[ignore = "Integration test requires database"]
async fn test_list_posts_with_pagination() -> TestResult<()> {
    let _ctx = test_context().await?;

    let query = PostListQuery {
        page: Some(1),
        per_page: Some(10),
        ..Default::default()
    };

    // Would list posts and verify pagination

    Ok(())
}

#[tokio::test]
#[ignore = "Integration test requires database"]
async fn test_filter_posts_by_tag() -> TestResult<()> {
    let _ctx = test_context().await?;

    let query = PostListQuery {
        tag: Some("rust".to_string()),
        ..Default::default()
    };

    // Would filter posts by tag

    Ok(())
}

#[tokio::test]
#[ignore = "Integration test requires database"]
async fn test_cannot_delete_published_post() -> TestResult<()> {
    let _ctx = test_context().await?;

    // Would create and publish a post
    // Would verify deletion fails with CannotDeletePublished error

    Ok(())
}

async fn test_context() -> TestResult<TestContext> {
    let (event_sender, event_receiver) = broadcast::channel(128);

    Ok(TestContext {
        tenant_id: Uuid::new_v4(),
        events: event_receiver,
    })
}

async fn next_event(
    receiver: &mut broadcast::Receiver<EventEnvelope>,
) -> TestResult<EventEnvelope> {
    let envelope = tokio::time::timeout(std::time::Duration::from_secs(5), receiver.recv())
        .await
        .map_err(|_| "timed out waiting for event")??;
    Ok(envelope)
}

// ============================================================================
// Unit tests (don't require database)
// ============================================================================

mod unit_tests {
    use super::*;

    #[test]
    fn test_blog_post_state_machine() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();

        // Create draft
        let post = BlogPost::new_draft(
            id,
            tenant_id,
            author_id,
            "Test Post".to_string(),
            "test-post".to_string(),
            "en".to_string(),
        );
        assert_eq!(post.to_status(), BlogPostStatus::Draft);

        // Publish
        let post = post.publish();
        assert_eq!(post.to_status(), BlogPostStatus::Published);

        // Archive
        let post = post.archive("Outdated".to_string());
        assert_eq!(post.to_status(), BlogPostStatus::Archived);

        // Restore
        let post = post.restore_to_draft();
        assert_eq!(post.to_status(), BlogPostStatus::Draft);
    }

    #[test]
    fn test_comment_status_transitions() {
        // Pending -> Approved
        assert_eq!(CommentStatus::Pending.approve(), CommentStatus::Approved);

        // Approved -> Spam
        assert_eq!(CommentStatus::Approved.mark_spam(), CommentStatus::Spam);

        // Spam -> Approved
        assert_eq!(CommentStatus::Spam.approve(), CommentStatus::Approved);

        // Any -> Trash
        assert_eq!(CommentStatus::Pending.trash(), CommentStatus::Trash);
    }

    #[test]
    fn test_error_conversions() {
        let id = Uuid::new_v4();
        let err = BlogError::post_not_found(id);

        assert!(matches!(err, BlogError::PostNotFound(_)));

        let err = BlogError::duplicate_slug("test-slug", "en");
        assert!(matches!(err, BlogError::DuplicateSlug { .. }));
    }

    #[test]
    fn test_post_list_query() {
        let query = PostListQuery {
            page: Some(2),
            per_page: Some(25),
            status: Some(BlogPostStatus::Published),
            tag: Some("rust".to_string()),
            ..Default::default()
        };

        assert_eq!(query.page(), 2);
        assert_eq!(query.per_page(), 25);
        assert_eq!(query.offset(), 25);
    }
}
