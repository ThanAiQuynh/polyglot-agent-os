# Agent Instructions: Task Tracking with Beads (bd)

> [!IMPORTANT]
> **STRICT ENFORCEMENT**: This repository follows the standards defined in `.agent/rules/project-standards.md`. Violation of these rules is considered a critical failure.

You MUST use the **Beads (bd)** library for task tracking in this repository. Beads provides a structured graph of tasks that persists across sessions via git.

## Workflow

0.  **Bootstrap**: Before any action, you MUST follow the bootstrap procedure in `.agent/rules/project-standards.md`.
1.  **Check Status**: Begin every session by running `.\scripts\beads_0.47.1_windows_amd64\bd.exe ready` to see what tasks are available for you.
2.  **Pick a Task**: Use `.\scripts\beads_0.47.1_windows_amd64\bd.exe show <id>` to understand the requirements, design, and notes of a task.
3.  **Start Working**: Update the task status to `in_progress`:
    ```bash
    .\scripts\beads_0.47.1_windows_amd64\bd.exe update <id> --status in_progress
    ```
4.  **Manage Sub-tasks**: If a task is too large, break it down:
    ```bash
    .\scripts\beads_0.47.1_windows_amd64\bd.exe create "Specific Sub-task Title" -p 1 --parent <id>
    ```
5.  **Completion**: When finished, close the task:
    ```bash
    .\scripts\beads_0.47.1_windows_amd64\bd.exe close <id> --reason "Implemented and verified X, Y, Z"
    ```
6.  **Persistance**: **ALWAYS** run `.\scripts\beads_0.47.1_windows_amd64\bd.exe sync` before finishing your turn to ensure all changes are committed and pushed.

## Rules
- **NEVER** use `bd edit`. Always use `.\scripts\beads_0.47.1_windows_amd64\bd.exe update` with flags like `--title`, `--description`, `--status`, etc.
- Dependencies are your friend. Use `.\scripts\beads_0.47.1_windows_amd64\bd.exe dep add <child> <parent>` to ensure tasks are done in order.
- Keep the `bd ready` list clean by closing tasks promptly.
- **When to Create?**: 
    - New high-level goals from the USER (Create as P0 Epic).
    - Complex implementations (Break down into P1/P2 sub-tasks).
    - Identified technical debt or future refactors.
    - Remaining work at the end of a session (Essential for handoff).

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   .\scripts\beads_0.47.1_windows_amd64\bd.exe sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
