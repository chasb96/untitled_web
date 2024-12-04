use axum::Router;

use crate::auth::AuthRouter;
use crate::index::IndexRouter;
use crate::users::UserRouter;
use crate::projects::ProjectsRouter;

pub trait WebBffRouter {
    fn register_web_bff_routes(self) -> Self;
}

impl WebBffRouter for Router {
    fn register_web_bff_routes(self) -> Self {
        self.register_index_routes()
            .register_auth_routes()
            .register_users_routes()
            .register_projects_routes()
    }
}