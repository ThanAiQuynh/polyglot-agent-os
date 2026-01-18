# Agent Instructions: Task Tracking with Beads (bd)

> [!IMPORTANT]
> **STRICT ENFORCEMENT**: This repository follows the standards defined in `.agent/rules/project-standards.md`. Violation of these rules is considered a critical failure.

You MUST use the **Beads (bd)** library for task tracking in this repository. Beads provides a structured graph of tasks that persists across sessions via git.

## Workflow

0.  **Bootstrap**: Before any action, you MUST follow the bootstrap procedure in `.agent/rules/project-standards.md`.
1.  **Check Status**: Begin every session by running `bd ready` to see what tasks are available for you.
2.  **Pick a Task**: Use `bd show <id>` to understand the requirements, design, and notes of a task.
3.  **Start Working**: Update the task status to `in_progress`:
    ```bash
    bd update <id> --status in_progress
    ```
4.  **Manage Sub-tasks**: If a task is too large, break it down:
    ```bash
    bd create "Specific Sub-task Title" -p 1 --parent <id>
    ```
5.  **Completion**: When finished, close the task:
    ```bash
    bd close <id> --reason "Implemented and verified X, Y, Z"
    ```
6.  **Persistance**: **ALWAYS** run `bd sync` before finishing your turn to ensure all changes are committed and pushed.

## Rules
- **NEVER** use `bd edit`. Always use `bd update` with flags like `--title`, `--description`, `--status`, etc.
- Dependencies are your friend. Use `bd dep add <child> <parent>` to ensure tasks are done in order.
- Keep the `bd ready` list clean by closing tasks promptly.
