use crate::client::PolishApiClient;
use crate::types::{
    Result, RequestHeaders,
    DomesticPaymentRequest, EeaPaymentRequest, NonEeaPaymentRequest, TaxPaymentRequest,
    PaymentInitiationResponse, PaymentStatusRequest, PaymentStatusResponse,
    PaymentInformationRequest, PaymentInformationResponse,
};

/// Payment Initiation Service implementation
pub struct PaymentService<'a> {
    client: &'a PolishApiClient,
}

impl<'a> PaymentService<'a> {
    /// Create a new PaymentService instance
    pub fn new(client: &'a PolishApiClient) -> Self {
        Self { client }
    }

    /// Initiate a domestic payment
    ///
    /// # Arguments
    /// * `request` - Domestic payment request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Payment initiation response
    pub async fn initiate_domestic_payment(
        &self,
        request: DomesticPaymentRequest,
        headers: RequestHeaders,
    ) -> Result<PaymentInitiationResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/payments/v3_0.1/domestic",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let payment_response: PaymentInitiationResponse = response.json().await?;
            Ok(payment_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "DOMESTIC_PAYMENT_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Initiate an EEA payment
    ///
    /// # Arguments
    /// * `request` - EEA payment request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Payment initiation response
    pub async fn initiate_eea_payment(
        &self,
        request: EeaPaymentRequest,
        headers: RequestHeaders,
    ) -> Result<PaymentInitiationResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/payments/v3_0.1/EEA",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let payment_response: PaymentInitiationResponse = response.json().await?;
            Ok(payment_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "EEA_PAYMENT_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Initiate a non-EEA payment
    ///
    /// # Arguments
    /// * `request` - Non-EEA payment request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Payment initiation response
    pub async fn initiate_non_eea_payment(
        &self,
        request: NonEeaPaymentRequest,
        headers: RequestHeaders,
    ) -> Result<PaymentInitiationResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/payments/v3_0.1/nonEEA",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let payment_response: PaymentInitiationResponse = response.json().await?;
            Ok(payment_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "NON_EEA_PAYMENT_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Initiate a tax payment
    ///
    /// # Arguments
    /// * `request` - Tax payment request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Payment initiation response
    pub async fn initiate_tax_payment(
        &self,
        request: TaxPaymentRequest,
        headers: RequestHeaders,
    ) -> Result<PaymentInitiationResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/payments/v3_0.1/tax",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let payment_response: PaymentInitiationResponse = response.json().await?;
            Ok(payment_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "TAX_PAYMENT_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Get payment status
    ///
    /// # Arguments
    /// * `request` - Payment status request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Payment status response
    pub async fn get_payment_status(
        &self,
        request: PaymentStatusRequest,
        headers: RequestHeaders,
    ) -> Result<PaymentStatusResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/payments/v3_0.1/status",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let status_response: PaymentStatusResponse = response.json().await?;
            Ok(status_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "PAYMENT_STATUS_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Get payment information
    ///
    /// # Arguments
    /// * `request` - Payment information request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Payment information response
    pub async fn get_payment_information(
        &self,
        request: PaymentInformationRequest,
        headers: RequestHeaders,
    ) -> Result<PaymentInformationResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/payments/v3_0.1/information",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let info_response: PaymentInformationResponse = response.json().await?;
            Ok(info_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "PAYMENT_INFO_ERROR".to_string(),
                message: error_text,
            })
        }
    }
}

