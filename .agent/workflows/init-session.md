---
description: Initialize the agent session with project context and task status
---

// turbo-all

1. Read the core project rules and map to align with technical constraints.
   - Read `docs/rules.md`
   - Read `docs/project-map.md`
2. Check for the current task status using Beads.
   - Run `bd ready`
3. List any open high-priority tasks.
   - Run `bd list --status in_progress`
4. Report the session status to the user and ask for the next task if none are in progress.
