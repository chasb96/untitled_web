use auth_client::axum::extractors::ClaimsUser;
use auth_client::axum::extractors::Authenticate;
use axum::http::StatusCode;
use axum::Json;
use projects_client::axum::extractors::ProjectsClient;
use projects_client::Error as ProjectsClientError;
use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    #[serde(rename = "n")]
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
    #[serde(rename = "i")]
    pub id: String,
}

pub async fn create_project(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Json(request): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<CreateProjectResponse>), StatusCode> {
    let project = projects_client
        .create_project(projects_client::create_project::CreateProjectRequest {
            name: request.name, 
            user_id: user.id 
        })
        .await;

    let project = match project {
        Ok(project) => project,
        Err(ProjectsClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response_body = CreateProjectResponse {
        id: project.id
    };

    Ok((StatusCode::CREATED, Json(response_body)))
}