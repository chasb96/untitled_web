mod axum;
mod message_queue;
mod get_project_by_id;
mod create_project;
mod create_event;
mod tags;
mod threads;
mod source_requests;

use ::axum::routing::get;
use ::axum::routing::post;
use ::axum::routing::put;
use ::axum::Router;
use get_project_by_id::get_project_by_id;
use create_project::create_project;
use create_event::create_event;
use serde::Serialize;
use tags::ProjectTagsRouter;
use threads::ProjectThreadsRouter;
use source_requests::ProjectSourceRequestsRouter;

#[derive(Serialize)]
pub struct Project {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "o")]
    pub owner: ProjectUser,
    #[serde(rename = "e")]
    pub event_id: String,
    #[serde(rename = "f")]
    pub files: Vec<ProjectFile>,
    #[serde(rename = "t")]
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct ProjectUser {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "u")]
    pub username: String,
}

#[derive(Serialize)]
pub struct ProjectFile {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "n")]
    pub name: String,
}

pub trait ProjectsRouter {
    fn register_projects_routes(self) -> Self;
}

impl ProjectsRouter for Router {
    fn register_projects_routes(self) -> Self {
        self.route("/projects", post(create_project))
            .route("/projects/:project_id", get(get_project_by_id))
            .route("/projects/:project_id", put(create_event))
            .register_project_tags_routes()
            .register_project_threads_routes()
            .register_project_source_requests_routes()
    }
}