use uuid::Uuid;
use crate::types::{RequestHeaders, PolishApiError, Result};

/// Builder for creating request headers
pub struct HeadersBuilder {
    headers: RequestHeaders,
}

impl HeadersBuilder {
    /// Create a new headers builder
    pub fn new() -> Self {
        Self {
            headers: RequestHeaders::default(),
        }
    }

    /// Set the authorization header
    pub fn authorization(mut self, token: impl Into<String>) -> Self {
        self.headers.authorization = format!("Bearer {}", token.into());
        self
    }

    /// Set the accept language header
    pub fn accept_language(mut self, language: impl Into<String>) -> Self {
        self.headers.accept_language = language.into();
        self
    }

    /// Set the request ID
    pub fn request_id(mut self, request_id: Uuid) -> Self {
        self.headers.x_request_id = request_id;
        self
    }

    /// Build the headers
    pub fn build(self) -> RequestHeaders {
        self.headers
    }
}

impl Default for HeadersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate authorization header format
pub fn validate_authorization_header(auth_header: &str) -> Result<()> {
    if !auth_header.starts_with("Bearer ") {
        return Err(PolishApiError::Validation(
            "Authorization header must start with 'Bearer '".to_string(),
        ));
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix
    if token.is_empty() {
        return Err(PolishApiError::Validation(
            "Authorization token cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Validate request ID format
pub fn validate_request_id(request_id: &Uuid) -> Result<()> {
    // UUID validation is handled by the uuid crate
    // Additional custom validation can be added here if needed
    if request_id.is_nil() {
        return Err(PolishApiError::Validation(
            "Request ID cannot be nil".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers_builder() {
        let headers = HeadersBuilder::new()
            .authorization("test-token")
            .accept_language("en-US")
            .build();

        assert_eq!(headers.authorization, "Bearer test-token");
        assert_eq!(headers.accept_language, "en-US");
        assert_eq!(headers.accept_encoding, "gzip, deflate");
        assert_eq!(headers.accept_charset, "utf-8");
    }

    #[test]
    fn test_validate_authorization_header() {
        assert!(validate_authorization_header("Bearer valid-token").is_ok());
        assert!(validate_authorization_header("Invalid token").is_err());
        assert!(validate_authorization_header("Bearer ").is_err());
    }

    #[test]
    fn test_validate_request_id() {
        let valid_id = Uuid::new_v4();
        assert!(validate_request_id(&valid_id).is_ok());

        let nil_id = Uuid::nil();
        assert!(validate_request_id(&nil_id).is_err());
    }
}

