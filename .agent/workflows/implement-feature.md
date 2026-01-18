---
description: Implement a feature while maintaining documentation integrity
---

1. Mark the task as `in_progress`.
   // turbo
   - `bd update <id> --status in_progress`
2. Perform the code changes according to `docs/rules.md`.
3. If files are created, renamed, or deleted:
   - Update `docs/project-map.md` immediately.
4. Verify the changes (run tests, manual check).
5. **Self-Audit**: Cross-reference your changes with `docs/rules.md` and `docs/project-map.md` to ensure zero discrepancies.
6. Document the results in a walkthrough or task notes.
   - `bd update <id> --notes "Verified locally. Tests passed. Project map updated."`
