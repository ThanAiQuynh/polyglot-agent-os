import grpc
import learning_service_pb2
import learning_service_pb2_grpc

class LinguisticClient:
    def __init__(self, target='localhost:50051'):
        self.target = target

    async def tokenize(self, text, language_code):
        async with grpc.aio.insecure_channel(self.target) as channel:
            stub = learning_service_pb2_grpc.LinguisticServiceStub(channel)
            return await stub.Tokenize(learning_service_pb2.TokenizeRequest(
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
