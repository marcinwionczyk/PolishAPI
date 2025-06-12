use thiserror::Error;

/// Main error type for the PolishAPI client
#[derive(Error, Debug)]
pub enum PolishApiError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Authentication error
    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    /// Authorization error
    #[error("Authorization failed: {message}")]
    Authorization { message: String },

    /// API error response
    #[error("API error {code}: {message}")]
    Api { code: String, message: String },

    /// Cryptographic error
    #[error("Cryptographic operation failed: {0}")]
    Crypto(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Network timeout
    #[error("Request timeout")]
    Timeout,

    /// Generic error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for PolishAPI operations
pub type Result<T> = std::result::Result<T, PolishApiError>;

/// API error response structure
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl From<ApiErrorResponse> for PolishApiError {
    fn from(error: ApiErrorResponse) -> Self {
        PolishApiError::Api {
            code: error.code,
            message: error.message,
        }
    }
}

