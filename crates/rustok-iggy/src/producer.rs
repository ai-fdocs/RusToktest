use rustok_core::events::EventEnvelope;
use rustok_core::Result;

use crate::config::IggyConfig;
use crate::serialization::EventSerializer;

pub async fn publish(
    config: &IggyConfig,
    serializer: &dyn EventSerializer,
    envelope: EventEnvelope,
) -> Result<()> {
    let topic = match envelope.event.event_type() {
        event_type if event_type.starts_with("system.") => "system",
        _ => "domain",
    };
    let partition_key = envelope.tenant_id.to_string();

    let payload = serializer.serialize(&envelope)?;

    tracing::debug!(
        stream = %config.topology.stream_name,
        topic,
        partition_key,
        event_id = %envelope.id,
        payload_size = payload.len(),
        "Publishing event to iggy"
    );

    Ok(())
}
