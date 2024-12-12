from typing import Union

import goose3
import requests
import logging
from goose3 import Goose
from goose3.configuration import Configuration

def extract_content(url: str) -> tuple[bool, Union[goose3.Article, str]]:
    """
    Extracts the main content from a given URL using Goose3.

    Args:
        url: The URL of the website.

    Returns:
        A string containing the extracted content, or None if an error occurs.
    """
    try:
        config = Configuration()
        config.http_timeout = 10
        config.enable_image_fetching = True
        config.use_meta_language = True
        config.browser_user_agent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36'
        goose = Goose(config)
        article = goose.extract(url=url)

        return True, article

    except requests.exceptions.RequestException as e:
        logging.error("Error fetching URL", e)
        return False, str(e)
    except Exception as e:
        logging.error("An unexpected error occurred", e)
        return False, str(e)
