use axum::{
    routing::get,
    Router,
};

use crate::handlers::hello;
use crate::handlers::api;
use super::routesconstants::*;

pub fn create_router() -> Router {
    Router::new()
        .route(ROOT_PATH, get(hello::hello))
        .route(API_CALL, get(api::api))
}
