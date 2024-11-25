use crate::models::{SummariseData, SummaryData};
use crate::scrapper;
use crate::summarizer;
use axum::{extract, http::StatusCode, Json};
use url::Url;

pub async fn root() -> String {
    "Welcome to Clarity!".to_string()
}

pub async fn summarise(
    extract::Json(payload): extract::Json<SummariseData>,
) -> (StatusCode, Json<SummaryData>) {
    let mut summary: SummaryData = SummaryData {
        url: payload.url.clone(),
        model: payload.model.unwrap_or_else(|| "llama3.2".to_string()),
        include_images: payload.include_images.unwrap_or_else(|| false),
        status: None,
        images: None,
        title: None,
        description: None,
        short_summary: None,
        long_summary: None,
    };

    let url = match Url::parse(&payload.url) {
        Ok(url) => url,
        Err(_) => {
            summary.status = Some("Supplied URL is invalid.".to_string());
            return (StatusCode::BAD_REQUEST, Json(summary));
        }
    };

    // Validate HTTPS URLs
    if url.scheme() != "https" {
        summary.status = Some(format!("URL scheme {} is not supported.", url.scheme()));
        return (StatusCode::BAD_REQUEST, Json(summary));
    }

    // Reject internal or invalid hosts
    let host = url.host_str().unwrap_or_default();
    if host.is_empty()
        || host.parse::<std::net::IpAddr>().is_ok()
        || ["localhost", "127.0.0.1", "::1"].contains(&host)
    {
        summary.status = Some("Nice try.".to_string());
        return (StatusCode::BAD_REQUEST, Json(summary));
    }

    // Scrape and summarize
    match scrapper::scrape_content(&payload.url).await {
        Ok((content, image_urls)) => {
            println!("Scraped content: {:?}", content);
            println!("Scraped images: {:?}", image_urls);
            if summary.include_images {
                summary.images = Some(image_urls);
            }

            //let summarised_text = summarizer::summarize_content(&payload.url, &content).await;
            // if summarised_text.is_ok() {
            //     summary.long_summary = summarised_text.ok();
            // }

            // Call the summarizer
            // match summarizer::summarize_content(&content).await {
            //     Ok(data) => {
            //         summary.status = Some("Success!".to_string());
            //         summary.long_summary = Some(data);
            //         if payload.include_images.unwrap_or(false) {
            //             summary.images = Some(image_urls);
            //         }
            //         (StatusCode::OK, Json(summary))
            //         //(StatusCode::INTERNAL_SERVER_ERROR, Json(summary))
            //     }
            //     Err(err) => {
            //         summary.status = Some(format!("Error summarizing the content: {:?}", err));
            //         (StatusCode::INTERNAL_SERVER_ERROR, Json(summary))
            //     }
            // }
            (StatusCode::OK, Json(summary))
        }
        Err(err) => {
            summary.status = Some(format!("Error scraping the content: {:?}", err));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(summary))
        }
    }
}

// pub async fn summarise(
//     extract::Json(payload): extract::Json<SummariseData>,
// ) -> (StatusCode, Json<SummaryData>) {
//     let mut summary: SummaryData = SummaryData {
//         url: payload.url.clone(),
//         model: payload.model.unwrap_or_else(|| "llama3.2".to_string()),
//         include_images: payload.include_images.unwrap_or_else(|| false),
//         status: None,
//         images: None,
//         title: None,
//         description: None,
//         short_summary: None,
//         long_summary: None,
//     };

//     let url = match Url::parse(&payload.url) {
//         Ok(url) => url,
//         Err(_) => {
//             summary.status = Some("Supplied URL is invalid.".to_string());
//             return (StatusCode::BAD_REQUEST, Json(summary));
//         }
//     };

//     // Validate HTTPS URLs
//     if url.scheme() != "https" {
//         summary.status = Some(format!("URL scheme {} is not supported.", url.scheme()));
//         return (StatusCode::BAD_REQUEST, Json(summary));
//     }

//     // Reject internal or invalid hosts
//     let host = url.host_str().unwrap_or_default();
//     if host.is_empty()
//         || host.parse::<std::net::IpAddr>().is_ok()
//         || ["localhost", "::1"].contains(&host)
//     {
//         summary.status = Some("Nice try.".to_string());
//         return (StatusCode::BAD_REQUEST, Json(summary));
//     }

//     match summarizer::summarize_content(&mut summary).await {
//         Ok(summary) => {
//             summary.status = Some("Success!".to_string());
//             //(StatusCode::OK, Json(summary))
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(summary))
//         }
//         Err(err) => {
//             summary.status = Some(format!("Error summarizing the content: {:?}", err));
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(summary))
//         }
//     }
// }
