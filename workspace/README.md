# Workspace

Workspace is the repository's active working area.

Unlike `docs/`, which stores accepted and long-term knowledge, a workspace captures work that is still evolving. It preserves reasoning, evidence, decisions, and implementation progress while the work is active.

A workspace is temporary.

Its purpose is to make the thinking process traceable until the work reaches an accepted outcome.

---

# When to Create a Workspace

Create a workspace whenever the work has a clear objective and its reasoning should be preserved.

Typical examples include:

* exploring a new idea
* evaluating a technology
* designing or changing architecture
* implementing a feature
* performing a significant refactoring

Small edits, typo fixes, or other isolated changes usually do not require a workspace.

---

# Workspace Lifecycle

A workspace typically evolves through the following stages:

```text
Idea
    ↓
Workspace
    ↓
Research / Design / Implementation
    ↓
Accepted Outcome
    ↓
Promote Stable Knowledge
    ↓
Archive
```

Not every workspace follows the same path.

The workflow should support the work rather than constrain it.

---

# Working in a Workspace

Every workspace starts with only one required file:

`README.md`

The workspace README should briefly describe:

* Purpose
* Scope
* Status
* Expected Outputs

Everything else is created only when it becomes useful.

Examples include:

* evidence/
* notes/
* proposal.md
* decision.md
* requirement.md
* plan.md
* delivery.md
* verification.md

Artifacts exist to support the work, not because a predefined template requires them.

---

# Promotion

A workspace is **not** the project's permanent source of knowledge.

Once work reaches an accepted outcome, stable information should be promoted into the repository.

Typical promotion targets include:

* `docs/`
* ADRs
* architecture documentation
* contracts
* tests
* source code

Working notes, proposals, brainstorming, and other intermediate artifacts should remain in the workspace.

---

# Closing a Workspace

A workspace should be considered complete when one of the following is true:

* the work has been completed
* the work has been abandoned
* the accepted outcomes have been promoted

Completed workspaces should be moved into `workspace/archive/`.

The archive preserves historical context without cluttering active work.

---

# Principles

* Workspaces preserve thinking.
* Documentation preserves accepted knowledge.
* Create artifacts only when they are useful.
* Promote stable knowledge.
* Archive completed work.
