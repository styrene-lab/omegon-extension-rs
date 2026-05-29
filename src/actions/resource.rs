//! `resource.open@1` HostAction types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Versioned action type for semantic resource opening.
pub const RESOURCE_OPEN_V1: &str = "resource.open@1";

/// Parameters for `resource.open@1`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceOpenParams {
    /// URI to open. v1 is for host-owned resources such as `file://` and
    /// host-defined resource handles. Web/browser session routing belongs to
    /// future browser.* HostActions, not resource.open@1.
    pub uri: String,
    /// Operator intent for the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<ResourceOpenIntent>,
    /// Advisory resource kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ResourceKind>,
    /// Advisory placement request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<ResourceOpenPlacement>,
    /// Optional origin-scoped reuse key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reuse_key: Option<String>,
    /// Optional human-readable title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ResourceOpenParams {
    /// Create resource open params for a URI with default host-selected behavior.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            intent: None,
            kind: None,
            placement: None,
            reuse_key: None,
            title: None,
        }
    }
}

/// Operator intent for opening a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceOpenIntent {
    View,
    Edit,
    Read,
    Inspect,
}

/// Advisory resource kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    Markdown,
    Code,
    Text,
    Diagram,
    Image,
    Ebook,
    Pdf,
    Directory,
    Unknown,
}

/// Advisory resource placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceOpenPlacement {
    Default,
    MainTab,
    SidePane,
    Editor,
    BackgroundSession,
}

/// Result payload for `resource.open@1`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceOpenResult {
    /// Host-assigned resource/open handle identifier.
    pub resource_id: String,
    /// Backend that satisfied the request, for example `flynt`, `zed`, or `bookokrat`.
    pub backend: String,
    /// Actual placement chosen by the host. Requested placement is advisory.
    pub actual_placement: String,
    /// Optional backend-specific handle data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<Value>,
    /// Optional warnings/degradations when opening succeeded with caveats.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_open_params_round_trip() {
        let params = ResourceOpenParams {
            uri: "file:///workspace/docs/architecture.md".to_string(),
            intent: Some(ResourceOpenIntent::View),
            kind: Some(ResourceKind::Markdown),
            placement: Some(ResourceOpenPlacement::MainTab),
            reuse_key: Some("docs/architecture.md".to_string()),
            title: Some("Architecture".to_string()),
        };

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["uri"], "file:///workspace/docs/architecture.md");
        assert_eq!(json["intent"], "view");
        assert_eq!(json["kind"], "markdown");
        assert_eq!(json["placement"], "main_tab");
        let parsed: ResourceOpenParams = serde_json::from_value(json).unwrap();
        assert_eq!(parsed, params);
    }

    #[test]
    fn resource_open_result_round_trip() {
        let result = ResourceOpenResult {
            resource_id: "res_123".to_string(),
            backend: "flynt".to_string(),
            actual_placement: "main_tab".to_string(),
            handle: Some(serde_json::json!({"tab_id": "tab-1"})),
            warnings: vec!["fallback used".to_string()],
        };

        let json = serde_json::to_value(&result).unwrap();
        let parsed: ResourceOpenResult = serde_json::from_value(json).unwrap();
        assert_eq!(parsed, result);
    }
}
