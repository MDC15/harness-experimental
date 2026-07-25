# Installation Profiles

The installers expose two product profiles plus one independent,
explicit-only advisory add-on. The default remains philosophy-neutral.

## Core

Core is the default. Its exact files are declared in
`scripts/harness-install-files.txt`:

```text
.agents/skills/audit-onboarding-proposal/SKILL.md
.agents/skills/audit-onboarding-proposal/agents/openai.yaml
.agents/skills/audit-onboarding-proposal/scripts/validate_evidence_capsule.py
.agents/skills/onboard-repository/SKILL.md
.agents/skills/onboard-repository/agents/openai.yaml
.agents/skills/onboard-repository/references/evidence-capsule-v1.md
.agents/skills/onboard-repository/references/evidence-capsule-v2.md
.agents/skills/onboard-repository/scripts/emit_evidence_bundle.py
.agents/skills/onboard-repository/scripts/render_patch.py
AGENTS.md
docs/WORKFLOW.md
docs/README.md
docs/product/README.md
docs/plans/README.md
docs/plans/active/README.md
docs/plans/completed/README.md
docs/decisions/README.md
docs/templates/decision.md
docs/templates/exec-plan.md
```

The installed `$onboard-repository` and `$audit-onboarding-proposal` skills are
explicit-only. Installation and ordinary repository work never invoke them.
When requested, the producer inspects and proposes without editing on its first
pass, while the auditor independently verifies the proposal before an approved
application.

The platform installer downloads a checksum-verified `harness` binary, places
it at `scripts/bin/harness` (or `.exe`), and delegates core installation to it.
The CLI records the exact upstream base in `.harness-core/`; future updates use
that base for a conflict-safe three-way merge and persistent backup.

Core performs no compatibility-CLI download, schema discovery, database
bootstrap installation, or database-specific `.gitignore` write. A core update
does not remove an existing `harness-cli` or database.

Core also excludes `.agents/skills/engineering-wisdom/`. The optional
engineering manifest is separate from `scripts/harness-install-files.txt`, so
normal install and update behavior cannot silently add that advice.

## Core Plus CLI

`--with-cli` in Bash or `-WithCli` in PowerShell adds the optional
compatibility manifest at
`scripts/harness-cli-install-files.txt`, every `scripts/schema/*.sql` migration,
generated database/binary ignore rules, and a checksum-verified platform
binary. `--upgrade-cli` / `-UpgradeCli` implies this profile.

The compatibility inputs and binary are staged before compatibility target
files change. A staging, download, checksum, or apply failure restores the
previous compatibility files. Core files already installed remain usable.

## Engineering Wisdom Add-on

`--with-engineering-wisdom` in Bash or `-WithEngineeringWisdom` in PowerShell
copies the files declared in
`scripts/engineering-wisdom-install-files.txt`. This selection is independent
of the core/CLI profile, so either profile may include the add-on.

Concrete behavior:

1. No flag: the installer does not read the optional manifest or copy the
   skill.
2. Opt-in flag: the installer copies the `engineering-wisdom` skill and its
   explicit-only metadata.
3. Installed skill: nothing runs until the consumer invokes
   `$engineering-wisdom`.
4. Later install without the flag: an existing copy is left untouched. Absence
   of the flag means non-activation, not uninstall.
5. Removal: delete `.agents/skills/engineering-wisdom/`. The add-on stores no
   database or other lifecycle state.

The skill reviews repository evidence using advice inspired by Robert C.
Martin's work across clean code, SOLID and design, testing, refactoring, clean
architecture, and professional practice. Each finding must separate:

- observation;
- applicable heuristic;
- counter-pressure or trade-off;
- any proposed repository-owned enforcement; and
- verification that could support or falsify the advice.

Installing or invoking the skill does not authorize application architecture
rewrites, lint rules, coverage targets, dependency rules, or other policy.
Those require consumer-repository authority and normal validation.

## Core Update Contract

`harness update --dry-run` resolves the published `harness-v*` core-release
pointer, downloads and checksum-verifies that exact release candidate, then
requires its reported version to match that release, then reports the
candidate's planned merge without writing. A
normal update takes upstream content when the consumer did not change a file,
keeps a consumer-only edit, and uses Git's three-way text merge when both sides
changed. A candidate older than either installed provenance or the executing
binary is rejected.

An overlapping edit stages an ignored resolution session under
`.harness-core/update/` with BASE, LOCAL, UPSTREAM, and agent-editable RESOLVED
copies plus a frozen copy of every other managed input. The remotely
re-verifiable candidate is retained under `.harness-core/update-candidate/`.
Managed workspace files, installed provenance, and the executable stay
unchanged. A normal `harness update` replaces a pending plan and jumps directly
from the installed version to the latest release; its dry-run previews that
new plan without removing the existing session. After human direction resolves
the semantic choice, `harness update --continue --dry-run` previews the accepted
result for the pinned candidate and `harness update --continue` applies it.
`harness update --abort` removes only the staged session. Continuation rejects
unresolved markers, malformed staged inputs, candidate tampering, and drift in
any managed workspace file.

A missing managed file, unsafe path, corrupt base, or other structural conflict
also stops the complete update but requires correcting the workspace and
starting again rather than editing a resolution packet. Successful updates
write provenance last, retain prior bytes under `.harness-backup/`, and replace
only the repository-local executing binary after the core-file transaction
succeeds. A retained candidate lets a later update repair rare core/executable
version skew. Repository executable and retained-candidate paths must contain
regular files and directories rather than symlinks.

The first release containing network discovery is a one-time bootstrap
boundary: older executables must be refreshed with the platform installer once.
After that transition, installed executables discover later core releases
themselves.

Installers also execute a candidate before replacing an existing binary. If the
candidate finds a conflict, resolve the staged file and rerun the installer; it
continues the exact pending version and replaces the binary only after success.

## Ownership

The installers do not copy this repository's root README, architecture, build
scripts, tests, CI, historical decisions, or provenance into a consumer. Those
paths describe upstream Harness or its evolution, not the consumer product.
