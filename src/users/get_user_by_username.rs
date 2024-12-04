use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use projects_client::axum::extractors::ProjectsClient;
use users_client::axum::extractors::UsersClient;
use users_client::Error as UsersClientError;
use projects_client::Error as ProjectsClientError;

use super::message_queue::UserViewed;
use super::User;
use super::UserProject;
use super::axum::extractors::message_queue::MessageQueueExtractor;

pub async fn get_user_by_username(
    users_client: UsersClient,
    projects_client: ProjectsClient,
    message_queue: MessageQueueExtractor,
    Path(username): Path<String>
) -> Result<Json<User>, StatusCode> {
    let user = users_client
        .get_user_by_username(username)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(UsersClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user_projects =  projects_client
        .list_projects(projects_client::list_projects::ListProjectsQuery::Users {
            user_id: user.id.clone(),
         })
        .await;

    let user_projects =  match user_projects {
        Ok(projects) => projects,
        Err(ProjectsClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    message_queue
        .send(UserViewed {
            id: user.id.clone(),
        })
        .await;

    Ok(Json(User {
        id: user.id,
        username: user.username,
        projects: user_projects.projects
            .into_iter()
            .map(|project| UserProject {
                id: project.id,
                name: project.name,
            })
            .collect(),
    }))
}