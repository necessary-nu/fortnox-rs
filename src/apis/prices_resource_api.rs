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

/// struct for passing parameters to the method [`create_prices_resource`]
#[derive(Clone, Debug, Default)]
pub struct CreatePricesResourceParams {
    /// price to create
    pub price: Option<crate::models::PriceWrap>,
}

/// struct for passing parameters to the method [`get_first_price`]
#[derive(Clone, Debug, Default)]
pub struct GetFirstPriceParams {
    /// identifies the price list
    pub price_list: String,
    /// identifies the article
    pub article_number: String,
}

/// struct for passing parameters to the method [`get_prices_resource`]
#[derive(Clone, Debug, Default)]
pub struct GetPricesResourceParams {
    /// identifies the price list
    pub price_list: String,
    /// identifies the article
    pub article_number: String,
    /// identifies from quantity
    pub from_quantity: f64,
}

/// struct for passing parameters to the method [`list_prices_resource`]
#[derive(Clone, Debug, Default)]
pub struct ListPricesResourceParams {
    /// identifies the price list of the prices
    pub price_list: String,
    /// identifies the article number of the prices
    pub article_number: String,
}

/// struct for passing parameters to the method [`remove_prices_resource`]
#[derive(Clone, Debug, Default)]
pub struct RemovePricesResourceParams {
    /// identifies the price list
    pub price_list: String,
    /// identifies the article number
    pub article_number: String,
    /// identifies the from quantity
    pub from_quantity: f64,
}

/// struct for passing parameters to the method [`update_prices_resource`]
#[derive(Clone, Debug, Default)]
pub struct UpdatePricesResourceParams {
    /// identifies the price list
    pub price_list: String,
    /// identifies the article number
    pub article_number: String,
    /// identifies the from quantity
    pub from_quantity: f64,
    /// price to update
    pub price: Option<crate::models::PriceWrap>,
}

/// struct for passing parameters to the method [`update_prices_resource1`]
#[derive(Clone, Debug, Default)]
pub struct UpdatePricesResource1Params {
    /// identifies the price list
    pub price_list: String,
    /// identifies the article number
    pub article_number: String,
    /// price to update
    pub price: Option<crate::models::PriceWrap>,
}

/// struct for typed errors of method [`create_prices_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePricesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_first_price`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFirstPriceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_prices_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPricesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_prices_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPricesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_prices_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemovePricesResourceError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_prices_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePricesResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_prices_resource1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePricesResource1Error {
    UnknownValue(serde_json::Value),
}

pub async fn create_prices_resource(
    configuration: &configuration::Configuration,
    params: CreatePricesResourceParams,
) -> Result<crate::models::PriceWrap, Error<CreatePricesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let price = params.price;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/3/prices/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&price);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreatePricesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_first_price(
    configuration: &configuration::Configuration,
    params: GetFirstPriceParams,
) -> Result<crate::models::PriceWrap, Error<GetFirstPriceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let price_list = params.price_list;
    let article_number = params.article_number;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/prices/{PriceList}/{ArticleNumber}",
        local_var_configuration.base_path,
        PriceList = crate::apis::urlencode(price_list),
        ArticleNumber = crate::apis::urlencode(article_number)
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
        let local_var_entity: Option<GetFirstPriceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_prices_resource(
    configuration: &configuration::Configuration,
    params: GetPricesResourceParams,
) -> Result<crate::models::PriceWrap, Error<GetPricesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let price_list = params.price_list;
    let article_number = params.article_number;
    let from_quantity = params.from_quantity;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/prices/{PriceList}/{ArticleNumber}/{FromQuantity}",
        local_var_configuration.base_path,
        PriceList = crate::apis::urlencode(price_list),
        ArticleNumber = crate::apis::urlencode(article_number),
        FromQuantity = from_quantity
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
        let local_var_entity: Option<GetPricesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The list contains a slimmer version of the prices. To get a full entity, use the GET with a price list, article number and from quantity.
pub async fn list_prices_resource(
    configuration: &configuration::Configuration,
    params: ListPricesResourceParams,
) -> Result<crate::models::PriceListItemList, Error<ListPricesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let price_list = params.price_list;
    let article_number = params.article_number;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/prices/sublist/{PriceList}/{ArticleNumber}",
        local_var_configuration.base_path,
        PriceList = crate::apis::urlencode(price_list),
        ArticleNumber = crate::apis::urlencode(article_number)
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
        let local_var_entity: Option<ListPricesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn remove_prices_resource(
    configuration: &configuration::Configuration,
    params: RemovePricesResourceParams,
) -> Result<(), Error<RemovePricesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let price_list = params.price_list;
    let article_number = params.article_number;
    let from_quantity = params.from_quantity;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/prices/{PriceList}/{ArticleNumber}/{FromQuantity}",
        local_var_configuration.base_path,
        PriceList = crate::apis::urlencode(price_list),
        ArticleNumber = crate::apis::urlencode(article_number),
        FromQuantity = from_quantity
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
        let local_var_entity: Option<RemovePricesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_prices_resource(
    configuration: &configuration::Configuration,
    params: UpdatePricesResourceParams,
) -> Result<crate::models::PriceWrap, Error<UpdatePricesResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let price_list = params.price_list;
    let article_number = params.article_number;
    let from_quantity = params.from_quantity;
    let price = params.price;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/prices/{PriceList}/{ArticleNumber}/{FromQuantity}",
        local_var_configuration.base_path,
        PriceList = crate::apis::urlencode(price_list),
        ArticleNumber = crate::apis::urlencode(article_number),
        FromQuantity = from_quantity
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&price);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdatePricesResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_prices_resource1(
    configuration: &configuration::Configuration,
    params: UpdatePricesResource1Params,
) -> Result<crate::models::PriceWrap, Error<UpdatePricesResource1Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let price_list = params.price_list;
    let article_number = params.article_number;
    let price = params.price;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/3/prices/{PriceList}/{ArticleNumber}",
        local_var_configuration.base_path,
        PriceList = crate::apis::urlencode(price_list),
        ArticleNumber = crate::apis::urlencode(article_number)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&price);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdatePricesResource1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}