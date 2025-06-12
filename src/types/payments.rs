use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::{
    BaseRequest, Amount, PaymentStatus, AccountReference,
    RemittanceInformation, Address, Links
};

/// Payment type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaymentType {
    Domestic,
    Eea,
    NonEea,
    Tax,
}

/// Payment product enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaymentProduct {
    Sepa,
    InstantSepa,
    Target2,
    CrossBorderCreditTransfer,
}

/// Domestic payment request
#[derive(Debug, Serialize, Deserialize)]
pub struct DomesticPaymentRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub instructed_amount: Amount,
    pub debtor_account: AccountReference,
    pub creditor_name: String,
    pub creditor_account: AccountReference,
    pub creditor_agent: Option<String>,
    pub creditor_address: Option<Address>,
    pub ultimate_creditor: Option<String>,
    pub debtor_name: Option<String>,
    pub ultimate_debtor: Option<String>,
    pub remittance_information_unstructured: Option<String>,
    pub remittance_information_structured: Option<RemittanceInformation>,
    pub requested_execution_date: Option<NaiveDate>,
    pub requested_execution_time: Option<DateTime<Utc>>,
}

/// EEA payment request
#[derive(Debug, Serialize, Deserialize)]
pub struct EeaPaymentRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub instructed_amount: Amount,
    pub debtor_account: AccountReference,
    pub creditor_name: String,
    pub creditor_account: AccountReference,
    pub creditor_agent: Option<String>,
    pub creditor_address: Option<Address>,
    pub ultimate_creditor: Option<String>,
    pub debtor_name: Option<String>,
    pub ultimate_debtor: Option<String>,
    pub remittance_information_unstructured: Option<String>,
    pub remittance_information_structured: Option<RemittanceInformation>,
    pub requested_execution_date: Option<NaiveDate>,
    pub requested_execution_time: Option<DateTime<Utc>>,
    pub charge_bearer: Option<String>,
    pub service_level: Option<String>,
    pub category_purpose: Option<String>,
}

/// Non-EEA payment request
#[derive(Debug, Serialize, Deserialize)]
pub struct NonEeaPaymentRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub instructed_amount: Amount,
    pub debtor_account: AccountReference,
    pub creditor_name: String,
    pub creditor_account: AccountReference,
    pub creditor_agent: Option<String>,
    pub creditor_address: Option<Address>,
    pub ultimate_creditor: Option<String>,
    pub debtor_name: Option<String>,
    pub ultimate_debtor: Option<String>,
    pub remittance_information_unstructured: Option<String>,
    pub remittance_information_structured: Option<RemittanceInformation>,
    pub requested_execution_date: Option<NaiveDate>,
    pub requested_execution_time: Option<DateTime<Utc>>,
    pub charge_bearer: Option<String>,
    pub service_level: Option<String>,
    pub category_purpose: Option<String>,
    pub exchange_rate_information: Option<ExchangeRateInformation>,
}

/// Tax payment request
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxPaymentRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub instructed_amount: Amount,
    pub debtor_account: AccountReference,
    pub creditor_name: String,
    pub creditor_account: AccountReference,
    pub creditor_agent: Option<String>,
    pub tax_identification: TaxIdentification,
    pub tax_period: Option<String>,
    pub tax_type: Option<String>,
    pub requested_execution_date: Option<NaiveDate>,
}

/// Tax identification information
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxIdentification {
    pub tax_identification_number: String,
    pub tax_identification_type: String,
    pub issuer: Option<String>,
}

/// Exchange rate information
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRateInformation {
    pub unit_currency: String,
    pub exchange_rate: Option<String>,
    pub rate_type: Option<String>,
    pub contract_identification: Option<String>,
}

/// Payment initiation response
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub transaction_status: PaymentStatus,
    pub payment_id: String,
    pub transaction_fees: Option<Amount>,
    pub currency_conversion_fee: Option<Amount>,
    pub estimated_total_amount: Option<Amount>,
    pub estimated_interbank_settlement_amount: Option<Amount>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
    pub psu_message: Option<String>,
}

/// Payment status request
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentStatusRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub payment_id: String,
}

/// Payment status response
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentStatusResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub transaction_status: PaymentStatus,
    pub funds_available: Option<bool>,
    pub psu_message: Option<String>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Payment information request
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInformationRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub payment_id: String,
}

/// Payment information response
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInformationResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub payment_data: PaymentData,
    pub transaction_status: PaymentStatus,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Payment data container
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentData {
    pub instructed_amount: Amount,
    pub debtor_account: AccountReference,
    pub creditor_name: String,
    pub creditor_account: AccountReference,
    pub creditor_agent: Option<String>,
    pub remittance_information_unstructured: Option<String>,
    pub requested_execution_date: Option<NaiveDate>,
}

