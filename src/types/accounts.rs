use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::{
    BaseRequest, Amount, Balance, TransactionStatus,
    ConsentStatus, AccountReference, RemittanceInformation, Links
};

/// Account information
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub resource_id: String,
    pub iban: Option<String>,
    pub bban: Option<String>,
    pub pan: Option<String>,
    pub masked_pan: Option<String>,
    pub msisdn: Option<String>,
    pub currency: String,
    pub name: Option<String>,
    pub product: Option<String>,
    pub cash_account_type: Option<String>,
    pub status: Option<String>,
    pub bic: Option<String>,
    pub linked_accounts: Option<String>,
    pub usage: Option<String>,
    pub details: Option<String>,
    pub balances: Option<Vec<Balance>>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Get accounts request
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountsRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub with_balance: Option<bool>,
}

/// Get accounts response
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountsResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub accounts: Vec<Account>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Get account request
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub account_id: String,
    pub with_balance: Option<bool>,
}

/// Get account response
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub account: Account,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Transaction information
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: Option<String>,
    pub entry_reference: Option<String>,
    pub end_to_end_id: Option<String>,
    pub mandate_id: Option<String>,
    pub check_id: Option<String>,
    pub creditor_id: Option<String>,
    pub booking_date: Option<NaiveDate>,
    pub value_date: Option<NaiveDate>,
    pub transaction_amount: Amount,
    pub currency_exchange: Option<Vec<CurrencyExchange>>,
    pub creditor_name: Option<String>,
    pub creditor_account: Option<AccountReference>,
    pub creditor_agent: Option<String>,
    pub ultimate_creditor: Option<String>,
    pub debtor_name: Option<String>,
    pub debtor_account: Option<AccountReference>,
    pub debtor_agent: Option<String>,
    pub ultimate_debtor: Option<String>,
    pub remittance_information_unstructured: Option<String>,
    pub remittance_information_structured: Option<RemittanceInformation>,
    pub additional_information: Option<String>,
    pub purpose_code: Option<String>,
    pub bank_transaction_code: Option<String>,
    pub proprietary_bank_transaction_code: Option<String>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Currency exchange information
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyExchange {
    pub source_currency: String,
    pub target_currency: String,
    pub exchange_rate: String,
    pub unit_currency: Option<String>,
    pub contract_identification: Option<String>,
    pub quotation_date: Option<NaiveDate>,
}

/// Get transactions request
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionsRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub account_id: String,
    pub booking_status: Option<TransactionStatus>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub entry_reference_from: Option<String>,
    pub entry_reference_to: Option<String>,
    pub delta_list: Option<bool>,
}

/// Get transactions response
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionsResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub account: AccountReference,
    pub transactions: TransactionList,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Transaction list container
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionList {
    pub booked: Option<Vec<Transaction>>,
    pub pending: Option<Vec<Transaction>>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Get transaction detail request
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionDetailRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub account_id: String,
    pub transaction_id: String,
}

/// Get transaction detail response
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionDetailResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub transactions_details: Transaction,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Account holds information
#[derive(Debug, Serialize, Deserialize)]
pub struct Hold {
    pub hold_id: String,
    pub hold_amount: Amount,
    pub hold_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub additional_information: Option<String>,
}

/// Get holds request
#[derive(Debug, Serialize, Deserialize)]
pub struct GetHoldsRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub account_id: String,
}

/// Get holds response
#[derive(Debug, Serialize, Deserialize)]
pub struct GetHoldsResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub account: AccountReference,
    pub holds: Vec<Hold>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Delete consent request
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteConsentRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub consent_id: String,
}

/// Delete consent response
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteConsentResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub consent_status: ConsentStatus,
}

