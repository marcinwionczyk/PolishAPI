# PolishAPI Rust Client Library

A comprehensive Rust client library for the PolishAPI - Polish Open Banking standard implementing PSD2 directive requirements. This library provides a type-safe, async interface for interacting with Polish banking APIs.

## Features

- **Complete API Coverage**: Supports all PolishAPI v3.0 endpoints
- **Authorization Service (AS)**: OAuth2 authorization flow with external authorization tool support
- **Account Information Service (AIS)**: Account and transaction data retrieval
- **Payment Initiation Service (PIS)**: Payment processing for domestic, EEA, non-EEA, and tax payments
- **Confirmation of Availability of Funds (CAF)**: Funds verification
- **JWS Signature Support**: Request signing using ring cryptography
- **Async/Await**: Full async support with tokio
- **Type Safety**: Comprehensive type definitions for all API structures
- **Error Handling**: Detailed error types with proper error propagation
- **Validation**: Built-in validation for IBAN, currency codes, amounts, and more

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
polishapi = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use polishapi::{PolishApiClient, Config};
use polishapi::types::*;
use polishapi::utils::HeadersBuilder;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the client
    let config = Config::new("https://api.bank.example.com")?
        .with_client_id("your-client-id")
        .with_client_secret("your-client-secret")
        .with_timeout(std::time::Duration::from_secs(30));
  
    // Create the client
    let client = PolishApiClient::new(config).await?;
  
    // Set up request headers
    let headers = HeadersBuilder::new()
        .authorization("your-access-token")
        .accept_language("en-US")
        .build();
  
    // Get accounts
    let accounts_request = GetAccountsRequest {
        base: BaseRequest {
            request_id: Uuid::new_v4(),
        },
        with_balance: Some(true),
    };
  
    let accounts_response = client
        .accounts()
        .get_accounts(accounts_request, headers)
        .await?;
  
    println!("Found {} accounts", accounts_response.accounts.len());
  
    Ok(())
}
```

## Authentication

### OAuth2 Authorization Flow

```rust
use polishapi::types::*;

// Step 1: Request authorization code
let auth_request = AuthorizeRequest {
    base: BaseRequest {
        request_id: Uuid::new_v4(),
    },
    response_type: "code".to_string(),
    client_id: "your-client-id".to_string(),
    redirect_uri: "https://your-app.com/callback".to_string(),
    scope: "AIS PIS".to_string(),
    state: Some("random-state".to_string()),
    code_challenge: Some("challenge".to_string()),
    code_challenge_method: Some("S256".to_string()),
};

let auth_response = client
    .auth()
    .authorize(auth_request, headers)
    .await?;

// Step 2: Exchange authorization code for access token
let token_request = TokenRequest {
    base: BaseRequest {
        request_id: Uuid::new_v4(),
    },
    grant_type: "authorization_code".to_string(),
    code: Some("received-auth-code".to_string()),
    redirect_uri: Some("https://your-app.com/callback".to_string()),
    client_id: "your-client-id".to_string(),
    client_secret: Some("your-client-secret".to_string()),
    code_verifier: Some("verifier".to_string()),
    refresh_token: None,
};

let token_response = client
    .auth()
    .token(token_request, headers)
    .await?;

println!("Access token: {}", token_response.access_token);
```

## Account Information Service

### Getting Account List

```rust
let accounts_request = GetAccountsRequest {
    base: BaseRequest {
        request_id: Uuid::new_v4(),
    },
    with_balance: Some(true),
};

let accounts_response = client
    .accounts()
    .get_accounts(accounts_request, headers)
    .await?;

