use axum::{
    routing::{get, post},
    Router,
};
mod handlers;
mod models;
mod scrapper;
mod summarizer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/api/summarise", post(handlers::summarise))
        .route("/api/summarize", post(handlers::summarise));

    println!("Running on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
