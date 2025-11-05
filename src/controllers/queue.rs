use std::collections::VecDeque;
use std::thread::sleep;
use axum::extract::State;
use axum::{Form, Json};
use axum::http::Response;
use axum::response::IntoResponse;
use crate::{AppState, AppState2};
use crate::models::queue::{DeadLetterQueue, Message, Queue};

pub async fn create_queue(State(state): State<AppState>,Form(queue_name): Form<Queue>) -> impl IntoResponse {
    tracing::info!("creating queue with name as {}", queue_name.name) ;
    if state.queues.read().await.contains_key(&queue_name.name) {
        "Queue already exists"
    }else {
        state.queues.write().await.insert(queue_name.name, VecDeque::new());
        // over here we are going to store the new data to the disk as well, when server goes down we are going to recover the data from the disk
        "Successfully created queue"
    }
}

pub async fn add_message(State(state): State<AppState>,Form(message): Form<Message>) -> impl IntoResponse {
    tracing::info!("adding message to the particular queue as follows {}", message.message) ;

    if state.queues.read().await.contains_key(&message.queue_name) {
        state.queues.write().await.get_mut(&message.queue_name).unwrap().push_back((message.message, 0));
        // over here we are going to store the new data to the disk as well, when server goes down we are going to recover the data from the disk
        "Successfully added message"
    }else {
        "Queue does not exist"
    }
}

// bottom 2 were message-passing queues concept

pub async fn create_message_queue(State(state): State<AppState2>, Form(queue_name): Form<Queue>) {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<crate::Message>();
    {
        state.queues.write().await.insert(queue_name.name.clone(), tx.clone());
    } // here the lock gets dropped
    tokio::spawn(
        async move {
            while let Some(message) = rx.recv().await {
                tracing::info!("received message from the queue {}", message.message) ;
                // here we will write logic for execution of the message of that queue
                let mut success = true ;
                if message.message.len()%2 != 0 {
                    success = false ;
                }
                if !success  && message.retries < 5{
                    tx.send(crate::Message {
                        message: message.message,
                        retries: message.retries + 1,
                    }).unwrap() ; // here we are adding the same message back to the queue, such that it executes again
                } else if !success && message.retries >= 5 {
                    // now we are going to add the message to the dead letter queue
                    state.dead_letter_queue.write().await.push_back(Message {
                        queue_name: queue_name.name.clone(),
                        message: message.message.clone(),
                    }) ;
                    tracing::info!("message added to the dead letter queue as it failed in all retries , the message was from queue {} and the value was {}", &queue_name.name, message.message) ;
                }else{
                    tracing::info!("message executed successfully") ;
                }
                // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    );
}
pub async fn add_message_to_queue(State(state): State<AppState2>, Form(message): Form<Message>) {
    if state.queues.read().await.contains_key(&message.queue_name) {
        state.queues.read().await.get(&message.queue_name).unwrap().send(crate::Message {
            message: message.message, // we are sending the actual message
            retries: 0, // we will set by default to 0, as it reaches max retries, it will not add it to the queue again
        }).unwrap() ;
    }else {
        tracing::info!("queue does not exist") ;
    }
}

pub async fn get_dead_letter_queue(State(state): State<AppState2>) -> impl IntoResponse {
    {
        Json(DeadLetterQueue {
            dead_letter_queue: state.dead_letter_queue.read().await.clone()
        })
    }
}