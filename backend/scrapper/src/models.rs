use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct ScraperRequestData {
    pub url: String,
    pub include_images: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ScraperResponseData {
    pub url: String,
    pub include_images: bool,
    pub status: String,
    pub images: Option<Vec<String>>,
    pub content: Option<String>,
}
