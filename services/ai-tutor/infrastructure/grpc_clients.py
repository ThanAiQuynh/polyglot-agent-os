import grpc
import learning_service_pb2
import learning_service_pb2_grpc

class LinguisticClient:
    def __init__(self, target='localhost:50051'):
        self.target = target

    async def tokenize(self, text, language_code):
        async with grpc.aio.insecure_channel(self.target) as channel:
            stub = learning_service_pb2_grpc.LinguisticServiceStub(channel)
            return await stub.TokenizeText(learning_service_pb2.TokenizeTextRequest(
                text=text,
                language_code=language_code
            ))

class MemoryClient:
    def __init__(self, target='localhost:50051'):
        self.target = target

    async def update_state(self, object_id, recall_quality, modality):
        async with grpc.aio.insecure_channel(self.target) as channel:
            stub = learning_service_pb2_grpc.MemoryServiceStub(channel)
            return await stub.UpdateMemoryState(learning_service_pb2.UpdateMemoryStateRequest(
                object_id=object_id,
                recall_quality=recall_quality,
                modality=modality
            ))

    async def get_due_items(self, user_id, limit=10):
        async with grpc.aio.insecure_channel(self.target) as channel:
            stub = learning_service_pb2_grpc.MemoryServiceStub(channel)
            return await stub.GetDueItems(learning_service_pb2.GetDueItemsRequest(
                user_id=user_id,
                limit=limit
            ))

class AgentClient:
    def __init__(self, target='localhost:50051'):
        self.target = target

    async def ingest(self, raw_content, user_id, target_language, context_hint=""):
        async with grpc.aio.insecure_channel(self.target) as channel:
            stub = learning_service_pb2_grpc.AgentServiceStub(channel)
            return await stub.IngestContent(learning_service_pb2.IngestContentRequest(
                raw_content=raw_content,
                user_id=user_id,
                target_language=target_language,
                context_hint=context_hint
            ))
