/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoicePaymentListItem {
    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    pub at_url: Option<String>,
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "Booked", skip_serializing_if = "Option::is_none")]
    pub booked: Option<bool>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "CurrencyRate", skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(rename = "CurrencyUnit", skip_serializing_if = "Option::is_none")]
    pub currency_unit: Option<f64>,
    #[serde(rename = "InvoiceNumber")]
    pub invoice_number: i32,
    #[serde(rename = "Number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "PaymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "WriteOffExist", skip_serializing_if = "Option::is_none")]
    pub write_off_exist: Option<bool>,
}

impl InvoicePaymentListItem {
    pub fn new(invoice_number: i32) -> InvoicePaymentListItem {
        InvoicePaymentListItem {
            at_url: None,
            amount: None,
            booked: None,
            currency: None,
            currency_rate: None,
            currency_unit: None,
            invoice_number,
            number: None,
            payment_date: None,
            source: None,
            write_off_exist: None,
        }
    }
}
