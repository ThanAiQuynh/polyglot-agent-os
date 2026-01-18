---
description: Finalize work, verify stability, and sync with remote
---

// turbo-all

1. Run all relevant tests for the service (Rust `cargo test` or Python `pytest`).
2. Close the task in Beads.
   - `bd close <id> --reason "<Detailed summary of work done>"`
3. Synchronize all changes to the remote repository.
   - `bd sync`
4. Notify the user of the completion.
