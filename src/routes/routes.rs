use axum::{Router, routing::get};
use sqlx::PgPool;

use super::routesconstants::*;
use crate::handlers::api;
use crate::handlers::create_user;
use crate::handlers::hello;
use crate::handlers::users;

pub fn create_router() -> Router<PgPool> {
    Router::new()
        .route(ROOT_PATH, get(hello::hello))
        .route(API_CALL, get(api::api))
        .route(
            USERS,
            get(users::get_all),
            post(create_user::create_user_handler),
        )
}
