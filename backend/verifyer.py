import validators
from urllib.parse import urlparse

def verify_url(url: str) -> tuple[bool, str]:
    """Verifies a URL for validity and basic security.

    Args:
        url: The URL string to verify.

    Returns:
        A tuple containing:
        - A boolean indicating whether the URL is valid (True) or not (False).
        - A message string providing details about the validity or any issues found.
    """

    if not url:
        return False, "URL cannot be empty."

    if not validators.url(url):
        return False, "Invalid URL format."

    try:
        parsed_url = urlparse(url)

        if not parsed_url.scheme:
            return False, "URL must have a scheme (e.g., http:// or https://)."

        if parsed_url.scheme.lower() not in ("http", "https"):
            return False, "URL scheme must be either http or https."

        if not parsed_url.netloc:
            return False, "URL must have a network location (domain/host)."

        # Prevent local file access
        if parsed_url.netloc.lower() in ("localhost", "127.0.0.1", "[::1]"):
            return False, "Access to local resources is not allowed."

        # Check for suspicious characters or encodings
        if "%" in url or ".." in url:
            return False, "URL contains suspicious characters."

        # Check for data URLs (data:image/png;base64,...)
        if url.lower().startswith("data:"):
            return False, "Data URLs are not allowed."

        return True, ""

    except ValueError as e:
        return False, f"Invalid URL: {e}"
    except Exception as e:
        return False, f"An unexpected error occurred: {e}"