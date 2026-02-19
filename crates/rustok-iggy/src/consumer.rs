use std::collections::HashMap;
use std::sync::Arc;

use rustok_core::Result;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Default)]
pub struct ConsumerGroupManager {
    groups: Arc<RwLock<HashMap<String, ConsumerGroup>>>,
}

#[derive(Debug, Clone)]
pub struct ConsumerGroup {
    pub name: String,
    pub stream: String,
    pub topic: String,
    pub partitions: Vec<u32>,
}

impl ConsumerGroup {
    pub fn new(name: String, stream: String, topic: String) -> Self {
        Self {
            name,
            stream,
            topic,
            partitions: Vec::new(),
        }
    }

    pub fn with_partitions(mut self, partitions: Vec<u32>) -> Self {
        self.partitions = partitions;
        self
    }
}

impl ConsumerGroupManager {
    pub fn new() -> Self {
        Self {
            groups: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn ensure_group(&self, group: ConsumerGroup) -> Result<()> {
        let name = group.name.clone();
        info!(
            group = %name,
            stream = %group.stream,
            topic = %group.topic,
            "Ensuring consumer group"
        );

        self.groups.write().await.insert(name, group);
        Ok(())
    }

    pub async fn get_group(&self, name: &str) -> Option<ConsumerGroup> {
        self.groups.read().await.get(name).cloned()
    }

    pub async fn list_groups(&self) -> Vec<String> {
        self.groups.read().await.keys().cloned().collect()
    }

    pub async fn remove_group(&self, name: &str) -> Option<ConsumerGroup> {
        self.groups.write().await.remove(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn consumer_group_manager_starts_empty() {
        let manager = ConsumerGroupManager::new();
        assert!(manager.list_groups().await.is_empty());
    }

    #[tokio::test]
    async fn consumer_group_manager_creates_group() {
        let manager = ConsumerGroupManager::new();
        let group = ConsumerGroup::new(
            "domain-consumers".to_string(),
            "rustok".to_string(),
            "domain".to_string(),
        );

        manager.ensure_group(group).await.unwrap();

        let groups = manager.list_groups().await;
        assert_eq!(groups.len(), 1);
        assert!(groups.contains(&"domain-consumers".to_string()));
    }

    #[tokio::test]
    async fn consumer_group_manager_retrieves_group() {
        let manager = ConsumerGroupManager::new();
        let group = ConsumerGroup::new(
            "test-group".to_string(),
            "test-stream".to_string(),
            "test-topic".to_string(),
        )
        .with_partitions(vec![1, 2, 3]);

        manager.ensure_group(group).await.unwrap();

        let retrieved = manager.get_group("test-group").await;
        assert!(retrieved.is_some());

        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.stream, "test-stream");
        assert_eq!(retrieved.topic, "test-topic");
        assert_eq!(retrieved.partitions, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn consumer_group_manager_removes_group() {
        let manager = ConsumerGroupManager::new();
        let group = ConsumerGroup::new("to-remove".to_string(), "s".to_string(), "t".to_string());

        manager.ensure_group(group).await.unwrap();
        let removed = manager.remove_group("to-remove").await;

        assert!(removed.is_some());
        assert!(manager.list_groups().await.is_empty());
    }
}
