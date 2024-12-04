use axum::Router;
use axum::routing::post;

mod list_threads;
mod create_thread;
mod get_thread_by_id;

use list_threads::list_threads;
use create_thread::create_thread;
use get_thread_by_id::get_thread_by_id;
use axum::routing::get;

pub trait ProjectThreadsRouter {
    fn register_project_threads_routes(self) -> Self;
}

impl ProjectThreadsRouter for Router {
    fn register_project_threads_routes(self) -> Self {
        self.route("/projects/:project_id/threads", get(list_threads))
            .route("/projects/:project_id/threads", post(create_thread))   
            .route("/projects/:project_id/threads/:thread_id", get(get_thread_by_id)) 
    }
}