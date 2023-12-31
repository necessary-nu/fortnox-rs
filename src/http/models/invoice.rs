/*
 * # Documentation   The Fortnox API is organized around REST. This means that we’ve designed it to have resource-oriented URLs and be as predictable as possible for you as developer.  It also means that we use HTTP status codes when something goes wrong and HTTP verbs understod by many API clients around the web.  We use a modified version of OAuth2 for authentication to offer a secure way for both you and our users to interact.  The API is generally built to support both XML and JSON but in this documentation all the examples will be in JSON.  We encourage you to read all the articles in the [general information section](https://developer.fortnox.se/general/)</a> first, before going forward and learning about the different resources.  This to ensure you get an understanding of some of the shared components of the API such as parameters and error handling.  ## Rate limits  The limit is 4 requests per second per access-token. This equals to a bit more than 200 requests per minute.  [Read more about this here.](https://developer.fortnox.se/general/regarding-fortnox-api-rate-limits/)  ## Query parameters  Use query parameters with the ?-character and separate parameters with the &-character.   **Example:**  GET - https://api.fortnox.se/3/invoices?accountnumberfrom=3000&accountnumberto=4000 Read more about our parameters [here](https://developer.fortnox.se/general/parameters/)   Search the documentation using the search field in the top left corner.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Invoice {
    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    pub at_url: Option<String>,
    #[serde(
        rename = "@urlTaxReductionList",
        skip_serializing_if = "Option::is_none"
    )]
    pub at_url_tax_reduction_list: Option<String>,
    #[serde(rename = "AdministrationFee", skip_serializing_if = "Option::is_none")]
    pub administration_fee: Option<f64>,
    #[serde(
        rename = "AdministrationFeeVAT",
        skip_serializing_if = "Option::is_none"
    )]
    pub administration_fee_vat: Option<f64>,
    #[serde(rename = "Address1", skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(rename = "Address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(rename = "Balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    #[serde(rename = "BasisTaxReduction", skip_serializing_if = "Option::is_none")]
    pub basis_tax_reduction: Option<f64>,
    #[serde(rename = "Booked", skip_serializing_if = "Option::is_none")]
    pub booked: Option<bool>,
    #[serde(rename = "Cancelled", skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<bool>,
    #[serde(rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "Comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "ContractReference", skip_serializing_if = "Option::is_none")]
    pub contract_reference: Option<String>,
    #[serde(
        rename = "ContributionPercent",
        skip_serializing_if = "Option::is_none"
    )]
    pub contribution_percent: Option<f64>,
    #[serde(rename = "ContributionValue", skip_serializing_if = "Option::is_none")]
    pub contribution_value: Option<f64>,
    #[serde(rename = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "CostCenter", skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "Credit", skip_serializing_if = "Option::is_none")]
    pub credit: Option<String>,
    #[serde(
        rename = "CreditInvoiceReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub credit_invoice_reference: Option<String>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "CurrencyRate", skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(rename = "CurrencyUnit", skip_serializing_if = "Option::is_none")]
    pub currency_unit: Option<i32>,
    #[serde(rename = "CustomerName", skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(rename = "CustomerNumber")]
    pub customer_number: String,
    #[serde(rename = "DeliveryAddress1", skip_serializing_if = "Option::is_none")]
    pub delivery_address1: Option<String>,
    #[serde(rename = "DeliveryAddress2", skip_serializing_if = "Option::is_none")]
    pub delivery_address2: Option<String>,
    #[serde(rename = "DeliveryCity", skip_serializing_if = "Option::is_none")]
    pub delivery_city: Option<String>,
    #[serde(rename = "DeliveryCountry", skip_serializing_if = "Option::is_none")]
    pub delivery_country: Option<String>,
    #[serde(rename = "DeliveryDate", skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<String>,
    #[serde(rename = "DeliveryName", skip_serializing_if = "Option::is_none")]
    pub delivery_name: Option<String>,
    #[serde(rename = "DeliveryZipCode", skip_serializing_if = "Option::is_none")]
    pub delivery_zip_code: Option<String>,
    #[serde(rename = "DocumentNumber", skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "EDIInformation", skip_serializing_if = "Option::is_none")]
    pub edi_information: Option<Box<crate::http::models::InvoiceEdiInformation>>,
    #[serde(rename = "EmailInformation", skip_serializing_if = "Option::is_none")]
    pub email_information: Option<Box<crate::http::models::InvoiceEmailInformation>>,
    #[serde(rename = "EUQuarterlyReport", skip_serializing_if = "Option::is_none")]
    pub eu_quarterly_report: Option<bool>,
    #[serde(
        rename = "ExternalInvoiceReference1",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_invoice_reference1: Option<String>,
    #[serde(
        rename = "ExternalInvoiceReference2",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_invoice_reference2: Option<String>,
    #[serde(rename = "Freight", skip_serializing_if = "Option::is_none")]
    pub freight: Option<f64>,
    #[serde(rename = "FreightVAT", skip_serializing_if = "Option::is_none")]
    pub freight_vat: Option<f64>,
    #[serde(rename = "Gross", skip_serializing_if = "Option::is_none")]
    pub gross: Option<f64>,
    #[serde(rename = "HouseWork", skip_serializing_if = "Option::is_none")]
    pub house_work: Option<bool>,
    #[serde(rename = "InvoiceDate", skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<String>,
    #[serde(rename = "InvoicePeriodStart", skip_serializing_if = "Option::is_none")]
    pub invoice_period_start: Option<String>,
    #[serde(rename = "InvoicePeriodEnd", skip_serializing_if = "Option::is_none")]
    pub invoice_period_end: Option<String>,
    #[serde(
        rename = "InvoicePeriodReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub invoice_period_reference: Option<String>,
    #[serde(rename = "InvoiceRows", skip_serializing_if = "Option::is_none")]
    pub invoice_rows: Option<Vec<crate::http::models::InvoiceInvoiceRow>>,
    #[serde(rename = "InvoiceType", skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<InvoiceType>,
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::http::models::InvoiceLabel>>,
    #[serde(rename = "Language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(rename = "LastRemindDate", skip_serializing_if = "Option::is_none")]
    pub last_remind_date: Option<String>,
    #[serde(rename = "Net", skip_serializing_if = "Option::is_none")]
    pub net: Option<f64>,
    #[serde(rename = "NotCompleted", skip_serializing_if = "Option::is_none")]
    pub not_completed: Option<bool>,
    #[serde(rename = "NoxFinans", skip_serializing_if = "Option::is_none")]
    pub nox_finans: Option<bool>,
    #[serde(rename = "OCR", skip_serializing_if = "Option::is_none")]
    pub ocr: Option<String>,
    #[serde(rename = "OfferReference", skip_serializing_if = "Option::is_none")]
    pub offer_reference: Option<String>,
    #[serde(rename = "OrderReference", skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<String>,
    #[serde(rename = "OrganisationNumber", skip_serializing_if = "Option::is_none")]
    pub organisation_number: Option<String>,
    #[serde(rename = "OurReference", skip_serializing_if = "Option::is_none")]
    pub our_reference: Option<String>,
    #[serde(rename = "PaymentWay", skip_serializing_if = "Option::is_none")]
    pub payment_way: Option<String>,
    #[serde(rename = "Phone1", skip_serializing_if = "Option::is_none")]
    pub phone1: Option<String>,
    #[serde(rename = "Phone2", skip_serializing_if = "Option::is_none")]
    pub phone2: Option<String>,
    #[serde(rename = "PriceList", skip_serializing_if = "Option::is_none")]
    pub price_list: Option<String>,
    #[serde(rename = "PrintTemplate", skip_serializing_if = "Option::is_none")]
    pub print_template: Option<String>,
    #[serde(rename = "Project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "WarehouseReady", skip_serializing_if = "Option::is_none")]
    pub warehouse_ready: Option<bool>,
    #[serde(rename = "OutboundDate", skip_serializing_if = "Option::is_none")]
    pub outbound_date: Option<String>,
    #[serde(rename = "Remarks", skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[serde(rename = "Reminders", skip_serializing_if = "Option::is_none")]
    pub reminders: Option<i32>,
    #[serde(rename = "RoundOff", skip_serializing_if = "Option::is_none")]
    pub round_off: Option<f64>,
    #[serde(rename = "Sent", skip_serializing_if = "Option::is_none")]
    pub sent: Option<bool>,
    #[serde(rename = "TaxReduction", skip_serializing_if = "Option::is_none")]
    pub tax_reduction: Option<i32>,
    #[serde(rename = "TermsOfDelivery", skip_serializing_if = "Option::is_none")]
    pub terms_of_delivery: Option<String>,
    #[serde(rename = "TermsOfPayment", skip_serializing_if = "Option::is_none")]
    pub terms_of_payment: Option<String>,
    #[serde(rename = "TimeBasisReference", skip_serializing_if = "Option::is_none")]
    pub time_basis_reference: Option<i32>,
    #[serde(rename = "Total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(rename = "TotalToPay", skip_serializing_if = "Option::is_none")]
    pub total_to_pay: Option<f64>,
    #[serde(rename = "TotalVAT", skip_serializing_if = "Option::is_none")]
    pub total_vat: Option<f64>,
    #[serde(rename = "VATIncluded", skip_serializing_if = "Option::is_none")]
    pub vat_included: Option<bool>,
    #[serde(rename = "VoucherNumber", skip_serializing_if = "Option::is_none")]
    pub voucher_number: Option<i32>,
    #[serde(rename = "VoucherSeries", skip_serializing_if = "Option::is_none")]
    pub voucher_series: Option<String>,
    #[serde(rename = "VoucherYear", skip_serializing_if = "Option::is_none")]
    pub voucher_year: Option<i32>,
    #[serde(rename = "WayOfDelivery", skip_serializing_if = "Option::is_none")]
    pub way_of_delivery: Option<String>,
    #[serde(rename = "YourOrderNumber", skip_serializing_if = "Option::is_none")]
    pub your_order_number: Option<String>,
    #[serde(rename = "YourReference", skip_serializing_if = "Option::is_none")]
    pub your_reference: Option<String>,
    #[serde(rename = "ZipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(rename = "AccountingMethod", skip_serializing_if = "Option::is_none")]
    pub accounting_method: Option<AccountingMethod>,
    #[serde(rename = "TaxReductionType", skip_serializing_if = "Option::is_none")]
    pub tax_reduction_type: Option<TaxReductionType>,
    #[serde(rename = "FinalPayDate", skip_serializing_if = "Option::is_none")]
    pub final_pay_date: Option<String>,
}

impl Invoice {
    pub fn new(customer_number: String) -> Invoice {
        Invoice {
            at_url: None,
            at_url_tax_reduction_list: None,
            administration_fee: None,
            administration_fee_vat: None,
            address1: None,
            address2: None,
            balance: None,
            basis_tax_reduction: None,
            booked: None,
            cancelled: None,
            city: None,
            comments: None,
            contract_reference: None,
            contribution_percent: None,
            contribution_value: None,
            country: None,
            cost_center: None,
            credit: None,
            credit_invoice_reference: None,
            currency: None,
            currency_rate: None,
            currency_unit: None,
            customer_name: None,
            customer_number,
            delivery_address1: None,
            delivery_address2: None,
            delivery_city: None,
            delivery_country: None,
            delivery_date: None,
            delivery_name: None,
            delivery_zip_code: None,
            document_number: None,
            due_date: None,
            edi_information: None,
            email_information: None,
            eu_quarterly_report: None,
            external_invoice_reference1: None,
            external_invoice_reference2: None,
            freight: None,
            freight_vat: None,
            gross: None,
            house_work: None,
            invoice_date: None,
            invoice_period_start: None,
            invoice_period_end: None,
            invoice_period_reference: None,
            invoice_rows: None,
            invoice_type: None,
            labels: None,
            language: None,
            last_remind_date: None,
            net: None,
            not_completed: None,
            nox_finans: None,
            ocr: None,
            offer_reference: None,
            order_reference: None,
            organisation_number: None,
            our_reference: None,
            payment_way: None,
            phone1: None,
            phone2: None,
            price_list: None,
            print_template: None,
            project: None,
            warehouse_ready: None,
            outbound_date: None,
            remarks: None,
            reminders: None,
            round_off: None,
            sent: None,
            tax_reduction: None,
            terms_of_delivery: None,
            terms_of_payment: None,
            time_basis_reference: None,
            total: None,
            total_to_pay: None,
            total_vat: None,
            vat_included: None,
            voucher_number: None,
            voucher_series: None,
            voucher_year: None,
            way_of_delivery: None,
            your_order_number: None,
            your_reference: None,
            zip_code: None,
            accounting_method: None,
            tax_reduction_type: None,
            final_pay_date: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvoiceType {
    #[serde(rename = "INVOICE")]
    Invoice,
    #[serde(rename = "AGREEMENTINVOICE")]
    Agreementinvoice,
    #[serde(rename = "INTRESTINVOICE")]
    Intrestinvoice,
    #[serde(rename = "SUMMARYINVOICE")]
    Summaryinvoice,
    #[serde(rename = "CASHINVOICE")]
    Cashinvoice,
}

impl Default for InvoiceType {
    fn default() -> InvoiceType {
        Self::Invoice
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "EN")]
    En,
}

impl Default for Language {
    fn default() -> Language {
        Self::Sv
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentWay {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CARD")]
    Card,
    #[serde(rename = "AG")]
    Ag,
}

impl Default for PaymentWay {
    fn default() -> PaymentWay {
        Self::Cash
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountingMethod {
    #[serde(rename = "ACCRUAL")]
    Accrual,
    #[serde(rename = "CASH")]
    Cash,
}

impl Default for AccountingMethod {
    fn default() -> AccountingMethod {
        Self::Accrual
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxReductionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "rot")]
    Rot,
    #[serde(rename = "rut")]
    Rut,
    #[serde(rename = "green")]
    Green,
}

impl Default for TaxReductionType {
    fn default() -> TaxReductionType {
        Self::None
    }
}
