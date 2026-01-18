---
description: Finalize work, verify stability, and sync with remote
---

// turbo-all

1. Run all relevant tests for the service (Rust `cargo test` or Python `pytest`).
2. Close the task in Beads.
   - `.\scripts\beads_0.47.1_windows_amd64\bd.exe close <id> --reason "<Detailed summary of work done>"`
3. Synchronize all changes to the remote repository.
   - `.\scripts\beads_0.47.1_windows_amd64\bd.exe sync`
4. Notify the user of the completion.
