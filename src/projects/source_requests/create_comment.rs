use serde::Deserialize;
use serde::Serialize;
use auth_client::axum::extractors::Authenticate;
use axum::response::IntoResponse;
use axum::extract::Path;
use axum::Json;
use auth_client::axum::extractors::ClaimsUser;
use projects_client::axum::extractors::ProjectsClient;
use projects_client::source_requests::comments::create_comment;
use projects_client::source_requests::comments::CommentsClient;
use axum::http::StatusCode;

#[derive(Deserialize)]
pub struct CreateCommentRequest {
    #[serde(rename = "c")]
    pub content: String,
}

#[derive(Serialize)]
pub struct CreateCommentResponse {
    #[serde(rename = "i")]
    pub id: String,
}

pub async fn create_comment(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Path((project_id, source_request_id)): Path<(String, String)>,
    Json(request): Json<CreateCommentRequest>
) -> impl IntoResponse {
    let create_comment_request = create_comment::CreateRequest {
        user_id: user.id,
        content: request.content,
    };
    
    let response = projects_client
        .create_comment(&project_id, &source_request_id, create_comment_request)
        .await;

    let source_request = match response {
        Ok(project) => project,
        Err(projects_client::Error::Status(StatusCode::NOT_FOUND)) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok((
        StatusCode::CREATED,
        Json(CreateCommentResponse {
            id: source_request.id,
        })
    ))
}