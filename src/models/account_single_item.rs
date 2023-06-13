/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountSingleItem {
    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    pub at_url: Option<String>,
    #[serde(rename = "Active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        rename = "BalanceBroughtForward",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_brought_forward: Option<f64>,
    #[serde(rename = "CostCenter", skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "CostCenterSettings", skip_serializing_if = "Option::is_none")]
    pub cost_center_settings: Option<CostCenterSettings>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Number")]
    pub number: i32,
    #[serde(rename = "Project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "ProjectSettings", skip_serializing_if = "Option::is_none")]
    pub project_settings: Option<ProjectSettings>,
    #[serde(rename = "SRU", skip_serializing_if = "Option::is_none")]
    pub sru: Option<i32>,
    #[serde(rename = "Year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "VATCode", skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(
        rename = "BalanceCarriedForward",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_carried_forward: Option<f64>,
    #[serde(
        rename = "TransactionInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_information: Option<String>,
    #[serde(
        rename = "TransactionInformationSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_information_settings: Option<TransactionInformationSettings>,
    #[serde(rename = "QuantitySettings", skip_serializing_if = "Option::is_none")]
    pub quantity_settings: Option<QuantitySettings>,
    #[serde(rename = "QuantityUnit", skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<String>,
    #[serde(rename = "OpeningQuantities", skip_serializing_if = "Option::is_none")]
    pub opening_quantities: Option<Vec<crate::models::AccountSingleItemOpeningQuantities>>,
}

impl AccountSingleItem {
    pub fn new(description: String, number: i32) -> AccountSingleItem {
        AccountSingleItem {
            at_url: None,
            active: None,
            balance_brought_forward: None,
            cost_center: None,
            cost_center_settings: None,
            description,
            number,
            project: None,
            project_settings: None,
            sru: None,
            year: None,
            vat_code: None,
            balance_carried_forward: None,
            transaction_information: None,
            transaction_information_settings: None,
            quantity_settings: None,
            quantity_unit: None,
            opening_quantities: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CostCenterSettings {
    #[serde(rename = "ALLOWED")]
    Allowed,
    #[serde(rename = "MANDATORY")]
    Mandatory,
    #[serde(rename = "NOTALLOWED")]
    Notallowed,
}

impl Default for CostCenterSettings {
    fn default() -> CostCenterSettings {
        Self::Allowed
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectSettings {
    #[serde(rename = "ALLOWED")]
    Allowed,
    #[serde(rename = "MANDATORY")]
    Mandatory,
    #[serde(rename = "NOTALLOWED")]
    Notallowed,
}

impl Default for ProjectSettings {
    fn default() -> ProjectSettings {
        Self::Allowed
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionInformationSettings {
    #[serde(rename = "ALLOWED")]
    Allowed,
    #[serde(rename = "MANDATORY")]
    Mandatory,
    #[serde(rename = "NOTALLOWED")]
    Notallowed,
}

impl Default for TransactionInformationSettings {
    fn default() -> TransactionInformationSettings {
        Self::Allowed
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QuantitySettings {
    #[serde(rename = "ALLOWED")]
    Allowed,
    #[serde(rename = "MANDATORY")]
    Mandatory,
    #[serde(rename = "NOTALLOWED")]
    Notallowed,
}

impl Default for QuantitySettings {
    fn default() -> QuantitySettings {
        Self::Allowed
    }
}
