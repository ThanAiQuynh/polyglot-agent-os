import grpc
import learning_service_pb2
import learning_service_pb2_grpc

class IngestionAgent:
    def __init__(self, kernel_address: str = "localhost:50051"):
        self.kernel_address = kernel_address

    def ingest_text(self, text: str, user_id: str, target_language: str = "jp"):
        """
        Sends raw text to the Rust Kernel for tokenization and memory ingestion.
        """
        with grpc.insecure_channel(self.kernel_address) as channel:
            stub = learning_service_pb2_grpc.AgentServiceStub(channel)
            
            request = learning_service_pb2.IngestContentRequest(
                raw_content=text,
                user_id=user_id,
                target_language=target_language,
                context_hint="Manual Ingestion via Python Agent"
            )
            
            try:
                response = stub.IngestContent(request)
                return {
                    "session_id": response.session_id,
                    "extracted_count": len(response.extracted_objects)
                }
            except grpc.RpcError as e:
                return {"error": str(e.code()), "details": e.details()}

if __name__ == "__main__":
    # Quick test
    agent = IngestionAgent()
    result = agent.ingest_text("日本語の勉強をしています。", "user_test_123")
    print(result)
