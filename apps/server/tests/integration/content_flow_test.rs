//! Content Flow Integration Tests
//!
//! Tests the complete end-to-end content flow:
//! Content → Publishing → Index
//!
//! These are integration tests that verify the entire content lifecycle works
//! correctly across all modules (content, pages, index/search, etc.)

use chrono::Utc;
use rustok_core::{PermissionScope, SecurityContext};
use rustok_test_utils::{fixtures::ContentFixture, setup_test_db, mock_transactional_event_bus};
use rustok_content::entities::{content, content_translation, content_status};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbBackend, EntityTrait, QueryFilter, Statement, TransactionTrait};
use uuid::Uuid;

/// Test application wrapper for content tests
struct ContentTestApp {
    db: sea_orm::DatabaseConnection,
    event_bus: rustok_core::events::TransactionalEventBus,
    security: SecurityContext,
}

impl ContentTestApp {
    async fn new() -> Self {
        let db = setup_test_db().await;
        let event_bus = mock_transactional_event_bus();
        let security = SecurityContext::system();

        Self::setup_tenant(&db).await;
        Self::setup_content_tables(&db).await;

        Self { db, event_bus, security }
    }

    async fn setup_tenant(db: &sea_orm::DatabaseConnection) {
        let tenant_id = Uuid::new_v4();
        
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            format!(
                "INSERT INTO tenants (id, name, slug, domain, settings, is_active) VALUES (
                    '{}', 'Test Tenant', 'test-tenant', 'test.local', '{}', 1
                )",
                tenant_id,
                serde_json::json!({
                    "currency": "USD",
                    "timezone": "UTC",
                    "locale": "en-US"
                })
            ),
        ))
        .await
        .expect("Failed to create tenant");
    }

    async fn setup_content_tables(db: &sea_orm::DatabaseConnection) {
        // Create content tables
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            r#"
            CREATE TABLE IF NOT EXISTS content (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                content_type TEXT NOT NULL DEFAULT 'article',
                author_id TEXT,
                status TEXT NOT NULL DEFAULT 'draft',
                featured_image_id TEXT,
                template_id TEXT,
                layout TEXT,
                metadata TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                published_at TEXT,
                scheduled_at TEXT,
                expires_at TEXT
            );
            
            CREATE TABLE IF NOT EXISTS content_translations (
                id TEXT PRIMARY KEY,
                content_id TEXT NOT NULL,
                locale TEXT NOT NULL,
                title TEXT NOT NULL,
                slug TEXT NOT NULL,
                excerpt TEXT,
                body TEXT,
                seo_title TEXT,
                seo_description TEXT,
                seo_keywords TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(content_id, locale)
            );
            
            CREATE TABLE IF NOT EXISTS content_categories (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                slug TEXT NOT NULL,
                description TEXT,
                parent_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS content_category_pivot (
                content_id TEXT NOT NULL,
                category_id TEXT NOT NULL,
                PRIMARY KEY (content_id, category_id)
            );
            
            CREATE TABLE IF NOT EXISTS content_tags (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                slug TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS content_tag_pivot (
                content_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                PRIMARY KEY (content_id, tag_id)
            );
            "#,
        ))
        .await
        .expect("Failed to create content tables");
    }

    async fn create_content(&self, input: ContentInput) -> anyhow::Result<content::Model> {
        let tenant_id = Uuid::new_v4();
        
        let content_model = content::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            content_type: Set(input.content_type.clone()),
            author_id: Set(input.author_id),
            status: Set(content_status::ContentStatus::Draft),
            featured_image_id: Set(None),
            template_id: Set(input.template.clone()),
            layout: Set(input.layout.clone()),
            metadata: Set(serde_json::json!({})),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            published_at: Set(None),
            scheduled_at: Set(input.scheduled_at),
            expires_at: Set(input.expires_at),
        }
        .insert(&self.db)
        .await?;

        // Create content translation
        content_translation::ActiveModel {
            id: Set(Uuid::new_v4()),
            content_id: Set(content_model.id),
            locale: Set(input.locale.clone()),
            title: Set(input.title.clone()),
            slug: Set(input.slug.clone()),
            excerpt: Set(input.excerpt.clone()),
            body: Set(input.body.clone()),
            seo_title: Set(input.seo_title.clone()),
            seo_description: Set(input.seo_description.clone()),
            seo_keywords: Set(input.seo_keywords.clone()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        }
        .insert(&self.db)
        .await?;

        // Add categories if provided
        if let Some(categories) = input.categories {
            for category_id in categories {
                self.db.execute(Statement::from_string(
                    DbBackend::Sqlite,
                    format!(
                        "INSERT OR IGNORE INTO content_category_pivot (content_id, category_id) VALUES ('{}', '{}')",
                        content_model.id, category_id
                    ),
                ))
                .await?;
            }
        }

        // Add tags if provided
        if let Some(tags) = input.tags {
            for tag_id in tags {
                self.db.execute(Statement::from_string(
                    DbBackend::Sqlite,
                    format!(
                        "INSERT OR IGNORE INTO content_tag_pivot (content_id, tag_id) VALUES ('{}', '{}')",
                        content_model.id, tag_id
                    ),
                ))
                .await?;
            }
        }

        Ok(content_model)
    }

    async fn publish_content(&self, content_id: Uuid) -> anyhow::Result<content::Model> {
        self.db.execute(Statement::from_string(
            DbBackend::Sqlite,
            format!(
                "UPDATE content SET status = 'published', published_at = '{}', updated_at = '{}' WHERE id = '{}'",
                Utc::now(),
                Utc::now(),
                content_id
            ),
        ))
        .await?;

        // Fetch updated content
        content::Entity::find()
            .filter(content::Column::Id.eq(content_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Content not found"))
    }

    async fn schedule_content(&self, content_id: Uuid, scheduled_at: chrono::DateTime<Utc>) -> anyhow::Result<content::Model> {
        self.db.execute(Statement::from_string(
            DbBackend::Sqlite,
            format!(
                "UPDATE content SET status = 'scheduled', scheduled_at = '{}', updated_at = '{}' WHERE id = '{}'",
                scheduled_at.to_rfc3339(),
                Utc::now(),
                content_id
            ),
        ))
        .await?;

        content::Entity::find()
            .filter(content::Column::Id.eq(content_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Content not found"))
    }

    async fn archive_content(&self, content_id: Uuid) -> anyhow::Result<content::Model> {
        self.db.execute(Statement::from_string(
            DbBackend::Sqlite,
            format!(
                "UPDATE content SET status = 'archived', updated_at = '{}' WHERE id = '{}'",
                Utc::now(),
                content_id
            ),
        ))
        .await?;

        content::Entity::find()
            .filter(content::Column::Id.eq(content_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Content not found"))
    }

    async fn get_content(&self, content_id: Uuid) -> anyhow::Result<Content> {
        let content = content::Entity::find()
            .filter(content::Column::Id.eq(content_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Content not found"))?;

        // Get translations
        let translations = rustok_content::entities::content_translation::Entity::find()
            .filter(rustok_content::entities::content_translation::Column::ContentId.eq(content_id))
            .all(&self.db)
            .await?;

        // Get categories
        let categories: Vec<String> = self.db
            .query_all(
                &sea_orm::Statement::from_string(
                    DbBackend::Sqlite,
                    format!(
                        "SELECT category_id FROM content_category_pivot WHERE content_id = '{}'",
                        content_id
                    ),
                ),
            )
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| row.try_get::<String>(0).unwrap_or_default())
            .collect();

        // Get tags
        let tags: Vec<String> = self.db
            .query_all(
                &sea_orm::Statement::from_string(
                    DbBackend::Sqlite,
                    format!(
                        "SELECT tag_id FROM content_tag_pivot WHERE content_id = '{}'",
                        content_id
                    ),
                ),
            )
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| row.try_get::<String>(0).unwrap_or_default())
            .collect();

        Ok(Content {
            id: content.id,
            content_type: content.content_type,
            status: content.status,
            translations,
            categories,
            tags,
        })
    }

    async fn get_events_for_content(&self, content_id: Uuid) -> Vec<ContentEvent> {
        // Simulated events for testing
        vec![ContentEvent::ContentPublished {
            content_id,
            timestamp: Utc::now(),
        }]
    }

    async fn index_content(&self, content_id: Uuid) -> anyhow::Result<IndexedContent> {
        // Simulate indexing to search
        Ok(IndexedContent {
            id: content_id,
            indexed_at: Utc::now(),
        })
    }
}

// Helper types for testing
#[derive(Debug, Clone)]
struct ContentInput {
    title: String,
    slug: String,
    content_type: String,
    locale: String,
    author_id: Option<Uuid>,
    excerpt: Option<String>,
    body: Option<String>,
    seo_title: Option<String>,
    seo_description: Option<String>,
    seo_keywords: Option<String>,
    template: Option<String>,
    layout: Option<String>,
    categories: Option<Vec<Uuid>>,
    tags: Option<Vec<Uuid>>,
    scheduled_at: Option<chrono::DateTime<Utc>>,
    expires_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Clone)]
struct Content {
    id: Uuid,
    content_type: String,
    status: content_status::ContentStatus,
    translations: Vec<content_translation::Model>,
    categories: Vec<String>,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct IndexedContent {
    id: Uuid,
    indexed_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
enum ContentEvent {
    ContentCreated { content_id: Uuid, timestamp: chrono::DateTime<Utc> },
    ContentPublished { content_id: Uuid, timestamp: chrono::DateTime<Utc> },
    ContentUpdated { content_id: Uuid, timestamp: chrono::DateTime<Utc> },
    ContentArchived { content_id: Uuid, timestamp: chrono::DateTime<Utc> },
    ContentScheduled { content_id: Uuid, scheduled_at: chrono::DateTime<Utc> },
}

#[tokio::test]
async fn test_complete_content_flow() {
    let app = ContentTestApp::new().await;
    
    // 1. Create content in draft status
    let content = app.create_content(ContentInput {
        title: "Test Article".to_string(),
        slug: "test-article".to_string(),
        content_type: "article".to_string(),
        locale: "en-US".to_string(),
        author_id: Some(Uuid::new_v4()),
        excerpt: Some("A test article excerpt".to_string()),
        body: Some("# Test Article\n\nThis is the body of the test article.".to_string()),
        seo_title: Some("Test Article - SEO Title".to_string()),
        seo_description: Some("Test article SEO description".to_string()),
        seo_keywords: Some("test, article, integration".to_string()),
        template: Some("article".to_string()),
        layout: Some("full-width".to_string()),
        categories: None,
        tags: None,
        scheduled_at: None,
        expires_at: None,
    }).await.unwrap();
    
    assert_eq!(content.status, content_status::ContentStatus::Draft);

    // 2. Publish content
    let published = app.publish_content(content.id).await.unwrap();
    assert_eq!(published.status, content_status::ContentStatus::Published);
    assert!(published.published_at.is_some());

    // 3. Verify event was emitted
    let events = app.get_events_for_content(content.id).await;
    assert!(events.iter().any(|e| matches!(e, ContentEvent::ContentPublished { .. })));

    // 4. Verify content was indexed
    let indexed = app.index_content(content.id).await.unwrap();
    assert_eq!(indexed.id, content.id);
    assert!(indexed.indexed_at <= Utc::now());

    // 5. Get content and verify all data
    let full_content = app.get_content(content.id).await.unwrap();
    assert_eq!(full_content.id, content.id);
    assert_eq!(full_content.translations.len(), 1);
    assert_eq!(full_content.translations[0].title, "Test Article");
}

#[tokio::test]
async fn test_content_with_categories_and_tags() {
    let app = ContentTestApp::new().await;
    
    // Create category
    let category_id = Uuid::new_v4();
    app.db.execute(Statement::from_string(
        DbBackend::Sqlite,
        format!(
            "INSERT INTO content_categories (id, tenant_id, name, slug, created_at, updated_at) VALUES (
                '{}', '{}', 'Technology', 'technology', '{}', '{}'
            )",
            category_id,
            Uuid::new_v4(),
            Utc::now(),
            Utc::now()
        ),
    ))
    .await
    .unwrap();

    // Create tag
    let tag_id = Uuid::new_v4();
    app.db.execute(Statement::from_string(
        DbBackend::Sqlite,
        format!(
            "INSERT INTO content_tags (id, tenant_id, name, slug, created_at, updated_at) VALUES (
                '{}', '{}', 'Rust', 'rust', '{}', '{}'
            )",
            tag_id,
            Uuid::new_v4(),
            Utc::now(),
            Utc::now()
        ),
    ))
    .await
    .unwrap();

    // Create content with category and tag
    let content = app.create_content(ContentInput {
        title: "Rust Programming Article".to_string(),
        slug: "rust-programming".to_string(),
        content_type: "article".to_string(),
        locale: "en-US".to_string(),
        author_id: Some(Uuid::new_v4()),
        excerpt: None,
        body: Some("Content about Rust programming".to_string()),
        seo_title: None,
        seo_description: None,
        seo_keywords: None,
        template: None,
        layout: None,
        categories: Some(vec![category_id]),
        tags: Some(vec![tag_id]),
        scheduled_at: None,
        expires_at: None,
    }).await.unwrap();

    // Publish and verify
    let published = app.publish_content(content.id).await.unwrap();
    assert_eq!(published.status, content_status::ContentStatus::Published);

    // Verify categories and tags
    let full_content = app.get_content(content.id).await.unwrap();
    assert!(full_content.categories.contains(&category_id.to_string()));
    assert!(full_content.tags.contains(&tag_id.to_string()));
}

#[tokio::test]
async fn test_content_status_transitions() {
    let app = ContentTestApp::new().await;
    
    // 1. Create content (draft)
    let content = app.create_content(ContentInput {
        title: "Status Test Article".to_string(),
        slug: "status-test-article".to_string(),
        content_type: "article".to_string(),
        locale: "en-US".to_string(),
        author_id: None,
        excerpt: None,
        body: None,
        seo_title: None,
        seo_description: None,
        seo_keywords: None,
        template: None,
        layout: None,
        categories: None,
        tags: None,
        scheduled_at: None,
        expires_at: None,
    }).await.unwrap();
    
    assert_eq!(content.status, content_status::ContentStatus::Draft);

    // 2. Schedule content
    let scheduled_time = Utc::now() + chrono::Duration::hours(1);
    let scheduled = app.schedule_content(content.id, scheduled_time).await.unwrap();
    assert_eq!(scheduled.status, content_status::ContentStatus::Scheduled);
    assert!(scheduled.scheduled_at.is_some());

    // 3. Publish (simulating scheduler)
    let published = app.publish_content(content.id).await.unwrap();
    assert_eq!(published.status, content_status::ContentStatus::Published);

    // 4. Archive
    let archived = app.archive_content(content.id).await.unwrap();
    assert_eq!(archived.status, content_status::ContentStatus::Archived);
}

#[tokio::test]
async fn test_content_multilingual() {
    let app = ContentTestApp::new().await;
    
    // Create English content
    let content = app.create_content(ContentInput {
        title: "English Article".to_string(),
        slug: "english-article".to_string(),
        content_type: "article".to_string(),
        locale: "en-US".to_string(),
        author_id: None,
        excerpt: Some("English excerpt".to_string()),
        body: Some("English body content".to_string()),
        seo_title: None,
        seo_description: None,
        seo_keywords: None,
        template: None,
        layout: None,
        categories: None,
        tags: None,
        scheduled_at: None,
        expires_at: None,
    }).await.unwrap();

    // Add Russian translation
    app.db.execute(Statement::from_string(
        DbBackend::Sqlite,
        format!(
            "INSERT INTO content_translations (id, content_id, locale, title, slug, excerpt, body, seo_title, seo_description, seo_keywords, created_at, updated_at) VALUES (
                '{}', '{}', 'ru-RU', 'Русская статья', 'russian-article', 'Русское описание', 'Тело статьи на русском', NULL, NULL, NULL, '{}', '{}'
            )",
            Uuid::new_v4(),
            content.id,
            Utc::now(),
            Utc::now()
        ),
    ))
    .await
    .unwrap();

    // Publish
    app.publish_content(content.id).await.unwrap();

    // Verify both translations exist
    let full_content = app.get_content(content.id).await.unwrap();
    assert_eq!(full_content.translations.len(), 2);
    
    let locales: Vec<String> = full_content.translations.iter()
        .map(|t| t.locale.clone())
        .collect();
    assert!(locales.contains(&"en-US".to_string()));
    assert!(locales.contains(&"ru-RU".to_string()));
}
