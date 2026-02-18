use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use rustok_content::NodeService;
use rustok_core::SecurityContext;
use rustok_outbox::TransactionalEventBus;

use crate::constants::{reply_status, topic_status, KIND_TOPIC};
use crate::error::{ForumError, ForumResult};

pub struct ModerationService {
    nodes: NodeService,
}

impl ModerationService {
    pub fn new(db: DatabaseConnection, event_bus: TransactionalEventBus) -> Self {
        Self {
            nodes: NodeService::new(db, event_bus),
        }
    }

    // ── Reply moderation ───────────────────────────────────────────────────

    #[instrument(skip(self, security))]
    pub async fn approve_reply(
        &self,
        reply_id: Uuid,
        security: SecurityContext,
    ) -> ForumResult<()> {
        self.update_reply_status(reply_id, security, reply_status::APPROVED)
            .await
    }

    #[instrument(skip(self, security))]
    pub async fn reject_reply(&self, reply_id: Uuid, security: SecurityContext) -> ForumResult<()> {
        self.update_reply_status(reply_id, security, reply_status::REJECTED)
            .await
    }

    #[instrument(skip(self, security))]
    pub async fn hide_reply(&self, reply_id: Uuid, security: SecurityContext) -> ForumResult<()> {
        self.update_reply_status(reply_id, security, reply_status::HIDDEN)
            .await
    }

    // ── Topic moderation ───────────────────────────────────────────────────

    #[instrument(skip(self, security))]
    pub async fn pin_topic(&self, topic_id: Uuid, security: SecurityContext) -> ForumResult<()> {
        self.update_topic_bool_flag(topic_id, security, "is_pinned", true)
            .await
    }

    #[instrument(skip(self, security))]
    pub async fn unpin_topic(&self, topic_id: Uuid, security: SecurityContext) -> ForumResult<()> {
        self.update_topic_bool_flag(topic_id, security, "is_pinned", false)
            .await
    }

    #[instrument(skip(self, security))]
    pub async fn lock_topic(&self, topic_id: Uuid, security: SecurityContext) -> ForumResult<()> {
        self.update_topic_bool_flag(topic_id, security, "is_locked", true)
            .await
    }

    #[instrument(skip(self, security))]
    pub async fn unlock_topic(&self, topic_id: Uuid, security: SecurityContext) -> ForumResult<()> {
        self.update_topic_bool_flag(topic_id, security, "is_locked", false)
            .await
    }

    #[instrument(skip(self, security))]
    pub async fn close_topic(&self, topic_id: Uuid, security: SecurityContext) -> ForumResult<()> {
        self.update_topic_forum_status(topic_id, security, topic_status::CLOSED)
            .await
    }

    #[instrument(skip(self, security))]
    pub async fn archive_topic(
        &self,
        topic_id: Uuid,
        security: SecurityContext,
    ) -> ForumResult<()> {
        self.update_topic_forum_status(topic_id, security, topic_status::ARCHIVED)
            .await
    }

    // ── Private helpers ────────────────────────────────────────────────────

    async fn update_reply_status(
        &self,
        reply_id: Uuid,
        security: SecurityContext,
        status: &str,
    ) -> ForumResult<()> {
        let node = self.nodes.get_node(reply_id).await?;
        let mut metadata = node.metadata;
        metadata["reply_status"] = serde_json::json!(status);

        self.nodes
            .update_node(
                reply_id,
                security,
                rustok_content::UpdateNodeInput {
                    metadata: Some(metadata),
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }

    async fn update_topic_bool_flag(
        &self,
        topic_id: Uuid,
        security: SecurityContext,
        flag: &str,
        value: bool,
    ) -> ForumResult<()> {
        let node = self.nodes.get_node(topic_id).await?;
        if node.kind != KIND_TOPIC {
            return Err(ForumError::TopicNotFound(topic_id));
        }
        let mut metadata = node.metadata;
        metadata[flag] = serde_json::json!(value);

        self.nodes
            .update_node(
                topic_id,
                security,
                rustok_content::UpdateNodeInput {
                    metadata: Some(metadata),
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }

    async fn update_topic_forum_status(
        &self,
        topic_id: Uuid,
        security: SecurityContext,
        status: &str,
    ) -> ForumResult<()> {
        let node = self.nodes.get_node(topic_id).await?;
        if node.kind != KIND_TOPIC {
            return Err(ForumError::TopicNotFound(topic_id));
        }
        let mut metadata = node.metadata;
        metadata["forum_status"] = serde_json::json!(status);

        self.nodes
            .update_node(
                topic_id,
                security,
                rustok_content::UpdateNodeInput {
                    metadata: Some(metadata),
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }
}
