use axum::Router;
use axum::routing::post;
use crate::AppState;
use crate::controllers::queue::{add_message, create_queue};

pub fn queue_routes() -> Router<AppState> {
    Router::new()
        .route("/add", post(add_message))
        .route("/create", post(create_queue))
}