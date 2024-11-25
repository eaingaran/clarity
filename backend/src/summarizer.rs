pub async fn summarize_content(
    url: &String,
    content: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let prompt: String = String::from(format!(
        "Following is the text extracted from the url {} and i want you to summarize it. {}",
        url, content
    ));
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "llama3.2",
            "prompt": prompt,
            "stream": false,
        }))
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    let summary_text: String = response_json["response"]["choices"][0]["text"]
        .as_str()
        .unwrap_or("No summary available")
        .to_string();

    println!("{:#?}", summary_text);

    Ok(summary_text)
}
