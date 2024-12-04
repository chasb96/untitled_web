mod popular;

use axum::routing::get;
use axum::Router;
use popular::popular;

pub trait IndexRouter {
    fn register_index_routes(self) -> Self;
}

impl IndexRouter for Router {
    fn register_index_routes(self) -> Self {
        self.route("/popular", get(popular))
    }
}