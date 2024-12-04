mod create_tag;
mod delete_tag;

use create_tag::create_tag;
use delete_tag::delete_tag;
use axum::Router;
use axum::routing::post;
use axum::routing::delete;

pub trait ProjectTagsRouter {
    fn register_project_tags_routes(self) -> Self;
}

impl ProjectTagsRouter for Router {
    fn register_project_tags_routes(self) -> Self {
        self.route("/projects/:project_id/tags", post(create_tag))
            .route("/projects/:project_id/tags/:tag", delete(delete_tag))
    }
}