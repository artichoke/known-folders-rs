# Agent Instructions

You are working in `artichoke/known-folders-rs`, a Rust crate that wraps the
Windows Known Folders API and compiles as an empty crate on non-Windows targets.

Users rely on safe path lookup behavior, Windows UTF-16 handling, off-target
build behavior, the supported `windows-sys` range, MSRV, and the public crate
API. Treat those as compatibility surfaces.

## Operating Loop

1. Classify the change before editing.
2. Use the matching workflow section below to choose the guardrails and runbooks
   to consult.
3. Keep the diff narrow. Do not mix behavior, dependency posture, release
   metadata, formatting, platform maintenance, and automation cleanup unless the
   task requires it.
4. Add or update focused tests for behavior changes, especially changes that
   affect Windows FFI, UTF-16 conversion, pointer ownership, or target gates.
5. Run checks that match the risk of the change; use
   [CONTRIBUTING.md](CONTRIBUTING.md) for local command expectations. If a
   relevant check is skipped, explain why in the PR.
6. Update README, crate docs, guardrails, or runbooks when public behavior,
   compatibility claims, target support, MSRV, dependency policy, or release
   process changes.

## Windows API Behavior And FFI

Use this workflow for known-folder lookup behavior, Windows target gates, raw
FFI calls, pointer ownership, allocation/freeing behavior, and UTF-16 path
handling.

Consult:

- [Platform-specific code](docs/guardrails/platform-specific-code.md), for
  target-gating and platform contract expectations.
- [FFI and foreign runtime integration](docs/guardrails/ffi-bindings-and-foreign-runtime-integration.md),
  for binding and ABI expectations.
- [Unsafe code](docs/guardrails/unsafe-code.md), for unsafe boundary review.
- [Testing and conformance](docs/guardrails/testing-compatibility-and-conformance.md),
  for target-matrix and regression coverage.

Keep the public API safe. Do not move raw Windows pointer ownership out of the
FFI boundary without explicit justification in the PR.

## `windows-sys` Maintenance

Use this workflow for `windows-sys` version range updates, feature changes, or
minimum-supported dependency checks.

Consult:

- [`windows-sys` automation](docs/automations/windows-sys.md), for the expected
  maintenance flow.
- [API stability, semver, and MSRV](docs/guardrails/api-stability-semver-and-msrv.md),
  for dependency range and public contract impact.
- [Testing and conformance](docs/guardrails/testing-compatibility-and-conformance.md),
  for lower-bound and latest-version coverage.

Verify both the minimum supported and latest supported `windows-sys` versions
when the range changes.

## Public API, Features, MSRV, And Releases

Use this workflow for API shape, feature flags, docs.rs metadata, crate
metadata, MSRV, semver, publishing, changelog, and release-readiness changes.

Consult:

- [API stability, semver, and MSRV](docs/guardrails/api-stability-semver-and-msrv.md),
  for public contract and compatibility impact.
- [Working in public and publishing](docs/guardrails/working-in-public-and-publishing-oss-crates.md),
  for OSS release and communication expectations.

Call out compatibility and target-support impact in the PR.

## Implementation Quality

Use this workflow for refactors, lint posture, error handling, documentation
quality, crate attributes, and maintainability changes that do not intentionally
change behavior.

Consult:

- [High-quality Rust code](docs/guardrails/high-quality-rust-code.md), for lint,
  documentation, and maintainability expectations.
- [Testing and conformance](docs/guardrails/testing-compatibility-and-conformance.md),
  if the refactor touches behavior-sensitive paths.

Prefer mechanical refactors that preserve behavior and are easy to review.

## Dependencies, CI, And Automation

Use this workflow for dependency ranges, audits, Dependabot, GitHub Actions,
runner image updates, labels, and recurring maintenance.

Consult:

- [Dependency posture](docs/dependencies.md), for supply-chain expectations.
- [Dependency sweep automation](docs/automations/dependency-sweep.md), for
  dependency update procedure.
- [GitHub Actions runner images](docs/automations/github-actions-runner-images.md),
  for runner maintenance.
- [Working in public and publishing](docs/guardrails/working-in-public-and-publishing-oss-crates.md),
  if the change affects release or user-facing maintenance policy.

Keep mechanical dependency and automation updates separate from behavior
changes.

## Documentation-Only Changes

Use this workflow for README, crate docs, guardrails, runbooks, and PR/process
documentation.

Consult:

- [High-quality Rust code](docs/guardrails/high-quality-rust-code.md), for
  documentation quality expectations.
- [Working in public and publishing](docs/guardrails/working-in-public-and-publishing-oss-crates.md),
  for public-facing OSS communication.
- The guardrail for the topic being documented when docs describe API, FFI,
  platform, dependency, or release behavior.

Docs-only PRs may skip Rust tests when the PR explains why. Still run the repo
formatter.

## Pull Requests

- State the change class and compatibility impact.
- Use labels from `.github/labels.yaml`; include at least one `A-*` label.
- For automation-generated work, use `C-automation` and the `codex` label.
- Do not add a Codex tag to the title or description.
