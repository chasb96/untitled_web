use auth_client::axum::extractors::AuthClientExtractor;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use auth_client::Error as AuthClientError;

#[derive(Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "u")]
    pub username: String,
    #[serde(rename = "p")]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    #[serde(rename = "t")]
    pub token: String,
}

pub async fn login(
    auth_client: AuthClientExtractor,
    Json(request): Json<LoginRequest>
) -> Result<Json<LoginResponse>, StatusCode> {
    let response = auth_client
        .login(auth_client::LoginRequest {
            username: request.username,
            password: request.password,
        })
        .await;

    match response {
        Ok(token) => Ok(Json(LoginResponse {
            token: token.token,
        })),
        Err(AuthClientError::Status(status_code)) => Err(status_code),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}