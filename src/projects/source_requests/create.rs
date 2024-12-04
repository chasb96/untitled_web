use serde::Deserialize;
use auth_client::axum::extractors::Authenticate;
use auth_client::axum::extractors::ClaimsUser;
use axum::Json;
use axum::http::StatusCode;
use axum::extract::Path;
use projects_client::source_requests::SourceRequestsClient;
use projects_client::axum::extractors::ProjectsClient;
use serde::Serialize;
use axum::response::IntoResponse;
use projects_client::source_requests::create;

#[derive(Deserialize)]
pub struct CreateSourceRequestRequest {
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    pub description: String,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

#[derive(Deserialize)]
pub struct FileMap {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

#[derive(Serialize)]
pub struct CreateSourceRequestResponse {
    #[serde(rename = "i")]
    pub id: String,
}

pub async fn create(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Path(project_id): Path<String>,
    Json(request): Json<CreateSourceRequestRequest>
) -> impl IntoResponse {
    let create_request = create::CreateRequest {
        user_id: user.id,
        title: request.title,
        description: request.description,
        files: request
            .files
            .into_iter()
            .map(|file| create::FileMap {
                path: file.path,
                file_id: file.file_id,
            })
            .collect(),
    };

    let response = projects_client
        .create_source_request(&project_id, create_request)
        .await;

    let source_request = match response {
        Ok(project) => project,
        Err(projects_client::Error::Status(StatusCode::NOT_FOUND)) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok((
        StatusCode::CREATED,
        Json(CreateSourceRequestResponse {
            id: source_request.id,
        })
    ))
}