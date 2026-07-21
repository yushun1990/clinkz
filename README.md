# ClinkZ

> An open IoT platform built on W3C Web of Things, where everything is a Thing.

ClinkZ is an open-source IoT platform built around the [W3C Web of Things](https://www.w3.org/WoT/) model.

It aims to provide a unified, protocol-independent foundation for connecting devices, edge systems, cloud services, applications, and software capabilities as **Things**.

## Vision

Traditional IoT platforms often couple device protocols, platform internals, and application logic. As systems grow, these couplings make integration, evolution, and reuse increasingly difficult.

ClinkZ takes a different approach:

- **Everything is a Thing.** Devices, services, data sources, and software capabilities share one interaction model.
- **Applications depend on capabilities, not protocols.** MQTT, Zenoh, HTTP, and other transports remain binding concerns.
- **Thing Descriptions are contracts.** Capabilities and interactions are described declaratively using W3C WoT concepts.
- **Edge and cloud form one system.** The same model should work across constrained, edge, and cloud environments.

## Relationship to `clinkz-wot`

ClinkZ is the platform built on top of [`clinkz-wot`](https://github.com/yushun1990/clinkz-wot), the protocol-neutral WoT runtime.

```text
Applications and Operators
           |
           v
      ClinkZ Platform
           |
           v
      clinkz-wot Runtime
           |
           v
     Protocol Bindings
           |
     +-----+-----+-----+
     |           |     |
   Zenoh       MQTT   HTTP ...
```

## Repository Model

This repository is intended to evolve as the ClinkZ platform monorepo. Frontend applications, backend services, shared contracts, deployment assets, product documentation, and decision workspaces will live together while retaining independent build and deployment boundaries.

```text
clinkz/
├── apps/          # User-facing applications
├── services/      # Deployable backend services
├── crates/        # Shared Rust libraries
├── packages/      # Shared frontend packages, SDKs, and contracts
├── docs/          # Accepted, long-lived project knowledge
├── workspace/     # Active investigation and decision work
├── tests/         # Cross-component integration and end-to-end tests
└── deploy/        # Deployment and operations assets
```

Only `docs/` and `workspace/` are established in the initial repository skeleton. Product code will be added as architectural and product requirements converge.

## Documentation and Workspace

ClinkZ distinguishes durable project knowledge from active reasoning:

- `workspace/` contains evolving inputs, evidence, questions, proposals, decisions, and implementation planning for a bounded initiative.
- `docs/` contains accepted requirements, architecture, ADRs, and contracts that remain authoritative after a workspace closes.
- source code contains the implementation.
- tests and delivery evidence demonstrate that the implementation satisfies the accepted requirements.

The intended flow is:

```text
Input -> Evidence -> Understanding -> Decision
      -> Requirement -> Plan -> Implementation -> Verification
```

See [`workspace/README.md`](workspace/README.md) for the initial method and templates.

## Project Status

ClinkZ is in its early architecture and foundation phase.

Current focus:

- validating the Workspace method;
- defining platform requirements and system boundaries;
- evolving `clinkz-wot` as the protocol-neutral runtime;
- preparing the first end-to-end platform slice.

The README intentionally avoids promising features that have not yet been accepted or implemented.

## Contributing

The project is currently design-led. Architectural review, requirement clarification, and implementation feedback are welcome.

Before proposing substantial implementation work, start or join a workspace so that inputs, evidence, decisions, and requirements remain traceable.

## License
Licensed under the [Apache License 2.0](LICENSE).
