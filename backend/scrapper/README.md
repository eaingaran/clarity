# Clarity Backend - Scraper

This is the scraper (backend) service for Clarity, a web application that summarizes content from any given URL.

## Features

* **URL content extraction:** Extracts the main content from a given URL.
* **Image extraction:** Optionally extracts images from the URL.
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
3. Navigate to the project directory: `cd clarity/backend/scrapper`
4. Build and run the server: `cargo run`

The backend will start running on `http://localhost:3000`.

## API Endpoints

* **GET `/`:** Root endpoint, returns a welcome message.
* **POST `/api/scrape`:**  Scrapes the content of the given URL.

### Request Body for `/api/scrape`

```json
{
  "url": "url-of-the-article",
  "include_images": true # Optional, defaults to false
}
```

### Response Body

```json
{
  "url": "url-of-the-article",
  "include_images": true,
  "status": "Success!", # Or an error message
  "images": ["image-url-1", "image-url-2"], # Optional, only sent when include_imagesis true
  "content": "Article Title", # Optional, only sent in success scenario
}
```
