/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractListItem {
    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    pub at_url: Option<String>,
    #[serde(rename = "Continuous", skip_serializing_if = "Option::is_none")]
    pub continuous: Option<bool>,
    #[serde(rename = "ContractLength", skip_serializing_if = "Option::is_none")]
    pub contract_length: Option<i32>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "CustomerName", skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(rename = "CustomerNumber")]
    pub customer_number: String,
    #[serde(rename = "DocumentNumber", skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(rename = "Invoiceinterval", skip_serializing_if = "Option::is_none")]
    pub invoiceinterval: Option<i32>,
    #[serde(rename = "InvoicesRemaining", skip_serializing_if = "Option::is_none")]
    pub invoices_remaining: Option<i32>,
    #[serde(rename = "LastInvoiceDate", skip_serializing_if = "Option::is_none")]
    pub last_invoice_date: Option<String>,
    #[serde(rename = "PeriodStart", skip_serializing_if = "Option::is_none")]
    pub period_start: Option<String>,
    #[serde(rename = "PeriodEnd")]
    pub period_end: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TemplateNumber", skip_serializing_if = "Option::is_none")]
    pub template_number: Option<i32>,
    #[serde(rename = "Total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}

impl ContractListItem {
    pub fn new(customer_number: String, period_end: String) -> ContractListItem {
        ContractListItem {
            at_url: None,
            continuous: None,
            contract_length: None,
            currency: None,
            customer_name: None,
            customer_number,
            document_number: None,
            invoiceinterval: None,
            invoices_remaining: None,
            last_invoice_date: None,
            period_start: None,
            period_end,
            status: None,
            template_number: None,
            total: None,
        }
    }
}