use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::BaseRequest;

/// OAuth2 authorization request
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizeRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
}

/// OAuth2 authorization response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizeResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub authorization_url: String,
    pub state: Option<String>,
}

/// External authorization tool code request
#[derive(Debug, Serialize, Deserialize)]
pub struct EatCodeRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub client_id: String,
    pub eat_code: String,
    pub redirect_uri: String,
}

/// OAuth2 token request
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub code_verifier: Option<String>,
    pub refresh_token: Option<String>,
}

/// OAuth2 token response
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: String,
}

/// Registration request
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    #[serde(flatten)]
    pub base: BaseRequest,
    pub client_name: String,
    pub client_uri: Option<String>,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub response_types: Vec<String>,
    pub scope: String,
    pub token_endpoint_auth_method: String,
}

/// Registration response
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub client_id_issued_at: Option<DateTime<Utc>>,
    pub client_secret_expires_at: Option<DateTime<Utc>>,
}

