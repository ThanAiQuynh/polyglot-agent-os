# Polyglot-Agent OS: Workflows

## Workflow: Smart_Content_Ingestion
1. **Trigger**: User provides a URL (e.g., NHK News).
2. **Action**: `ai-tutor` calls `extract_text`.
3. **Action**: `ai-tutor` performs RAG to identify high-value vocabulary.
4. **Action**: `ai-tutor` calls `core-linguistic` via gRPC to get Han-Viet meanings and current memory status.
5. **Action**: `ai-tutor` pushes a `MEMORY_OBJECT_CREATE` event to Kafka.
6. **Finalize**: `api-gateway` notifies user via WebSocket once Kafka consumer finishes persistence.

## Workflow: SRS_Review_Session
1. **Trigger**: User starts a review.
2. **Action**: `core-linguistic` fetches due items based on SRS algorithms.
3. **Action**: `ai-tutor` generates a contextual conversation using those items.
4. **Action**: User responds -> `ai-tutor` evaluates and pushes `REVIEW_RESULT` to Kafka.