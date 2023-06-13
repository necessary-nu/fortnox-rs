/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SupplierInvoiceSupplierInvoiceRow {
    #[serde(rename = "Account", skip_serializing_if = "Option::is_none")]
    pub account: Option<i32>,
    #[serde(rename = "ArticleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    #[serde(rename = "CostCenter", skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "AccountDescription", skip_serializing_if = "Option::is_none")]
    pub account_description: Option<String>,
    #[serde(rename = "ItemDescription", skip_serializing_if = "Option::is_none")]
    pub item_description: Option<String>,
    #[serde(rename = "Debit", skip_serializing_if = "Option::is_none")]
    pub debit: Option<f64>,
    #[serde(rename = "DebitCurrency", skip_serializing_if = "Option::is_none")]
    pub debit_currency: Option<f64>,
    #[serde(rename = "Credit", skip_serializing_if = "Option::is_none")]
    pub credit: Option<f64>,
    #[serde(rename = "CreditCurrency", skip_serializing_if = "Option::is_none")]
    pub credit_currency: Option<f64>,
    #[serde(rename = "Project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(
        rename = "TransactionInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_information: Option<String>,
    #[serde(rename = "Price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "Quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "Total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "StockPointCode", skip_serializing_if = "Option::is_none")]
    pub stock_point_code: Option<String>,
    #[serde(rename = "StockLocationCode", skip_serializing_if = "Option::is_none")]
    pub stock_location_code: Option<String>,
}

impl SupplierInvoiceSupplierInvoiceRow {
    pub fn new() -> SupplierInvoiceSupplierInvoiceRow {
        SupplierInvoiceSupplierInvoiceRow {
            account: None,
            article_number: None,
            code: None,
            cost_center: None,
            account_description: None,
            item_description: None,
            debit: None,
            debit_currency: None,
            credit: None,
            credit_currency: None,
            project: None,
            transaction_information: None,
            price: None,
            quantity: None,
            total: None,
            unit: None,
            stock_point_code: None,
            stock_location_code: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "TOT")]
    Tot,
    #[serde(rename = "VAT")]
    Vat,
    #[serde(rename = "FRT")]
    Frt,
    #[serde(rename = "AFE")]
    Afe,
    #[serde(rename = "ROV")]
    Rov,
    #[serde(rename = "CND")]
    Cnd,
    #[serde(rename = "CNC")]
    Cnc,
    #[serde(rename = "PRD")]
    Prd,
    #[serde(rename = "PRC")]
    Prc,
    #[serde(rename = "SRD")]
    Srd,
    #[serde(rename = "SRC")]
    Src,
    #[serde(rename = "PRE")]
    Pre,
    #[serde(rename = "GWB")]
    Gwb,
}

impl Default for Code {
    fn default() -> Code {
        Self::Tot
    }
}
