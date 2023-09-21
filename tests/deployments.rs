mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::deployments::{
    CreateDeploymentParams, DeleteDeploymentParams, DeploymentCondition, GetDeploymentParams,
    ListDeploymentParams, UpdateDeployment, UpdateDeploymentCondition, UpdateDeploymentParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_deployment() {
    let organization_name = "org-1";

    let expected_conditions = DeploymentCondition {
        tags: vec!["tag-1".to_string(), "tag-2".to_string()],
        version: Some("1.0.0".to_string()),
    };
    let expected_firmware = "4dd9ff49-ec74-45fc-8558-7f98839019ec";
    let expected_is_active = false;
    let expected_name = "a";
    let expected_product = "pro-1";
    let expected_state = "off";
    let expected_delta_updatable = true;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "POST",
        &*format!("/orgs/{organization_name}/products/{expected_product}/deployments"),
    )
    .with_status(201)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/deployments-create-201.json")
    .create();

    let params = CreateDeploymentParams {
        product_name: expected_product.to_string(),
        conditions: &expected_conditions,
        firmware: expected_firmware.to_string(),
        name: expected_name.to_string(),
        organization_name: organization_name.to_string(),
        is_active: true, // it must default to false
        delta_updatable: expected_delta_updatable,
    };

    match api.deployments().create(params).await.unwrap() {
        Some(deployment) => {
            assert_eq!(deployment.data.conditions.tags, expected_conditions.tags);
            assert_eq!(deployment.data.firmware_uuid, expected_firmware);
            assert_eq!(deployment.data.is_active, expected_is_active);
            assert_eq!(deployment.data.name, expected_name);
            assert_eq!(deployment.data.state, expected_state);
            assert_eq!(deployment.data.delta_updatable, expected_delta_updatable);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn delete_deployment() {
    let organization_name = "org-1";
    let product_name = "test";
    let deployment_name = "a";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "DELETE",
        &*format!(
            "/orgs/{organization_name}/products/{product_name}/deployments/{deployment_name}"
        ),
    )
    .with_status(204)
    .with_body("")
    .create();

    let params = DeleteDeploymentParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        deployment_name: deployment_name.to_string(),
    };

    match api.deployments().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn get_deployment() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let deployment_name = "a";

    let expected_conditions = DeploymentCondition {
        tags: vec!["tag-1".to_string(), "tag-2".to_string()],
        version: Some("1.0.0".to_string()),
    };
    let expected_firmware = "4dd9ff49-ec74-45fc-8558-7f98839019ec";
    let expected_is_active = false;
    let expected_name = "a";
    let expected_state = "off";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "GET",
        &*format!(
            "/orgs/{organization_name}/products/{product_name}/deployments/{deployment_name}"
        ),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/deployments-get-200.json")
    .create();

    let params = GetDeploymentParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        deployment_name: deployment_name.to_string(),
    };

    match api.deployments().get(params).await.unwrap() {
        Some(deployment) => {
            assert_eq!(deployment.data.conditions.tags, expected_conditions.tags);
            assert_eq!(deployment.data.firmware_uuid, expected_firmware);
            assert_eq!(deployment.data.is_active, expected_is_active);
            assert_eq!(deployment.data.name, expected_name);
            assert_eq!(deployment.data.state, expected_state);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn list_deployments() {
    let organization_name = "org-1";
    let product_name = "pro-1";

    let expected_condition_0 = DeploymentCondition {
        tags: vec!["tag-1".to_string(), "tag-2".to_string()],
        version: Some("1.0.0".to_string()),
    };
    let expected_firmware_uuid_0 = "4dd9ff49-ec74-45fc-8558-7f98839019ec";
    let expected_name_0 = "a";

    let expected_condition_1 = DeploymentCondition {
        tags: vec!["tag-3".to_string(), "tag-4".to_string()],
        version: Some("2.0.0".to_string()),
    };
    let expected_firmware_uuid_1 = "1634a3c6-00d2-4cfe-8e2a-d7d1473c940a";
    let expected_name_1 = "b";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "GET",
        &*format!("/orgs/{organization_name}/products/{product_name}/deployments"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/deployments-list-200.json")
    .create();

    let params = ListDeploymentParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
    };

    match api.deployments().list(params).await.unwrap() {
        Some(deployments) => {
            assert_eq!(deployments.data.len(), 2);

            assert_eq!(
                deployments.data[0].conditions.tags,
                expected_condition_0.tags
            );
            assert_eq!(deployments.data[0].firmware_uuid, expected_firmware_uuid_0);
            assert_eq!(deployments.data[0].name, expected_name_0);

            assert_eq!(
                deployments.data[1].conditions.tags,
                expected_condition_1.tags
            );
            assert_eq!(deployments.data[1].firmware_uuid, expected_firmware_uuid_1);
            assert_eq!(deployments.data[1].name, expected_name_1);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn update_deployment() {
    let organization_name = "org-1";
    let deployment_name = "a";

    let expected_conditions = UpdateDeploymentCondition {
        tags: Some(vec!["tag-3".to_string(), "tag-4".to_string()]),
        version: Some("2.0.0".to_string()),
    };
    let expected_firmware = "4dd9ff49-ec74-45fc-8558-7f98839019ec";
    let expected_is_active = true;
    let expected_name = "a";
    let expected_product = "pro-1";
    let expected_state = "off";
    let expected_delta_updatable = false;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "PUT",
        &*format!(
            "/orgs/{organization_name}/products/{expected_product}/deployments/{deployment_name}"
        ),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/deployments-update-200.json")
    .create();

    let params = UpdateDeploymentParams {
        product_name: expected_product.to_string(),
        deployment_name: deployment_name.to_string(),
        deployment: UpdateDeployment {
            name: None,
            firmware: None,
            conditions: Some(&expected_conditions),
            is_active: Some(expected_is_active),
            delta_updatable: Some(expected_delta_updatable),
        },
        organization_name: organization_name.to_string(),
    };

    match api.deployments().update(params).await.unwrap() {
        Some(deployment) => {
            assert_eq!(
                deployment.data.conditions.tags,
                expected_conditions.tags.unwrap()
            );
            assert_eq!(deployment.data.firmware_uuid, expected_firmware);
            assert_eq!(deployment.data.is_active, expected_is_active);
            assert_eq!(deployment.data.name, expected_name);
            assert_eq!(deployment.data.state, expected_state);
            assert_eq!(deployment.data.delta_updatable, expected_delta_updatable);
        }
        _ => panic!(),
    }

    m.assert();
}
