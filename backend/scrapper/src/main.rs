use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
mod handlers;
mod models;
mod scrapper;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/api/scrape", post(handlers::summarise))
        .layer(
            CorsLayer::new()
                //.allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_origin(
                    "https://clarity.aingaran.dev"
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_credentials(false)
                .allow_headers([header::CONTENT_TYPE]),
        );

    serve(app, 3000).await;
}

async fn serve(app: Router, port: u16) {
    println!("Running on http://localhost:{:?}", port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
