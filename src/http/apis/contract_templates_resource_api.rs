/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::http::apis::ResponseContent;

/// struct for passing parameters to the method [`create_contract_templates_resource`]
#[derive(Clone, Debug, Default)]
pub struct CreateContractTemplatesResourceParams {
    /// contract template to create
    pub contract_template: Option<crate::http::models::ContractTemplateWrap>,
}

/// struct for passing parameters to the method [`get_contract_templates_resource`]
#[derive(Clone, Debug, Default)]
pub struct GetContractTemplatesResourceParams {
    /// identifies the contract template
    pub template_number: i32,
}

/// struct for passing parameters to the method [`update_contract_templates_resource`]
#[derive(Clone, Debug, Default)]
pub struct UpdateContractTemplatesResourceParams {
    /// identifies the contract template
    pub template_number: i32,
    /// contract template to update
    pub contract_template: Option<crate::http::models::ContractTemplateWrap>,
}

/// struct for typed errors of method [`create_contract_templates_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateContractTemplatesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_contract_templates_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContractTemplatesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_contract_templates_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListContractTemplatesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_contract_templates_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateContractTemplatesResourceError {
    UnknownValue(serde_json::Value),
}

pub async fn create_contract_templates_resource(
    configuration: &configuration::Configuration,
    params: CreateContractTemplatesResourceParams,
) -> Result<crate::http::models::ContractTemplateWrap, Error<CreateContractTemplatesResourceError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let contract_template = params.contract_template;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/3/contracttemplates/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&contract_template);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateContractTemplatesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_contract_templates_resource(
    configuration: &configuration::Configuration,
    params: GetContractTemplatesResourceParams,
) -> Result<crate::http::models::ContractTemplateWrap, Error<GetContractTemplatesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let template_number = params.template_number;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/contracttemplates/{TemplateNumber}",
        local_var_configuration.base_path,
        TemplateNumber = template_number
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
        let local_var_entity: Option<GetContractTemplatesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The contract template resource can return a list of records or a single record. By specifying a TemplateNumber in the URL, a single record will be returned. Not specifying a TemplateNumber will return a list of records.
pub async fn list_contract_templates_resource(
    configuration: &configuration::Configuration,
) -> Result<
    crate::http::models::ContractTemplateListItemList,
    Error<ListContractTemplatesResourceError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/3/contracttemplates/", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListContractTemplatesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_contract_templates_resource(
    configuration: &configuration::Configuration,
    params: UpdateContractTemplatesResourceParams,
) -> Result<crate::http::models::ContractTemplateWrap, Error<UpdateContractTemplatesResourceError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let template_number = params.template_number;
    let contract_template = params.contract_template;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/contracttemplates/{TemplateNumber}",
        local_var_configuration.base_path,
        TemplateNumber = template_number
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&contract_template);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateContractTemplatesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}