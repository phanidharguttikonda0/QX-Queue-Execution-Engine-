


fn routes() -> impl Into<axum::Router> {
    axum::Router::new().route("/", axum::routing::get(|| async { 
        tracing::info!("base route hit") ;
        "Hello, World!" 
    }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init() ;
    tracing::info!("Initialized tracing getting started with the subscriber") ;
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    tracing::info!("running on the port {}", port) ;
    let tcp_connetion = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",port)).await.unwrap();
    tracing::info!("finally establishing the connection") ;
    axum::serve(tcp_connetion, routes()).await.unwrap()
}
