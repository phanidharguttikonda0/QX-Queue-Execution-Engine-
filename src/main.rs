mod routes;
mod controllers;
mod models;
mod processor;

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use axum::Router;
use crate::processor::queue_data_execution;
use crate::routes::queue::{message_queues_routes, queue_routes};

// this is the type of the main data structure that we are going to use to store the queues and access them
pub type  Queues = Arc<RwLock<HashMap<String, VecDeque<(String, u8)>>>> ; // message, retries

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub queues: Queues,
}

#[derive(Debug)]
pub struct Message {
    pub message: String,
    pub retries: u8,
}
pub type MessageQueues = Arc<RwLock<HashMap<String, tokio::sync::mpsc::UnboundedSender<Message>>>>;

#[derive(Clone, Debug, Default)]
pub struct AppState2 {
    pub queues: MessageQueues,
}

fn routes() -> (Router, AppState) {
    let state = AppState {
      queues: Arc::new(RwLock::new(HashMap::new())),
    };
    (axum::Router::new().route("/", axum::routing::get(|| async {
        tracing::info!("base route hit") ;
        "Hello, World!"
    })).nest("/queue", queue_routes().with_state(state.clone()))
        .with_state(state.clone()), state)
}

fn message_queue_routes() -> (Router, AppState2) {
    let state = AppState2 {
        queues: Arc::new(RwLock::new(HashMap::new())),
    } ;
    (Router::new().route("/", axum::routing::get(|| async {
        tracing::info!("base route hit") ;
        "Hello, World!"
    })).nest("/queue", message_queues_routes().with_state(state.clone())).with_state(state.clone()), state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init() ;
    tracing::info!("Initialized tracing getting started with the subscriber") ;
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let is_message_queues = std::env::var("MESSAGE_QUEUES").unwrap_or("false".to_string());
    tracing::info!("running on the port {}", port) ;
    // over here we are going to read from the disk, whether there is any unfinished task in the disk
    tracing::info!("Running the processor such that it runs the logic for execution of the queue") ;
    // after execution, we are going to remove that message from the queue such that, we are going to remove it from the disk as well
    let tcp_connection = tokio::net::TcpListener::bind(format!("[::]:{}", port)).await.unwrap();
    tracing::info!("finally establishing the connection") ;

    if is_message_queues == "true" {
        let (app,  state) = message_queue_routes() ;
        axum::serve(tcp_connection, app).await.unwrap()
    }else{
        let (app, state) = routes();
        queue_data_execution(Arc::from(state)).await;
        axum::serve(tcp_connection, app).await.unwrap()
    }
}
