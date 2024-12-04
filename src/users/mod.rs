mod message_queue;
mod axum;
mod get_user_by_id;
mod get_user_by_username;

use ::axum::routing::get;
use ::axum::Router;
use get_user_by_id::get_user_by_id;
use get_user_by_username::get_user_by_username;

use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    #[serde(rename = "i")]
    id: String,
    #[serde(rename = "u")]
    username: String,
    #[serde(rename = "p")]
    projects: Vec<UserProject>,
}

#[derive(Serialize)]
pub struct UserProject {
    #[serde(rename = "i")]
    id: String,
    #[serde(rename = "n")]
    name: String,
}

pub trait UserRouter {
    fn register_users_routes(self) -> Self;
}

impl UserRouter for Router {
    fn register_users_routes(self) -> Self {
        self.route("/users/:user_id", get(get_user_by_id))
            .route("/users/@:username", get(get_user_by_username))
    }
}