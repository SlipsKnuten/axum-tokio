use axum::{
    routing::get,
    Router,
};

use super::constants::{
    SERVER_GREETING,
    ROOT_PATH,
};

pub fn create_router() -> Router {
    Router::new()
        .route(ROOT_PATH, get(|| async { SERVER_GREETING }))
}
