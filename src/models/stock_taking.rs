/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StockTaking {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "responsible")]
    pub responsible: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<crate::models::StockTakingRow>>,
    #[serde(rename = "sortingId", skip_serializing_if = "Option::is_none")]
    pub sorting_id: Option<i32>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "costCenterCode", skip_serializing_if = "Option::is_none")]
    pub cost_center_code: Option<String>,
    #[serde(rename = "usingStockPoints", skip_serializing_if = "Option::is_none")]
    pub using_stock_points: Option<bool>,
}

impl StockTaking {
    pub fn new(name: String, responsible: String, state: String) -> StockTaking {
        StockTaking {
            id: None,
            date: None,
            name,
            responsible,
            state,
            rows: None,
            sorting_id: None,
            project_id: None,
            cost_center_code: None,
            using_stock_points: None,
        }
    }
}