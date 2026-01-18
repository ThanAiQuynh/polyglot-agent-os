---
description: Create or update a task in the Beads system
---

1. Ensure the task has a clear title and description.
2. Create the task with appropriate priority (0-4).
   - `.\scripts\beads_0.47.1_windows_amd64\bd.exe create "<Title>" -p <priority> --description "<Description>"`
3. If it's a sub-task, link it to a parent.
   - `.\scripts\beads_0.47.1_windows_amd64\bd.exe update <id> --parent <parent_id>`
4. Add any design notes or acceptance criteria if known.
   - `.\scripts\beads_0.47.1_windows_amd64\bd.exe update <id> --design "<Design Notes>" --acceptance "<Acceptance Criteria>"`
5. Sync changes to git.
   // turbo
   - `.\scripts\beads_0.47.1_windows_amd64\bd.exe sync`
