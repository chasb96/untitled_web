use axum::Json;
use axum::http::StatusCode;
use axum::extract::Path;
use projects_client::axum::extractors::ProjectsClient;
use tokio::join;
use users_client::axum::extractors::UsersClient;
use users_client::Error as UsersClientError;
use projects_client::Error as ProjectsClientError;

use super::User;
use super::UserProject;
use super::message_queue::UserViewed;
use super::axum::extractors::message_queue::MessageQueueExtractor;
use projects_client::list_projects::ListProjectsQuery;

pub async fn get_user_by_id(
    users_client: UsersClient,
    projects_client: ProjectsClient,
    message_queue: MessageQueueExtractor,
    Path(user_id): Path<String>
) -> Result<Json<User>, StatusCode> {
    let (user, user_projects) = join!(
        users_client.get_user_by_id(user_id.clone()),
        projects_client.list_projects(ListProjectsQuery::Users { user_id: user_id.clone(), })
    );

    let user = match user {
        Ok(user) => user,
        Err(UsersClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user_projects =  match user_projects {
        Ok(projects) => projects,
        Err(ProjectsClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    message_queue
        .send(UserViewed {
            id: user_id,
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