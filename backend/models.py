from typing import Optional
from pydantic import BaseModel

class SummaryRequestData(BaseModel):
    url: str
    model: Optional[str] = "llama3.2"

class SummaryResponseData(BaseModel):
    url: str
    model: str
    status: Optional[str] = None
    title: Optional[str] = None
    short_summary: Optional[str] = None
    long_summary: Optional[str] = None
    image: Optional[str] = None
    duration: Optional[float] = 0
