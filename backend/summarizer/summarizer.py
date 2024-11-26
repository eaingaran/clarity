import requests
import logging

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

def summarize_content(url: str, model: str, content: str) -> tuple[str, int]:
    prompt = f"""Following is the text extracted from the url {url} and I want you to summarize it and give me your response in a JSON format with the following fields (the entire response should be a JSON and nothing else): title, short_summary, and long_summary. {content}"""

    try:
        logging.info(f"Sending summarization request for URL: {url}")  # Log request

        response = requests.post(
            "http://localhost:11434/api/generate",
            json={
                "model": model,
                "prompt": prompt,
                "stream": False,
            },
        )

        response.raise_for_status()
        response_json = response.json()

        summary_text = response_json.get("response", "No summary available")
        summary_duration = int(response_json.get("total_duration", 0))

        logging.info(f"Summarization successful for URL: {url}")  # Log success
        return summary_text, summary_duration

    except requests.exceptions.RequestException as e:
        logging.error(f"Error communicating with summarization API for URL {url}: {e}")  # Log error
        return "Error summarizing content", 0
