import requests
import logging

def summarize_content(url: str, model: str, content: str) -> tuple[str, int]:
    prompt = f"""Following is the text extracted from the url {url} and I want you to summarize it and give me just a crisp short summary without anything else. {content}"""

    try:
        logging.info(f"Sending summarization request for URL: {url}")

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

        logging.info(f"Summarization successful for URL: {url}")
        return summary_text, summary_duration

    except requests.exceptions.RequestException as e:
        logging.error(f"Error communicating with summarization API for URL {url}: {e}")
        return "Error summarizing content", 0
