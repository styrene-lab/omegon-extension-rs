//! package.install@1 HostAction types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PACKAGE_INSTALL_V1: &str = "package.install@1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageInstallParams {
    pub package: String,
    pub source: PackageInstallSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation: Option<PackageActivation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reuse_key: Option<String>,
}

impl PackageInstallParams {
    pub fn dry_run_registry(package: impl Into<String>) -> Self {
        Self {
            package: package.into(),
            source: PackageInstallSource::Registry,
            version: None,
            digest: None,
            dry_run: true,
            activation: None,
            reuse_key: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageInstallSource {
    Registry,
    LocalPath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageActivation {
    None,
    Enable,
    Restart,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageInstallResult {
    pub install_id: String,
    pub package: String,
    pub source: PackageInstallSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub dry_run: bool,
    pub status: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plan: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_install_params_round_trip() {
        let params = PackageInstallParams {
            package: "omegon-nex-rs".to_string(),
            source: PackageInstallSource::Registry,
            version: Some("0.1.0".to_string()),
            digest: Some("sha256:abc".to_string()),
            dry_run: true,
            activation: Some(PackageActivation::Enable),
            reuse_key: Some("nex".to_string()),
        };

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["package"], "omegon-nex-rs");
        assert_eq!(json["source"], "registry");
        assert_eq!(json["dry_run"], true);
        assert_eq!(json["activation"], "enable");
        let parsed: PackageInstallParams = serde_json::from_value(json).unwrap();
        assert_eq!(parsed, params);
    }

    #[test]
    fn package_install_result_round_trip() {
        let result = PackageInstallResult {
            install_id: "install_123".to_string(),
            package: "omegon-nex-rs".to_string(),
            source: PackageInstallSource::Registry,
            version: Some("0.1.0".to_string()),
            dry_run: true,
            status: "planned".to_string(),
            plan: vec!["download package".to_string(), "verify digest".to_string()],
            effects: vec!["omegon/extensions/omegon-nex-rs".to_string()],
            handle: Some(serde_json::json!({"registry": "omegon"})),
            warnings: vec!["manual approval required".to_string()],
        };

        let json = serde_json::to_value(&result).unwrap();
        let parsed: PackageInstallResult = serde_json::from_value(json).unwrap();
        assert_eq!(parsed, result);
    }
}
