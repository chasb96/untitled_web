use serde::Serialize;
use projects_client::axum::extractors::ProjectsClient;
use users_client::axum::extractors::UsersClient;
use axum::extract::Path;
use axum::http::StatusCode;
use projects_client::threads::ThreadsClient;
use axum::Json;
use std::collections::HashSet;
use tokio::join;

#[derive(Serialize)]
pub struct Thread {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "o")]
    pub owner: Option<ThreadOwner>,
    #[serde(rename = "c")]
    pub comments: Vec<Comment>,
}

#[derive(Serialize)]
pub struct ThreadOwner {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "u")]
    pub username: String,
}

#[derive(Serialize)]
pub struct Comment {
    #[serde(rename = "c")]
    pub content: String,
    #[serde(rename = "o")]
    pub owner: Option<CommentOwner>
}

#[derive(Serialize)]
pub struct CommentOwner {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "u")]
    pub username: String,
}

pub async fn get_thread_by_id(
    projects_client: ProjectsClient,
    users_client: UsersClient,
    Path((project_id, thread_id)): Path<(String, String)>
) -> Result<Json<Thread>, StatusCode> {
    let get_thread_response = projects_client
        .get_thread_by_id(&project_id, &thread_id)
        .await;

    let get_thread_response = match get_thread_response {
        Ok(get_thread_response) => get_thread_response,
        Err(projects_client::Error::Status(StatusCode::NOT_FOUND)) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user_ids = get_thread_response
        .comments
        .iter()
        .map(|comment| comment.user_id.clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    let (get_user_response, map_users_response) = join!(
        users_client.get_user_by_id(get_thread_response.user_id),
        users_client.map_users(user_ids)
    );
    
    let get_user_response = match get_user_response {
        Ok(get_user_response) => Some(get_user_response),
        Err(users_client::Error::Status(StatusCode::NOT_FOUND)) => None,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let users = match map_users_response {
        Ok(map_users_response) => map_users_response.users,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let thread = Thread {
        id: get_thread_response.id,
        title: get_thread_response.title,
        owner: get_user_response
            .map(|user| ThreadOwner {
                id: user.id,
                username: user.username,
            }),
        comments: get_thread_response
            .comments
            .into_iter()
            .map(|comment| Comment {
                content: comment.content,
                owner: users
                    .get(&comment.user_id)
                    .map(|user| CommentOwner {
                        id: comment.user_id,
                        username: user.username.clone(),
                    })
            })
            .collect()
    };

    Ok(Json(thread))
}