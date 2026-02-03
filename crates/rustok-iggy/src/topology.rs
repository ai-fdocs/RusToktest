use rustok_core::Result;

use crate::config::IggyConfig;

#[derive(Debug, Default)]
pub struct TopologyManager;

pub async fn ensure_topology(config: &IggyConfig) -> Result<()> {
    tracing::debug!(
        stream = %config.topology.stream_name,
        domain_partitions = config.topology.domain_partitions,
        replication_factor = config.topology.replication_factor,
        domain_retention_days = config.retention.domain_max_age_days,
        system_retention_days = config.retention.system_max_age_days,
        dlq_retention_days = config.retention.dlq_max_age_days,
        "Ensuring iggy topology"
    );
    Ok(())
}
