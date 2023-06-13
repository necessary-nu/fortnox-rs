/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceAccrual {
    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    pub at_url: Option<String>,
    #[serde(rename = "AccrualAccount")]
    pub accrual_account: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "InvoiceAccrualRows")]
    pub invoice_accrual_rows: Vec<crate::models::InvoiceAccrualInvoiceAccrualRow>,
    #[serde(rename = "InvoiceNumber")]
    pub invoice_number: i32,
    #[serde(rename = "Period", skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    #[serde(rename = "RevenueAccount")]
    pub revenue_account: i32,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "Times", skip_serializing_if = "Option::is_none")]
    pub times: Option<i32>,
    #[serde(rename = "Total")]
    pub total: f64,
    #[serde(rename = "VATIncluded", skip_serializing_if = "Option::is_none")]
    pub vat_included: Option<bool>,
}

impl InvoiceAccrual {
    pub fn new(
        accrual_account: i32,
        description: String,
        end_date: String,
        invoice_accrual_rows: Vec<crate::models::InvoiceAccrualInvoiceAccrualRow>,
        invoice_number: i32,
        revenue_account: i32,
        start_date: String,
        total: f64,
    ) -> InvoiceAccrual {
        InvoiceAccrual {
            at_url: None,
            accrual_account,
            description,
            end_date,
            invoice_accrual_rows,
            invoice_number,
            period: None,
            revenue_account,
            start_date,
            times: None,
            total,
            vat_included: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Period {
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "BIMONTHLY")]
    Bimonthly,
    #[serde(rename = "QUARTERLY")]
    Quarterly,
    #[serde(rename = "SEMIANNUALLY")]
    Semiannually,
    #[serde(rename = "ANNUALLY")]
    Annually,
    #[serde(rename = "1_MONTHS")]
    Variant1Months,
    #[serde(rename = "2_MONTHS")]
    Variant2Months,
    #[serde(rename = "3_MONTHS")]
    Variant3Months,
    #[serde(rename = "6_MONTHS")]
    Variant6Months,
    #[serde(rename = "12_MONTHS")]
    Variant12Months,
}

impl Default for Period {
    fn default() -> Period {
        Self::Monthly
    }
}
