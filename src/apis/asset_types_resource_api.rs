/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`create_asset_types_resource`]
#[derive(Clone, Debug, Default)]
pub struct CreateAssetTypesResourceParams {
    /// id
    pub id: i32,
    /// request
    pub create_asset_request: Option<crate::models::CreateAssetWrap>,
}

/// struct for passing parameters to the method [`delete_asset_types_resource`]
#[derive(Clone, Debug, Default)]
pub struct DeleteAssetTypesResourceParams {
    /// id
    pub id: i32,
}

/// struct for passing parameters to the method [`get_asset_types_resource`]
#[derive(Clone, Debug, Default)]
pub struct GetAssetTypesResourceParams {
    /// id
    pub id: i32,
}

/// struct for passing parameters to the method [`update_asset_types_resource`]
#[derive(Clone, Debug, Default)]
pub struct UpdateAssetTypesResourceParams {
    /// id
    pub id: i32,
    /// request
    pub update_asset_request: Option<crate::models::UpdateAssetWrap>,
}

/// struct for typed errors of method [`create_asset_types_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssetTypesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_asset_types_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAssetTypesResourceError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_asset_types_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllAssetTypesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_asset_types_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAssetTypesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_asset_types_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAssetTypesResourceError {
    UnknownValue(serde_json::Value),
}

pub async fn create_asset_types_resource(
    configuration: &configuration::Configuration,
    params: CreateAssetTypesResourceParams,
) -> Result<crate::models::AssetTypeWrapSingle, Error<CreateAssetTypesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let create_asset_request = params.create_asset_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/assets/types/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_asset_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAssetTypesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_asset_types_resource(
    configuration: &configuration::Configuration,
    params: DeleteAssetTypesResourceParams,
) -> Result<(), Error<DeleteAssetTypesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/assets/types/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteAssetTypesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_all_asset_types_resource(
    configuration: &configuration::Configuration,
) -> Result<crate::models::AssetTypeWrapList, Error<GetAllAssetTypesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/3/assets/types/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllAssetTypesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_asset_types_resource(
    configuration: &configuration::Configuration,
    params: GetAssetTypesResourceParams,
) -> Result<crate::models::AssetTypeWrapSingle, Error<GetAssetTypesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/assets/types/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAssetTypesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_asset_types_resource(
    configuration: &configuration::Configuration,
    params: UpdateAssetTypesResourceParams,
) -> Result<crate::models::AssetTypeWrapSingle, Error<UpdateAssetTypesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let update_asset_request = params.update_asset_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/assets/types/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&update_asset_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateAssetTypesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
