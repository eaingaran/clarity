use crate::models::{ScraperRequestData, ScraperResponseData};
use crate::scrapper;
use axum::{extract, http::StatusCode, Json};
use url::Url;

pub async fn root() -> String {
    "Welcome to Clarity (Rust)!".to_string()
}

pub async fn summarise(
    extract::Json(payload): extract::Json<ScraperRequestData>,
) -> (StatusCode, Json<ScraperResponseData>) {
    let mut response: ScraperResponseData = ScraperResponseData {
        url: payload.url.clone(),
        include_images: payload.include_images.unwrap_or_else(|| false),
        status: "".to_string(),
        images: None,
        content: None,
    };

    let url = match Url::parse(&payload.url) {
        Ok(url) => url,
        Err(_) => {
            response.status = "Supplied URL is invalid.".to_string();
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    // Validate HTTPS URLs
    if url.scheme() != "https" {
        response.status = format!("URL scheme {} is not supported.", url.scheme());
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    // Reject internal or invalid hosts
    let host = url.host_str().unwrap_or_default();
    if host.is_empty()
        || host.parse::<std::net::IpAddr>().is_ok()
        || ["localhost", "127.0.0.1", "::1"].contains(&host)
    {
        response.status = "Nice try.".to_string();
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    // Scrape
    match scrapper::scrape_content(&payload.url).await {
        Ok((content, image_urls)) => {
            println!("Scraped content: {:?}", content);
            println!("Scraped images: {:?}", image_urls);
            response.content = Some(content);
            if response.include_images {
                response.images = Some(image_urls);
            }
            response.status = "Success!".to_string();
            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            response.status = format!("Error scraping the content: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}
