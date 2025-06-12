use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common request headers for PolishAPI
#[derive(Debug, Clone)]
pub struct RequestHeaders {
    pub authorization: String,
    pub accept_encoding: String,
    pub accept_language: String,
    pub accept_charset: String,
    pub x_jws_signature: String,
    pub x_request_id: Uuid,
}

impl Default for RequestHeaders {
    fn default() -> Self {
        Self {
            authorization: String::new(),
            accept_encoding: "gzip, deflate".to_string(),
            accept_language: "en-US".to_string(),
            accept_charset: "utf-8".to_string(),
            x_jws_signature: String::new(),
            x_request_id: Uuid::new_v4(),
        }
    }
}

/// Base request structure with common fields
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
}

/// Base response structure with common fields
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
}

/// Account identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountId {
    pub iban: Option<String>,
    pub bban: Option<String>,
    pub pan: Option<String>,
    pub masked_pan: Option<String>,
    pub msisdn: Option<String>,
}

/// Amount with currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub currency: String,
    pub amount: String,
}

/// Address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street_name: Option<String>,
    pub building_number: Option<String>,
    pub town_name: Option<String>,
    pub post_code: Option<String>,
    pub country: String,
}

/// Transaction status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatus {
    Booked,
    Pending,
    Rejected,
    Cancelled,
    Scheduled,
}

/// Payment status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PaymentStatus {
    Received,
    Pending,
    Accepted,
    AcceptedCustomerProfile,
    AcceptedTechnicalValidation,
    AcceptedWithChange,
    Rejected,
    Cancelled,
    Executed,
}

/// Consent status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConsentStatus {
    Received,
    Rejected,
    Valid,
    RevokedByPsy,
    Expired,
    TerminatedByTPP,
}

/// Frequency code for recurring transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrequencyCode {
    Daily,
    Weekly,
    EveryTwoWeeks,
    Monthly,
    EveryTwoMonths,
    Quarterly,
    SemiAnnual,
    Annual,
}

/// Balance type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BalanceType {
    ClosingBooked,
    Expected,
    Authorised,
    OpeningBooked,
    InterimAvailable,
    InterimBooked,
    ForwardAvailable,
    NonInvoiced,
}

/// Balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub balance_amount: Amount,
    pub balance_type: BalanceType,
    pub credit_limit_included: Option<bool>,
    pub last_change_date_time: Option<DateTime<Utc>>,
    pub reference_date: Option<chrono::NaiveDate>,
    pub last_committed_transaction: Option<String>,
}

/// Creditor/Debtor account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountReference {
    pub iban: Option<String>,
    pub bban: Option<String>,
    pub pan: Option<String>,
    pub masked_pan: Option<String>,
    pub msisdn: Option<String>,
    pub currency: Option<String>,
}

/// Remittance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation {
    pub unstructured: Option<Vec<String>>,
    pub structured: Option<Vec<StructuredRemittanceInformation>>,
}

/// Structured remittance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation {
    pub reference: Option<String>,
    pub reference_type: Option<String>,
    pub reference_issuer: Option<String>,
}

/// Links for HATEOAS navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: Option<String>,
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}

