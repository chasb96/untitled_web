use axum::response::IntoResponse;
use auth_client::axum::extractors::Authenticate;
use auth_client::axum::extractors::ClaimsUser;
use projects_client::axum::extractors::ProjectsClient;
use projects_client::source_requests::SourceRequestsClient;
use axum::extract::Path;
use projects_client::source_requests::approve;
use axum::http::StatusCode;

pub async fn approve(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Path((project_id, source_request_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let approve_request = approve::ApproveRequest {
        user_id: user.id,
    };

    let approve_response = projects_client
        .approve_source_request(&project_id, &source_request_id, approve_request)
        .await;

    match approve_response {
        Ok(_) => Ok(()),
        Err(projects_client::Error::Status(StatusCode::NOT_FOUND)) => Err(StatusCode::NOT_FOUND),
        Err(projects_client::Error::Status(StatusCode::FORBIDDEN)) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}