mod create;
mod list;
mod get_by_id;
mod approve;
mod complete;
mod comments;

use axum::Router;
use axum::routing::post;
use create::create;
use list::list;
use get_by_id::get_by_id;
use axum::routing::get;
use approve::approve;
use complete::complete;
use comments::ProjectSourceRequestCommentsRouter;

pub trait ProjectSourceRequestsRouter {
    fn register_project_source_requests_routes(self) -> Self;
}

impl ProjectSourceRequestsRouter for Router {
    fn register_project_source_requests_routes(self) -> Self {
        self.route("/projects/:project_id/source_requests", post(create))
            .route("/projects/:project_id/source_requests", get(list))
            .route("/projects/:project_id/source_requests/:source_request_id", get(get_by_id))
            .route("/projects/:project_id/source_requests/:source_request_id/approve", post(approve))
            .route("/projects/:project_id/source_requests/:source_request_id/complete", post(complete))
            .register_project_source_request_comments_routes()
    }
}