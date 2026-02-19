# rustok-iggy-connector docs

Documentation for `crates/rustok-iggy-connector`.

## Documents

- [Implementation Plan](./implementation-plan.md) - Delivery phases and component status

## Overview

`rustok-iggy-connector` provides connection abstraction for Iggy in two modes:
- **Embedded**: In-process Iggy server
- **Remote**: External Iggy server via TCP/HTTP

## Key Types

| Type | Description |
|------|-------------|
| `IggyConnector` | Trait for connector implementations |
| `RemoteConnector` | Connects to external Iggy server |
| `EmbeddedConnector` | Runs embedded Iggy server |
| `ConnectorConfig` | Configuration for both modes |
| `PublishRequest` | Message publishing request |
| `MessageSubscriber` | Message consumption interface |

## Quick Start

```rust
use rustok_iggy_connector::{ConnectorConfig, IggyConnector, RemoteConnector, PublishRequest};

// Create connector
let connector = RemoteConnector::new();

// Connect
let config = ConnectorConfig::default();
connector.connect(&config).await?;

// Publish
let request = PublishRequest::simple("tenant-1", b"data".to_vec(), "event-1");
connector.publish(request).await?;

// Shutdown
connector.shutdown().await?;
```

## Configuration

### Embedded Mode

```rust
ConnectorConfig {
    mode: ConnectorMode::Embedded,
    embedded: EmbeddedConnectorConfig {
        data_dir: "./data/iggy".to_string(),
        tcp_port: 8090,
        http_port: 3000,
        persistent: true,
    },
    stream_name: "rustok".to_string(),
    topic_name: "domain".to_string(),
    partitions: 8,
    ..Default::default()
}
```

### Remote Mode

```rust
ConnectorConfig {
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
}
```

## Feature Flags

- `iggy`: Enable full Iggy SDK support (optional)

Without the `iggy` feature, connectors run in simulation mode with logging only.

## Error Handling

```rust
use rustok_iggy_connector::ConnectorError;

match connector.publish(request).await {
    Ok(()) => println!("Published"),
    Err(ConnectorError::NotConnected) => println!("Not connected"),
    Err(ConnectorError::Publish(msg)) => println!("Publish error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```
