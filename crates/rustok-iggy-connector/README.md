# rustok-iggy-connector

Connection abstraction layer for Iggy message streaming platform.

## Overview

This crate provides a unified interface for connecting to Iggy in two modes:
- **Embedded**: Run Iggy server within your application process
- **Remote**: Connect to an external Iggy server cluster

It is used by `rustok-iggy` as the underlying transport layer.

## Features

- Trait-based abstraction for flexibility
- Embedded and Remote mode support
- Automatic topology management (streams, topics)
- Message subscription interface
- Partition calculation for routing
- Optional Iggy SDK integration via feature flag

## Usage

### Remote Mode

```rust
use rustok_iggy_connector::{
    ConnectorConfig, ConnectorMode, IggyConnector, RemoteConnector, PublishRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = RemoteConnector::new();

    let config = ConnectorConfig {
        mode: ConnectorMode::Remote,
        remote: RemoteConnectorConfig {
            addresses: vec!["127.0.0.1:8090".to_string()],
            protocol: "tcp".to_string(),
            username: "rustok".to_string(),
            password: "secret".to_string(),
            tls_enabled: false,
        },
        stream_name: "rustok".to_string(),
        topic_name: "domain".to_string(),
        partitions: 8,
        ..Default::default()
    };

    connector.connect(&config).await?;

    let request = PublishRequest::simple("tenant-123", b"Hello, Iggy!".to_vec(), "event-1");
    connector.publish(request).await?;

    connector.shutdown().await?;
    Ok(())
}
```

### Embedded Mode

```rust
use rustok_iggy_connector::{
    ConnectorConfig, ConnectorMode, EmbeddedConnector, IggyConnector,
};

let connector = EmbeddedConnector::new();
let config = ConnectorConfig {
    mode: ConnectorMode::Embedded,
    embedded: EmbeddedConnectorConfig {
        data_dir: "./data/iggy".to_string(),
        tcp_port: 8090,
        http_port: 3000,
        persistent: true,
    },
    ..Default::default()
};

connector.connect(&config).await?;
// ... use connector ...
connector.shutdown().await?;
```

## Configuration

### EmbeddedConnectorConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data_dir` | String | `"./data/iggy"` | Data storage directory |
| `tcp_port` | u16 | `8090` | TCP listener port |
| `http_port` | u16 | `3000` | HTTP dashboard port |
| `persistent` | bool | `true` | Enable persistence |

### RemoteConnectorConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `addresses` | Vec<String> | `["127.0.0.1:8090"]` | Server addresses |
| `protocol` | String | `"tcp"` | Protocol (tcp/http) |
| `username` | String | `"rustok"` | Auth username |
| `password` | String | `""` | Auth password |
| `tls_enabled` | bool | `false` | Enable TLS |

## Feature Flags

- `iggy`: Enable full Iggy SDK integration for actual message transport

Without this flag, connectors operate in simulation mode with logging only.

## API Reference

### IggyConnector Trait

```rust
#[async_trait]
pub trait IggyConnector: Send + Sync + 'static {
    async fn connect(&self, config: &ConnectorConfig) -> Result<(), ConnectorError>;
    fn is_connected(&self) -> bool;
    async fn publish(&self, request: PublishRequest) -> Result<(), ConnectorError>;
    async fn subscribe(&self, stream: &str, topic: &str, partition: u32)
        -> Result<Box<dyn MessageSubscriber>, ConnectorError>;
    async fn shutdown(&self) -> Result<(), ConnectorError>;
}
```

### PublishRequest

```rust
let request = PublishRequest::new("stream", "topic", "partition-key", payload, "event-id");
// or
let request = PublishRequest::simple("partition-key", payload, "event-id");
```

## Error Handling

```rust
pub enum ConnectorError {
    Connection(String),
    NotConnected,
    Publish(String),
    Subscribe(String),
    Receive(String),
    Topology(String),
    Auth(String),
    Timeout(String),
    Config(String),
    Iggy(/* IggyError when feature enabled */),
}
```

## Dependencies

- `async-trait`: Async trait support
- `tokio`: Async runtime
- `serde`/`serde_json`: Serialization
- `tracing`: Logging
- `thiserror`: Error derivation
- `iggy` (optional): Iggy SDK

## Documentation

- [Local docs](./docs/README.md)
- [Implementation plan](./docs/implementation-plan.md)
