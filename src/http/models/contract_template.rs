/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractTemplate {
    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    pub at_url: Option<String>,
    #[serde(rename = "AdministrationFee", skip_serializing_if = "Option::is_none")]
    pub administration_fee: Option<f64>,
    #[serde(rename = "ContractLength", skip_serializing_if = "Option::is_none")]
    pub contract_length: Option<i32>,
    #[serde(rename = "Freight", skip_serializing_if = "Option::is_none")]
    pub freight: Option<f64>,
    #[serde(rename = "InvoiceInterval", skip_serializing_if = "Option::is_none")]
    pub invoice_interval: Option<i32>,
    #[serde(rename = "InvoiceRows", skip_serializing_if = "Option::is_none")]
    pub invoice_rows: Option<Vec<crate::http::models::ContractTemplateInvoiceRow>>,
    #[serde(rename = "Continuous", skip_serializing_if = "Option::is_none")]
    pub continuous: Option<bool>,
    #[serde(rename = "OurReference", skip_serializing_if = "Option::is_none")]
    pub our_reference: Option<String>,
    #[serde(rename = "PrintTemplate", skip_serializing_if = "Option::is_none")]
    pub print_template: Option<String>,
    #[serde(rename = "Remarks", skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    #[serde(rename = "TemplateNumber", skip_serializing_if = "Option::is_none")]
    pub template_number: Option<i32>,
    #[serde(rename = "TermsOfDelivery", skip_serializing_if = "Option::is_none")]
    pub terms_of_delivery: Option<String>,
    #[serde(rename = "TermsOfPayment", skip_serializing_if = "Option::is_none")]
    pub terms_of_payment: Option<String>,
    #[serde(rename = "WayOfDelivery", skip_serializing_if = "Option::is_none")]
    pub way_of_delivery: Option<String>,
}

impl ContractTemplate {
    pub fn new(template_name: String) -> ContractTemplate {
        ContractTemplate {
            at_url: None,
            administration_fee: None,
            contract_length: None,
            freight: None,
            invoice_interval: None,
            invoice_rows: None,
            continuous: None,
            our_reference: None,
            print_template: None,
            remarks: None,
            template_name,
            template_number: None,
            terms_of_delivery: None,
            terms_of_payment: None,
            way_of_delivery: None,
        }
    }
}
