use axum::{http::StatusCode, Json};
use auth_client::{axum::extractors::AuthClientExtractor, Error as AuthClientError};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignUpRequest {
    #[serde(rename = "u")]
    pub username: String,
    #[serde(rename = "p")]
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    #[serde(rename = "t")]
    pub token: String,
}

pub async fn sign_up(
    auth_client: AuthClientExtractor,
    Json(request): Json<SignUpRequest>
) -> Result<Json<SignUpResponse>, StatusCode> {
    let sign_up_response = auth_client
        .sign_up(auth_client::SignUpRequest {
            username: request.username.clone(),
            password: request.password.clone(),
        })
        .await;

    let sign_up_response = match sign_up_response {
        Ok(response) => response,
        Err(AuthClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let token_response = auth_client
        .create_token(auth_client::CreateTokenRequest {
            user_id: sign_up_response.id,
        })
        .await;

    match token_response {
        Ok(token) => Ok(Json(SignUpResponse {
            token: token.token,
        })),
        Err(AuthClientError::Status(status_code)) => Err(status_code),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}