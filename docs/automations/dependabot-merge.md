# Dependabot Merge Automation

The Dependabot merge automation is a scheduled repository-maintenance role for
reviewing GitHub dependency bump pull requests and moving low-risk updates to
auto-merge. It should keep routine dependency PRs moving while preserving the
dependency posture documented in [`docs/dependencies.md`](../dependencies.md).

The automation must read `docs/automations/README.md`, this document, and
`docs/dependencies.md` before reviewing pull requests. Human feedback on any
output it produces, including pull request comments, review decisions, failed
validation, or inbox follow-ups, should be reviewed and used to update these
guardrails before repeating the same class of issue. Automation-authored pull
request comments must start with the stable prefix `Codex automation note:`.

## Schedule

Run once per week against this repository. Dependabot is configured for monthly
dependency checks, but a weekly review cadence keeps queued updates from waiting
for the next broad dependency sweep.

## Scope

This automation owns only open pull requests created by Dependabot or other
clearly identified dependency-bump automation. It may:

- inspect the pull request metadata, diff, checks, and recent comments;
- review dependency release notes, changelogs, advisories, and compatibility
  notes from authoritative upstream sources;
- leave a short automation note when a pull request needs human review;
- enable GitHub auto-merge for low-risk dependency updates after validation
  passes.

Do not make repository file edits, create dependency-update branches, refresh
lockfiles, or open new dependency pull requests from this automation. Those
tasks remain owned by the broader [Dependency Sweep](./dependency-sweep.md) and
the [windows-sys Maintenance](./windows-sys.md) automation.

## Review Rules

Start by listing open pull requests in `artichoke/known-folders-rs` whose author
or branch indicates a dependency bump. Prioritize Dependabot pull requests.

For each candidate, inspect:

- changed files and whether the diff is mechanical;
- package ecosystem and dependency group from `.github/dependabot.yml`;
- CI and required status checks;
- Dependabot cooldown behavior and whether the update is at least seven days old
  when the dependency posture requires cooldown;
- upstream release notes, changelogs, comparison links, and advisories when the
  update is not obviously patch-level and mechanical;
- existing human comments, review requests, labels, and assignees.

Low-risk updates may be moved to auto-merge when all of these are true:

- the pull request was created by Dependabot or equivalent dependency
  automation;
- the diff only changes dependency metadata, lockfiles, or generated dependency
  state expected for the ecosystem;
- CI and required checks are passing, or GitHub auto-merge can wait for checks
  that are already running;
- release notes do not require code, workflow, MSRV, target-support, or
  compatibility changes;
- the update does not change this crate's public compatibility surfaces,
  including Windows FFI behavior, off-target empty-crate behavior, MSRV,
  `windows-sys` range policy, or public API shape.

Leave the pull request for human review when any of these apply:

- the diff includes handwritten source, workflow logic, release metadata,
  repository policy, or automation documentation changes;
- validation fails or required checks are missing;
- release notes are ambiguous, unavailable, or describe a migration;
- the update touches `windows-sys` in a way that requires release-track judgment
  beyond Dependabot's mechanical version bump;
- the update appears security-sensitive, semver-major with behavioral impact, or
  likely to affect downstream users;
- there is unresolved human feedback or an existing label such as `S-blocked`,
  `S-do-not-merge`, `S-wip`, or `S-postponed`.

When a pull request needs human review and the reason is not already clear, add
a pull request comment beginning with `Codex automation note:` that explains the
specific blocker. Do not repeat the same comment on later runs unless new
information changes the assessment.

## Validation and Merge Handling

Use GitHub's merge queue or auto-merge mechanism rather than merging directly.
Do not bypass branch protection, dismiss human reviews, or force-push Dependabot
branches.

If auto-merge is enabled for one or more pull requests, wait for GitHub to merge
them only when doing so is necessary to produce an accurate final summary. Do
not continue into dependency sweep work after merges land. The next scheduled
dependency sweep can rebase and handle any follow-up maintenance.

## Summary

Open an inbox item after every run summarizing:

- candidate pull requests reviewed;
- pull requests moved to auto-merge;
- pull requests left for human review and the reason;
- release-note or advisory sources consulted for non-trivial updates;
- validation state for each accepted pull request;
- any skipped checks or permission limits.
