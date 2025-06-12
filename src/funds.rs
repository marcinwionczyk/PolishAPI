use crate::client::PolishApiClient;
use crate::types::{
    Result, RequestHeaders,
    FundsConfirmationRequest, FundsConfirmationResponse,
};

/// Funds Confirmation Service implementation
pub struct FundsService<'a> {
    client: &'a PolishApiClient,
}

impl<'a> FundsService<'a> {
    /// Create a new FundsService instance
    pub fn new(client: &'a PolishApiClient) -> Self {
        Self { client }
    }

    /// Confirm availability of funds
    ///
    /// # Arguments
    /// * `request` - Funds confirmation request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Funds confirmation response
    pub async fn confirm_funds(
        &self,
        request: FundsConfirmationRequest,
        headers: RequestHeaders,
    ) -> Result<FundsConfirmationResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/funds/v3_0.1/confirmation",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let funds_response: FundsConfirmationResponse = response.json().await?;
            Ok(funds_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "FUNDS_CONFIRMATION_ERROR".to_string(),
                message: error_text,
            })
        }
    }
}

