# Repository Map

This file is a map for agents working in this repository. It points to the
source-of-truth docs, configuration, and code landmarks; it should not duplicate
the policy held by those files.

## Start Here

- `README.md`: crate purpose, supported platform behavior, and public examples.
- `CONTRIBUTING.md`: local development setup and command expectations.
- `Cargo.toml`: crate metadata, feature flags, MSRV, dependency ranges, and
  docs.rs metadata.
- `docs/guardrails/README.md`: index for Rust, OSS, unsafe, platform, testing,
  API, FFI, and performance guardrails.
- `docs/dependencies.md`: dependency and supply-chain posture.
- `docs/automations/README.md`: recurring maintenance map.
- `.github/labels.yaml`: PR label vocabulary for this repository.

## Change Map

- Public API, semver, features, MSRV, target support, or publishing:
  `docs/guardrails/api-stability-semver-and-msrv.md`,
  `docs/guardrails/working-in-public-and-publishing-oss-crates.md`,
  `Cargo.toml`, `README.md`, and `src/lib.rs`.
- Rust implementation quality, lints, generated docs, or error handling:
  `docs/guardrails/high-quality-rust-code.md`, `CONTRIBUTING.md`, `src/lib.rs`,
  and `.github/workflows/ci.yaml`.
- Windows FFI, UTF-16 conversion, pointer ownership, or off-target behavior:
  `docs/guardrails/platform-specific-code.md`,
  `docs/guardrails/ffi-bindings-and-foreign-runtime-integration.md`,
  `docs/guardrails/unsafe-code.md`, `src/win.rs`, `src/win/ffi.rs`, and
  `src/win/known_folder.rs`.
- `windows-sys` version range maintenance: `docs/automations/windows-sys.md`,
  `Cargo.toml`, and `.github/workflows/ci.yaml`.
- Tests, examples, or target matrix coverage:
  `docs/guardrails/testing-compatibility-and-conformance.md`,
  `examples/get_profile_dir.rs`, and `.github/workflows/ci.yaml`.
- Dependency, audit, or runner maintenance: `docs/dependencies.md`,
  `docs/automations/dependency-sweep.md`,
  `docs/automations/github-actions-runner-images.md`, `.github/dependabot.yml`,
  `.github/workflows/audit.yaml`, and `.github/workflows/repo-labels.yaml`.
- Markdown, YAML, JSON, or generated formatting changes: `package.json`,
  `.prettierrc.yaml`, and `pnpm-lock.yaml`.

## Code Map

- `src/lib.rs`: crate-level docs, target gates, feature gates, and public
  exports.
- `src/win.rs`: Windows implementation entry point.
- `src/win/ffi.rs`: raw Windows API boundary.
- `src/win/known_folder.rs`: typed known-folder identifiers and lookup surface.
- `examples/get_profile_dir.rs`: public usage example for profile-directory
  lookup.

## Pull Request Map

- Use labels from `.github/labels.yaml`; lopopolo-owned repositories require at
  least one `A-*` label.
- For automation-generated work, use `C-automation` and add the `codex` label.
  Keep `codex` as the last label definition in `.github/labels.yaml`.
- Do not add a Codex tag to PR titles or descriptions.
