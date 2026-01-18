---
description: Create or update a task in the Beads system
---

1. Ensure the task has a clear title and description.
2. Create the task with appropriate priority (0-4).
   - `bd create "<Title>" -p <priority> --description "<Description>"`
3. If it's a sub-task, link it to a parent.
   - `bd update <id> --parent <parent_id>`
4. Add any design notes or acceptance criteria if known.
   - `bd update <id> --design "<Design Notes>" --acceptance "<Acceptance Criteria>"`
5. Sync changes to git.
   // turbo
   - `bd sync`
