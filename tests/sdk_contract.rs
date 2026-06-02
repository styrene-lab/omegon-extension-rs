use omegon_extension::actions::package::{
    PACKAGE_INSTALL_V1, PackageActivation, PackageInstallParams, PackageInstallResult,
    PackageInstallSource,
};
use omegon_extension::actions::resource::{
    RESOURCE_OPEN_V1, ResourceKind, ResourceOpenIntent, ResourceOpenParams, ResourceOpenPlacement,
    ResourceOpenResult,
};
use omegon_extension::actions::terminal::{
    TERMINAL_CREATE_V1, TerminalCreateParams, TerminalCreateResult, TerminalPlacement,
};
use omegon_extension::{
    Capabilities, ErrorCode, HostAction, HostActionExecution, HostActionOutcome, HostActionStatus,
    JSONRPC_VERSION, METHOD_ACTIONS_EXECUTE, METHOD_BOOTSTRAP_SECRETS, METHOD_EXECUTE_TOOL,
    METHOD_GET_TOOLS, METHOD_INITIALIZE, METHOD_TOOLS_CALL, METHOD_TOOLS_LIST,
    NOTIFICATION_TOOLS_LIST_CHANGED, NOTIFICATION_TOOLS_PROGRESS, NOTIFICATION_WIDGETS_UPDATED,
    PROTOCOL_VERSION, SDK_CONTRACT_JSON, SDK_CONTRACT_VERSION,
};
use serde::Serialize;
use serde_json::{Value, json};

fn contract() -> Value {
    serde_json::from_str(SDK_CONTRACT_JSON).expect("embedded sdk-contract.json must be valid JSON")
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("contract value must be an array")
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("array values must be strings")
                .to_string()
        })
        .collect()
}

fn serialized_keys(value: impl Serialize) -> Vec<String> {
    let object = serde_json::to_value(value)
        .expect("value must serialize")
        .as_object()
        .expect("value must serialize as object")
        .clone();
    object.keys().cloned().collect()
}

#[test]
fn contract_version_matches_exported_constant() {
    assert_eq!(contract()["sdk_contract_version"], SDK_CONTRACT_VERSION);
}

#[test]
fn jsonrpc_version_matches_exported_constant() {
    assert_eq!(contract()["jsonrpc_version"], JSONRPC_VERSION);
}

#[test]
fn protocol_version_matches_exported_constant() {
    assert_eq!(contract()["protocol_version"], PROTOCOL_VERSION);
}

#[test]
fn rpc_method_constants_match_contract() {
    let contract = contract();
    let host_methods = vec![
        METHOD_INITIALIZE,
        METHOD_GET_TOOLS,
        METHOD_TOOLS_LIST,
        METHOD_EXECUTE_TOOL,
        METHOD_TOOLS_CALL,
        METHOD_BOOTSTRAP_SECRETS,
    ];
    let extension_methods = vec![
        METHOD_ACTIONS_EXECUTE,
        NOTIFICATION_TOOLS_PROGRESS,
        NOTIFICATION_TOOLS_LIST_CHANGED,
        NOTIFICATION_WIDGETS_UPDATED,
    ];

    assert_eq!(
        strings(&contract["methods"]["host_to_extension"]),
        host_methods
    );
    assert_eq!(
        strings(&contract["methods"]["extension_to_host"]),
        extension_methods
    );
}

#[test]
fn error_codes_match_contract() {
    let contract = contract();
    let expected = contract["error_codes"]
        .as_object()
        .expect("error_codes must be an object");

    let actual = [
        ErrorCode::ParseError,
        ErrorCode::InvalidRequest,
        ErrorCode::MethodNotFound,
        ErrorCode::InvalidParams,
        ErrorCode::InternalError,
        ErrorCode::Timeout,
        ErrorCode::NotImplemented,
        ErrorCode::ManifestError,
        ErrorCode::VersionMismatch,
        ErrorCode::Cancelled,
        ErrorCode::ResourceNotFound,
        ErrorCode::SamplingDenied,
    ];

    assert_eq!(
        expected.len(),
        actual.len(),
        "contract and Rust error count drifted"
    );
    for code in actual {
        let label = code.label();
        let expected_numeric = expected[label]
            .as_i64()
            .unwrap_or_else(|| panic!("contract missing numeric code for {label}"));
        assert_eq!(
            i64::from(code.numeric()),
            expected_numeric,
            "numeric code drifted for {label}"
        );
        assert_eq!(
            ErrorCode::from_label(label),
            Some(code),
            "label lookup drifted for {label}"
        );
        assert_eq!(
            ErrorCode::from_numeric(code.numeric()),
            Some(code),
            "numeric lookup drifted for {label}"
        );
    }
}

