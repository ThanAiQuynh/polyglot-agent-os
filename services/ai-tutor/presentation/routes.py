from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List, Optional
from datetime import datetime
import grpc
from infrastructure.grpc_clients import LinguisticClient, MemoryClient, AgentClient

router = APIRouter()
linguistic_client = LinguisticClient()
memory_client = MemoryClient()
agent_client = AgentClient()

class TokenizeRequest(BaseModel):
    text: str
    language_code: str

class TokenResponse(BaseModel):
    text: str
    part_of_speech: str
    lemma: str

class AnalyzeResponse(BaseModel):
    tokens: List[TokenResponse]

class MemoryObject(BaseModel):
    id: str
    user_id: str
    linguistic_unit: str
    han_viet: Optional[str]
    stability: float
    difficulty: float
    last_modality: int
    last_review: Optional[datetime]

@router.get("/")
async def root():
    return {"message": "AI Tutor Orchestrator is running (Clean Architecture)"}

@router.post("/analyze", response_model=AnalyzeResponse)
async def analyze_text(request: TokenizeRequest):
    try:
        response = await linguistic_client.tokenize(request.text, request.language_code)
        tokens = [TokenResponse(text=t.text, part_of_speech=t.part_of_speech, lemma=t.lemma) for t in response.tokens]
        return AnalyzeResponse(tokens=tokens)
    except grpc.RpcError as e:
        raise HTTPException(status_code=500, detail=f"gRPC error: {e.details()}")

@router.post("/memory/update", response_model=MemoryObject)
async def update_memory(object_id: str, recall_quality: int, modality: int):
    try:
        response = await memory_client.update_state(object_id, recall_quality, modality)
        return MemoryObject(
            id=response.id,
            user_id=response.user_id,
            linguistic_unit=response.linguistic_unit,
            han_viet=response.sino_vietnamese,
            stability=response.stability,
            difficulty=response.difficulty,
            last_modality=response.last_modality,
            last_review=datetime.fromtimestamp(response.last_review.seconds) if response.HasField('last_review') else None
        )
    except grpc.RpcError as e:
        raise HTTPException(status_code=500, detail=f"gRPC error: {e.details()}")

@router.get("/memory/due", response_model=List[MemoryObject])
async def get_due_items(user_id: str, limit: int = 10):
    try:
        response = await memory_client.get_due_items(user_id, limit)
        items = [
            MemoryObject(
                id=item.id,
                user_id=item.user_id,
                linguistic_unit=item.linguistic_unit,
                han_viet=item.sino_vietnamese,
                stability=item.stability,
                difficulty=item.difficulty,
                last_modality=item.last_modality,
                last_review=datetime.fromtimestamp(item.last_review.seconds) if item.HasField('last_review') else None
            ) for item in response.items
        ]
        return items
    except grpc.RpcError as e:
        raise HTTPException(status_code=500, detail=f"gRPC error: {e.details()}")

@router.post("/ingest")
async def ingest_content(text: str, user_id: str, language: str = "jp"):
    try:
        response = await agent_client.ingest(text, user_id, language)
        return {
            "session_id": response.session_id,
            "extracted_objects_count": len(response.extracted_objects)
        }
    except grpc.RpcError as e:
        raise HTTPException(status_code=500, detail=f"gRPC error: {e.details()}")
