use super::app_error::{AppError, StatusCode};
use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env::var;

pub async fn GeneratePassword(password: String) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub async fn ComparePassword(password: String, hashed: &str) -> bool {
    bcrypt::verify(password, hashed).unwrap()
}

pub async fn ValidatePassword(entered_password: String, hashed_password: String) -> bool {
    let is_valid = verify(entered_password, &hashed_password).unwrap();
    is_valid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub id: String,
}

pub async fn GenerateSignature(payload: TokenClaims) -> Result<String, String> {
    dotenv().ok();

    let claims = TokenClaims {
        email: payload.email,
        id: payload.id,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(var("SECRET_KEY").unwrap().as_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => return Err("Failed to generate token".to_string()),
    }
}

pub async fn ValidateSignature(token: String) -> Result<bool, AppError> {
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(var("SECRET_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        AppError::new(
            "Failed to validate token".to_string(),
            StatusCode::Unauthorized,
            "Unauthorized".to_string(),
        )
    })?
    .claims;

    Ok(true)
}

pub fn FormateData<T>(data: T) -> Result<T, AppError>  {
    match data {
        data => Ok(data),
        _ => Err(AppError::new(
            "Data not found".to_string(),
            StatusCode::NotFound,
            "Not Found".to_string(),
        )),
    }
}