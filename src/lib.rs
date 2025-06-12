//! # PolishAPI Rust Client Library
//!
//! A Rust client library for the PolishAPI - Polish Open Banking standard.
//! This library provides a type-safe, async interface for interacting with
//! Polish banking APIs that implement the PSD2 directive.
//!
//! ## Features
//!
//! - **Authorization Service (AS)**: OAuth2 authorization flow
//! - **Account Information Service (AIS)**: Account and transaction data
//! - **Payment Initiation Service (PIS)**: Payment processing
//! - **Confirmation of Availability of Funds (CAF)**: Funds verification
//! - **JWS Signature Support**: Request signing using ring cryptography
//! - **Async/Await**: Full async support with tokio
//! - **Type Safety**: Comprehensive type definitions for all API structures
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use polishapi::{PolishApiClient, Config};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::new("https://api.bank.example.com")
//!         .with_client_id("your-client-id")
//!         .with_client_secret("your-client-secret");
//!
//!     let client = PolishApiClient::new(config).await?;
//!
//!     // Use the client for API calls
//!     // let accounts = client.accounts().get_accounts().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod auth;
pub mod accounts;
pub mod payments;
pub mod funds;
pub mod client;
pub mod types;
pub mod crypto;
pub mod utils;

// Re-export main types for convenience
pub use client::{PolishApiClient, Config};
pub use types::errors::{PolishApiError, Result};

// Re-export service modules
pub use auth::AuthService;
pub use accounts::AccountService;
pub use payments::PaymentService;
pub use funds::FundsService;

