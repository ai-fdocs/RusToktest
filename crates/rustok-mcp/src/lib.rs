//! RusToK MCP Server
//!
//! This crate provides a Model Context Protocol (MCP) server for exploring
//! and interacting with RusToK modules.

pub mod server;
pub mod tools;

pub use server::{serve_stdio, McpServerConfig, RusToKMcpServer};
pub use tools::{
    McpState, McpToolError, McpToolResponse, ModuleDetailsResponse, ModuleInfo,
    ModuleListResponse, ModuleLookupRequest, ModuleLookupResponse, TOOL_LIST_MODULES,
    TOOL_MODULE_DETAILS, TOOL_MODULE_EXISTS,
};
