from fastapi import FastAPI, HTTPException, Request, Response, status
from fastapi.middleware.cors import CORSMiddleware
import uvicorn
from models import SummaryRequestData, SummaryResponseData
from summarizer import summarize_content
import requests
import json

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
    return {"message": "Welcome to Clarity (Python)!"}


@app.post("/api/summarise", response_model=SummaryResponseData, status_code=200)
@app.post("/api/summarize", response_model=SummaryResponseData, status_code=200)
async def summarise(request: Request, payload: SummaryRequestData, response: Response):
    summary = SummaryResponseData(
        url=payload.url,
        model=payload.model if payload.model is not None else "llama3.2",
    )

    try:
        res, duration = summarize_content(payload.url, summary.model, payload.content)
        res_json = json.loads(res)
        summary.title = res_json.get("title", None)
        summary.short_summary = res_json.get("short_summary", None)
        summary.long_summary = res_json.get("long_summary", None)
        summary.duration = duration/1000000
        summary.status = "Success!"
    except Exception as e:
        summary.status = f"An unexpected error occurred: {e}"
        response.status_code = status.HTTP_500_INTERNAL_SERVER_ERROR

    return summary

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=3001)
