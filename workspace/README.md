# Decision Workspace

`workspace/` is the active reasoning area for bounded features, architectural changes, investigations, and refactors.

A workspace preserves the path from an initial request to an accepted requirement and verified implementation.

## Core Model

```text
Input -> Evidence -> Understanding -> Proposal -> Decision
      -> Accepted Requirement -> Implementation Plan
      -> Implementation -> Verification
```

The workspace is not the final source of truth for accepted product knowledge. Once accepted, durable outputs are promoted to `docs/`, machine-readable contracts, source code, or tests.

## Information Classes

### Input

What stakeholders, external systems, or source materials stated. Preserve original meaning and provenance; do not silently rewrite it as fact.

### Evidence

Facts established through code inspection, experiments, system observation, standards, or other investigation.

### Understanding

The workspace's current interpretation of the problem, normally captured in `brief.md`. It may evolve while questions remain open.

### Proposal

A candidate solution or direction. Multiple competing proposals may coexist.

### Decision

An accepted conclusion, including rationale and consequences.

### Requirement

A stable, verifiable commitment promoted to `docs/requirements/` after acceptance.

### Plan

A staged implementation approach. Plans describe intent; they must not be rewritten later to pretend implementation followed them exactly.

### Delivery

Evidence of what was actually implemented, deviations from plan, requirement coverage, and verification results.

## Workspace Lifecycle

1. Create a bounded workspace from `workspace/templates/feature/`.
2. Capture original inputs without rewriting them.
3. Investigate the current system and record evidence.
4. Maintain a concise `brief.md` representing the current understanding.
5. Record unresolved matters in `questions.md`.
6. Compare proposals and record explicit decisions.
7. Promote accepted requirements, architecture, ADRs, and contracts into `docs/`.
8. Produce implementation plans only after requirements and key contracts are sufficiently stable.
9. Record actual delivery and verification.
10. Close the workspace without deleting its history.

## Naming

Use a short, descriptive, kebab-case identifier:

```text
workspace/features/alarm-acknowledgement/
workspace/architecture/protocol-binding-lifecycle/
workspace/investigations/telemetry-storage-options/
```

Avoid creating one permanent workspace for the entire project. Each workspace should have a clear scope and closure condition.
