use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use rustok_content::{BodyInput, CreateNodeInput, ListNodesFilter, NodeService, UpdateNodeInput};
use rustok_core::SecurityContext;
use rustok_outbox::TransactionalEventBus;

use crate::constants::{reply_status, KIND_REPLY};
use crate::dto::{CreateReplyInput, ReplyListItem, ReplyResponse, UpdateReplyInput};
use crate::error::{ForumError, ForumResult};

pub struct ReplyService {
    nodes: NodeService,
}

impl ReplyService {
    pub fn new(db: DatabaseConnection, event_bus: TransactionalEventBus) -> Self {
        Self {
            nodes: NodeService::new(db, event_bus),
        }
    }

    #[instrument(skip(self, security, input))]
    pub async fn create(
        &self,
        tenant_id: Uuid,
        security: SecurityContext,
        topic_id: Uuid,
        input: CreateReplyInput,
    ) -> ForumResult<ReplyResponse> {
        if input.content.trim().is_empty() {
            return Err(ForumError::Validation(
                "Reply content cannot be empty".to_string(),
            ));
        }

        let metadata = serde_json::json!({
            "parent_reply_id": input.parent_reply_id,
            "reply_status": reply_status::APPROVED,
        });

        let node = self
            .nodes
            .create_node(
                tenant_id,
                security,
                CreateNodeInput {
                    kind: KIND_REPLY.to_string(),
                    status: Some(rustok_content::entities::node::ContentStatus::Published),
                    parent_id: Some(topic_id),
                    author_id: None,
                    category_id: None,
                    position: None,
                    depth: None,
                    reply_count: None,
                    metadata,
                    translations: Vec::new(),
                    bodies: vec![BodyInput {
                        locale: input.locale.clone(),
                        body: Some(input.content),
                        format: Some("markdown".to_string()),
                    }],
                },
            )
            .await?;

        Ok(Self::node_to_reply(node, topic_id, &input.locale))
    }

    #[instrument(skip(self))]
    pub async fn get(&self, reply_id: Uuid, locale: &str) -> ForumResult<ReplyResponse> {
        let node = self.nodes.get_node(reply_id).await?;

        if node.kind != KIND_REPLY {
            return Err(ForumError::ReplyNotFound(reply_id));
        }

        let topic_id = node.parent_id.ok_or(ForumError::ReplyNotFound(reply_id))?;
        Ok(Self::node_to_reply(node, topic_id, locale))
    }

    #[instrument(skip(self, security, input))]
    pub async fn update(
        &self,
        reply_id: Uuid,
        security: SecurityContext,
        input: UpdateReplyInput,
    ) -> ForumResult<ReplyResponse> {
        let existing = self.get(reply_id, &input.locale).await?;
        let bodies = input.content.map(|content| {
            vec![BodyInput {
                locale: input.locale.clone(),
                body: Some(content),
                format: Some("markdown".to_string()),
            }]
        });

        let node = self
            .nodes
            .update_node(
                reply_id,
                security,
                UpdateNodeInput {
                    bodies,
                    ..UpdateNodeInput::default()
                },
            )
            .await?;

        Ok(Self::node_to_reply(node, existing.topic_id, &input.locale))
    }

    #[instrument(skip(self, security))]
    pub async fn delete(&self, reply_id: Uuid, security: SecurityContext) -> ForumResult<()> {
        self.nodes.delete_node(reply_id, security).await?;
        Ok(())
    }

    #[instrument(skip(self, security))]
    pub async fn list_for_topic(
        &self,
        tenant_id: Uuid,
        security: SecurityContext,
        topic_id: Uuid,
        locale: &str,
    ) -> ForumResult<Vec<ReplyListItem>> {
        let (items, _) = self
            .nodes
            .list_nodes(
                tenant_id,
                security,
                ListNodesFilter {
                    kind: Some(KIND_REPLY.to_string()),
                    status: None,
                    parent_id: Some(topic_id),
                    author_id: None,
                    locale: Some(locale.to_string()),
                    page: 1,
                    per_page: 200,
                },
            )
            .await?;

        let replies = items
            .into_iter()
            .map(|item| ReplyListItem {
                id: item.id,
                locale: locale.to_string(),
                topic_id,
                content_preview: item.excerpt.unwrap_or_default(),
                status: reply_status::APPROVED.to_string(),
                created_at: item.created_at,
            })
            .collect();

        Ok(replies)
    }

