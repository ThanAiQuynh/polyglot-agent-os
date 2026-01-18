# Architecture & Service Constraints
@docs/rules.md

- **Pattern**: Strict Clean Architecture (Domain -> Application -> Infrastructure -> Presentation).
- **Isolation**: No shared databases between services.
- **Communication**: gRPC for sync, Kafka for async (>500ms tasks).
