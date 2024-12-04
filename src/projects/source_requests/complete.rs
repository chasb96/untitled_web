use axum::response::IntoResponse;
use auth_client::axum::extractors::Authenticate;
use auth_client::axum::extractors::ClaimsUser;
use projects_client::axum::extractors::ProjectsClient;
use projects_client::source_requests::SourceRequestsClient;
use axum::extract::Path;
use axum::http::StatusCode;
use projects_client::source_requests::complete;

pub async fn complete(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Path((project_id, source_request_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let complete_request = complete::CompleteRequest {
        user_id: user.id,
    };

    let approve_response = projects_client
        .complete_source_request(&project_id, &source_request_id, complete_request)
        .await;

    match approve_response {
        Ok(_) => Ok(()),
        Err(projects_client::Error::Status(StatusCode::NOT_FOUND)) => Err(StatusCode::NOT_FOUND),
        Err(projects_client::Error::Status(StatusCode::FORBIDDEN)) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}