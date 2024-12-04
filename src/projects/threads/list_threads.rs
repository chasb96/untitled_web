use axum::extract::Path;
use serde::Serialize;
use axum::http::StatusCode;
use axum::Json;
use projects_client::threads::ThreadsClient;
use projects_client::axum::extractors::ProjectsClient;
use std::collections::HashSet;
use users_client::axum::extractors::UsersClient;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ThreadList {
    #[serde(rename = "t")]
    pub threads: Vec<Thread>
}

#[derive(Serialize)]
pub struct Thread {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "o")]
    pub owner: Option<ThreadOwner>,
}

#[derive(Serialize)]
pub struct ThreadOwner {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "u")]
    pub username: String,
}

pub async fn list_threads(
    projects_client: ProjectsClient,
    users_client: UsersClient,
    Path(project_id): Path<String>
) -> Result<Json<ThreadList>, StatusCode> {
    let list_threads_response = projects_client
        .list_threads(&project_id)
        .await;

    let list_threads_response = match list_threads_response {
        Ok(list_threads_response) => list_threads_response,
        Err(projects_client::Error::Status(StatusCode::NOT_FOUND)) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user_ids = list_threads_response
        .threads
        .iter()
        .map(|thread| thread.user_id.clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    let list_users_response = users_client
        .list_users(user_ids)
        .await;

    let users = match list_users_response {
        Ok(list_users_response) => list_users_response
            .users
            .into_iter()
            .map(|user| (user.id, user.username))
            .collect::<HashMap<String, String>>(),
        Err(users_client::Error::Status(StatusCode::NOT_FOUND)) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let threads = ThreadList {
        threads: list_threads_response
            .threads
            .into_iter()
            .map(|thread| Thread {
                id: thread.id,
                title: thread.title,
                owner: users
                    .get(&thread.user_id)
                    .map(|username| ThreadOwner {
                        id: thread.user_id,
                        username: username.clone(),
                    })
            })
            .collect()
    };

    Ok(Json(threads))
}