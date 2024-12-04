use std::collections::HashMap;

use axum::http::StatusCode;
use axum::Json;
use metrics_client::axum::extractors::MetricsClient;
use projects_client::axum::extractors::ProjectsClient;
use serde::Serialize;
use tokio::join;
use users_client::axum::extractors::UsersClient;
use projects_client::list_projects::ListProjectsQuery;

#[derive(Serialize)]
pub struct PopularResponse {
    #[serde(rename = "p")]
    pub projects: Vec<Project>,
    #[serde(rename = "u")]
    pub users: Vec<User>,
}

#[derive(Serialize)]
pub struct Project {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "s")]
    pub score: u32,
}

#[derive(Serialize)]
pub struct User {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "u")]
    pub username: String,
    #[serde(rename = "s")]
    pub score: u32,
}

pub async fn popular(
    metrics_client: MetricsClient,
    projects_client: ProjectsClient,
    users_client: UsersClient,
) -> Result<Json<PopularResponse>, StatusCode> {
    let (projects_response, users_response) = join!(
        metrics_client.popular_projects(),
        metrics_client.popular_users()
    );

    let project_scores: HashMap<String, u32> = projects_response
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .projects
        .into_iter()
        .map(|project| (project.id, project.score))
        .collect();

    let user_scores: HashMap<String, u32> = users_response
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .users
        .into_iter()
        .map(|user| (user.id, user.score))
        .collect();

    let project_ids = project_scores
        .keys()
        .cloned()
        .collect();

    let user_ids = user_scores
        .keys()
        .cloned()
        .collect();

    let (projects_response, users_response) = join!(
        projects_client.list_projects(ListProjectsQuery::Projects { project_ids }),
        users_client.list_users(user_ids)
    );

    let (projects_details, users_details) = match (projects_response, users_response) {
        (Ok(projects_response), Ok(users_response)) => (projects_response.projects, users_response.users),
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut projects: Vec<Project> = projects_details
        .into_iter()
        .map(|project| {
            let score = project_scores
                .get(&project.id)
                .unwrap_or(&0);

            Project {
                id: project.id,
                name: project.name,
                score: *score
            }
        })
        .collect();

    let mut users: Vec<User> = users_details
        .into_iter()
        .map(|user| {
            let score = user_scores
                .get(&user.id)
                .unwrap_or(&0);
        
            User {
                id:user.id,
                username: user.username,
                score: *score
            }
        })
        .collect();
    
    projects.sort_by_key(|project| u32::MAX - project.score);
    users.sort_by_key(|user| u32::MAX - user.score);
    
    Ok(Json(PopularResponse {
        projects,
        users
    }))
}