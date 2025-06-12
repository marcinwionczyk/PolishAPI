use crate::client::PolishApiClient;
use crate::types::{
    Result, RequestHeaders,
    AuthorizeRequest, AuthorizeResponse,
    EatCodeRequest, TokenRequest, TokenResponse,
    RegisterRequest, RegisterResponse,
};

/// Authorization Service implementation
pub struct AuthService<'a> {
    client: &'a PolishApiClient,
}

impl<'a> AuthService<'a> {
    /// Create a new AuthService instance
    pub fn new(client: &'a PolishApiClient) -> Self {
        Self { client }
    }

    /// Request OAuth2 authorization code
    ///
    /// # Arguments
    /// * `request` - Authorization request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Authorization response with authorization URL
    pub async fn authorize(
        &self,
        request: AuthorizeRequest,
        headers: RequestHeaders,
    ) -> Result<AuthorizeResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/auth/v3_0.1/authorize",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let auth_response: AuthorizeResponse = response.json().await?;
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "AUTH_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Request OAuth2 authorization code using External Authorization Tool
    ///
    /// # Arguments
    /// * `request` - EAT code request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Success (204) or error
    pub async fn authorize_ext(
        &self,
        request: EatCodeRequest,
        headers: RequestHeaders,
    ) -> Result<()> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/auth/v3_0.1/authorizeExt",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "AUTH_EXT_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Request OAuth2 access token
    ///
    /// # Arguments
    /// * `request` - Token request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Token response with access token
    pub async fn token(
        &self,
        request: TokenRequest,
        headers: RequestHeaders,
    ) -> Result<TokenResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/auth/v3_0.1/token",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "TOKEN_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Register a new client
    ///
    /// # Arguments
    /// * `request` - Registration request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Registration response with client credentials
    pub async fn register(
        &self,
        request: RegisterRequest,
        headers: RequestHeaders,
    ) -> Result<RegisterResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/auth/v3_0.1/register",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let register_response: RegisterResponse = response.json().await?;
            Ok(register_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "REGISTER_ERROR".to_string(),
                message: error_text,
            })
        }
    }
}

