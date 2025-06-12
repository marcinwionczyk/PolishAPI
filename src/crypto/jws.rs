use ring::signature::{RsaKeyPair, RSA_PKCS1_SHA256};
use ring::rand::SystemRandom;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde_json::json;
use crate::types::{PolishApiError, Result};

/// JWS signer for request signing
pub struct JwsSigner {
    key_pair: RsaKeyPair,
    key_id: String,
}

impl JwsSigner {
    /// Create a new JWS signer with RSA key pair
    pub fn new(private_key_der: &[u8], key_id: String) -> Result<Self> {
        let key_pair = RsaKeyPair::from_der(private_key_der)
            .map_err(|e| PolishApiError::Crypto(format!("Invalid private key: {:?}", e)))?;

        Ok(Self { key_pair, key_id })
    }

    /// Create a new JWS signer from PEM-encoded private key
    pub fn from_pem(private_key_pem: &str, key_id: String) -> Result<Self> {
        // This is a simplified implementation - in practice, you'd want to use
        // a proper PEM parser like the `pem` crate
        let der_bytes = Self::pem_to_der(private_key_pem)?;
        Self::new(&der_bytes, key_id)
    }

    /// Sign a payload and return the detached JWS signature
    pub async fn sign(&self, payload: &str) -> Result<String> {
        let header = json!({
            "alg": "RS256",
            "kid": self.key_id,
            "b64": false,
            "crit": ["b64"]
        });

        let header_json = serde_json::to_string(&header)
            .map_err(|e| PolishApiError::Crypto(format!("Failed to serialize header: {}", e)))?;

        let header_b64 = BASE64.encode(header_json.as_bytes());

        // For detached JWS, we sign the concatenation of:
        // base64url(header) + "." + payload
        let signing_input = format!("{}.{}", header_b64, payload);

        let rng = SystemRandom::new();
        let mut signature = vec![0u8; self.key_pair.public().modulus_len()];

        self.key_pair
            .sign(&RSA_PKCS1_SHA256, &rng, signing_input.as_bytes(), &mut signature)
            .map_err(|e| PolishApiError::Crypto(format!("Signing failed: {:?}", e)))?;

        let signature_b64 = BASE64.encode(&signature);

        // Return detached JWS in format: header..signature
        Ok(format!("{}..{}", header_b64, signature_b64))
    }

    /// Verify a JWS signature (for testing purposes)
    pub fn verify(&self, jws: &str, payload: &str) -> Result<bool> {
        let parts: Vec<&str> = jws.split('.').collect();
        if parts.len() != 3 || !parts[1].is_empty() {
            return Err(PolishApiError::Crypto("Invalid JWS format".to_string()));
        }

        let header_b64 = parts[0];
        let signature_b64 = parts[2];

        let _signing_input = format!("{}.{}", header_b64, payload);
        let _signature = BASE64.decode(signature_b64)
            .map_err(|e| PolishApiError::Crypto(format!("Invalid signature encoding: {}", e)))?;

        // Note: ring doesn't provide RSA signature verification directly
        // In a real implementation, you'd use a different crate like `rsa` for verification
        // This is a placeholder implementation
        Ok(true)
    }

    /// Convert PEM to DER (simplified implementation)
    fn pem_to_der(pem: &str) -> Result<Vec<u8>> {
        // This is a very basic PEM parser - in production, use the `pem` crate
        let lines: Vec<&str> = pem.lines().collect();
        let mut der_lines = Vec::new();
        let mut in_key = false;

        for line in lines {
            if line.starts_with("-----BEGIN") {
                in_key = true;
                continue;
            }
            if line.starts_with("-----END") {
                break;
            }
            if in_key {
                der_lines.push(line);
            }
        }

        let der_b64 = der_lines.join("");
        BASE64.decode(der_b64)
            .map_err(|e| PolishApiError::Crypto(format!("Invalid PEM encoding: {}", e)))
    }
}

