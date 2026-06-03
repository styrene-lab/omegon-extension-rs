//! Public SDK contract constants shared by Rust, Python, and TypeScript SDKs.
//!
//! These constants are the Rust projection of `schema/sdk-contract.json`.
//! Prefer these over string literals in SDK and extension code so contract drift
//! is caught by tests instead of becoming an ecosystem fork.

/// SDK contract compatibility version shared across Rust, Python, and TypeScript SDKs.
pub const SDK_CONTRACT_VERSION: &str = "0.25";

/// Canonical SDK contract artifact embedded in the Rust crate.
pub const SDK_CONTRACT_JSON: &str = include_str!("../schema/sdk-contract.json");

/// JSON-RPC protocol version string used on every wire message.
pub const JSONRPC_VERSION: &str = "2.0";

/// Omegon extension protocol version. Bump when the wire format changes incompatibly.
pub const PROTOCOL_VERSION: u16 = 2;

/// Host-to-extension `initialize` request.
pub const METHOD_INITIALIZE: &str = "initialize";

/// Legacy host-to-extension tool listing request.
pub const METHOD_GET_TOOLS: &str = "get_tools";

/// MCP-compatible host-to-extension tool listing request.
pub const METHOD_TOOLS_LIST: &str = "tools/list";

/// Legacy host-to-extension tool execution request.
pub const METHOD_EXECUTE_TOOL: &str = "execute_tool";

/// MCP-compatible host-to-extension tool execution request.
pub const METHOD_TOOLS_CALL: &str = "tools/call";

/// Host-to-extension bootstrap secrets request.
pub const METHOD_BOOTSTRAP_SECRETS: &str = "bootstrap_secrets";

/// Extension-to-host HostAction execution request.
pub const METHOD_ACTIONS_EXECUTE: &str = "actions/execute";

/// Extension-to-host tool progress notification.
pub const NOTIFICATION_TOOLS_PROGRESS: &str = "notifications/tools/progress";

/// Extension-to-host tool list changed notification.
pub const NOTIFICATION_TOOLS_LIST_CHANGED: &str = "notifications/tools/list_changed";

/// Extension-to-host widget update notification.
pub const NOTIFICATION_WIDGETS_UPDATED: &str = "notifications/widgets/updated";
