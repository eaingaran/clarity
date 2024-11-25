pub async fn summarize_content(
    url: &String,
    content: &String,
    model: &String,
) -> Result<(String, u64), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let prompt: String = String::from(format!(
        "Following is the text extracted from the url {} and i want you to summarize it and give me your response in a json format with the following fields (the entire response should be a json and nothing else): title, description, short_summary and long_summary. {}",
        url, content
    ));
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
        }))
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    let summary_text: String = response_json["response"]
        .as_str()
        .unwrap_or("No summary available")
        .to_string();
    let summary_duration: u64 = response_json["total_duration"].as_u64().unwrap_or(0);

    Ok((summary_text, summary_duration))
}
