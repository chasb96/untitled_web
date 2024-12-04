use projects_client::tags::TagsClient;
use auth_client::axum::extractors::ClaimsUser;
use auth_client::axum::extractors::Authenticate;
use axum::extract::Path;
use axum::http::StatusCode;
use projects_client::axum::extractors::ProjectsClient;

pub async fn delete_tag(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Path((project_id, tag)): Path<(String, String)>
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
        .delete_tag(&project.id, &tag)
        .await;

    match result {
        Ok(_) => Ok(StatusCode::ACCEPTED),
        Err(projects_client::Error::Status(status_code)) => Err(status_code),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}