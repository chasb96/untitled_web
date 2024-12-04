use axum::{routing::get, Router};
use routes::WebBffRouter;

mod health;
mod routes;
mod index;
mod auth;
mod users;
mod projects;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .register_web_bff_routes()
}