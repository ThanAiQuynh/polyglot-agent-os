-- Kích hoạt extension vector cho RAG
CREATE EXTENSION IF NOT EXISTS vector;

-- Khởi tạo bảng MemoryObject cơ bản (Schema mẫu cho Phase 1)
CREATE TABLE IF NOT EXISTS memory_objects (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    linguistic_unit TEXT NOT NULL,
    han_viet TEXT,
    embedding vector(1536), -- Phù hợp với OpenAI/Gemini embeddings
    stability FLOAT DEFAULT 0.1,
    difficulty FLOAT DEFAULT 0.5,
    last_review TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);