use tonic::{Request, Response, Status};
use prost_types::Timestamp;
use crate::learning::linguistic_service_server::LinguisticService;
use crate::learning::{TokenizeRequest, TokenizeResponse, GetHanVietRequest, GetHanVietResponse, Token};
use crate::learning::memory_service_server::MemoryService;
use crate::learning::{UpdateMemoryStateRequest, GetDueItemsRequest, GetDueItemsResponse, MemoryObject};

#[derive(Default)]
pub struct LinguisticServer {}

#[tonic::async_trait]
impl LinguisticService for LinguisticServer {
    async fn tokenize(&self, request: Request<TokenizeRequest>) -> Result<Response<TokenizeResponse>, Status> {
        let req = request.into_inner();
        let reply = TokenizeResponse {
            tokens: vec![Token {
                text: req.text,
                part_of_speech: "UNKNOWN".to_string(),
                lemma: "TODO".to_string(),
            }],
        };
        Ok(Response::new(reply))
    }

    async fn get_han_viet(&self, request: Request<GetHanVietRequest>) -> Result<Response<GetHanVietResponse>, Status> {
        let _req = request.into_inner();
        let reply = GetHanVietResponse { phonetic: "TODO".to_string() };
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
            han_viet: "TODO".to_string(),
            stability: 1.0,
            difficulty: 0.5,
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
