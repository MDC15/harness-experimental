# Engineering Heuristic Catalog

Use a heuristic only when repository evidence makes its expected benefit
plausible. Each entry includes counter-pressure and a way to test the advice.

## Code Clarity

### Reveal intent in names and units

- Helps when a reader must mentally simulate a long routine or decode generic
  names before finding the business rule.
- Can hurt when extraction scatters one simple flow across many tiny functions
  or renaming replaces familiar domain language.
- Example: replace `process(data)` with a domain verb, or extract the tax
  decision from a mixed parse-calculate-persist routine, only when the new
  boundary tells the story more directly.
- Verify by tracing one representative change: count the files and jumps needed
  to locate the rule, then run its focused behavior test.

### Remove duplication after the shared concept is stable

- Helps when multiple copies represent the same rule and must change together.
- Can hurt when similar-looking code serves different policies; premature
  unification couples their future changes.
- Example: share one currency-rounding rule used by three invoices, but keep
  two coincidentally similar validation flows separate when their owners differ.
- Verify by inspecting change history or tests for common reasons to change and
  by changing one case without breaking the others.

## SOLID And Design

### Group code by reason to change

- Helps when one module mixes responsibilities owned by different actors or
  changes for unrelated reasons.
- Can hurt when splitting a cohesive, stable module creates ceremony and
  coordination overhead.
- Example: separate pricing policy from HTTP serialization when product and
  protocol changes evolve independently.
- Verify with recent diffs, ownership evidence, and dependency-focused tests:
  do unrelated changes stop touching the same unit?

### Put abstractions at volatile boundaries

- Helps when business policy depends directly on an unstable external service,
  storage detail, clock, or framework and needs independent testing.
- Can hurt when an interface has one stable implementation and merely mirrors
  it, increasing navigation without isolating volatility.
- Example: isolate a payment gateway contract, but do not wrap every standard
  library call.
- Verify by substituting the boundary in a focused test and by checking whether
  a likely provider change stays outside the policy code.

### Prefer substitutable contracts over type tests

- Helps when callers branch on concrete subtype and every new implementation
  requires another branch.
- Can hurt when variants genuinely have different capabilities and a single
  contract would hide invalid operations.
- Example: let delivery methods implement a common quote operation only if all
  can honor its preconditions; otherwise model capabilities explicitly.
- Verify with contract tests across implementations and tests for rejected
  unsupported operations.

## Testing

### Test observable behavior at the narrowest trustworthy boundary

- Helps when tests can protect a user-visible rule without binding to internal
  steps.
- Can hurt when an isolated test misses serialization, persistence, timing, or
  integration failures at the real boundary.
- Example: unit-test discount rules, then keep one API-and-database test for the
  wiring that matters.
- Verify by introducing a representative logic defect and a representative
  integration defect; the appropriate test should fail for each.

### Keep doubles at owned seams

- Helps when a slow or nondeterministic external dependency prevents fast,
  controlled feedback.
- Can hurt when mocks reproduce third-party behavior inaccurately or assertions
  lock tests to call order instead of outcomes.
- Example: fake an owned payment-port interface in policy tests and retain a
  provider sandbox contract test.
- Verify by comparing the double with the real boundary and by refactoring
  internal calls without changing behavior.

## Refactoring

### Separate behavior-preserving restructuring from feature change

- Helps when small steps make regressions and review causality easier to locate.
- Can hurt when forced commit separation creates churn for a trivial local
  rename or blocks an atomic migration that cannot exist halfway.
- Example: characterize a legacy parser, extract a seam, then add the new
  format; keep an inseparable schema-and-reader migration atomic.
- Verify that tests pass before and after each safe checkpoint and that diffs
  make behavior changes identifiable.

### Add characterization before changing unclear legacy behavior

- Helps when current behavior is relied on but not specified and a rewrite
  could erase edge cases.
- Can hurt when it canonizes an obvious defect or snapshots irrelevant output.
- Example: capture representative billing outputs before decomposing the
  calculator, explicitly excluding a confirmed rounding bug from the contract.
- Verify with production examples, issue history, or owner confirmation that
  each characterized behavior deserves preservation.

## Architecture

### Point dependencies toward stable business policy

- Helps when core rules are entangled with frameworks, delivery mechanisms, or
  storage and need to survive those changes.
- Can hurt in a small CRUD system where layers add mapping and indirection
  without protecting meaningful policy.
- Example: keep eligibility rules independent of the web framework, while
  allowing a simple admin record screen to use the framework directly.
- Verify with a dependency inspection and a focused test that executes policy
  without booting external infrastructure.

### Introduce boundaries where change cost justifies them

- Helps when teams, deployment, security, scaling, or change cadence require an
  explicit seam.
- Can hurt when speculative boundaries fragment transactions, duplicate models,
  or impose distributed-system costs.
- Example: extract an audit service only after independent retention or access
  requirements appear, not solely because it is a separate noun.
- Verify with measured change coupling, runtime requirements, and a reversible
  prototype before a large migration.

## Professional Practice

### Make uncertainty and commitments explicit

- Helps when hidden assumptions or optimistic estimates create surprise and
  quality pressure.
- Can hurt when exhaustive caveats obscure a small, reversible task.
- Example: state the unknown migration volume and the observation that will
  narrow it before committing to a cutover date.
- Verify by tracking assumptions, actual outcomes, and whether the next
  decision becomes clearer.

### Protect the definition of done

- Helps when schedule pressure invites skipped tests, concealed risk, or a
  completion claim without observable evidence.
- Can hurt when an inherited check is unrelated, flaky, or too expensive for
  the risk; in that case disclose and repair the proof gap rather than silently
  treating the check as law.
- Example: require the focused payment test and deployment smoke check for a
  payment change, not an arbitrary coverage percentage.
- Verify with repository-owned acceptance criteria, executable evidence, and
  explicit disclosure of any unrun or unreliable proof.
