mod common;

use common::API_KEY;
use mockito::Server;
use serde_json::{Map, Value};

use peridio_sdk::api::bundles::{
    Bundle, CreateBundleBinary, CreateBundleParams, CreateBundleParamsV1, CreateBundleParamsV2,
    DeleteBundleParams, GetBundleParams, UpdateBundleParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

// V1 Bundle Tests

#[tokio::test]
async fn create_bundle_v1() {
    let mut server = Server::new_async().await;
    let expected_organization_prn = "organization_prn";
    let expected_artifact_versions = [
        "artifact_version_prn_1".to_string(),
        "artifact_version_prn_2".to_string(),
    ]
    .to_vec();
    let expected_name = "a";
    let expected_id = "uuid";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/bundles")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-create-201.json")
        .create_async()
        .await;

    let params = CreateBundleParams::V1(CreateBundleParamsV1 {
        artifact_version_prns: expected_artifact_versions.clone(),
        id: Some(expected_id.to_string()),
        name: Some(expected_name.to_string()),
    });

    match api.bundles().create(params).await.unwrap() {
        Some(bundle) => match bundle.bundle {
            Bundle::V1(bundle_v1) => {
                assert_eq!(
                    bundle_v1.organization_prn,
                    expected_organization_prn.to_string()
                );
                assert_eq!(bundle_v1.artifact_versions, expected_artifact_versions);
                assert_eq!(bundle_v1.name, Some(expected_name.to_string()));
            }
            Bundle::V2(_) => panic!("Expected V1 bundle"),
        },
        None => panic!("Expected Some(bundle)"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_bundle_v2() {
    let mut server = Server::new_async().await;
    let expected_organization_prn = "organization_prn";
    let expected_name = "v2_bundle";
    let expected_id = "uuid";

    let mut custom_metadata_1 = Map::new();
    custom_metadata_1.insert("version".to_string(), Value::String("1.0.0".to_string()));
    custom_metadata_1.insert(
        "architecture".to_string(),
        Value::String("arm64".to_string()),
    );

    let mut custom_metadata_2 = Map::new();
    custom_metadata_2.insert("version".to_string(), Value::String("2.0.0".to_string()));
    custom_metadata_2.insert(
        "architecture".to_string(),
        Value::String("x86_64".to_string()),
    );

    let expected_binaries = vec![
        CreateBundleBinary {
            prn: "binary_prn_1".to_string(),
            custom_metadata: Some(custom_metadata_1),
        },
        CreateBundleBinary {
            prn: "binary_prn_2".to_string(),
            custom_metadata: Some(custom_metadata_2),
        },
    ];

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 2,
    });

    let m = server
        .mock("POST", "/bundles")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-v2-create-201.json")
        .create_async()
        .await;

    let params = CreateBundleParams::V2(CreateBundleParamsV2 {
        binaries: expected_binaries,
        id: Some(expected_id.to_string()),
        name: Some(expected_name.to_string()),
    });

    match api.bundles().create(params).await.unwrap() {
        Some(bundle) => match bundle.bundle {
            Bundle::V2(bundle_v2) => {
                assert_eq!(
                    bundle_v2.organization_prn,
                    expected_organization_prn.to_string()
                );
                assert_eq!(bundle_v2.name, Some(expected_name.to_string()));
                assert_eq!(bundle_v2.binaries.len(), 2);

                // Verify first binary
                let binary_1 = &bundle_v2.binaries[0];
                assert_eq!(binary_1.prn, "binary_prn_1");
                assert!(binary_1.custom_metadata.is_some());

                // Verify second binary
                let binary_2 = &bundle_v2.binaries[1];
                assert_eq!(binary_2.prn, "binary_prn_2");
                assert!(binary_2.custom_metadata.is_some());
            }
            Bundle::V1(_) => panic!("Expected V2 bundle"),
        },
        None => panic!("Expected Some(bundle)"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_bundle_v1() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("DELETE", &*format!("/bundles/{expected_prn}"))
        .with_status(204)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let params = DeleteBundleParams {
        prn: expected_prn.to_string(),
    };

    match api.bundles().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_bundle_v2() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 2,
    });

    let m = server
        .mock("DELETE", &*format!("/bundles/{expected_prn}"))
        .with_status(204)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let params = DeleteBundleParams {
        prn: expected_prn.to_string(),
    };

    match api.bundles().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_bundle_v1() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";
    let expected_organization_prn = "organization_prn";
    let expected_artifact_versions = [
        "artifact_version_prn_1".to_string(),
        "artifact_version_prn_2".to_string(),
    ]
    .to_vec();

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("GET", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-get-200.json")
        .create_async()
        .await;

    let params = GetBundleParams {
        prn: expected_prn.to_string(),
    };

    match api.bundles().get(params).await.unwrap() {
        Some(bundle) => match bundle.bundle {
            Bundle::V1(bundle_v1) => {
                assert_eq!(
                    bundle_v1.organization_prn,
                    expected_organization_prn.to_string()
                );
                assert_eq!(bundle_v1.artifact_versions, expected_artifact_versions);
                assert_eq!(bundle_v1.prn, expected_prn);
                assert_eq!(bundle_v1.name, Some("a".to_string()));
            }
            Bundle::V2(_) => panic!("Expected V1 bundle"),
        },
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_bundle_v2() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";
    let expected_organization_prn = "organization_prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 2,
    });

    let m = server
        .mock("GET", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-v2-get-200.json")
        .create_async()
        .await;

    let params = GetBundleParams {
        prn: expected_prn.to_string(),
    };

    match api.bundles().get(params).await.unwrap() {
        Some(bundle) => match bundle.bundle {
            Bundle::V2(bundle_v2) => {
                assert_eq!(
                    bundle_v2.organization_prn,
                    expected_organization_prn.to_string()
                );
                assert_eq!(bundle_v2.prn, expected_prn);
                assert_eq!(bundle_v2.name, Some("v2_bundle".to_string()));
                assert_eq!(bundle_v2.binaries.len(), 2);

                // Verify binaries
                let binary_1 = &bundle_v2.binaries[0];
                assert_eq!(binary_1.prn, "binary_prn_1");

                let binary_2 = &bundle_v2.binaries[1];
                assert_eq!(binary_2.prn, "binary_prn_2");
            }
            Bundle::V1(_) => panic!("Expected V2 bundle"),
        },
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_bundle_v1() {
    let mut server = Server::new_async().await;
    let expected_name = "b";
    let expected_prn = "1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("PATCH", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-update-200.json")
        .create_async()
        .await;

    let params = UpdateBundleParams {
        prn: expected_prn.to_string(),
        name: Some(expected_name.to_string()),
    };

    match api.bundles().update(params).await.unwrap() {
        Some(response) => match response.bundle {
            Bundle::V1(bundle_v1) => {
                assert_eq!(bundle_v1.name, Some(expected_name.to_string()));
                assert_eq!(bundle_v1.prn, "prn");
            }
            Bundle::V2(_) => panic!("Expected V1 bundle"),
        },
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_bundle_v2() {
    let mut server = Server::new_async().await;
    let expected_name = "updated_v2_bundle";
    let expected_prn = "1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 2,
    });

    let m = server
        .mock("PATCH", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-v2-update-200.json")
        .create_async()
        .await;

    let params = UpdateBundleParams {
        prn: expected_prn.to_string(),
        name: Some(expected_name.to_string()),
    };

    match api.bundles().update(params).await.unwrap() {
        Some(response) => match response.bundle {
            Bundle::V2(bundle_v2) => {
                assert_eq!(bundle_v2.name, Some(expected_name.to_string()));
                assert_eq!(bundle_v2.prn, "prn");
                assert_eq!(bundle_v2.binaries.len(), 2);
            }
            Bundle::V1(_) => panic!("Expected V2 bundle"),
        },
        _ => panic!(),
    }

    m.assert_async().await;
}

// Keep the original tests for backward compatibility
#[tokio::test]
async fn create_bundle() {
    let mut server = Server::new_async().await;
    let expected_organization_prn = "organization_prn";
    let expected_artifact_versions = [
        "artifact_version_prn_1".to_string(),
        "artifact_version_prn_2".to_string(),
    ]
    .to_vec();
    let expected_name = "a";
    let expected_id = "uuid";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/bundles")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-create-201.json")
        .create_async()
        .await;

    let params = CreateBundleParams::V1(CreateBundleParamsV1 {
        artifact_version_prns: expected_artifact_versions.clone(),
        id: Some(expected_id.to_string()),
        name: Some(expected_name.to_string()),
    });

    match api.bundles().create(params).await.unwrap() {
        Some(bundle) => match (api.api_version, bundle.bundle) {
            (1, Bundle::V1(bundle_v1)) => {
                assert_eq!(
                    bundle_v1.organization_prn,
                    expected_organization_prn.to_string()
                );
                assert_eq!(bundle_v1.artifact_versions, expected_artifact_versions);
            }
            (version, bundle) => panic!(
                "Version mismatch: api_version={}, bundle={:?}",
                version, bundle
            ),
        },
        None => panic!("Expected Some(bundle)"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_bundle() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("DELETE", &*format!("/bundles/{expected_prn}"))
        .with_status(204)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let params = DeleteBundleParams {
        prn: expected_prn.to_string(),
    };

    match api.bundles().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_bundle() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";
    let expected_organization_prn = "organization_prn";
    let expected_artifact_versions = [
        "artifact_version_prn_1".to_string(),
        "artifact_version_prn_2".to_string(),
    ]
    .to_vec();

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("GET", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-get-200.json")
        .create_async()
        .await;

    let params = GetBundleParams {
        prn: expected_prn.to_string(),
    };

    match api.bundles().get(params).await.unwrap() {
        Some(bundle) => match bundle.bundle {
            Bundle::V1(bundle_v1) => {
                assert_eq!(
                    bundle_v1.organization_prn,
                    expected_organization_prn.to_string()
                );
                assert_eq!(bundle_v1.artifact_versions, expected_artifact_versions);
            }
            Bundle::V2(_) => panic!("Expected V1 bundle"),
        },
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_bundle() {
    let mut server = Server::new_async().await;
    let expected_name = "b";
    let expected_prn = "1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("PATCH", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-update-200.json")
        .create_async()
        .await;

    let params = UpdateBundleParams {
        prn: expected_prn.to_string(),
        name: Some(expected_name.to_string()),
    };

    match api.bundles().update(params).await.unwrap() {
        Some(response) => match response.bundle {
            Bundle::V1(bundle_v1) => {
                assert_eq!(bundle_v1.name, Some(expected_name.to_string()));
            }
            Bundle::V2(_) => panic!("Expected V1 bundle"),
        },
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn test_bundle_deserialization() {
    // Test V1 bundle deserialization
    let v1_json = r#"
    {
        "artifact_versions": [
            "artifact_version_prn_1",
            "artifact_version_prn_2"
        ],
        "organization_prn": "organization_prn",
        "prn": "prn",
        "inserted_at": "2000-01-01T00:00:00Z",
        "updated_at": "2000-01-01T00:00:00Z",
        "name": "v1_bundle"
    }
    "#;

    let bundle_v1: Bundle = serde_json::from_str(v1_json).unwrap();
    match bundle_v1 {
        Bundle::V1(bundle) => {
            assert_eq!(bundle.name, Some("v1_bundle".to_string()));
            assert_eq!(bundle.artifact_versions.len(), 2);
            assert_eq!(bundle.organization_prn, "organization_prn");
            assert_eq!(bundle.prn, "prn");
            assert_eq!(bundle.inserted_at, "2000-01-01T00:00:00Z");
            assert_eq!(bundle.updated_at, "2000-01-01T00:00:00Z");
        }
        Bundle::V2(_) => panic!("Expected V1 bundle"),
    }

    // Test V2 bundle deserialization
    let v2_json = r#"
    {
        "binaries": [
            {
                "custom_metadata": {
                    "key1": "value1",
                    "key2": "value2"
                },
                "prn": "binary_prn_1"
            },
            {
                "custom_metadata": null,
                "prn": "binary_prn_2"
            },
            {
                "prn": "binary_prn_3"
            }
        ],
        "organization_prn": "organization_prn",
        "prn": "prn",
        "inserted_at": "2000-01-01T00:00:00Z",
        "updated_at": "2000-01-01T00:00:00Z",
        "name": "v2_bundle"
    }
    "#;

    let bundle_v2: Bundle = serde_json::from_str(v2_json).unwrap();
    match bundle_v2 {
        Bundle::V2(bundle) => {
            assert_eq!(bundle.name, Some("v2_bundle".to_string()));
            assert_eq!(bundle.binaries.len(), 3);
            assert_eq!(bundle.organization_prn, "organization_prn");
            assert_eq!(bundle.prn, "prn");
            assert_eq!(bundle.inserted_at, "2000-01-01T00:00:00Z");
            assert_eq!(bundle.updated_at, "2000-01-01T00:00:00Z");

            // Test first binary with custom metadata
            let binary_1 = &bundle.binaries[0];
            assert_eq!(binary_1.prn, "binary_prn_1");
            assert!(binary_1.custom_metadata.is_some());
            if let Some(metadata) = &binary_1.custom_metadata {
                assert_eq!(metadata.len(), 2);
                assert!(metadata.contains_key("key1"));
                assert!(metadata.contains_key("key2"));
            }

            // Test second binary with null custom metadata
            let binary_2 = &bundle.binaries[1];
            assert_eq!(binary_2.prn, "binary_prn_2");
            assert!(binary_2.custom_metadata.is_none());

            // Test third binary with missing custom_metadata field
            let binary_3 = &bundle.binaries[2];
            assert_eq!(binary_3.prn, "binary_prn_3");
            assert!(binary_3.custom_metadata.is_none());
        }
        Bundle::V1(_) => panic!("Expected V2 bundle"),
    }
}

#[tokio::test]
async fn test_bundle_edge_cases() {
    // Test V1 bundle with minimal fields
    let v1_minimal_json = r#"
    {
        "artifact_versions": [],
        "organization_prn": "org_prn",
        "prn": "bundle_prn",
        "inserted_at": "2000-01-01T00:00:00Z",
        "updated_at": "2000-01-01T00:00:00Z",
        "name": null
    }
    "#;

    let bundle_v1: Bundle = serde_json::from_str(v1_minimal_json).unwrap();
    match bundle_v1 {
        Bundle::V1(bundle) => {
            assert_eq!(bundle.name, None);
            assert_eq!(bundle.artifact_versions.len(), 0);
            assert_eq!(bundle.organization_prn, "org_prn");
        }
        Bundle::V2(_) => panic!("Expected V1 bundle"),
    }

    // Test V2 bundle with empty binaries
    let v2_empty_json = r#"
    {
        "binaries": [],
        "organization_prn": "org_prn",
        "prn": "bundle_prn",
        "inserted_at": "2000-01-01T00:00:00Z",
        "updated_at": "2000-01-01T00:00:00Z",
        "name": null
    }
    "#;

    let bundle_v2: Bundle = serde_json::from_str(v2_empty_json).unwrap();
    match bundle_v2 {
        Bundle::V2(bundle) => {
            assert_eq!(bundle.name, None);
            assert_eq!(bundle.binaries.len(), 0);
            assert_eq!(bundle.organization_prn, "org_prn");
        }
        Bundle::V1(_) => panic!("Expected V2 bundle"),
    }

    // Test V2 bundle with missing custom_metadata field
    let v2_missing_metadata_json = r#"
    {
        "binaries": [
            {
                "prn": "binary_prn_1"
            },
            {
                "prn": "binary_prn_2",
                "custom_metadata": null
            },
            {
                "prn": "binary_prn_3",
                "custom_metadata": {
                    "key": "value"
                }
            }
        ],
        "organization_prn": "org_prn",
        "prn": "bundle_prn",
        "inserted_at": "2000-01-01T00:00:00Z",
        "updated_at": "2000-01-01T00:00:00Z",
        "name": "test_bundle"
    }
    "#;

    let bundle_v2_missing: Bundle = serde_json::from_str(v2_missing_metadata_json).unwrap();
    match bundle_v2_missing {
        Bundle::V2(bundle) => {
            assert_eq!(bundle.binaries.len(), 3);

            // Binary with missing custom_metadata field
            let binary_1 = &bundle.binaries[0];
            assert_eq!(binary_1.prn, "binary_prn_1");
            assert!(binary_1.custom_metadata.is_none());

            // Binary with null custom_metadata
            let binary_2 = &bundle.binaries[1];
            assert_eq!(binary_2.prn, "binary_prn_2");
            assert!(binary_2.custom_metadata.is_none());

            // Binary with custom_metadata map
            let binary_3 = &bundle.binaries[2];
            assert_eq!(binary_3.prn, "binary_prn_3");
            assert!(binary_3.custom_metadata.is_some());
        }
        Bundle::V1(_) => panic!("Expected V2 bundle"),
    }
}

#[tokio::test]
async fn test_custom_metadata_scenarios() {
    // Test JSON with all three scenarios for custom_metadata
    let json_with_all_scenarios = r#"
    {
        "binaries": [
            {
                "prn": "binary_missing_metadata"
            },
            {
                "prn": "binary_null_metadata",
                "custom_metadata": null
            },
            {
                "prn": "binary_with_metadata",
                "custom_metadata": {
                    "version": "1.0.0",
                    "arch": "arm64"
                }
            }
        ],
        "organization_prn": "org_prn",
        "prn": "bundle_prn",
        "inserted_at": "2000-01-01T00:00:00Z",
        "updated_at": "2000-01-01T00:00:00Z",
        "name": "test_bundle"
    }
    "#;

    let bundle: Bundle = serde_json::from_str(json_with_all_scenarios).unwrap();

    match bundle {
        Bundle::V2(bundle) => {
            assert_eq!(bundle.binaries.len(), 3);

            // Missing custom_metadata field should be None
            assert!(bundle.binaries[0].custom_metadata.is_none());

            // Null custom_metadata should be None
            assert!(bundle.binaries[1].custom_metadata.is_none());

            // Map custom_metadata should be Some
            assert!(bundle.binaries[2].custom_metadata.is_some());
            if let Some(metadata) = &bundle.binaries[2].custom_metadata {
                assert_eq!(metadata.len(), 2);
                assert!(metadata.contains_key("version"));
                assert!(metadata.contains_key("arch"));
            }
        }
        Bundle::V1(_) => panic!("Expected V2 bundle"),
    }
}

#[tokio::test]
async fn test_create_bundle_params_v1() {
    let params = CreateBundleParams::V1(CreateBundleParamsV1 {
        artifact_version_prns: vec!["prn1".to_string(), "prn2".to_string()],
        id: Some("test_id".to_string()),
        name: Some("test_bundle".to_string()),
    });

    let json = serde_json::to_string(&params).unwrap();
    assert!(json.contains("artifact_version_prns"));
    assert!(json.contains("test_id"));
    assert!(json.contains("test_bundle"));
    assert!(!json.contains("binaries"));
}

#[tokio::test]
async fn test_create_bundle_params_v2() {
    let mut custom_metadata = Map::new();
    custom_metadata.insert("version".to_string(), Value::String("1.0.0".to_string()));

    let params = CreateBundleParams::V2(CreateBundleParamsV2 {
        binaries: vec![CreateBundleBinary {
            prn: "binary_prn".to_string(),
            custom_metadata: Some(custom_metadata),
        }],
        id: Some("test_id".to_string()),
        name: Some("test_v2_bundle".to_string()),
    });

    let json = serde_json::to_string(&params).unwrap();
    assert!(json.contains("binaries"));
    assert!(json.contains("binary_prn"));
    assert!(json.contains("test_id"));
    assert!(json.contains("test_v2_bundle"));
    assert!(!json.contains("artifact_version_prns"));
}

#[tokio::test]
async fn test_list_bundles_v1() {
    let mut server = Server::new_async().await;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("GET", "/bundles")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "bundles": [
                {
                    "artifact_versions": ["artifact_version_prn_1"],
                    "organization_prn": "organization_prn",
                    "prn": "bundle_prn_1",
                    "inserted_at": "2000-01-01T00:00:00Z",
                    "updated_at": "2000-01-01T00:00:00Z",
                    "name": "bundle_1"
                }
            ],
            "next_page": null
        }"#,
        )
        .create_async()
        .await;

    use peridio_sdk::api::bundles::ListBundlesParams;
    use peridio_sdk::list_params::ListParams;

    let params = ListBundlesParams {
        list: ListParams::default(),
    };

    match api.bundles().list(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.bundles.len(), 1);
            match &response.bundles[0] {
                Bundle::V1(bundle) => {
                    assert_eq!(bundle.name, Some("bundle_1".to_string()));
                    assert_eq!(bundle.artifact_versions.len(), 1);
                }
                Bundle::V2(_) => panic!("Expected V1 bundle"),
            }
            assert!(response.next_page.is_none());
        }
        None => panic!("Expected Some(response)"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn test_list_bundles_v2() {
    let mut server = Server::new_async().await;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 2,
    });

    let m = server
        .mock("GET", "/bundles")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "bundles": [
                {
                    "binaries": [
                        {
                            "prn": "binary_prn_1",
                            "custom_metadata": {
                                "version": "1.0.0"
                            }
                        }
                    ],
                    "organization_prn": "organization_prn",
                    "prn": "bundle_prn_1",
                    "inserted_at": "2000-01-01T00:00:00Z",
                    "updated_at": "2000-01-01T00:00:00Z",
                    "name": "v2_bundle_1"
                }
            ],
            "next_page": null
        }"#,
        )
        .create_async()
        .await;

    use peridio_sdk::api::bundles::ListBundlesParams;
    use peridio_sdk::list_params::ListParams;

    let params = ListBundlesParams {
        list: ListParams::default(),
    };

    match api.bundles().list(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.bundles.len(), 1);
            match &response.bundles[0] {
                Bundle::V2(bundle) => {
                    assert_eq!(bundle.name, Some("v2_bundle_1".to_string()));
                    assert_eq!(bundle.binaries.len(), 1);
                    assert_eq!(bundle.binaries[0].prn, "binary_prn_1");
                }
                Bundle::V1(_) => panic!("Expected V2 bundle"),
            }
            assert!(response.next_page.is_none());
        }
        None => panic!("Expected Some(response)"),
    }

    m.assert_async().await;
}
