use serde::Deserialize;
use auth_client::axum::extractors::Authenticate;
use auth_client::axum::extractors::ClaimsUser;
use axum::Json;
use axum::http::StatusCode;
use axum::extract::Path;
use serde::Serialize;
use projects_client::axum::extractors::ProjectsClient;
use projects_client::threads::ThreadsClient;

#[derive(Deserialize)]
pub struct CreateThreadRequest {
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "c")]
    pub comment: String,
}

#[derive(Serialize)]
pub struct CreateThreadResponse {
    #[serde(rename  = "i")]
    pub id: String,
}

pub async fn create_thread(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Path(project_id): Path<String>,
    Json(request): Json<CreateThreadRequest>,
) -> Result<(StatusCode, Json<CreateThreadResponse>), StatusCode> {
    let create_thread_response = projects_client
        .create_thread(
            &project_id, 
            projects_client::threads::create_thread::CreateThreadRequest {
                title: request.title,
                user_id: user.id.clone(),
            }
        )
        .await;

    let create_thread_response = match create_thread_response {
        Ok(create_thread_response) => create_thread_response,
        Err(projects_client::Error::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let create_comment_response = projects_client
        .create_comment(
            &project_id, 
            &create_thread_response.id, 
            projects_client::threads::create_comment::CreateCommentRequest {
                content: request.comment,
                user_id: user.id,
            }
        )
        .await;

    let response_body = match create_comment_response {
        Ok(_) => CreateThreadResponse {
            id: create_thread_response.id
        },
        Err(projects_client::Error::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok((StatusCode::CREATED, Json(response_body)))
}