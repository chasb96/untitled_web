use auth_client::axum::extractors::ClaimsUser;
use auth_client::axum::extractors::Authenticate;
use axum::http::StatusCode;
use axum::extract::Path;
use axum::Json;
use projects_client::axum::extractors::ProjectsClient;
use serde::Deserialize;
use projects_client::tags::TagsClient;

#[derive(Deserialize)]
pub struct CreateTagRequest {
    #[serde(rename = "t")]
    pub tag: String,
}

pub async fn create_tag(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Path(project_id): Path<String>,
    Json(request): Json<CreateTagRequest>,
) -> Result<StatusCode, StatusCode> {
    let project = projects_client
        .get_project_by_id(&project_id)
        .await;

    let project = match project {
        Ok(project) => project,
        Err(projects_client::Error::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = projects_client
        .create_tag(&project.id, projects_client::tags::create_tag::CreateTagRequest {
            tag: request.tag,
        })
        .await;

    match result {
        Ok(_) => Ok(StatusCode::ACCEPTED),
        Err(projects_client::Error::Status(status_code)) => Err(status_code),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}