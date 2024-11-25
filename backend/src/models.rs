use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct SummariseData {
    pub url: String,
    pub model: Option<String>,
    pub include_images: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SummaryData {
    pub url: String,
    pub model: String,
    pub include_images: bool,
    pub status: Option<String>,
    pub images: Option<Vec<String>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub short_summary: Option<String>,
    pub long_summary: Option<String>,
}
