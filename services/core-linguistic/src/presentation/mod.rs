use tonic::{Request, Response, Status};
use prost_types::Timestamp;
use crate::learning::linguistic_service_server::LinguisticService;
use crate::learning::{TokenizeTextRequest, TokenizeTextResponse, GetSinoVietnameseRequest, GetSinoVietnameseResponse};
use crate::learning::memory_service_server::MemoryService;
use crate::learning::{UpdateMemoryStateRequest, GetDueItemsRequest, GetDueItemsResponse, MemoryObject, IngestContentRequest, IngestContentResponse};
use crate::learning::agent_service_server::AgentService;

use crate::application::tokenizer::JapaneseTokenizer;
use crate::learning::ErrorCode;

pub struct LinguisticServer {
    tokenizer: JapaneseTokenizer,
}

impl Default for LinguisticServer {
    fn default() -> Self {
        Self {
            tokenizer: JapaneseTokenizer::default(),
        }
    }
}

#[tonic::async_trait]
impl LinguisticService for LinguisticServer {
    async fn tokenize_text(&self, request: Request<TokenizeTextRequest>) -> Result<Response<TokenizeTextResponse>, Status> {
        let req = request.into_inner();
        
        if req.language_code != "jp" {
            return Ok(Response::new(TokenizeTextResponse {
                tokens: vec![],
                error_code: ErrorCode::LanguageNotSupported as i32,
                error_message: format!("Language '{}' not supported yet", req.language_code),
            }));
        }

        let tokens = self.tokenizer.tokenize(&req.text);
        
        let reply = TokenizeTextResponse {
            tokens,
            error_code: ErrorCode::Unspecified as i32,
            error_message: String::new(),
        };
        Ok(Response::new(reply))
    }

    async fn get_sino_vietnamese(&self, request: Request<GetSinoVietnameseRequest>) -> Result<Response<GetSinoVietnameseResponse>, Status> {
        let _req = request.into_inner();
        let reply = GetSinoVietnameseResponse { phonetic: "TODO".to_string() };
        Ok(Response::new(reply))
    }
}

#[derive(Default)]
pub struct MemoryServer {}

#[tonic::async_trait]
impl MemoryService for MemoryServer {
    async fn update_memory_state(&self, request: Request<UpdateMemoryStateRequest>) -> Result<Response<MemoryObject>, Status> {
        let req = request.into_inner();
        let reply = MemoryObject {
            id: req.object_id,
            user_id: "user_123".to_string(),
            linguistic_unit: "TODO".to_string(),
            sino_vietnamese: "TODO".to_string(),
            stability: 1.0,
            difficulty: 0.5,
            decay_curve: 0.1,
            modality_weights: std::collections::HashMap::new(),
            last_modality: req.modality,
            last_review: Some(Timestamp::default()),
        };
        Ok(Response::new(reply))
    }

    async fn get_due_items(&self, request: Request<GetDueItemsRequest>) -> Result<Response<GetDueItemsResponse>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(GetDueItemsResponse { items: vec![] }))
    }
}

#[derive(Default)]
pub struct AgentServer {}

#[tonic::async_trait]
impl AgentService for AgentServer {
    async fn ingest_content(&self, request: Request<IngestContentRequest>) -> Result<Response<IngestContentResponse>, Status> {
        let _req = request.into_inner();
        // In Phase 1, this is a stub that returns an empty response
        let reply = IngestContentResponse {
            session_id: "session_stub".to_string(),
            extracted_objects: vec![],
        };
        Ok(Response::new(reply))
    }
}
