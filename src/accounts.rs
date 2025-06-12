use crate::client::PolishApiClient;
use crate::types::{
    Result, RequestHeaders,
    GetAccountsRequest, GetAccountsResponse,
    GetAccountRequest, GetAccountResponse,
    GetTransactionsRequest, GetTransactionsResponse,
    GetTransactionDetailRequest, GetTransactionDetailResponse,
    GetHoldsRequest, GetHoldsResponse,
    DeleteConsentRequest, DeleteConsentResponse,
};

/// Account Information Service implementation
pub struct AccountService<'a> {
    client: &'a PolishApiClient,
}

impl<'a> AccountService<'a> {
    /// Create a new AccountService instance
    pub fn new(client: &'a PolishApiClient) -> Self {
        Self { client }
    }

    /// Get list of accounts
    ///
    /// # Arguments
    /// * `request` - Get accounts request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// List of accounts
    pub async fn get_accounts(
        &self,
        request: GetAccountsRequest,
        headers: RequestHeaders,
    ) -> Result<GetAccountsResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/accounts/v3_0.1/getAccounts",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let accounts_response: GetAccountsResponse = response.json().await?;
            Ok(accounts_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "ACCOUNTS_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Get specific account details
    ///
    /// # Arguments
    /// * `request` - Get account request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Account details
    pub async fn get_account(
        &self,
        request: GetAccountRequest,
        headers: RequestHeaders,
    ) -> Result<GetAccountResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/accounts/v3_0.1/getAccount",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let account_response: GetAccountResponse = response.json().await?;
            Ok(account_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "ACCOUNT_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Get completed transactions
    ///
    /// # Arguments
    /// * `request` - Get transactions request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// List of completed transactions
    pub async fn get_transactions_done(
        &self,
        request: GetTransactionsRequest,
        headers: RequestHeaders,
    ) -> Result<GetTransactionsResponse> {
        self.get_transactions_by_status(request, headers, "getTransactionsDone").await
    }

    /// Get pending transactions
    ///
    /// # Arguments
    /// * `request` - Get transactions request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// List of pending transactions
    pub async fn get_transactions_pending(
        &self,
        request: GetTransactionsRequest,
        headers: RequestHeaders,
    ) -> Result<GetTransactionsResponse> {
        self.get_transactions_by_status(request, headers, "getTransactionsPending").await
    }

    /// Get rejected transactions
    ///
    /// # Arguments
    /// * `request` - Get transactions request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// List of rejected transactions
    pub async fn get_transactions_rejected(
        &self,
        request: GetTransactionsRequest,
        headers: RequestHeaders,
    ) -> Result<GetTransactionsResponse> {
        self.get_transactions_by_status(request, headers, "getTransactionsRejected").await
    }

    /// Get cancelled transactions
    ///
    /// # Arguments
    /// * `request` - Get transactions request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// List of cancelled transactions
    pub async fn get_transactions_cancelled(
        &self,
        request: GetTransactionsRequest,
        headers: RequestHeaders,
    ) -> Result<GetTransactionsResponse> {
        self.get_transactions_by_status(request, headers, "getTransactionsCancelled").await
    }

    /// Get scheduled transactions
    ///
    /// # Arguments
    /// * `request` - Get transactions request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// List of scheduled transactions
    pub async fn get_transactions_scheduled(
        &self,
        request: GetTransactionsRequest,
        headers: RequestHeaders,
    ) -> Result<GetTransactionsResponse> {
        self.get_transactions_by_status(request, headers, "getTransactionsScheduled").await
    }

    /// Get transaction details
    ///
    /// # Arguments
    /// * `request` - Get transaction detail request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Transaction details
    pub async fn get_transaction_detail(
        &self,
        request: GetTransactionDetailRequest,
        headers: RequestHeaders,
    ) -> Result<GetTransactionDetailResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/accounts/v3_0.1/getTransactionDetail",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let transaction_response: GetTransactionDetailResponse = response.json().await?;
            Ok(transaction_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "TRANSACTION_DETAIL_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Get account holds
    ///
    /// # Arguments
    /// * `request` - Get holds request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// List of account holds
    pub async fn get_holds(
        &self,
        request: GetHoldsRequest,
        headers: RequestHeaders,
    ) -> Result<GetHoldsResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/accounts/v3_0.1/getHolds",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let holds_response: GetHoldsResponse = response.json().await?;
            Ok(holds_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "HOLDS_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Delete consent
    ///
    /// # Arguments
    /// * `request` - Delete consent request parameters
    /// * `headers` - Request headers including authentication
    ///
    /// # Returns
    /// Consent deletion confirmation
    pub async fn delete_consent(
        &self,
        request: DeleteConsentRequest,
        headers: RequestHeaders,
    ) -> Result<DeleteConsentResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let builder = self.client.request_builder(
            reqwest::Method::POST,
            "/v3_0.1/accounts/v3_0.1/deleteConsent",
        )?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let consent_response: DeleteConsentResponse = response.json().await?;
            Ok(consent_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "DELETE_CONSENT_ERROR".to_string(),
                message: error_text,
            })
        }
    }

    /// Helper method to get transactions by status
    async fn get_transactions_by_status(
        &self,
        request: GetTransactionsRequest,
        headers: RequestHeaders,
        endpoint: &str,
    ) -> Result<GetTransactionsResponse> {
        let payload = serde_json::to_string(&request)?;
        let signature = self.client.sign_payload(&payload).await?;

        let mut request_headers = headers;
        request_headers.x_jws_signature = signature;

        let path = format!("/v3_0.1/accounts/v3_0.1/{}", endpoint);
        let builder = self.client.request_builder(reqwest::Method::POST, &path)?;

        let builder = self.client.add_auth_headers(builder, &request_headers);

        let response = builder
            .body(payload)
            .send()
            .await?;

        if response.status().is_success() {
            let transactions_response: GetTransactionsResponse = response.json().await?;
            Ok(transactions_response)
        } else {
            let error_text = response.text().await?;
            Err(crate::types::PolishApiError::Api {
                code: "TRANSACTIONS_ERROR".to_string(),
                message: error_text,
            })
        }
    }
}