    fn node_to_reply(
        node: rustok_content::NodeResponse,
        topic_id: Uuid,
        locale: &str,
    ) -> ReplyResponse {
        let body = node
            .bodies
            .iter()
            .find(|b| b.locale == locale)
            .or_else(|| node.bodies.first());
        let metadata = node.metadata;

        ReplyResponse {
            id: node.id,
            locale: locale.to_string(),
            topic_id,
            content: body.and_then(|b| b.body.clone()).unwrap_or_default(),
            status: metadata
                .get("reply_status")
                .and_then(|v| v.as_str())
                .unwrap_or(reply_status::APPROVED)
                .to_string(),
            parent_reply_id: metadata
                .get("parent_reply_id")
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok()),
            created_at: node.created_at,
            updated_at: node.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{reply_status, KIND_REPLY};
    use rustok_content::dto::{BodyResponse, NodeResponse, NodeTranslationResponse};
    use rustok_content::entities::node::ContentStatus;

    fn make_reply_node(
        topic_id: Uuid,
        content: Option<&str>,
        locale: &str,
        status: &str,
        parent_reply_id: Option<Uuid>,
    ) -> NodeResponse {
        let metadata = serde_json::json!({
            "reply_status": status,
            "parent_reply_id": parent_reply_id.map(|u| u.to_string())
        });
        NodeResponse {
            id: Uuid::nil(),
            tenant_id: Uuid::nil(),
            kind: KIND_REPLY.to_string(),
            status: ContentStatus::Published,
            parent_id: Some(topic_id),
            author_id: None,
            category_id: None,
            position: 0,
            depth: 0,
            reply_count: 0,
            metadata,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            published_at: None,
            translations: vec![NodeTranslationResponse {
                locale: locale.to_string(),
                title: None,
                slug: None,
                excerpt: None,
            }],
            bodies: content
                .map(|c| {
                    vec![BodyResponse {
                        locale: locale.to_string(),
                        body: Some(c.to_string()),
                        format: "markdown".to_string(),
                        updated_at: "2024-01-01T00:00:00Z".to_string(),
                    }]
                })
                .unwrap_or_default(),
        }
    }

    #[test]
    fn node_to_reply_maps_fields() {
        let topic_id = Uuid::new_v4();
        let parent_reply_id = Uuid::new_v4();
        let node =
            make_reply_node(topic_id, Some("Hello!"), "en", reply_status::APPROVED, Some(parent_reply_id));

        let result = ReplyService::node_to_reply(node, topic_id, "en");

        assert_eq!(result.topic_id, topic_id);
        assert_eq!(result.content, "Hello!");
        assert_eq!(result.status, reply_status::APPROVED);
        assert_eq!(result.parent_reply_id, Some(parent_reply_id));
    }

    #[test]
    fn node_to_reply_defaults_on_missing_fields() {
        let topic_id = Uuid::new_v4();
        let node = make_reply_node(topic_id, None, "en", reply_status::PENDING, None);

        let result = ReplyService::node_to_reply(node, topic_id, "en");

        assert_eq!(result.content, "");
        assert_eq!(result.status, reply_status::PENDING);
        assert_eq!(result.parent_reply_id, None);
    }

    #[test]
    fn node_to_reply_falls_back_to_first_body_locale() {
        let topic_id = Uuid::new_v4();
        // body locale is "de" but we request "en"
        let mut node = make_reply_node(topic_id, Some("Hallo!"), "de", reply_status::APPROVED, None);
        node.bodies[0].locale = "de".to_string();

        let result = ReplyService::node_to_reply(node, topic_id, "en");
        assert_eq!(result.content, "Hallo!");
    }
}
