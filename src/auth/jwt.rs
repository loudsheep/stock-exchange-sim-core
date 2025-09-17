use axum::extract::FromRequestParts;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32, // user id
    pub exp: usize,   // expiration timestamp
}

pub fn create_jwt(user_id: i32, secret: &str) -> anyhow::Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .ok_or_else(|| anyhow::anyhow!("Failed to calculate expiration time"))?
        .timestamp();

    let claims = Claims {
        user_id,
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn decode_jwt(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((
                axum::http::StatusCode::UNAUTHORIZED,
                "Missing Authorization header".into(),
            ))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                axum::http::StatusCode::UNAUTHORIZED,
                "Invalid Authorization header".into(),
            ));
        }

        let token = &auth_header[7..]; // Skip "Bearer "

        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            tracing::error!("JWT_SECRET not set in environment");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            )
        })?;

        let claims = decode_jwt(token, &secret).map_err(|_| {
            (
                axum::http::StatusCode::UNAUTHORIZED,
                "Invalid or expired token".into(),
            )
        })?;

        Ok(claims)
    }
}
