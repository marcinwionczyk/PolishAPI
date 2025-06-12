use reqwest::{Client, RequestBuilder};
use std::time::Duration;
use url::Url;

use crate::types::{PolishApiError, Result, RequestHeaders};
use crate::crypto::JwsSigner;

/// Configuration for the PolishAPI client
#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: Url,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub timeout: Duration,
    pub user_agent: String,
}

impl Config {
    /// Create a new configuration with the given base URL
    pub fn new(base_url: &str) -> Result<Self> {
        let base_url = Url::parse(base_url)
            .map_err(|e| PolishApiError::Config(format!("Invalid base URL: {}", e)))?;

        Ok(Self {
            base_url,
            client_id: String::new(),
            client_secret: None,
            timeout: Duration::from_secs(30),
            user_agent: format!("polishapi-rust/{}", env!("CARGO_PKG_VERSION")),
        })
    }

    /// Set the client ID
    pub fn with_client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = client_id.into();
        self
    }

    /// Set the client secret
    pub fn with_client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the user agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }
}

/// Main PolishAPI client
pub struct PolishApiClient {
    config: Config,
    http_client: Client,
    jws_signer: Option<JwsSigner>,
}

impl PolishApiClient {
    /// Create a new PolishAPI client
    pub async fn new(config: Config) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(PolishApiError::Http)?;

        Ok(Self {
            config,
            http_client,
            jws_signer: None,
        })
    }

    /// Set the JWS signer for request signing
    pub fn with_jws_signer(mut self, signer: JwsSigner) -> Self {
        self.jws_signer = Some(signer);
        self
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the HTTP client
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Create a new request builder with common headers
    pub fn request_builder(&self, method: reqwest::Method, path: &str) -> Result<RequestBuilder> {
        let url = self.config.base_url.join(path)
            .map_err(|e| PolishApiError::Config(format!("Invalid path: {}", e)))?;

        let mut builder = self.http_client.request(method, url);

        // Add common headers
        builder = builder
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Accept-Encoding", "gzip, deflate")
            .header("Accept-Charset", "utf-8");

        Ok(builder)
    }

    /// Add authentication headers to a request
    pub fn add_auth_headers(&self, builder: RequestBuilder, headers: &RequestHeaders) -> RequestBuilder {
        builder
            .header("Authorization", &headers.authorization)
            .header("Accept-Language", &headers.accept_language)
            .header("X-JWS-SIGNATURE", &headers.x_jws_signature)
            .header("X-REQUEST-ID", headers.x_request_id.to_string())
    }

    /// Sign a request payload and return the JWS signature
    pub async fn sign_payload(&self, payload: &str) -> Result<String> {
        match &self.jws_signer {
            Some(signer) => signer.sign(payload).await,
            None => Err(PolishApiError::Config("JWS signer not configured".to_string())),
        }
    }

    /// Get the authorization service
    pub fn auth(&self) -> crate::auth::AuthService {
        crate::auth::AuthService::new(self)
    }

    /// Get the account information service
    pub fn accounts(&self) -> crate::accounts::AccountService {
        crate::accounts::AccountService::new(self)
    }

    /// Get the payment initiation service
    pub fn payments(&self) -> crate::payments::PaymentService {
        crate::payments::PaymentService::new(self)
    }

    /// Get the funds confirmation service
    pub fn funds(&self) -> crate::funds::FundsService {
        crate::funds::FundsService::new(self)
    }
}

