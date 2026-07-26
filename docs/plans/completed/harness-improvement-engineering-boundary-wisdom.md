# Harness Improvement: Engineering Boundary Wisdom

Date: 2026-07-26

## Status

Completed

## Representative Job

Strengthen the optional, explicit-only `engineering-wisdom` skill using the
accepted read-only audit of `unclebob/missile-command` at
`f16f8ab94e0eb729840ebf3d9fc53e4f6d02165c`. The accepted outcome is a
repository-grounded review that tests host boundaries and delivered behavior
without treating one audit as universal policy.

The fixed worker is Codex in `repository-harness` at
`5ceb95c3dc5819a99a0443e174e1b4d2628ac8d4`, with local read/write and commit
authority, no network or external-action authority, and the audit evidence and
pinned clone available read-only.

## Baseline

The audited repository had a strong pure shared core and thin JVM/browser
hosts, but boundary behavior escaped its proof:

- browser startup invoked the composition root twice;
- browser simulation used a fixed frame increment while the JVM host measured
  wall-clock time;
- resize updated only part of live state;
- persisted and QA input was parsed without sufficient safety or schema checks;
- browser build and QA automation could report success after build failure;
- cumulative sound events were never consumed or truncated; and
- a caller path was embedded in shell source.

The evidence is
`/Users/tubakhuym/projects/khuym/.herdr-runtime/unclebob-missile-command/ledger/repository-audit.evidence.json`.
The original audit required human relay into this improvement task. Its native
unit, acceptance, architecture, and property commands could not run because
Babashka and Clojure were unavailable, so the baseline is strong source
evidence, not runtime reproduction.

## Earliest Gap

**Proof:** the existing catalog says to test observable behavior at the
narrowest trustworthy boundary, but does not make composition roots, adapter
semantics, shipped artifacts, defensive boundary decoding, or bounded
cumulative state concrete enough for a future agent to select and verify.

## Correct Owner

`repository-harness`, specifically the optional `engineering-wisdom` pack,
owns reusable advisory heuristics. Consumer repositories still own their
runtime semantics, enforcement, and acceptance criteria.

## Intervention

If concise boundary-review prompts are added to `SKILL.md` and detailed,
cause-and-effect boundary heuristics are added to `references/heuristics.md`,
then a fresh agent will identify host-level failure modes and propose
claim-matched verification on an equivalent repository review, because the
skill will route it beyond isolated core tests to composition roots, external
data, adapter semantics, shipped artifacts, and cumulative state.

Evidence that would weaken this: the fresh agent does not invoke or retrieve the
skill, repeats the examples as universal rules, misses the observed boundary
risks, or proposes checks that do not exercise delivered behavior.

Maintenance owner: `repository-harness`. Remove or revise the added guidance if
fresh reviews do not use it, it creates policy without consumer authority, or
its context cost exceeds its review value.

## Native Validation

- `python3 /Users/tubakhuym/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/engineering-wisdom`
  passed.
- `tests/installer/test-engineering-wisdom-opt-in.sh` passed, proving default
  exclusion, explicit inclusion, installed guidance, and explicit-only
  metadata.
- `tests/installer/assert-install-manifest-links.sh` passed, proving exact
  optional payload and installed links.
- `bash -n tests/installer/test-engineering-wisdom-opt-in.sh` and
  `git diff --check` passed.
- `agents/openai.yaml` remains aligned: the five-part review prompt is still
  accurate and `allow_implicit_invocation: false` remains unchanged.

## Fresh Rerun

A fresh Codex session received only the updated skill path, pinned repository,
and a read-only request to assess whether tests and automation prove behavior
across JVM and browser hosts.

- **Available:** yes, at the supplied optional skill path.
- **Retrieved/invoked:** yes; the response used the skill's exact five-part
  finding shape.
- **Relevant:** yes; it independently identified false-green browser artifact
  QA, duplicate composition-root startup, persistence tests bypassing host
  adapters, real-input QA bypass, and wall-clock/fixed-frame semantic drift.
- **Proof quality:** it cited concrete paths and proposed clean-build artifact
  loading, exactly-one-canvas startup, real persistence reloads, host input, and
  delayed-frame cross-adapter checks.
- **Human intervention/retries:** none.
- **Authority behavior:** it kept the review read-only, proposed no enforcement,
  and disclosed that runtime launch remained unperformed.
- **Maintenance cost:** 6 lines in the main workflow and 84 lines in the
  progressively loaded catalog, plus focused installed-payload assertions.

## Decision

Keep.

The rerun exercised the intervention and improved the bounded review from
shared-core/source evidence to claim-matched host-boundary proof without
promoting advice into repository policy.

## Result

The optional skill now prompts agents to inspect composition roots, boundary
data, adapter semantics, shipped artifacts, honest automation status, and
bounded cumulative state when repository evidence makes them relevant.

Detailed cause-and-effect examples remain in the progressive reference catalog.
The main skill stays concise, evidence-first, explicit-only, and subordinate to
consumer repository authority. No external systems were changed. The source
audit's unavailable Babashka/Clojure runtime proof remains a baseline
limitation, not a limitation of this repository's validation.
