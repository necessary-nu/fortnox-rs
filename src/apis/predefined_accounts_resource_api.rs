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

/// struct for passing parameters to the method [`get_predefined_accounts_resource`]
#[derive(Clone, Debug, Default)]
pub struct GetPredefinedAccountsResourceParams {
    /// identifies the predefined account
    pub name: String,
}

/// struct for passing parameters to the method [`update_predefined_accounts_resource`]
#[derive(Clone, Debug, Default)]
pub struct UpdatePredefinedAccountsResourceParams {
    /// identifies the predefined account
    pub name: String,
    /// predefined account to update
    pub predefined_account: Option<crate::models::PredefinedAccountWrap>,
}

/// struct for typed errors of method [`get_predefined_accounts_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPredefinedAccountsResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_predefined_accounts_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPredefinedAccountsResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_predefined_accounts_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePredefinedAccountsResourceError {
    UnknownValue(serde_json::Value),
}

pub async fn get_predefined_accounts_resource(
    configuration: &configuration::Configuration,
    params: GetPredefinedAccountsResourceParams,
) -> Result<crate::models::PredefinedAccountWrap, Error<GetPredefinedAccountsResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/predefinedaccounts/{name}",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
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
        let local_var_entity: Option<GetPredefinedAccountsResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_predefined_accounts_resource(
    configuration: &configuration::Configuration,
) -> Result<crate::models::PredefinedAccountList, Error<ListPredefinedAccountsResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/predefinedaccounts/",
        local_var_configuration.base_path
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
        let local_var_entity: Option<ListPredefinedAccountsResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// An endpoint for updating a Predefined Account. Predefined Accounts are identified by their <em>name</em>-field, and as such must be unique.  Some Predefined Accounts distinguish between Goods and Services.  In this case, the former retains the original name whereas the latter ends with a 2. Such as <em>SALES</em> and <em>SALES2</em>.  Accounts are chosen from the Account Registry, and if you have EasyVat enabled then the new EasyVat Predefined Accounts (<em>SALES_25_SE</em>, etc.) have certain restrictions on the accounts that can be selected.  Refer to the table below.   <table>      <caption>Account restrictions when EasyVat has been enabled.</caption>      <tr>          <th>Name</th>          <th>VAT Code</th>          <th>Restrictions</th>      </tr>      <tr>          <td>SALES_25_SE</td>          <td>MP1</td>          <td>Must have a compatible VAT Code.</td>      </tr>      <tr>          <td>SALES_12_SE</td>          <td>MP2</td>          <td>Must have a compatible VAT Code.</td>      </tr>      <tr>          <td>SALES_6_SE</td>          <td>MP3</td>          <td>Must have a compatible VAT Code.</td>      </tr>      <tr>          <td>SALES_0_SE</td>          <td>MF</td>          <td>Must have a compatible VAT Code.</td>      </tr>  </table>   This endpoint can produce errors, some of which may only be relevant for EasyVat. Refer to the table below.  <table>      <caption>Errors that can be raised by this endpoint.</caption>      <tr>          <th>Error Code</th>          <th>HTTP Code</th>          <th>Description</th>          <th>Solution</th>      </tr>      <tr>          <td>2001265</td>          <td>400</td>          <td>The provided account is invalid. It either has not been provided, does not exist, or is inactive.</td>          <td>Verify that an account has been provided and that it exists and is active.</td>      </tr>      <tr>          <td>2002462</td>          <td>400</td>          <td>The account is not in a valid format.</td>          <td>Verify that the format of the account is correct. It has to consist of 4 digits.</td>      </tr>      <tr>          <td>2000729</td>          <td>400</td>          <td>A Predefined Account has not been provided.</td>          <td>Verify that a valid Predefined Account has been provided as a PATH-parameter.</td>      </tr>      <tr>          <td>2004052</td>          <td>400</td>          <td>The provided account has an incompatible VAT Code. Only applies if EasyVat has been enabled.</td>          <td>Verify that the provided account has a VAT Code that is compatible with the selected Predefined Account. Refer to the table above for more information about compatibility.</td>      </tr>  </table>   If you have activated EasyVat, you can read more about how to use the new Predefined Accounts with your documents in their respective api documentation.
pub async fn update_predefined_accounts_resource(
    configuration: &configuration::Configuration,
    params: UpdatePredefinedAccountsResourceParams,
) -> Result<crate::models::PredefinedAccountWrap, Error<UpdatePredefinedAccountsResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let predefined_account = params.predefined_account;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/predefinedaccounts/{name}",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&predefined_account);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdatePredefinedAccountsResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
