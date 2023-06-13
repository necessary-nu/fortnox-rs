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

/// struct for passing parameters to the method [`create_financial_years_resource`]
#[derive(Clone, Debug, Default)]
pub struct CreateFinancialYearsResourceParams {
    /// to create
    pub financial_year: Option<crate::models::FinancialYearWrap>,
}

/// struct for passing parameters to the method [`get_by_date`]
#[derive(Clone, Debug, Default)]
pub struct GetByDateParams {
    /// date to filter on, for example 2020-06-30
    pub date: Option<String>,
}

/// struct for passing parameters to the method [`get_by_id`]
#[derive(Clone, Debug, Default)]
pub struct GetByIdParams {
    /// identifies the year
    pub id: i32,
}

/// struct for typed errors of method [`create_financial_years_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFinancialYearsResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_by_date`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetByDateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetByIdError {
    UnknownValue(serde_json::Value),
}

pub async fn create_financial_years_resource(
    configuration: &configuration::Configuration,
    params: CreateFinancialYearsResourceParams,
) -> Result<crate::models::FinancialYearWrap, Error<CreateFinancialYearsResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let financial_year = params.financial_year;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/3/financialyears", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&financial_year);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateFinancialYearsResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Add the query param to filter on specific date.
pub async fn get_by_date(
    configuration: &configuration::Configuration,
    params: GetByDateParams,
) -> Result<crate::models::FinancialYearWrapList, Error<GetByDateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let date = params.date;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/3/financialyears", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = date {
        local_var_req_builder =
            local_var_req_builder.query(&[("Date", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetByDateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_by_id(
    configuration: &configuration::Configuration,
    params: GetByIdParams,
) -> Result<crate::models::FinancialYearWrap, Error<GetByIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/financialyears/{Id}",
        local_var_configuration.base_path,
        Id = id
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
        let local_var_entity: Option<GetByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}