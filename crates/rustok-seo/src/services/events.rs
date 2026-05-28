use uuid::Uuid;

use rustok_core::{simple_hash, DomainEvent};

use super::SeoService;

impl SeoService {
    pub(super) async fn publish_seo_meta_upserted_event(
        &self,
        tenant_id: Uuid,
        target_kind: &str,
        target_id: Uuid,
        locale: &str,
        source: &str,
        transition_ref: Option<&str>,
    ) {
        let idempotency_key = self.build_event_key(
            "seo.meta.upserted",
            tenant_id,
            &[
                target_kind.to_string(),
                target_id.to_string(),
                locale.to_string(),
                transition_ref.unwrap_or("direct").to_string(),
            ],
        );
        self.publish_seo_event(
            tenant_id,
            DomainEvent::SeoMetaUpserted {
                target_kind: target_kind.to_string(),
                target_id,
                locale: locale.to_string(),
                source: source.to_string(),
                idempotency_key,
            },
        )
        .await;
    }

    pub(super) async fn publish_seo_revision_published_event(
        &self,
        tenant_id: Uuid,
        target_kind: &str,
        target_id: Uuid,
        revision: i32,
    ) {
        let idempotency_key = self.build_event_key(
            "seo.revision.published",
            tenant_id,
            &[
                target_kind.to_string(),
                target_id.to_string(),
                revision.to_string(),
            ],
        );
        self.publish_seo_event(
            tenant_id,
            DomainEvent::SeoRevisionPublished {
                target_kind: target_kind.to_string(),
                target_id,
                revision,
                idempotency_key,
            },
        )
        .await;
    }

    pub(super) async fn publish_seo_revision_rolled_back_event(
        &self,
        tenant_id: Uuid,
        target_kind: &str,
        target_id: Uuid,
        revision: i32,
    ) {
        let idempotency_key = self.build_event_key(
            "seo.revision.rolled_back",
            tenant_id,
            &[
                target_kind.to_string(),
                target_id.to_string(),
                revision.to_string(),
            ],
        );
        self.publish_seo_event(
            tenant_id,
            DomainEvent::SeoRevisionRolledBack {
                target_kind: target_kind.to_string(),
                target_id,
                revision,
                idempotency_key,
            },
        )
        .await;
    }

    pub(super) async fn publish_seo_redirect_upserted_event(
        &self,
        tenant_id: Uuid,
        redirect_id: Uuid,
        source_pattern: &str,
        target_url: &str,
        status_code: i32,
        is_active: bool,
    ) {
        let idempotency_key = self.build_event_key(
            "seo.redirect.upserted",
            tenant_id,
            &[
                redirect_id.to_string(),
                source_pattern.to_string(),
                target_url.to_string(),
                status_code.to_string(),
                is_active.to_string(),
            ],
        );
        self.publish_seo_event(
            tenant_id,
            DomainEvent::SeoRedirectUpserted {
                redirect_id,
                source_pattern: source_pattern.to_string(),
                target_url: target_url.to_string(),
                status_code,
                is_active,
                idempotency_key,
            },
        )
        .await;
    }

    pub(super) async fn publish_seo_redirect_disabled_event(
        &self,
        tenant_id: Uuid,
        redirect_id: Uuid,
        source_pattern: &str,
    ) {
        let idempotency_key = self.build_event_key(
            "seo.redirect.disabled",
            tenant_id,
            &[redirect_id.to_string(), source_pattern.to_string()],
        );
        self.publish_seo_event(
            tenant_id,
            DomainEvent::SeoRedirectDisabled {
                redirect_id,
                source_pattern: source_pattern.to_string(),
                idempotency_key,
            },
        )
        .await;
    }

    pub(super) async fn publish_seo_sitemap_generated_event(
        &self,
        tenant_id: Uuid,
        job_id: Uuid,
        file_count: i32,
    ) {
        let idempotency_key = self.build_event_key(
            "seo.sitemap.generated",
            tenant_id,
            &[job_id.to_string(), file_count.to_string()],
        );
        self.publish_seo_event(
            tenant_id,
            DomainEvent::SeoSitemapGenerated {
                job_id,
                file_count,
                idempotency_key,
            },
        )
        .await;
    }

    pub(super) async fn publish_seo_sitemap_submitted_event(
        &self,
        tenant_id: Uuid,
        job_id: Uuid,
        endpoint_count: i32,
        success: bool,
        error: Option<String>,
    ) {
        let idempotency_key = self.build_event_key(
            "seo.sitemap.submitted",
            tenant_id,
            &[
                job_id.to_string(),
                endpoint_count.to_string(),
                success.to_string(),
            ],
        );
        self.publish_seo_event(
            tenant_id,
            DomainEvent::SeoSitemapSubmitted {
                job_id,
                endpoint_count,
                success,
                error,
                idempotency_key,
            },
        )
        .await;
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn publish_seo_bulk_completed_event(
        &self,
        tenant_id: Uuid,
        job_id: Uuid,
        target_kind: &str,
        locale: &str,
        status: &str,
        processed_count: i32,
        succeeded_count: i32,
        failed_count: i32,
    ) {
        let idempotency_key = self.build_event_key(
            "seo.bulk.completed",
            tenant_id,
            &[
                job_id.to_string(),
                status.to_string(),
                processed_count.to_string(),
                succeeded_count.to_string(),
                failed_count.to_string(),
            ],
        );
        self.publish_seo_event(
            tenant_id,
            DomainEvent::SeoBulkCompleted {
                job_id,
                target_kind: target_kind.to_string(),
                locale: locale.to_string(),
                status: status.to_string(),
                processed_count,
                succeeded_count,
                failed_count,
                idempotency_key,
            },
        )
        .await;
    }

    async fn publish_seo_event(&self, tenant_id: Uuid, event: DomainEvent) {
        if let Err(error) = self.event_bus.publish(tenant_id, None, event.clone()).await {
            tracing::warn!(
                tenant_id = %tenant_id,
                event_type = event.event_type(),
                error = %error,
                "failed to publish SEO domain event"
            );
        }
    }

    fn build_event_key(&self, scope: &str, tenant_id: Uuid, parts: &[String]) -> String {
        let mut payload = format!("{scope}|{tenant_id}");
        for part in parts {
            payload.push('|');
            payload.push_str(part.as_str());
        }
        format!("{scope}:{:016x}", simple_hash(payload.as_str()))
    }
}
