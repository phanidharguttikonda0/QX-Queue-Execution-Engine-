use std::collections::VecDeque;
use axum::extract::State;
use axum::Form;
use axum::response::IntoResponse;
use crate::AppState;
use crate::models::queue::Message;

pub async fn create_queue(State(state): State<AppState>,Form(queue_name): Form<String>) -> impl IntoResponse {
    tracing::info!("creating queue with name as {}", queue_name) ;
    if state.queues.read().await.contains_key(&queue_name) {
        "Queue already exists"
    }else {
        state.queues.write().await.insert(queue_name, VecDeque::new());
        // over here we are going to store the new data to the disk as well, when server goes down we are going to recover the data from the disk
        "Successfully created queue"
    }
}

pub async fn add_message(State(state): State<AppState>,Form(message): Form<Message>) -> impl IntoResponse {
    tracing::info!("adding message to the particular queue as follows {}", message.value) ;

    if state.queues.read().await.contains_key(&message.name) {
        state.queues.write().await.get_mut(&message.name).unwrap().push_back((message.value, 0));
        // over here we are going to store the new data to the disk as well, when server goes down we are going to recover the data from the disk
        "Successfully added message"
    }else {
        "Queue does not exist"
    }
}