use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    static ref JWT_SECRET: Vec<u8> = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "nova_secret_key_change_in_production".to_string())
        .into_bytes();
}
const TOKEN_EXPIRY_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user email
    pub user_id: i64,     // user id
    pub exp: i64,         // expiry timestamp
}

/// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let valid = verify(password, hash)?;
    Ok(valid)
}

/// Generate a JWT token for a user
pub fn generate_token(user_id: i64, email: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(TOKEN_EXPIRY_HOURS))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: email.to_owned(),
        user_id,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&JWT_SECRET),
    )?;

    Ok(token)
}

/// Validate a JWT token and extract claims
pub fn validate_token(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&JWT_SECRET),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "test123";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong", &hash).unwrap());
    }

    #[test]
    fn test_jwt_token() {
        let token = generate_token(1, "test@example.com").unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.user_id, 1);
        assert_eq!(claims.sub, "test@example.com");
    }
}
