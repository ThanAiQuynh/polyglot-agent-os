# Beads (bd) Integration Guide

**Beads** is a distributed, git-backed graph issue tracker designed to provide AI agents with a structured and persistent memory. It replaces bulky markdown plans with a dependency-aware graph stored directly in your repository.

## 📦 Installation

Choose the method that fits your environment:

- **Local Binary (Already in-project)**:
  Available at `.\scripts\beads_0.47.1_windows_amd64\bd.exe`.
- **NPM (Optional)**:
  ```bash
  npm install -g @beads/bd
  ```
- **Go**:
  ```bash
  go install github.com/steveyegge/beads/cmd/bd@latest
  ```

---

## ⚡ Setup & Initialization

1. **Git Initialization**: (Project requirement)
   ```bash
   git init
   ```
2. **Initialize Beads**: (Must be run once by a human in the repo root)
   ```bash
   .\scripts\beads_0.47.1_windows_amd64\bd.exe init
   ```
3. **Standard Workflow**:
   ```bash
   # Optional: Install git hooks for automated syncing
   .\scripts\beads_0.47.1_windows_amd64\bd.exe hooks install
   ```
4. **Stealth Mode**: (If you want to use Beads locally without committing `.beads/` files to the repository)
   ```bash
   .\scripts\beads_0.47.1_windows_amd64\bd.exe init --stealth
   ```

---

## 🤖 Agent Workflow

AI agents in this project MUST use Beads for task tracking to maintain context and dependency awareness.

### Core Commands for Agents

| Command | Purpose |
| :--- | :--- |
| `.\scripts\beads_0.47.1_windows_amd64\bd.exe ready` | Lists tasks that are ready to be worked on (dependencies met). |
| `.\scripts\beads_0.47.1_windows_amd64\bd.exe create "Title" -p <0-4>` | Creates a new task with a priority (0=highest, 4=lowest). |
| `.\scripts\beads_0.47.1_windows_amd64\bd.exe update <id> --status <status>` | Updates task status (e.g., `in_progress`, `closed`). |
| `.\scripts\beads_0.47.1_windows_amd64\bd.exe show <id>` | Shows detailed information about a task. |
| `.\scripts\beads_0.47.1_windows_amd64\bd.exe dep add <child> <parent>` | Creates a dependency link between tasks. |
| `.\scripts\beads_0.47.1_windows_amd64\bd.exe sync` | **CRITICAL**: Flushes changes, commits to git, and pushes/pulls. |

### Important Rules for Agents
- **Don't use `bd edit`**: It opens an interactive terminal editor. Use `bd update` with flags instead.
- **Always `bd sync`**: At the end of a session, always run `bd sync` to ensure your state is persistent and shared.
- **Hierarchical IDs**: Use dot notation for sub-tasks (e.g., `bd-a3f8.1`).

---

## 🔗 Hierarchy Example

- `polyglot-agent-os-a3f8` (Epic)
  - `polyglot-agent-os-a3f8.1` (Task)
    - `polyglot-agent-os-a3f8.1.1` (Sub-task)

---

## 🌐 Documentation
- [Official GitHub Repository](https://github.com/steveyegge/beads)
- [Agent Instructions](https://github.com/steveyegge/beads/blob/main/AGENT_INSTRUCTIONS.md)
- [Installation Detailed](https://github.com/steveyegge/beads/blob/main/docs/INSTALLING.md)