for account in accounts_response.accounts {
    println!("Account: {} ({})", account.name.unwrap_or_default(), account.iban.unwrap_or_default());
}
```

### Getting Transaction History

```rust
let transactions_request = GetTransactionsRequest {
    base: BaseRequest {
        request_id: Uuid::new_v4(),
    },
    account_id: "account-123".to_string(),
    booking_status: Some(TransactionStatus::Booked),
    date_from: Some(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
    date_to: Some(chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
    entry_reference_from: None,
    entry_reference_to: None,
    delta_list: Some(false),
};

let transactions_response = client
    .accounts()
    .get_transactions_done(transactions_request, headers)
    .await?;

if let Some(booked_transactions) = transactions_response.transactions.booked {
    for transaction in booked_transactions {
        println!("Transaction: {} {}", 
            transaction.transaction_amount.amount,
            transaction.transaction_amount.currency
        );
    }
}
```

## Payment Initiation Service

### Domestic Payment

```rust
let payment_request = DomesticPaymentRequest {
    base: BaseRequest {
        request_id: Uuid::new_v4(),
    },
    instructed_amount: Amount {
        currency: "PLN".to_string(),
        amount: "100.00".to_string(),
    },
    debtor_account: AccountReference {
        iban: Some("PL61109010140000071219812874".to_string()),
        bban: None,
        pan: None,
        masked_pan: None,
        msisdn: None,
        currency: Some("PLN".to_string()),
    },
    creditor_name: "John Doe".to_string(),
    creditor_account: AccountReference {
        iban: Some("PL27114020040000300201355387".to_string()),
        bban: None,
        pan: None,
        masked_pan: None,
        msisdn: None,
        currency: Some("PLN".to_string()),
    },
    creditor_agent: None,
    creditor_address: None,
    ultimate_creditor: None,
    debtor_name: Some("Jane Smith".to_string()),
    ultimate_debtor: None,
    remittance_information_unstructured: Some("Payment for services".to_string()),
    remittance_information_structured: None,
    requested_execution_date: Some(chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()),
    requested_execution_time: None,
};

let payment_response = client
    .payments()
    .initiate_domestic_payment(payment_request, headers)
    .await?;

println!("Payment initiated with ID: {}", payment_response.payment_id);
```

## Funds Confirmation Service

```rust
let funds_request = FundsConfirmationRequest {
    base: BaseRequest {
        request_id: Uuid::new_v4(),
    },
    card_number: None,
    account: AccountReference {
        iban: Some("PL61109010140000071219812874".to_string()),
        bban: None,
        pan: None,
        masked_pan: None,
        msisdn: None,
        currency: Some("PLN".to_string()),
    },
    payee: Some("Merchant Name".to_string()),
    instructed_amount: Amount {
        currency: "PLN".to_string(),
        amount: "50.00".to_string(),
    },
};

let funds_response = client
    .funds()
    .confirm_funds(funds_request, headers)
    .await?;

println!("Funds available: {}", funds_response.funds_available);
```

## JWS Signature Support

The library supports JWS (JSON Web Signature) for request signing as required by the PolishAPI specification:

```rust
use polishapi::crypto::JwsSigner;

// Create JWS signer from PEM-encoded private key
let private_key_pem = r#"
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC...
-----END PRIVATE KEY-----
"#;

let jws_signer = JwsSigner::from_pem(private_key_pem, "key-id-123".to_string())?;

// Add signer to client
let client = PolishApiClient::new(config)
    .await?
    .with_jws_signer(jws_signer);
```

## Error Handling

The library provides comprehensive error handling:

```rust
use polishapi::types::{PolishApiError, Result};

match client.accounts().get_accounts(request, headers).await {
    Ok(response) => {
        // Handle successful response
        println!("Got {} accounts", response.accounts.len());
    }
    Err(PolishApiError::Authentication { message }) => {
        eprintln!("Authentication failed: {}", message);
    }
    Err(PolishApiError::Api { code, message }) => {
        eprintln!("API error {}: {}", code, message);
    }
    Err(PolishApiError::Http(err)) => {
        eprintln!("HTTP error: {}", err);
    }
    Err(err) => {
        eprintln!("Other error: {}", err);
    }
}
```

## Configuration Options

```rust
use std::time::Duration;

let config = Config::new("https://api.bank.example.com")?
    .with_client_id("your-client-id")
    .with_client_secret("your-client-secret")
    .with_timeout(Duration::from_secs(60))
    .with_user_agent("MyApp/1.0");
```

## Validation

The library includes built-in validation utilities:

```rust
use polishapi::utils::validation::*;

// Validate IBAN
validate_iban("PL61109010140000071219812874")?;

// Validate currency code
validate_currency_code("PLN")?;

// Validate amount
validate_amount("100.50")?;

// Validate BIC code
validate_bic("DEUTDEFF")?;
```

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Disclaimer

This library is not officially endorsed by the Polish Bank Association or any banking institution. Use at your own risk and ensure compliance with all applicable regulations and terms of service.

## Support

For issues and questions:

- Create an issue on GitHub
- Check the documentation at [docs.rs](https://docs.rs/polishapi)
- Review the PolishAPI specification at [polishapi.org](https://polishapi.org)

## Changelog

### Version 0.1.0

- Initial release
- Support for PolishAPI v3.0
- Complete implementation of AS, AIS, PIS, and CAF services
- JWS signature support
- Comprehensive error handling and validation
