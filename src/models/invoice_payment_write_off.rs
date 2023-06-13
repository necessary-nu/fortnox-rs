/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoicePaymentWriteOff {
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "AccountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<i32>,
    #[serde(rename = "CostCenter", skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "TransactionInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_information: Option<String>,
    #[serde(rename = "Project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl InvoicePaymentWriteOff {
    pub fn new() -> InvoicePaymentWriteOff {
        InvoicePaymentWriteOff {
            amount: None,
            account_number: None,
            cost_center: None,
            currency: None,
            description: None,
            transaction_information: None,
            project: None,
        }
    }
}
