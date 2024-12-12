# Clarity Backend - Summarizer

This project is a FastAPI-based backend service that provides text summarization functionality. It leverages a language model (currently set to llama3.2 by default) to generate summaries of content provided via a URL or directly as text.

## Features

- **Summarization:**  Generates short and long summaries of content from a given URL.
- **Language Model Integration:**  Connects to a language model API (e.g., llama3.2) for summarization.

## Setup

1. **Clone the repository:**
    ```bash
    git clone https://github.com/eaingaran/clarity.git
    cd clarity/backend
    ```

2. **Create virtual environment** (optional, but highly recommended)
    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    ```

3.  **Install dependencies:**
    ```bash
    pip install -r requirements.txt
    ```

4.  **Configure the language model API:**
    - Ensure the language model API is running (e.g., llama3.2 on http://localhost:11434).
    - You can modify the summarizer.py file to adjust the API endpoint if needed.

## Running the application
```bash
  uvicorn main:app --reload --host 0.0.0.0 --port 3001
```

This will start the FastAPI server on `http://0.0.0.0:3001`.

## API Endpoints

  - **GET /**:  Returns a welcome message.
  - **POST /api/summarise or /api/summarize**:
      - **Request Body:**
        ```json
        {
          "url": "the url of the article",
          "model": "llama3.2"  // Optional, defaults to llama3.2
        }
        ```
      - **Response Body:**
        ```json
        {
          "url": "the url of the article",
          "model": "llama3.2",
          "status": "Success!",
          "title": "Extracted Title",
          "short_summary": "Short summary of the content",
          "long_summary": "Detailed summary of the content",
          "image": "url of the image related to the article", // May not always be reliable
          "duration": 0.123  // Summarization time in milliseconds
        }
        ```