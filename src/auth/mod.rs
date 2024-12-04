mod login;
mod sign_up;

use axum::routing::post;
use axum::Router;
use login::login;
use sign_up::sign_up;

pub trait AuthRouter {
    fn register_auth_routes(self) -> Self;
}

impl AuthRouter for Router {
    fn register_auth_routes(self) -> Self {
        self.route("/sign_up", post(sign_up))
            .route("/login", post(login))
    }
}