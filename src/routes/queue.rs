use axum::Router;
use axum::routing::{post, get};
use crate::{AppState, AppState2};
use crate::controllers::queue::{add_message, add_message_to_queue, create_message_queue, create_queue, get_dead_letter_queue};

pub fn queue_routes() -> Router<AppState> {
    Router::new()
        .route("/add", post(add_message))
        .route("/create", post(create_queue))
}

pub fn message_queues_routes() -> Router<AppState2> {
    Router::new().route("/add", post(add_message_to_queue))
        .route("/create", post(create_message_queue))
        .route("/get-dead-letter", get(get_dead_letter_queue))
}