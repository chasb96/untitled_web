mod create;

use axum::routing::post;
use axum::Router;
use create::create;

pub trait ProjectSourceRequestCommentsRouter {
    fn register_project_source_request_comments_routes(self) -> Self;
}

impl ProjectSourceRequestCommentsRouter for Router {
    fn register_project_source_request_comments_routes(self) -> Self {
        self.route("/projects/:project_id/source_requests/:source_request_id/comments", post(create))
    }
}