#[test]
fn capability_defaults_match_contract() {
    let contract = contract();
    let capability_contract = &contract["capabilities"];
    let defaults = serde_json::to_value(Capabilities::default()).expect("serialize defaults");

    for field in capability_contract["default_true"]
        .as_array()
        .expect("default_true array")
    {
        let field = field.as_str().expect("field name");
        assert_eq!(defaults[field], true, "{field} should default true");
    }

    for field in capability_contract["default_false"]
        .as_array()
        .expect("default_false array")
    {
        let field = field.as_str().expect("field name");
        assert_eq!(defaults[field], false, "{field} should default false");
    }

    let fields = capability_contract["fields"]
        .as_array()
        .expect("fields array");
    assert_eq!(
        fields.len(),
        defaults.as_object().expect("capabilities object").len(),
        "contract and Rust capability field count drifted"
    );
    for field in fields {
        let field = field.as_str().expect("field name");
        assert!(
            defaults.get(field).is_some(),
            "contract field {field} missing from Rust capabilities"
        );
    }
}

#[test]
fn host_action_shapes_match_contract() {
    let contract = contract();
    let host_actions = &contract["host_actions"];

    assert_eq!(
        strings(&host_actions["types"]),
        vec![TERMINAL_CREATE_V1, PACKAGE_INSTALL_V1, RESOURCE_OPEN_V1]
    );
    assert_eq!(
        strings(&host_actions["host_action_fields"]),
        serialized_keys(HostAction::new("open-reader", TERMINAL_CREATE_V1, json!({})).unwrap())
    );
    assert_eq!(
        strings(&host_actions["host_action_execution_values"]),
        vec![
            serde_json::to_value(HostActionExecution::Manual).unwrap(),
            serde_json::to_value(HostActionExecution::AutoIfAllowed).unwrap(),
        ]
        .into_iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect::<Vec<_>>()
    );
    assert_eq!(
        strings(&host_actions["host_action_status_values"]),
        vec![
            HostActionStatus::Completed,
            HostActionStatus::NeedsApproval,
            HostActionStatus::Denied,
            HostActionStatus::Unsupported,
            HostActionStatus::Invalid,
            HostActionStatus::Failed,
        ]
        .into_iter()
        .map(|status| serde_json::to_value(status)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string())
        .collect::<Vec<_>>()
    );
    assert_eq!(
        strings(&host_actions["host_action_outcome_fields"]),
        serialized_keys(HostActionOutcome {
            action_id: "open-reader".to_string(),
            status: HostActionStatus::Completed,
            result: Some(json!({})),
            error: None,
        })
    );
}

