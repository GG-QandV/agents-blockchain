---
name: architecture-review
description: Review and design software architecture using gold-standard quality gates.
  Use when users need architecture critique, draft/spec evaluation, tradeoff analysis,
  or detection of gaps, bottlenecks, overlap, hidden coupling, or concept-to-implementation
  misalignment.
---

# Architecture Review

## When to Use
Use this skill when the task is about:
- Architecture design or redesign
- Review of drafts/specs/ADRs before implementation
- Audit of codebase architecture against stated goals
- Comparing architecture options with tradeoffs
- Finding risks: bottlenecks, tails, gaps, overlaps, parasitic couplings, hidden dependencies
- Validating consistency between concept, constraints, and implementation reality

Do not use this skill for pure style/lint/code-format tasks.

## Review Contract
Always produce outputs in this order:
1. `Findings` — concrete issues first, sorted by severity
2. `Impact` — why each finding matters (reliability, scale, security, cost, delivery)
3. `Evidence` — files/spec sections/diagrams/assumptions used
4. `Decision Options` — minimally sufficient alternatives with tradeoffs
5. `Recommendation` — one preferred path with rationale and rollback/fallback
6. `Validation Plan` — checks/tests/metrics proving the decision works

If no major issues are found, state that explicitly and list residual risks.

## Gold Standards (Quality Gates)
Apply these gates systematically:

1. `Concept Integrity`
- The architecture directly serves product goals and constraints.
- Responsibility boundaries are explicit and stable.
- No components exist without clear business/technical purpose.

2. `Separation of Concerns`
- Domain logic is isolated from transport/UI/infrastructure.
- Cross-cutting concerns (auth, logging, retries, observability) are centralized, not duplicated.

3. `Coupling & Cohesion`
- Coupling between modules is intentional, minimal, and observable.
- High internal cohesion; low external dependency fan-out.
- Detect and flag parasitic links and hidden runtime coupling.

4. `Data & Control Flow Clarity`
- Single source of truth for critical state.
- Data ownership, schema boundaries, and flow direction are unambiguous.
- No circular flows without explicit control mechanisms.

5. `Failure Semantics`
- Failure modes are known (timeouts, retries, backpressure, partial writes, split brain).
- Degraded behavior is defined, not accidental.
- Error handling does not create data corruption or silent loss.

6. `Operability`
- Health checks, metrics, logs, and trace points are enough for diagnosis.
- Runbook-level behavior is clear: startup order, dependency health, recovery.

7. `Scalability & Performance Envelope`
- Hot paths and bottlenecks are identified.
- Capacity assumptions are explicit.
- Architecture can scale without violating invariants.

8. `Security & Trust Boundaries`
- Trust boundaries are explicit at process/network/storage levels.
- AuthN/AuthZ model matches exposure surface.
- Sensitive flows are minimized and auditable.

9. `Evolvability`
- Change impact is localized where possible.
- Contracts/versioning strategy prevents cascading breakages.
- Migration paths are feasible under real constraints.

10. `Spec-to-Code Alignment`
- Docs/specs/ADRs and implementation agree on ports, protocols, ownership, and behavior.
- Contradictions are reported as first-class findings.

## Review Procedures

### A) Draft/Spec Review (pre-code)
1. Extract goals, constraints, non-goals.
2. Build component/responsibility map from the draft.
3. Run all Quality Gates.
4. Stress-test assumptions with 3 lenses:
- failure lens,
- growth lens,
- operations lens.
5. Produce findings + option set.

### B) Code-to-Architecture Review
1. Derive actual architecture from code/runtime configs.
2. Compare actual state to declared architecture/spec.
3. Classify mismatches:
- harmless drift,
- risky drift,
- breaking contradiction.
4. Propose minimal corrective actions first.

### C) Option Comparison
For each option, score (Low/Medium/High risk):
- Delivery risk
- Runtime risk
- Operational complexity
- Migration cost
- Reversibility

Prefer options with highest reversibility under uncertainty.

## Anti-Patterns to Detect
Flag explicitly when found:
- God-component / mega-adapter
- Transport logic mixed with domain decisions
- Hidden shared mutable state
- Circular dependency chains
- Implicit protocol contracts (undocumented)
- Side effects hidden in convenience layers
- One-way architectural door without rollback path
- Spec says A, code does B (port/protocol/ownership drift)

## Output Format
Use this compact structure:

`Architecture Verdict`: Green | Yellow | Red

`Critical Findings`:
- [Severity] Finding
  - Evidence
  - Impact
  - Minimal fix

`Decision Options`:
1. Option name — tradeoff summary
2. Option name — tradeoff summary

`Recommended Path`:
- Choice
- Why now
- Rollback trigger

`Validation`:
- Technical checks
- Runtime signals/metrics
- Acceptance criteria

## Decision Rules
- Prefer minimal viable correction over broad refactor.
- Prefer explicit contracts over convention.
- Prefer observable behavior over implicit behavior.
- Prefer migration-safe changes over irreversible rewrites.
- If data safety and speed conflict, protect correctness first.
