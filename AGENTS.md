# AGENTS.md

This file defines how coding and reasoning agents should work in the ClinkZ repository.

ClinkZ uses a workspace-driven development method. The purpose of this method is to preserve the path from an initial request to an accepted requirement, an explicit implementation plan, and verified delivery.

## Repository Operating Model

The repository separates active reasoning from durable project knowledge:

- `workspace/` contains bounded, evolving work.
- `docs/` contains accepted and durable project knowledge.
- contracts contain machine-readable interfaces and schemas.
- source code contains implementation.
- tests contain executable verification.

The workspace is not the final source of truth. Stable conclusions must be promoted to their authoritative destination.

## Before Starting Work

Before modifying code or durable documentation:

1. Read the repository-level `AGENTS.md`.
2. Read any more specific `AGENTS.md` files in the directories being changed.
3. Inspect the latest relevant source code, tests, contracts, and documentation.
4. Check for an existing workspace covering the same bounded problem.
5. Reuse and update that workspace when its scope matches; do not create competing workspaces for the same decision.

Do not assume that a plan, proposal, or earlier discussion reflects the current implementation. Verify current facts from the repository.

## When a Workspace Is Required

Create or use a workspace when the task involves one or more of the following:

- a new product capability;
- an architectural change;
- a cross-module refactor;
- a change to public APIs, contracts, schemas, or persistence models;
- a change with meaningful alternatives or unresolved questions;
- an investigation whose result may affect later implementation;
- work that spans multiple implementation steps or pull requests;
- a decision with consequences that future contributors must understand.

A workspace is usually unnecessary for a small, local, low-risk correction whose expected behavior is already established, such as:

- fixing a typo;
- correcting an obviously incorrect local implementation;
- adding a missing test for already documented behavior;
- mechanical formatting or dependency maintenance without design impact.

When uncertain, prefer a small bounded workspace over hiding reasoning in chat, commit messages, or code comments.

## Creating a Workspace

Use the closest template under `workspace/templates/`.

For a feature:

```text
workspace/templates/feature/
    -> workspace/features/<short-kebab-case-name>/
```

Each workspace must have:

- a bounded purpose;
- explicit scope and non-scope;
- expected durable outputs;
- closure criteria;
- a status that reflects its real state.

Do not create one permanent workspace for the entire project. A workspace must describe one bounded problem and have a clear end condition.

## Workspace Information Classes

Keep different information classes distinct. Do not silently convert one into another.

### Input

Original stakeholder requests, source material, external constraints, and initial ideas.

Preserve original meaning and provenance. Input is not automatically fact or an accepted requirement.

### Evidence

Facts established through code inspection, experiments, standards, system observation, tests, or other investigation.

Evidence should identify its source and distinguish observed facts from inference.

### Understanding

The current interpretation of the problem, normally summarized in `brief.md`.

Update the brief as understanding changes. Do not preserve obsolete interpretations as though they remain current.

### Questions

Unresolved matters that block or materially affect the decision.

Keep questions explicit. Mark them resolved with the resulting conclusion or a link to the decision that resolved them.

### Proposal

A candidate solution or direction.

Multiple competing proposals may coexist. A proposal is not accepted merely because it is the newest or most detailed document.

### Decision

An accepted conclusion with rationale, rejected alternatives, and consequences.

A local workspace decision may remain in the workspace. A durable cross-cutting architecture decision should be promoted to `docs/adr/`.

### Requirement

A stable and verifiable commitment.

Accepted requirements belong in `docs/requirements/`. Do not write implementation details as requirements unless they are intentional constraints.

### Plan

A staged implementation approach derived from sufficiently stable requirements and key contracts.

A plan records intended execution. Do not rewrite it after implementation to pretend the work followed the original plan exactly.

### Delivery

A factual record of what was actually implemented, including deviations, deferred work, and affected artifacts.

### Verification

Evidence that accepted requirements and contracts are satisfied.

Verification should point to tests, checks, experiments, or other reproducible evidence. “Implemented” is not verification.

## Required Workflow

For workspace-driven work, follow this progression:

```text
Input
  -> Evidence
  -> Understanding
  -> Proposals
  -> Decisions
  -> Accepted Requirements and Contracts
  -> Implementation Plan
  -> Implementation
  -> Delivery Record
  -> Verification
  -> Closure
```

This is a reasoning order, not a requirement to fill every template file mechanically. Use only the artifacts needed for the task, but do not skip unresolved decisions by disguising them as implementation details.

## Promotion to Durable Documentation

Promote stable outputs from the workspace to authoritative project locations:

- accepted product commitments -> `docs/requirements/`;
- system structure and enduring architectural descriptions -> `docs/architecture/`;
- durable architecture decisions -> `docs/adr/`;
- cross-feature or long-lived execution roadmaps -> `docs/plans/`;
- API, event, schema, and boundary definitions -> `docs/contracts/` or their machine-readable source;
- operational and development procedures -> `docs/guides/`;
- executable behavior -> source code and tests.

Promotion means extracting a clean, current, durable document. Do not simply copy the full reasoning history into `docs/`.

The workspace should link to promoted outputs, and durable documents should link back to relevant decisions when the history is useful.

## Planning Rules

Do not produce a detailed implementation plan while the requirement or key boundary contract remains materially ambiguous.

A good implementation plan:

- names affected modules and boundaries;
- orders work by dependency;
- identifies migrations and compatibility concerns;
- includes tests and verification work;
- calls out rollout, observability, and failure handling when relevant;
- separates required work from optional or deferred work.

During implementation, update task status and record discoveries that invalidate earlier assumptions.

## Implementation Rules

Implementation must follow accepted requirements and decisions, not merely the first proposal.

When implementation reveals a design problem:

1. stop treating the affected assumption as settled;
2. record new evidence;
3. reopen the relevant question or decision;
4. update requirements, contracts, or plans as needed;
5. continue only after the changed direction is explicit.

Do not bury architecture changes inside code patches.

Keep changes scoped to the active workspace. Unrelated cleanup should be avoided or separated unless it is required for correctness.

## Delivery and Verification Rules

Before declaring work complete:

- record what was actually delivered;
- list deviations from the plan;
- identify deferred or follow-up work;
- update all promoted durable documents;
- run relevant formatting, linting, tests, and validation;
- map verification evidence to the accepted requirements;
- ensure public contracts and examples match implementation;
- confirm the workspace closure criteria are satisfied.

If verification is incomplete, state that explicitly. Do not mark the workspace completed.

## Preserving History

Do not delete or rewrite workspace history merely because the final decision differs from an earlier proposal.

Preserve:

- meaningful alternatives;
- evidence used at the time;
- superseded decisions with clear status;
- deviations between plan and delivery.

Correct factual errors when discovered, but make the correction visible when the previous statement influenced a decision.

## Agent Communication

Agents should make uncertainty explicit and distinguish:

- repository facts;
- user-provided input;
- assumptions;
- inferences;
- proposals;
- accepted decisions.

When reporting completion, summarize:

- the workspace used or created;
- durable outputs added or changed;
- implementation changes;
- verification performed;
- unresolved or deferred work.

## Source of Truth

When artifacts disagree, use this precedence unless a more specific repository rule says otherwise:

1. accepted machine-readable contracts and executable tests;
2. current accepted requirements and ADRs;
3. current architecture documentation;
4. active workspace decisions and plans;
5. proposals, briefs, and raw inputs.

Do not resolve contradictions silently. Record and correct them in the appropriate authoritative artifact.
