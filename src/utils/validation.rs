use crate::types::{PolishApiError, Result};

/// Validate IBAN format
pub fn validate_iban(iban: &str) -> Result<()> {
    // Basic IBAN validation - in production, use a proper IBAN validation library
    if iban.len() < 15 || iban.len() > 34 {
        return Err(PolishApiError::Validation(
            "IBAN length must be between 15 and 34 characters".to_string(),
        ));
    }

    if !iban.chars().all(|c| c.is_alphanumeric()) {
        return Err(PolishApiError::Validation(
            "IBAN must contain only alphanumeric characters".to_string(),
        ));
    }

    // Check country code (first 2 characters should be letters)
    let country_code = &iban[0..2];
    if !country_code.chars().all(|c| c.is_alphabetic()) {
        return Err(PolishApiError::Validation(
            "IBAN country code must be alphabetic".to_string(),
        ));
    }

    Ok(())
}

/// Validate currency code (ISO 4217)
pub fn validate_currency_code(currency: &str) -> Result<()> {
    if currency.len() != 3 {
        return Err(PolishApiError::Validation(
            "Currency code must be exactly 3 characters".to_string(),
        ));
    }

    if !currency.chars().all(|c| c.is_alphabetic() && c.is_uppercase()) {
        return Err(PolishApiError::Validation(
            "Currency code must be uppercase alphabetic characters".to_string(),
        ));
    }

    Ok(())
}

/// Validate amount format
pub fn validate_amount(amount: &str) -> Result<()> {
    if amount.is_empty() {
        return Err(PolishApiError::Validation(
            "Amount cannot be empty".to_string(),
        ));
    }

    // Try to parse as decimal number
    match amount.parse::<f64>() {
        Ok(value) => {
            if value < 0.0 {
                return Err(PolishApiError::Validation(
                    "Amount cannot be negative".to_string(),
                ));
            }
            if value == 0.0 {
                return Err(PolishApiError::Validation(
                    "Amount cannot be zero".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(PolishApiError::Validation(
                "Amount must be a valid decimal number".to_string(),
            ));
        }
    }

    Ok(())
}

/// Validate BIC/SWIFT code
pub fn validate_bic(bic: &str) -> Result<()> {
    if bic.len() != 8 && bic.len() != 11 {
        return Err(PolishApiError::Validation(
            "BIC code must be 8 or 11 characters long".to_string(),
        ));
    }

    if !bic.chars().all(|c| c.is_alphanumeric() && c.is_uppercase()) {
        return Err(PolishApiError::Validation(
            "BIC code must be uppercase alphanumeric characters".to_string(),
        ));
    }

    Ok(())
}

/// Validate email address format
pub fn validate_email(email: &str) -> Result<()> {
    if !email.contains('@') {
        return Err(PolishApiError::Validation(
            "Email must contain @ symbol".to_string(),
        ));
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(PolishApiError::Validation(
            "Email must have exactly one @ symbol".to_string(),
        ));
    }

    if parts[0].is_empty() || parts[1].is_empty() {
        return Err(PolishApiError::Validation(
            "Email local and domain parts cannot be empty".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_iban() {
        assert!(validate_iban("PL61109010140000071219812874").is_ok());
        assert!(validate_iban("DE89370400440532013000").is_ok());
        assert!(validate_iban("INVALID").is_err());
        assert!(validate_iban("").is_err());
    }

    #[test]
    fn test_validate_currency_code() {
        assert!(validate_currency_code("PLN").is_ok());
        assert!(validate_currency_code("EUR").is_ok());
        assert!(validate_currency_code("USD").is_ok());
        assert!(validate_currency_code("pln").is_err());
        assert!(validate_currency_code("EURO").is_err());
        assert!(validate_currency_code("").is_err());
    }

    #[test]
    fn test_validate_amount() {
        assert!(validate_amount("100.50").is_ok());
        assert!(validate_amount("1000").is_ok());
        assert!(validate_amount("0.01").is_ok());
        assert!(validate_amount("-100").is_err());
        assert!(validate_amount("0").is_err());
        assert!(validate_amount("").is_err());
        assert!(validate_amount("invalid").is_err());
    }

    #[test]
    fn test_validate_bic() {
        assert!(validate_bic("BREXPLPW").is_ok());  // MBank
        assert!(validate_bic("BREXPLPWXXX").is_ok());   //MBank
        assert!(validate_bic("INVALID").is_err());
        assert!(validate_bic("").is_err());
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("user@domain.org").is_ok());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("").is_err());
    }
}

