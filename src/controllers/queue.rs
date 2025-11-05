use std::collections::VecDeque;
use axum::extract::State;
use axum::Form;
use axum::response::IntoResponse;
use crate::{AppState, AppState2};
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

// bottom 2 were message passing queues

pub async fn create_message_queue(State(state): State<AppState2>, Form(queue_name): Form<String>) {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<crate::Message>();
    {
        state.queues.write().await.insert(queue_name, tx.clone());
    } // here the lock gets dropped
    tokio::spawn(
        async move {
            while let Some(message) = rx.recv().await {
                tracing::info!("received message from the queue {}", message.message) ;
                // here we will write logic for execution of the message of that queue
                let success = true ;
                if !success  && message.retries < 5{
                    tx.send(crate::Message {
                        message: message.message,
                        retries: message.retries + 1,
                    }).unwrap() ; // here we are adding the same message back to the queue, such that it executes again
                }
            }
        }
    );
}
pub async fn add_message_to_queue(State(state): State<AppState2>, Form(message): Form<Message>) {
    if state.queues.read().await.contains_key(&message.name) {
        state.queues.read().await.get(&message.name).unwrap().send(crate::Message {
            message: message.value, // we are sending the actual message
            retries: 0, // we will set by default to 0, as it reaches max retries, it will not add it to the queue again
        }).unwrap() ;
    }else {
        tracing::info!("queue does not exist") ;
    }
}