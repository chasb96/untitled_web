mod create_source_request;
mod list_source_requests;
mod get_source_request_by_id;
mod create_comment;
mod approve;
mod complete;

use axum::Router;
use axum::routing::post;
use create_source_request::create_source_request;
use list_source_requests::list_source_requests;
use get_source_request_by_id::get_source_request_by_id;
use axum::routing::get;
use create_comment::create_comment;
use approve::approve;
use complete::complete;

pub trait ProjectSourceRequestsRouter {
    fn register_project_source_requests_routes(self) -> Self;
}

impl ProjectSourceRequestsRouter for Router {
    fn register_project_source_requests_routes(self) -> Self {
        self.route("/projects/:project_id/source_requests", post(create_source_request))
            .route("/projects/:project_id/source_requests", get(list_source_requests))
            .route("/projects/:project_id/source_requests/:source_request_id", get(get_source_request_by_id))
            .route("/projects/:project_id/source_requests/:source_request_id/approve", post(approve))
            .route("/projects/:project_id/source_requests/:source_request_id/complete", post(complete))
            .route("/projects/:project_id/source_requests/:source_request_id/comments", post(create_comment))
    }
}