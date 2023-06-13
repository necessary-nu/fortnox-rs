/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Article {
    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    pub at_url: Option<String>,
    #[serde(rename = "ArticleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(rename = "Bulky", skip_serializing_if = "Option::is_none")]
    pub bulky: Option<bool>,
    #[serde(
        rename = "ConstructionAccount",
        skip_serializing_if = "Option::is_none"
    )]
    pub construction_account: Option<i32>,
    #[serde(rename = "Depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "DisposableQuantity", skip_serializing_if = "Option::is_none")]
    pub disposable_quantity: Option<f64>,
    #[serde(rename = "EAN", skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,
    #[serde(rename = "EUAccount", skip_serializing_if = "Option::is_none")]
    pub eu_account: Option<i32>,
    #[serde(rename = "EUVATAccount", skip_serializing_if = "Option::is_none")]
    pub euvat_account: Option<i32>,
    #[serde(rename = "ExportAccount", skip_serializing_if = "Option::is_none")]
    pub export_account: Option<i32>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "Housework", skip_serializing_if = "Option::is_none")]
    pub housework: Option<bool>,
    #[serde(rename = "HouseworkType", skip_serializing_if = "Option::is_none")]
    pub housework_type: Option<HouseworkType>,
    #[serde(rename = "Active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(
        rename = "ManufacturerArticleNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub manufacturer_article_number: Option<String>,
    #[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "PurchaseAccount", skip_serializing_if = "Option::is_none")]
    pub purchase_account: Option<i32>,
    #[serde(rename = "PurchasePrice", skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<f64>,
    #[serde(rename = "QuantityInStock", skip_serializing_if = "Option::is_none")]
    pub quantity_in_stock: Option<f64>,
    #[serde(rename = "ReservedQuantity", skip_serializing_if = "Option::is_none")]
    pub reserved_quantity: Option<f64>,
    #[serde(rename = "SalesAccount", skip_serializing_if = "Option::is_none")]
    pub sales_account: Option<i32>,
    #[serde(rename = "StockGoods", skip_serializing_if = "Option::is_none")]
    pub stock_goods: Option<bool>,
    #[serde(rename = "StockPlace", skip_serializing_if = "Option::is_none")]
    pub stock_place: Option<String>,
    #[serde(rename = "StockValue", skip_serializing_if = "Option::is_none")]
    pub stock_value: Option<f64>,
    #[serde(rename = "StockWarning", skip_serializing_if = "Option::is_none")]
    pub stock_warning: Option<f64>,
    #[serde(rename = "SupplierName", skip_serializing_if = "Option::is_none")]
    pub supplier_name: Option<String>,
    #[serde(rename = "SupplierNumber", skip_serializing_if = "Option::is_none")]
    pub supplier_number: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "VAT", skip_serializing_if = "Option::is_none")]
    pub vat: Option<f64>,
    #[serde(rename = "WebshopArticle", skip_serializing_if = "Option::is_none")]
    pub webshop_article: Option<bool>,
    #[serde(rename = "Weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename = "Expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(rename = "SalesPrice", skip_serializing_if = "Option::is_none")]
    pub sales_price: Option<f64>,
    #[serde(
        rename = "CostCalculationMethod",
        skip_serializing_if = "Option::is_none"
    )]
    pub cost_calculation_method: Option<String>,
    #[serde(rename = "StockAccount", skip_serializing_if = "Option::is_none")]
    pub stock_account: Option<i32>,
    #[serde(rename = "StockChangeAccount", skip_serializing_if = "Option::is_none")]
    pub stock_change_account: Option<i32>,
    #[serde(rename = "DirectCost", skip_serializing_if = "Option::is_none")]
    pub direct_cost: Option<f64>,
    #[serde(rename = "FreightCost", skip_serializing_if = "Option::is_none")]
    pub freight_cost: Option<f64>,
    #[serde(rename = "OtherCost", skip_serializing_if = "Option::is_none")]
    pub other_cost: Option<f64>,
    #[serde(rename = "DefaultStockPoint", skip_serializing_if = "Option::is_none")]
    pub default_stock_point: Option<String>,
    #[serde(
        rename = "DefaultStockLocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_stock_location: Option<String>,
}

impl Article {
    pub fn new(description: String) -> Article {
        Article {
            at_url: None,
            article_number: None,
            bulky: None,
            construction_account: None,
            depth: None,
            description,
            disposable_quantity: None,
            ean: None,
            eu_account: None,
            euvat_account: None,
            export_account: None,
            height: None,
            housework: None,
            housework_type: None,
            active: None,
            manufacturer: None,
            manufacturer_article_number: None,
            note: None,
            purchase_account: None,
            purchase_price: None,
            quantity_in_stock: None,
            reserved_quantity: None,
            sales_account: None,
            stock_goods: None,
            stock_place: None,
            stock_value: None,
            stock_warning: None,
            supplier_name: None,
            supplier_number: None,
            r#type: None,
            unit: None,
            vat: None,
            webshop_article: None,
            weight: None,
            width: None,
            expired: None,
            sales_price: None,
            cost_calculation_method: None,
            stock_account: None,
            stock_change_account: None,
            direct_cost: None,
            freight_cost: None,
            other_cost: None,
            default_stock_point: None,
            default_stock_location: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HouseworkType {
    #[serde(rename = "CONSTRUCTION")]
    Construction,
    #[serde(rename = "ELECTRICITY")]
    Electricity,
    #[serde(rename = "GLASSMETALWORK")]
    Glassmetalwork,
    #[serde(rename = "GROUNDDRAINAGEWORK")]
    Grounddrainagework,
    #[serde(rename = "MASONRY")]
    Masonry,
    #[serde(rename = "PAINTINGWALLPAPERING")]
    Paintingwallpapering,
    #[serde(rename = "HVAC")]
    Hvac,
    #[serde(rename = "CLEANING")]
    Cleaning,
    #[serde(rename = "TEXTILECLOTHING")]
    Textileclothing,
    #[serde(rename = "COOKING")]
    Cooking,
    #[serde(rename = "SNOWPLOWING")]
    Snowplowing,
    #[serde(rename = "GARDENING")]
    Gardening,
    #[serde(rename = "BABYSITTING")]
    Babysitting,
    #[serde(rename = "OTHERCARE")]
    Othercare,
    #[serde(rename = "TUTORING")]
    Tutoring,
    #[serde(rename = "OTHERCOSTS")]
    Othercosts,
}

impl Default for HouseworkType {
    fn default() -> HouseworkType {
        Self::Construction
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "STOCK")]
    Stock,
    #[serde(rename = "SERVICE")]
    Service,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Stock
    }
}
