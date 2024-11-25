# Clarity Backend

This is the backend service for Clarity, a web application that summarizes content from any given URL.

## Features

* **URL summarization:** Extracts the main content from a given URL and provides a short and long summary.
* **Image extraction:** Optionally extracts images from the URL.
* **Flexible model selection:** Allows users to choose different summarization models (currently defaults to llama3.2).
* **HTTPS validation:** Ensures that only secure URLs are processed.
* **Host validation:** Rejects internal or invalid hosts for security.

## Technologies Used

* **Rust:** Programming language for building the backend.
* **Axum:** Web framework for Rust.
* **Reqwest:** HTTP client for making requests.
* **Scraper:** HTML parsing library.
* **Serde:** Serialization/deserialization library.
* **Tokio:** Asynchronous runtime for Rust.

## Dependencies

All dependencies are managed through Cargo and listed in the `Cargo.toml` file.

## Running the Backend

1. Ensure you have Rust and Cargo installed.
2. Clone the repository: `git clone https://github.com/eaingaran/clarity.git`
3. Navigate to the project directory: `cd clarity/backend`
4. Build and run the server: `cargo run`

The backend will start running on `http://localhost:3000`.

## API Endpoints

* **GET `/`:** Root endpoint, returns a welcome message.
* **POST `/api/summarise` or `/api/summarize`:**  Summarizes the content of the given URL.

### Request Body for `/api/summarise`

```json
{
  "url": "url-of-the-article",
  "model": "llama3.2", # Optional, defaults to llama3.2
  "include_images": true # Optional, defaults to false
}
```

### Response Body

```json
{
  "url": "url-of-the-article",
  "model": "llama3.2",
  "include_images": true,
  "status": "Success!", # Or an error message
  "images": ["image-url-1", "image-url-2"], # Optional, only sent when include_imagesis true
  "title": "Article Title", # Optional
  "description": "Article Description", # Optional
  "short_summary": "Short summary of the article", # Optional
  "long_summary": "Detailed summary of the article" # Optional
}
```

## Testing

Run the tests using `cargo test -- --nocapture`. This runs a basic test that scrapes and summarizes content from a predefined URL. You can update the url, model and other parameters in src/main.rs

## Future Improvements

* **Implement more summarization models:** Offer a wider variety of models with different strengths and weaknesses.
* **Improve content extraction:** Handle more complex website structures and edge cases.
* **Add user authentication:** Secure the API and allow users to save their summarization history.
* **Deploy to a cloud platform:** Make the service publicly accessible.
* **Implement the screenshot and audio modules:** Expand the functionality to include screenshots and audio summarization.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.
