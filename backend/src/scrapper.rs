use reqwest;
use scraper::{Html, Selector};

pub async fn scrape_content(
    url: &str,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);

    // Extract the page title and meta description
    let title = extract_meta_content(&document, "title").unwrap_or_default();
    let description =
        extract_meta_content(&document, "meta[name=\"description\"]").unwrap_or_default();

    // Extract main content
    let content = extract_main_content(&document);

    // Extract image URLs
    let image_urls = extract_images(&document);

    // Combine title, description, and main content
    let combined_content = format!(
        "{}\n\n{}\n\n{}",
        title.trim(),
        description.trim(),
        content.trim()
    );

    Ok((combined_content, image_urls))
}

// Extract the main content using semantic tags and filtering
fn extract_main_content(document: &Html) -> String {
    let main_selectors = [
        "article",
        "main",
        "section",
        "div.content",
        "#content",
        ".article-body",
        ".blog-post",
    ];
    let text_selectors = ["p", "h1", "h2", "h3", "li"];

    for selector in main_selectors.iter() {
        if let Ok(content_selector) = Selector::parse(selector) {
            if let Some(content_element) = document.select(&content_selector).next() {
                // Extract relevant text elements within the container
                return content_element
                    .select(
                        &Selector::parse(&text_selectors.join(","))
                            .unwrap_or_else(|_| Selector::parse("p").unwrap()),
                    )
                    .flat_map(|e| e.text())
                    .filter(|text| !text.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join(" ");
            }
        }
    }

    // Fallback: extract all paragraphs and headers if no container is found
    document
        .select(&Selector::parse(&text_selectors.join(",")).unwrap())
        .flat_map(|e| e.text())
        .filter(|text| !text.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

// Extract the page's title or meta content
fn extract_meta_content(document: &Html, selector: &str) -> Option<String> {
    if let Ok(meta_selector) = Selector::parse(selector) {
        document
            .select(&meta_selector)
            .next()
            .and_then(|e| e.value().attr("content").or_else(|| e.text().next()))
            .map(|text| text.trim().to_string())
    } else {
        None
    }
}

// Extract image URLs from the document
fn extract_images(document: &Html) -> Vec<String> {
    let img_selector = Selector::parse("img").unwrap();
    document
        .select(&img_selector)
        .filter_map(|element| element.value().attr("src"))
        .map(|src| src.to_string())
        .collect()
}
