# Polyglot-Agent OS: Rules

## Architecture Constraints

* **Pattern**: Must follow Clean Architecture (Domain -> Application -> Infrastructure -> Presentation).
* **Service Isolation**: Services MUST NOT share databases. Communication is via gRPC (synchronous) or Kafka (asynchronous).
* **Async Priority**: Any task taking >500ms (LLM calls, audio processing) MUST be offloaded to Kafka.

## Language Standards

### Import Rules (No Local Import)

* **No Local Import**: Local imports inside functions, methods, or runtime scopes are **strictly forbidden**.
* **All imports MUST be declared at the top of the file/module**.
* **Rationale**: Ensure deterministic dependency graphs, improve static analysis, readability, and avoid hidden runtime costs.
* **Applies To**: Rust (`use`), Python (`import` / `from ... import`), and generated glue code.
* **Exceptions**: Only allowed in test code or explicitly documented experimental prototypes (must be commented).

### Global

* **Naming Language**: **All variables, functions, classes, methods, files, and identifiers MUST be written in English only**. No mixed-language or non-English identifiers are allowed.

### Rust (Core Linguistic Service)

* **Always**: Use `Result` or `Option` for error handling. Use `tonic` for gRPC.
* **Never**: Use `.unwrap()` or `.expect()` in production code. Avoid `unsafe` blocks unless absolutely necessary for performance.
* **Concurrency**: Use `DashMap` or `tokio::sync::RwLock` for thread-safe state management.

### Python (AI Tutor Orchestrator)

* **Framework**: FastAPI with `asyncio`.
* **Typing**: Use Pydantic v2 for all data models.
* **LLM**: Use structured output (Function Calling) to interact with the Core Service.

## Protobuf Naming Convention (snake_case)

* **Files**: Proto filenames MUST use `snake_case` (e.g. `learning_event.proto`, `srs_state.proto`).
* **Package Names**: MUST use `snake_case` and reflect domain boundaries (e.g. `core_linguistic.srs.v1`).
* **Message Names**: Use **PascalCase** (Protobuf convention) but field names inside messages MUST use `snake_case`.
* **Field Names**: MUST use `snake_case` only. No abbreviations unless domain-standard (e.g. `srs_id`, `llm_score`).
* **Enum Names**: Use **PascalCase** for enum type, `SCREAMING_SNAKE_CASE` for enum values.
* **Service Names**: Use **PascalCase** (e.g. `LinguisticService`).
* **RPC Method Names**: Use **PascalCase**, verb-first (e.g. `GetLearningState`, `UpdateMemoryScore`).
* **Versioning**: Version MUST be encoded in the package name, not the filename (e.g. `package core_linguistic.learning.v1;`).
* **No Language Mixing**: All identifiers MUST be English-only.

## Dependency Management

* **Mandatory Sync**: Whenever an agent installs or adds a new library/dependency, it MUST update the corresponding dependency manifest in the same change set.
  - **Rust**: Update `Cargo.toml`.
  - **Python**: Update `requirements.txt` or `pyproject.toml`.
* **Rationale**: Prevent "works on my machine" issues and ensure reproducible builds for all contributors and agents.

---

## Documentation Integrity Rules (Project Map)

* **Mandatory Project Map Update**:

  * Whenever an **agent creates, deletes, or renames** any **file or folder**, it **MUST update** `docs/project-map.md` in the same change set.
  * Agents SHOULD follow the standard procedures defined in `.agent/workflows/`.

* **Project Map Purpose**:

  * Acts as the **single source of truth** for project structure
  * Allows agents to understand system layout without filesystem scanning
  * Enables deterministic reasoning about ownership and boundaries

* **Required Content for Each Entry**:

  * Path
  * Responsibility (1–2 lines)
  * Layer (Domain / Application / Infrastructure / Presentation)

* **Enforcement**:

  * If updating `docs/project-map.md` is not possible, the agent **MUST refuse** to create new files and explain why.

* **Agent Behavior**:

  * Agent MUST read `docs/project-map.md` before proposing structural changes
  * Agent SHOULD warn when a new file violates existing layer or service boundaries

---

### Example

```proto
syntax = "proto3";

package core_linguistic.learning.v1;

service LinguisticService {
  rpc GetLearningState(GetLearningStateRequest) returns (GetLearningStateResponse);
}

message GetLearningStateRequest {
  string user_id = 1;
  string memory_id = 2;
}

message GetLearningStateResponse {
  float retention_score = 1;
  int64 last_review_timestamp = 2;
}

enum LearningStatus {
  LEARNING_STATUS_UNSPECIFIED = 0;
  LEARNING_STATUS_NEW = 1;
  LEARNING_STATUS_REVIEW = 2;
}
```
