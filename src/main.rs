mod routes;
mod controllers;
mod models;
mod processor;

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc};
use tokio::sync::RwLock;
use axum::Router;
use crate::processor::queue_data_execution;
use crate::routes::queue::queue_routes;

// this is the type of the main data structure that we are going to use to store the queues and access them
pub type  Queues = Arc<RwLock<HashMap<String, VecDeque<(String, u8)>>>> ; // message, retries

#[derive(Clone, Debug, Default, )]
pub struct AppState {
    pub queues: Queues,
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init() ;
    tracing::info!("Initialized tracing getting started with the subscriber") ;
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    tracing::info!("running on the port {}", port) ;
    let (app, state) = routes();
    // over here we are going to read from the disk, whether there is any unfinished task in the disk
    tracing::info!("Running the processor such that it runs the logic for execution of the queue") ;
    queue_data_execution(Arc::from(state)).await;
    // after execution, we are going to remove that message from the queue such that, we are going to remove it from the disk as well
    let tcp_connetion = tokio::net::TcpListener::bind(format!("[::]:{}", port)).await.unwrap();
    tracing::info!("finally establishing the connection") ;
    axum::serve(tcp_connetion, app).await.unwrap()
}
