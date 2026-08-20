use axum::{Router, routing::get};

use super::routesconstants::*;
use crate::handlers::api;
use crate::handlers::get_all_users;
use crate::handlers::hello;

pub fn create_router() -> Router {
    Router::new()
        .route(ROOT_PATH, get(hello::hello))
        .route(API_CALL, get(api::api))
        .route(GET_ALL_USERS, get(get_all_users::get_all_users))
}
