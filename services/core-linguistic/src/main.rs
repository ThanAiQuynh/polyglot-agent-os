use tonic::transport::Server;
use core_linguistic::presentation::{LinguisticServer, MemoryServer, AgentServer};
use core_linguistic::learning::linguistic_service_server::LinguisticServiceServer;
use core_linguistic::learning::memory_service_server::MemoryServiceServer;
use core_linguistic::learning::agent_service_server::AgentServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    println!("Core-Linguistic (Clean) listening on {}", addr);

    Server::builder()
        .add_service(LinguisticServiceServer::new(LinguisticServer::default()))
        .add_service(MemoryServiceServer::new(MemoryServer::default()))
        .add_service(AgentServiceServer::new(AgentServer::default()))
        .serve(addr)
        .await?;

    Ok(())
}
