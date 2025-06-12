use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::{BaseRequest, Amount, AccountReference};

/// Confirmation of availability of funds request
#[derive(Debug, Serialize, Deserialize)]
pub struct FundsConfirmationRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub card_number: Option<String>,
    pub account: AccountReference,
    pub payee: Option<String>,
    pub instructed_amount: Amount,
}

/// Confirmation of availability of funds response
#[derive(Debug, Serialize, Deserialize)]
pub struct FundsConfirmationResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub funds_available: bool,
}

