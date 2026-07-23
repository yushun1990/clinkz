# ClinkZ Platform Shape Proposal

## Summary

ClinkZ should evolve as a **Thing-native IoT application platform**.

It should provide the common infrastructure required to describe, connect, operate, discover, secure, automate, and integrate Things.

It should not attempt to implement every vertical IoT application itself.

Applications such as SCADA, energy management, digital twins, reporting, MES integration, and industry-specific solutions should be built on top of ClinkZ or integrate with it through stable platform interfaces.

## Context

ClinkZ is currently described as:

> An open IoT platform built on W3C Web of Things, where everything is a Thing.

Its relationship with `clinkz-wot` is already defined at a high level:

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
```

`clinkz-wot` provides the protocol-neutral Web of Things runtime.

ClinkZ provides the platform capabilities around that runtime.

What remains undefined is the internal shape of the platform and its relationship with business applications.

## Problem

Commercial IoT projects commonly require capabilities such as:

* dashboards
* SCADA
* reports
* alarms
* automation
* historical data
* digital twins
* energy management
* GIS and 3D visualization
* ERP and MES integration
* industry-specific workflows

There are two undesirable extremes.

### Monolithic platform

ClinkZ could implement all of these capabilities directly.

This would eventually turn the core platform into a large collection of loosely related vertical features.

The result would be difficult to maintain, difficult to reuse, and difficult to adapt across industries.

### Minimal infrastructure

ClinkZ could provide only low-level Thing connectivity and APIs.

This would preserve a small core, but every real project would need to rebuild identity, history, automation, application integration, deployment, and user-facing tooling.

The result would be technically clean but difficult to adopt.

## Proposal

ClinkZ should be structured as three distinct layers.

```text
Vertical Applications and Existing Systems
                    |
                    v
       ClinkZ Application Platform
                    |
                    v
             ClinkZ Core
                    |
                    v
            clinkz-wot Runtime
                    |
                    v
            Protocol Bindings
```

## Layer 1: `clinkz-wot`

`clinkz-wot` remains a protocol-neutral Web of Things runtime.

Its responsibilities include:

* producing Things
* consuming Things
* discovering Things
* fetching Thing Descriptions
* Property, Action, and Event interaction
* protocol binding integration

It should not contain ClinkZ product, user interface, workflow, or business application concerns.

## Layer 2: ClinkZ Core

ClinkZ Core provides the shared infrastructure for operating a Thing-based system.

Its responsibilities may include:

* Thing registry
* Thing Description management
* discovery
* identity and access control
* Thing lifecycle management
* interaction routing
* event distribution
* historical data interfaces
* automation
* deployment and operational management
* platform APIs

ClinkZ Core owns the Thing layer.

It does not own every application built from Things.

## Layer 3: ClinkZ Application Platform

The Application Platform provides the common environment required to build applications on ClinkZ.

Its responsibilities may include:

* application SDKs
* application authentication context
* Thing access context
* history and event APIs
* application installation and lifecycle
* permission integration
* navigation and UI integration
* deployment integration
* shared frontend components
* AI-assisted development tools

The Application Platform allows vertical solutions to use ClinkZ capabilities without rebuilding the platform infrastructure.

## Application Model

Vertical functionality should be implemented as applications rather than added directly to the platform core.

Examples include:

* ClinkZ SCADA
* ClinkZ Energy
* ClinkZ Digital Twin
* ClinkZ Building
* ClinkZ Robotics
* ClinkZ Reports

An application may contain:

* frontend code
* backend services
* databases
* business logic
* platform permissions
* application-specific APIs
* application-specific data models

Applications consume ClinkZ capabilities through stable SDKs and APIs.

They may expose important business capabilities as Things, but their internal implementation does not need to be modelled entirely as Things.

## Integration Modes

ClinkZ should support two equally important integration modes.

### Build on ClinkZ

A vertical application is installed into or deployed alongside the ClinkZ platform.

```text
User
 |
 v
ClinkZ Application
 |
 v
ClinkZ Application Platform
 |
 v
ClinkZ Core
```

This mode is suitable for new applications and integrated product experiences.

### Embed ClinkZ

An existing business system keeps its own user interface, workflows, and business logic while using ClinkZ as its IoT infrastructure.

```text
Existing ERP / MES / Business System
                 |
                 v
          ClinkZ SDK or API
                 |
                 v
             ClinkZ Core