#[test]
fn terminal_create_shapes_match_contract() {
    let contract = contract();
    let host_actions = &contract["host_actions"];

    assert_eq!(
        strings(&host_actions["terminal_create_params_fields"]),
        serialized_keys(TerminalCreateParams {
            command: "bookokrat".to_string(),
            args: vec!["/tmp/book.epub".to_string()],
            cwd: Some("/workspace".to_string()),
            env: [("BOOKOKRAT_THEME".to_string(), "dark".to_string())].into(),
            title: Some("Reader".to_string()),
            placement: Some(TerminalPlacement::SidePane),
            reuse_key: Some("reader".to_string()),
        })
    );
    assert_eq!(
        strings(&host_actions["terminal_create_placement_values"]),
        vec![
            TerminalPlacement::Default,
            TerminalPlacement::SidePane,
            TerminalPlacement::BottomPane,
            TerminalPlacement::NewTab,
            TerminalPlacement::BackgroundSession,
        ]
        .into_iter()
        .map(|placement| {
            serde_json::to_value(placement)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>()
    );
    assert_eq!(
        strings(&host_actions["terminal_create_result_fields"]),
        serialized_keys(TerminalCreateResult {
            terminal_id: "term_123".to_string(),
            backend: "zellij".to_string(),
            actual_placement: "background_session".to_string(),
            warnings: vec!["placement degraded".to_string()],
        })
    );
}

#[test]
fn package_install_shapes_match_contract() {
    let contract = contract();
    let host_actions = &contract["host_actions"];

    let params = PackageInstallParams {
        package: "omegon-nex-rs".to_string(),
        source: PackageInstallSource::Registry,
        version: Some("0.1.0".to_string()),
        digest: Some("sha256:abc".to_string()),
        dry_run: true,
        activation: Some(PackageActivation::Enable),
        reuse_key: Some("nex".to_string()),
    };
    assert_eq!(
        serialized_keys(&params),
        strings(&host_actions["package_install_params_fields"])
    );
    assert_eq!(
        serde_json::to_value([
            PackageInstallSource::Registry,
            PackageInstallSource::LocalPath
        ])
        .expect("serialize package sources"),
        host_actions["package_install_source_values"]
    );
    assert_eq!(
        serde_json::to_value([
            PackageActivation::None,
            PackageActivation::Enable,
            PackageActivation::Restart,
        ])
        .expect("serialize package activations"),
        host_actions["package_activation_values"]
    );

    let result = PackageInstallResult {
        install_id: "install_123".to_string(),
        package: "omegon-nex-rs".to_string(),
        source: PackageInstallSource::Registry,
        version: Some("0.1.0".to_string()),
        dry_run: true,
        status: "planned".to_string(),
        plan: vec!["download package".to_string()],
        effects: vec!["omegon/extensions/omegon-nex-rs".to_string()],
        handle: Some(json!({"registry": "omegon"})),
        warnings: vec!["manual approval required".to_string()],
    };
    assert_eq!(
        serialized_keys(&result),
        strings(&host_actions["package_install_result_fields"])
    );
}

#[test]
fn resource_open_shapes_match_contract() {
    let contract = contract();
    let host_actions = &contract["host_actions"];

    assert_eq!(
        strings(&host_actions["resource_open_params_fields"]),
        serialized_keys(ResourceOpenParams {
            uri: "file:///workspace/docs/architecture.md".to_string(),
            intent: Some(ResourceOpenIntent::View),
            kind: Some(ResourceKind::Markdown),
            placement: Some(ResourceOpenPlacement::MainTab),
            reuse_key: Some("docs/architecture.md".to_string()),
            title: Some("Architecture".to_string()),
        })
    );
    assert_eq!(
        strings(&host_actions["resource_open_intent_values"]),
        vec![
            ResourceOpenIntent::View,
            ResourceOpenIntent::Edit,
            ResourceOpenIntent::Read,
            ResourceOpenIntent::Inspect,
        ]
        .into_iter()
        .map(|intent| serde_json::to_value(intent)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string())
        .collect::<Vec<_>>()
    );
    assert_eq!(
        strings(&host_actions["resource_open_kind_values"]),
        vec![
            ResourceKind::Markdown,
            ResourceKind::Code,
            ResourceKind::Text,
            ResourceKind::Diagram,
            ResourceKind::Image,
            ResourceKind::Ebook,
            ResourceKind::Pdf,
            ResourceKind::Directory,
            ResourceKind::Unknown,
        ]
        .into_iter()
        .map(|kind| serde_json::to_value(kind)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string())
        .collect::<Vec<_>>()
    );
    assert_eq!(
        strings(&host_actions["resource_open_placement_values"]),
        vec![
            ResourceOpenPlacement::Default,
            ResourceOpenPlacement::MainTab,
            ResourceOpenPlacement::SidePane,
            ResourceOpenPlacement::Editor,
            ResourceOpenPlacement::BackgroundSession,
        ]
        .into_iter()
        .map(|placement| {
            serde_json::to_value(placement)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>()
    );
    assert_eq!(
        strings(&host_actions["resource_open_result_fields"]),
        serialized_keys(ResourceOpenResult {
            resource_id: "res_123".to_string(),
            backend: "flynt".to_string(),
            actual_placement: "main_tab".to_string(),
            handle: Some(json!({"tab_id": "tab-1"})),
            warnings: vec!["fallback used".to_string()],
        })
    );
}
