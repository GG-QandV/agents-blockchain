---
name: system-architect
description: 'Holistic system architecture design: C4 model, architectural patterns,
  system design methodology, trade-off analysis, technology selection, and capacity
  planning.'
---

# System Architect

Design scalable, maintainable, and cost-effective system architectures. Covers the full spectrum from requirements analysis through architecture evaluation.

## When to Activate

- New project or greenfield system design
- Major refactoring or re-architecture initiative
- Technology selection or migration evaluation
- System design interview preparation
- Architecture review requires design-level analysis
- Performance, scalability, or cost issues require architectural solution

## Architecture Design Process

### Phase 1: Requirements & Constraints

1. **Business objectives** — what problem is being solved? Success criteria?
2. **Functional requirements** — core capabilities the system must provide
3. **Non-functional requirements** (the architect's domain):
   - Scalability (users, data volume, throughput)
   - Availability (uptime SLAs, RTO/RPO)
   - Latency (P50/P95/P99 targets)
   - Security (compliance, data classification)
   - Cost (budget constraints, operational costs)
   - Maintainability (team size, deployment frequency)
4. **Constraints** — existing systems, team skills, budget, timeline, regulatory

### Phase 2: High-Level Architecture

Define system boundaries, major components, and their interactions:

1. **System context** — what is inside vs outside your system?
2. **Container view** — applications, services, databases, message queues
3. **Technology stack selection** — frameworks, databases, infrastructure
4. **Integration patterns** — sync vs async, event-driven vs request/response
5. **Data flow** — how data moves through the system end-to-end

### Phase 3: Component Design

For each major component:

1. **API boundaries** — interfaces, contracts, protocols
2. **Data model** — schemas, storage strategy, caching approach
3. **Internal architecture** — layers, modules, patterns
4. **Failure modes** — what breaks? How does it degrade?
5. **Observability** — metrics, logs, traces for this component

### Phase 4: Cross-Cutting Concerns

1. **Authentication/authorization** — identity model, token strategy, permissions
2. **Observability** — distributed tracing, structured logging, metrics aggregation
3. **Deployment** — CI/CD strategy, environment topology, canary/blue-green
4. **Security** — encryption (at rest, in transit), secrets management, network zones
5. **Disaster recovery** — backup strategy, failover plan, RTO/RPO validation

### Phase 5: Architecture Evaluation

1. **Trade-off analysis** — document what is gained and lost with each decision
2. **Risk assessment** — identify architectural risks (unknowns, brittle points)
3. **Review against requirements** — does this architecture satisfy all NFRs?
4. **Lightweight ATAM** — evaluate modifiability, performance, availability scenarios

## Architecture Patterns (Decision Matrix)

| Pattern | Best for | Complexity | When NOT to use |
|---------|----------|-----------|-----------------|
| **Modular Monolith** | Small teams, early stage, simple domains | Low | Distributed team, independent scaling needed |
| **Layered (n-tier)** | CRUD apps, well-understood domains | Low-medium | Complex domain logic (use hexagonal instead) |
| **Hexagonal (Ports & Adapters)** | Domain-driven, testability-critical | Medium | Simple CRUD, overkill for small projects |
| **Clean Architecture** | Large codebases, framework independence | Medium-High | Simple apps where framework coupling is acceptable |
| **Microservices** | Large team, independent deployability | High | Small team, simple domain, network overhead |
| **Event-Driven** | Async workflows, real-time processing, decoupling | High | Simple request-response, strong consistency needed |
| **CQRS** | Read/write disparity, complex queries | Medium | Simple CRUD, single-model-works scenarios |
| **Event Sourcing** | Audit trail, temporal queries, complex state | High | Simple state, storage cost concerns |
| **Saga** | Distributed transactions, compensating actions | High | Single-service, local-transaction scenarios |
| **Strangler Fig** | Legacy migration, incremental replacement | Medium | Greenfield, small systems |

## Technology Selection Framework

1. **Define selection criteria** weighted by project context:
   - Team expertise (30%)
   - Operational maturity (25%)
   - Community/ecosystem (20%)
   - Performance characteristics (15%)
   - License/cost (10%)
2. **Shortlist 3-5 candidates** per category
3. **Evaluate each against criteria** with evidence (not opinion)
4. **Document the decision** as an ADR with rejected alternatives
5. **Validate with a proof of concept** for high-risk choices

## C4 Model (Visual Notation Guidelines)

Use C4 levels progressively based on audience:

| Level | Audience | Elements | Abstraction |
|-------|----------|----------|-------------|
| **Context** (L1) | Everyone | System, users, external systems | One box per system |
| **Container** (L2) | Technical team | Apps, services, databases, queues | Running processes + data stores |
| **Component** (L3) | Developers | Modules, controllers, repositories | Inside a single container |
| **Code** (L4) | Developers | Classes, interfaces, functions | UML-level detail (on-demand only) |

- L1-L2 are mandatory for any architecture document
- L3 is needed for complex containers only
- L4 is rarely useful — prefer code + tests

## Scalability Dimensions

| Dimension | Strategy | Common patterns |
|-----------|----------|-----------------|
| **Read throughput** | Cache + CDN + read replicas | Cache-aside, CDN distribution, read-through cache |
| **Write throughput** | Partition + async processing | Command queue, event stream, database sharding |
| **Data volume** | Partition + archive + summarization | Time-based partitioning, cold storage, roll-up tables |
| **Users** | Horizontal scale + connection pooling | Stateless services, auto-scaling groups, connection multiplexing |
| **Geographic** | Multi-region + global routing | Active-active, active-passive, global load balancer |
| **Team size** | Service boundary + API contract | Microservices, API versioning, contract testing |

## Output Format

```
# Architecture: <system-name>

## Context (L1)
<!-- one-paragraph system context with diagram reference -->

## Containers (L2)
<!-- key containers, their responsibilities, and interactions -->

## Architectural Decisions
- ADR-001: <decision with rationale>
- ADR-002: <decision with rationale>

## Key Trade-offs
| Decision | Chosen | Rejected | Rationale |
|----------|--------|----------|-----------|

## Risk Register
| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|

## Technology Stack
<!-- table of technology choices with justification -->
```

## Architecture Quality Gates

Apply these gates to every architecture design before implementation:

### 1. Concept Integrity
- Architecture directly serves product goals and constraints
- Responsibility boundaries are explicit and stable
- No components exist without clear business/technical purpose

### 2. Separation of Concerns
- Domain logic is isolated from transport/UI/infrastructure
- Cross-cutting concerns (auth, logging, retries, observability) are centralized, not duplicated

### 3. Coupling & Cohesion
- Coupling between modules is intentional, minimal, and observable
- High internal cohesion; low external dependency fan-out
- No parasitic links or hidden runtime coupling

### 4. Data & Control Flow Clarity
- Single source of truth for critical state
- Data ownership, schema boundaries, and flow direction are unambiguous
- No circular flows without explicit control mechanisms

### 5. Failure Semantics
- Failure modes are known (timeouts, retries, backpressure, partial writes, split brain)
- Degraded behavior is defined, not accidental
- Error handling does not create data corruption or silent loss

### 6. Operability
- Health checks, metrics, logs, and trace points are sufficient for diagnosis
- Runbook-level behavior is clear: startup order, dependency health, recovery

### 7. Scalability & Performance Envelope
- Hot paths and bottlenecks are identified
- Capacity assumptions are explicit
- Architecture can scale without violating invariants

### 8. Security & Trust Boundaries
- Trust boundaries are explicit at process/network/storage levels
- AuthN/AuthZ model matches exposure surface
- Sensitive flows are minimized and auditable

### 9. Evolvability
- Change impact is localized where possible
- Contracts/versioning strategy prevents cascading breakages
- Migration paths are feasible under real constraints

### 10. Spec-to-Code Alignment
- Docs/specs/ADRs and implementation agree on ports, protocols, ownership, behavior
- Contradictions are reported as first-class findings

## Pattern Implementation Guides

### Hexagonal Architecture (Ports & Adapters)

Core structure:
- **Domain model** — business rules and entities. No framework imports.
- **Use cases (application layer)** — orchestrate domain behavior.
- **Inbound ports** — contracts for what the app can do.
- **Outbound ports** — contracts for dependencies (repositories, gateways, clock, UUID).
- **Adapters** — implementations of ports (HTTP controllers, DB repos, queue consumers).
- **Composition root** — single wiring location.

Dependency direction is always inward: Adapters → Application → Domain → nothing external.

For full implementation guide covering TypeScript, Java, Kotlin, and Go, see the `hexagonal-architecture` skill. Key anti-patterns:
- Domain entities importing ORM models, web framework types, or SDK clients
- Use cases reading directly from HTTP request/response objects
- Returning database rows directly from use cases without domain/application mapping
- Adapters calling each other instead of flowing through use-case ports

### Clean Architecture (Android/KMP)

For mobile and Kotlin Multiplatform projects:
- `domain` — UseCases, domain models, repository interfaces (pure Kotlin, no framework deps)
- `data` — Repository implementations, DataSources, DB, network
- `presentation` — Screens, ViewModels, UI models, navigation

Critical rule: `domain` must NEVER depend on `data`, `presentation`, or any framework.

For full guide, see the `android-clean-architecture` skill.

## Related Skills

- `architecture-decision-records` — capture decisions during design
- `architecture-diagram` — visualize architecture as SVG/HTML
- `architecture-compliance` — verify architecture is followed
- `architecture-documentation` — publish architecture docs
- `architecture-review` — structured architecture review with quality gates
- `hexagonal-architecture` — detailed ports & adapters implementation
- `android-clean-architecture` — clean architecture for Android/KMP
- `backend-architect` — detailed backend/service design
- `ai-architect` — AI/ML system architecture