```

This mode is suitable for existing enterprise systems that should not be migrated into ClinkZ.

ClinkZ therefore needs to work both as an integrated platform and as a headless platform.

## SDK as the Primary Developer Interface

The preferred application integration surface should be a Thing-native SDK.

For example:

```rust
let pump = ctx.things().get("pump-01").await?;
let pressure = pump.read_property("pressure").await?;
```

The SDK should hide unnecessary platform internals such as:

* protocol bindings
* transport details
* registry lookup
* authentication propagation
* event subscription mechanics
* service locations

Applications should depend on Thing capabilities rather than infrastructure topology or device protocols.

## Dashboards

A basic dashboard capability may belong to the platform because it provides immediate visibility into newly connected Things.

However, the platform should not attempt to become a universal SCADA, BI, reporting, or visualization product.

The distinction is:

* basic Thing inspection and visualization are platform capabilities
* domain-specific operational interfaces are applications

## SCADA

SCADA should be implemented as an application on ClinkZ rather than as part of the core platform.

Common SCADA concepts map naturally to the Thing model:

```text
Tag       -> Thing Property
Command   -> Thing Action
Alarm     -> Thing Event
Trend     -> Historical Data
HMI       -> Application UI
```

ClinkZ provides the reusable infrastructure.

The SCADA application provides the operational experience and domain-specific behavior.

## Enterprise Systems

ERP, MES, and other enterprise systems may be represented as Things where useful.

For example, an ERP OpenAPI description could be transformed into one or more Thing Descriptions, allowing selected ERP capabilities to participate in the same interaction model as devices and services.

The ERP remains an independent business system.

ClinkZ provides a common capability and integration layer rather than replacing it.

## AI

AI belongs to the ClinkZ platform, not to `clinkz-wot`.

Potential AI-assisted workflows include:

* generating Thing Descriptions from manuals or API documentation
* configuring protocol bindings
* onboarding devices and systems
* generating dashboards
* generating automation rules
* scaffolding applications
* generating integration code
* assisting deployment and diagnostics

A likely onboarding flow is:

```text
Manual / OpenAPI / Register Map / Protocol Documentation
                           |
                           v
                          AI
                           |
                           v
             Thing Description and Binding
                           |
                           v
                   Deployed Thing
```

In the longer term, AI may also generate complete ClinkZ applications from existing Things and business requirements.

## Platform Boundary

ClinkZ should own:

* the Thing model
* Thing lifecycle
* Thing interaction
* discovery
* identity and authorization
* history interfaces
* event distribution
* automation infrastructure
* deployment infrastructure
* application integration infrastructure
* AI-assisted onboarding

ClinkZ should not directly own:

* complete SCADA products
* MES
* ERP
* energy management logic
* industry-specific workflows
* general-purpose BI
* general-purpose reporting
* domain-specific digital twins

Those belong to applications or external systems.

## Product Surfaces

ClinkZ may expose two distinct product surfaces.

### ClinkZ Console

The Console is the administration and operation interface for the platform.

Possible areas include:

* Things
* applications
* automation
* data
* deployments
* access control
* system administration

### ClinkZ Application Platform

The Application Platform is the developer-facing surface.

Possible components include:

* SDKs
* application runtime
* application manifest
* UI extension mechanisms
* permission model
* Thing context
* event and history clients
* development and deployment tooling

The Console is an application built on the platform rather than the definition of the platform itself.

## Application Packaging

Applications may eventually declare their platform integration through an application manifest.

For example:

```yaml
name: clinkz-energy
version: 0.1.0

frontend:
  entry: web

backend:
  service: energy-api

permissions:
  - things:read
  - history:read
  - automation:write

navigation:
  title: Energy
  path: /energy
```

The exact format is outside the scope of this proposal.

## Non-goals

This proposal does not decide:

* monolith versus microservices
* frontend framework
* storage implementation
* application isolation technology
* deployment orchestration technology
* exact SDK language support
* exact application manifest format
* commercial packaging

These decisions should follow after the platform shape is accepted.

## Consequences

If accepted, this proposal establishes the following direction:

1. `clinkz-wot` remains a focused WoT runtime.
2. ClinkZ Core provides common Thing infrastructure.
3. ClinkZ provides an application platform above the core.
4. Vertical capabilities are developed as applications.
5. Existing systems integrate through SDKs and APIs.
6. ClinkZ supports both hosted and headless usage.
7. AI assists the creation of Things and applications.
8. The core remains small by keeping domain logic outside it.

## Open Questions

The following questions require later work:

* What is the minimum ClinkZ Core?
* What belongs in the first Application SDK?
* Is an application runtime required for the first release?
* How are applications packaged and deployed?
* How are application permissions represented?
* How are frontend routes and navigation extended?
* Which historical data responsibilities belong to the platform?
* What is the minimum built-in dashboard capability?
* Which end-to-end application should validate the architecture first?
* Which parts should become formal requirements or ADRs?

## Acceptance Criteria

This proposal is ready to be accepted when the project agrees on:

* the definition of ClinkZ as a platform
* the boundary between core capabilities and applications
* the two application integration modes
* the role of the Application SDK
* the role of AI
* the first vertical application used to validate the model


