use anyhow::{bail, Result};

use rustok_core::registry::ModuleRegistry;

use crate::tools::{list_modules, module_exists, McpState};

pub struct McpServerConfig {
    pub registry: ModuleRegistry,
}

impl McpServerConfig {
    pub fn new(registry: ModuleRegistry) -> Self {
        Self { registry }
    }
}

pub async fn serve_stdio(config: McpServerConfig) -> Result<()> {
    let state = Box::leak(Box::new(McpState {
        registry: config.registry,
    }));

    // Initialize the MCP server with stdio transport
    let mut server = rmcp::server::Server::new(state)
        .with_tool(list_modules)
        .with_tool(module_exists);

    // Start serving over stdio
    server.serve_stdio().await?;

    Ok(())
}
