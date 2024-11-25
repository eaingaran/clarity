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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test() {
        let model: String = "llama3.2".to_string();
        let url: String = "https://www.merkur.de/deutschland/hessen/suedkorea-tway-airline-seoul-asia-billigflieger-erhoeht-angebot-flughafen-frankfurt-93431020.html".to_string();
        match scrapper::scrape_content(&url).await {
            Ok((content, images)) => {
                match summarizer::summarize_content(&url, &content, &model).await {
                    Ok((text, duration)) => {
                        println!("Summarising took {:#?}", Duration::from_nanos(duration));
                        println!("{:#?}", text);
                        println!("{:#?}", images);
                        assert_eq!(1, 1);
                    }
                    Err(err) => {
                        println!("{:#?}", err);
                        assert_eq!(1, 2);
                    }
                }
            }
            Err(err) => {
                println!("{:#?}", err);
                assert_eq!(1, 2);
            }
        }
    }
}
