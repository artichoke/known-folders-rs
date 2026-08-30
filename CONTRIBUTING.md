# Contributing to known-folders-rs

Welcome to [Artichoke]. Thanks for taking the time to contribute.

known-folders-rs provides Rust bindings to the Windows [Known Folders] API. This
crate resolves full paths for well-known Windows directories using
`SHGetKnownFolderPath`.

If known-folders-rs does not resolve a known folder correctly, or if its
platform behavior is surprising, please [file an issue].

Maintenance of this repository is Codex-first. Prefer asking Codex to prepare
routine code, documentation, CI, and dependency changes. Contributors should
focus on issue selection, review, release decisions, and validating that the
resulting diff and CI status match the intended change.

## Setup

known-folders-rs includes Rust and text sources. Developing on known-folders-rs
requires configuring several dependencies.

known-folders-rs uses [mise] to manage the local development toolchain declared
in [`mise.toml`](mise.toml), including Node.js, Rust, `cargo-deny`, and
`zizmor`. For Rust, `mise` uses [rustup] under the hood.

### Rust Toolchain

known-folders-rs depends on Rust and compiler plugins for linting and
formatting. The crate is guaranteed to build on the minimum supported Rust
version documented in [`README.md`](README.md), and it is tested on the latest
stable Rust compiler.

#### Installation

Install and activate [mise], then install the toolchains declared in
[`mise.toml`](mise.toml):

```sh
mise install
```

`mise.toml` configures the latest stable Rust toolchain with the `minimal`
profile plus the `clippy` and `rustfmt` components. `mise` installs that
toolchain via [rustup].

To update your stable Rust compiler to the latest version, run:

```sh
rustup update stable
```

### Rust Crates

known-folders-rs depends on Rust crates from crates.io. Once you have the Rust
toolchain installed, you can fetch and build the crates specified in
[`Cargo.toml`](Cargo.toml) by running:

```sh
cargo build
```

known-folders-rs uses direct tool invocations instead of a repository task
runner. The most common development commands are:

```sh
cargo build --workspace
cargo test --workspace
cargo fmt
cargo clippy --workspace --all-features --all-targets
pnpm run fmt
pnpm run fmt:check
cargo doc --workspace
```

### Node.js

Node.js is an optional dependency used for formatting text sources with
[prettier].

Node.js is only required for formatting if modifying the following filetypes:

- `md`
- `yaml`
- `yml`

Install Node.js with `mise`:

```sh
mise install
```

Then install repo-local pnpm dependencies:

```sh
pnpm install
```

## Linting

To lint and format Rust sources run:

```sh
cargo clippy --workspace --all-features --all-targets
cargo fmt
```

To lint and format text sources run:

```sh
pnpm run fmt
pnpm run fmt:check
```

## Testing

A PR must have new or existing tests for it to be merged. The [Rust book chapter
on testing] is a good place to start.

To run tests:

```sh
cargo test
```

`cargo test` accepts a filter argument that limits test execution to tests that
substring match.

Tests are run for every PR. All builds must pass before merging a PR.

## Codex Maintenance Workflow

Prefer asking Codex to prepare changes on a branch, including any docs and CI
updates needed for the patch. Review the resulting diff as authored code:

- Confirm the change is scoped to the issue or maintenance task.
- Confirm generated or mechanical changes are intentional.
- Confirm CI passes before merging.
- Ask Codex to follow up on review comments or failed checks.

## Publishing

Maintainers publish releases through crates.io trusted publishing. See
[`docs/publishing.md`](docs/publishing.md) for the trust configuration, release
procedure, and failure-recovery guidance.

## Updating Dependencies

### Rust Crates

Version specifiers in `Cargo.toml` are NPM caret-style by default. A version
specifier of `4.1.2` means `4.1.2 <= version < 5.0.0`.

To see what crates are outdated, you can use [cargo-outdated].

If you need to pull in an updated version of a crate for a bugfix or a new
feature, update the version number in `Cargo.toml`.

Regular dependency bumps are handled by [@dependabot].

[artichoke]: https://github.com/artichoke
[known folders]:
  https://learn.microsoft.com/en-us/windows/win32/shell/known-folders
[file an issue]: https://github.com/artichoke/known-folders-rs/issues/new
[mise]: https://mise.jdx.dev/
[rustup]: https://rustup.rs/
[prettier]: https://prettier.io/
[rust book chapter on testing]:
  https://doc.rust-lang.org/book/ch11-00-testing.html
[cargo-outdated]: https://github.com/kbknapp/cargo-outdated
[@dependabot]: https://dependabot.com/
