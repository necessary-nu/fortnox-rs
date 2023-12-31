/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceResponse {
    #[serde(rename = "@url")]
    pub at_url: String,
    #[serde(rename = "BalanceIncludeFees")]
    pub balance_include_fees: f32,
    #[serde(rename = "BalanceIncludeFeesCurrency")]
    pub balance_include_fees_currency: f32,
    #[serde(rename = "CurrentCapitalBalance")]
    pub current_capital_balance: f32,
    #[serde(rename = "CurrentCapitalBalanceCurrency")]
    pub current_capital_balance_currency: f32,
    #[serde(rename = "InvoiceDocumentURL", skip_serializing_if = "Option::is_none")]
    pub invoice_document_url: Option<String>,
    #[serde(rename = "InvoiceNumber")]
    pub invoice_number: i32,
    #[serde(rename = "NextEvent")]
    pub next_event: String,
    #[serde(rename = "NextEventDate")]
    pub next_event_date: String,
    #[serde(rename = "OCRNumber")]
    pub ocr_number: String,
    #[serde(rename = "Service")]
    pub service: String,
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[serde(rename = "Status")]
    pub status: String,
}

impl InvoiceResponse {
    pub fn new(
        at_url: String,
        balance_include_fees: f32,
        balance_include_fees_currency: f32,
        current_capital_balance: f32,
        current_capital_balance_currency: f32,
        invoice_number: i32,
        next_event: String,
        next_event_date: String,
        ocr_number: String,
        service: String,
        service_name: String,
        status: String,
    ) -> InvoiceResponse {
        InvoiceResponse {
            at_url,
            balance_include_fees,
            balance_include_fees_currency,
            current_capital_balance,
            current_capital_balance_currency,
            invoice_document_url: None,
            invoice_number,
            next_event,
            next_event_date,
            ocr_number,
            service,
            service_name,
            status,
        }
    }
}
