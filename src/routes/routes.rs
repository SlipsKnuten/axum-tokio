use axum::{
    routing::get,
    Router,
};

use crate::handlers::hello;

use super::routesconstants::ROOT_PATH;

pub fn create_router() -> Router {
    Router::new()
        .route(ROOT_PATH, get(hello::hello))
}
