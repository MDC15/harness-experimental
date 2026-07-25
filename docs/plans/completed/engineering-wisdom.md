# Execution Plan: Optional Engineering Wisdom Pack

Date: 2026-07-25

## Status

Completed

## Outcome

A consumer can explicitly install and invoke an `engineering-wisdom` skill,
while a normal Harness installation remains philosophy-neutral and excludes
the pack.

## Context

- The task contract authorizes one optional advisory pack and requires Unix and
  PowerShell installer parity.
- `docs/product/installation-profiles.md` defines the existing installation
  boundary.
- `scripts/harness-install-files.txt` is the default core authority and must
  remain unchanged.
- Existing skill packages use `agents/openai.yaml` with
  `allow_implicit_invocation: false`.

## Scope

In scope:

- One explicit-only skill covering code clarity, design, testing, refactoring,
  architecture, and professional practice.
- One separate optional manifest and matching Bash/PowerShell installer flags.
- Focused default-exclusion and explicit-inclusion proof.
- User-facing activation, use, non-activation, and removal documentation.

Out of scope:

- Changing default Harness policy or core payload.
- Automatically rewriting application architecture.
- Adding SQLite state or another control plane.
- Publishing, pushing, tagging, or opening a pull request.

## Approach

1. Add the skill with a compact workflow and a reference catalog. Require
   observation, heuristic, trade-off, proposed enforcement, and verification
   to remain distinct.
2. Add an optional manifest consumed only by
   `--with-engineering-wisdom` / `-WithEngineeringWisdom`.
3. Add focused installer proof and extend the Windows runtime contract.
4. Update the installation and repository maps, validate, and move this plan
   to `docs/plans/completed/`.

## Risks And Recovery

- Risk: advice becomes implicit policy. Mitigation: explicit-only metadata and
  output rules requiring repository authority before enforcement.
- Risk: the pack leaks into core. Mitigation: do not modify the core manifest;
  compare default and opt-in installations in executable proof.
- Risk: installer surfaces drift. Mitigation: use the same optional manifest
  and parallel flags on Bash and PowerShell.
- Recovery: remove the installer flag, optional manifest, and skill package.
  A consumer removes an installed copy by deleting only
  `.agents/skills/engineering-wisdom/`.

## Progress

- [x] Confirm repository authority and existing explicit-only conventions.
- [x] Implement the skill, installer opt-in, documentation, and proof.
- [x] Run focused and repository-required validation.
- [x] Record the verified result and move this plan to completed.

## Decisions

- 2026-07-25: Treat `engineering-wisdom` as an optional add-on rather than a
  third core/CLI product profile, so CLI selection and advisory selection stay
  independent.
- 2026-07-25: Keep detailed heuristics in one reference file and sources in a
  concise bibliography; the skill workflow stays small.
- 2026-07-25: Repository policy remains authoritative. The pack may propose
  enforcement but cannot apply it without explicit authority and validation.

## Validation

- Focused proof: `tests/installer/test-engineering-wisdom-opt-in.sh` passed.
- Integration or end-to-end proof:
  `tests/installer/test-install-harness-modes.sh` and the Windows
  `tests/installer/test-install-harness-modes.ps1` contract. Bash runtime and
  cross-platform static contracts passed; the Windows runtime assertion was
  added to its existing CI test but `pwsh` was not available locally.
- Repository-required checks: `scripts/validate-premerge.sh` passed after
  bootstrapping its expected ignored local CLI/database artifacts;
  `git diff --check` passed.

## Result

The optional `engineering-wisdom` skill now covers code clarity, SOLID and
design, testing, refactoring, architecture, and professional practice. Each
catalog heuristic states applicability, counter-pressure, a concrete example,
and verification.

The default core manifest is unchanged. Bash and PowerShell consume one
separate optional manifest only after explicit selection. Focused installation
proof observed default exclusion, opt-in inclusion, explicit-only metadata,
and non-removal on a later normal install.

No unresolved product risk remains. Windows runtime execution remains delegated
to the existing Windows CI installer test; local static parity and the full
repository gate passed.
