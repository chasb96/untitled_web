use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use projects_client::axum::extractors::ProjectsClient;
use tokio::join;
use users_client::axum::extractors::UsersClient;
use users_client::Error as UsersClientError;
use projects_client::Error as ProjectsClientError;
use super::ProjectUser;
use super::ProjectFile;
use super::Project;
use super::axum::extractors::message_queue::MessageQueueExtractor;
use super::message_queue::ProjectViewed;
use projects_client::tags::TagsClient;

pub async fn get_project_by_id(
    projects_client: ProjectsClient,
    users_client: UsersClient,
    message_queue: MessageQueueExtractor,
    Path(project_id): Path<String>
) -> Result<Json<Project>, StatusCode> {
    let (project, tags) = join!(
        projects_client.get_project_by_id(&project_id),
        projects_client.list_tags(&project_id),
    );

    let project =  match project {
        Ok(project) => project,
        Err(ProjectsClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let tags = match tags {
        Ok(tags) => tags,
        Err(ProjectsClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user = users_client
        .get_user_by_id(project.user_id)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(UsersClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    message_queue
        .send(ProjectViewed {
            id: project_id
        })
        .await;

    Ok(Json(Project { 
        id: project.id, 
        name: project.name, 
        owner: ProjectUser {
            id: user.id,
            username: user.username,
        },
        event_id: project.event_id,
        tags: tags.tags,
        files: project.files
            .into_iter()
            .map(|file| ProjectFile {
                id: file.id,
                name: file.name,
            })
            .collect(), 
    }))
}