# Rust Coding Standards
@docs/rules.md

- **Service**: Core Linguistic Service.
- **Error Handling**: Use `Result`/`Option`. No `.unwrap()` or `.expect()` in production.
- **Async**: Use `tonic` for gRPC.
- **Concurrency**: `DashMap` or `tokio::sync::RwLock` for state.
