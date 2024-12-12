import logging

import goose3
from fastapi import FastAPI, Request, Response, status
from fastapi.middleware.cors import CORSMiddleware
import uvicorn
from models import SummaryRequestData, SummaryResponseData
from summarizer import summarize_content
from extractor import extract_content
from verifyer import verify_url

logger = logging.getLogger('uvicorn.error')

app = FastAPI()

# CORS Configuration
origins = [
    "https://clarity.aingaran.dev",
    "http://localhost:5173"
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["POST", "OPTIONS"],
    allow_headers=["Content-Type"],
)


@app.get("/")
async def root():
    return {"message": "Welcome to Clarity backend!"}


@app.post("/api/summarise", response_model=SummaryResponseData, status_code=200)
@app.post("/api/summarize", response_model=SummaryResponseData, status_code=200)
async def summarise(request: Request, payload: SummaryRequestData, response: Response):
    summary = SummaryResponseData(
        url=payload.url,
        model=payload.model if payload.model is not None else "llama3.2",
    )

    try:
        verified, message = verify_url(summary.url)
        if not verified:
            summary.status = message
            response.status_code = status.HTTP_406_NOT_ACCEPTABLE
        else:
            extracted, extracted_content = extract_content(summary.url)
            extracted_content: goose3.Article = extracted_content
            if extracted:
                res, duration = summarize_content(summary.url, summary.model, extracted_content.cleaned_text)
                summary.title = extracted_content.title
                summary.short_summary = res
                summary.long_summary = extracted_content.cleaned_text
                summary.image = extracted_content.top_image.src
                summary.duration = duration/1000000
                summary.status = "Success!"
            else:
                summary.status = extracted_content
                response.status_code = status.HTTP_500_INTERNAL_SERVER_ERROR
    except Exception as e:
        logger.error(e)
        summary.status = f"An unexpected error occurred: {e}"
        response.status_code = status.HTTP_500_INTERNAL_SERVER_ERROR

    return summary

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=3001)
