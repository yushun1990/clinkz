# AGENTS.md

# ClinkZ Repository Working Agreement

This repository follows a workspace-driven development model.

A workspace is a bounded area for active work. Its purpose is to preserve reasoning, decisions, and evidence while work is in progress.

Workspaces are temporary.

Accepted outcomes are promoted into the project's permanent knowledge (`docs/`), executable contracts, tests, or source code.

---

# Before Starting Work

Before making significant changes, determine:

- What problem is being solved?
- Is there already an active workspace?
- What type of work is this?

Reuse an existing workspace whenever practical.

Create a new workspace only when the work has a distinct objective.

---

# Workspace Types

Choose the workspace type that best matches the work.

Typical workspace categories include:

- research
- architecture
- feature
- refactoring

A workspace represents a bounded piece of work, not a project phase.

---

# Workspace Structure

Each workspace starts with only one required file:

README.md

The README should briefly describe:

- purpose
- scope
- current status
- expected outputs

Everything else is created only when needed.

Examples:

- evidence/
- notes/
- proposal.md
- decision.md
- requirement.md
- plan.md
- delivery.md
- verification.md

Do not create artifacts that are unlikely to be used.

Artifacts exist to support the work, not because a template requires them.

---

# Workspace Flow

A typical workspace evolves naturally.

Input

↓

Evidence

↓

Understanding

↓

Questions

↓

Proposal

↓

Decision

↓

Outputs

Not every workspace passes through every stage.

The flow should follow the work rather than forcing the work to follow the flow.

---

# Outputs

Workspace outputs depend on the type of work.

Research may produce:

- architecture documents
- design principles
- future work

Architecture may produce:

- architecture documentation
- ADRs
- interface contracts

Feature work may produce:

- requirements
- implementation plans
- source code
- tests
- user documentation

Refactoring may produce:

- implementation improvements
- simplified architecture
- migration notes

---

# Promotion

Workspaces are temporary.

Only stable, accepted knowledge should be promoted.

Typical promotion targets include:

- docs/
- ADRs
- architecture documents
- contracts
- tests
- source code

A workspace should never become the long-term source of truth.

---

# Source of Truth

When information conflicts, use the following order:

1. Executable contracts
2. Tests
3. Source code
4. Accepted documentation
5. Workspace decisions
6. Workspace proposals
7. Raw notes

---

# Agent Principles

Work incrementally.

Record important decisions.

Keep assumptions explicit.

Separate facts from opinions.

Separate proposals from decisions.

Avoid speculative implementation.

Prefer evidence over intuition.

Promote stable knowledge.

Delete obsolete reasoning instead of maintaining contradictory documents.

Keep workspaces focused.

Avoid unnecessary structure.

The workflow exists to support thinking, not to constrain it.
