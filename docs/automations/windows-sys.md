# windows-sys Maintenance Automation

The windows-sys maintenance automation is a scheduled repository maintenance
role for keeping this crate's `windows-sys` dependency range current while
preserving source compatibility where possible.

The automation must read `docs/automations/README.md` and this document before
running. Human feedback on any output it produces, including inbox follow-ups,
pull request comments, review decisions, failed validation, source compatibility
misses, or release follow-ups, should be reviewed and used to update this
document before repeating the same class of issue. Automation-authored pull
request comments must start with the stable prefix `Codex automation note:`.
Treat comments with that prefix as automation state, not human feedback for this
learning loop.

## Schedule

Run once per week against this repository. `windows-sys` releases are infrequent
enough that weekly checking is sufficient.

## Policy

Prefer widening the accepted `windows-sys` version range when source
compatibility permits it. Widening the range is the lowest-risk maintenance path
because downstream users can adopt newer `windows-sys` versions without forcing
older compatible users to upgrade.

If source compatibility with the current lower bound is no longer possible,
still update to the newer compatible `windows-sys` range. Drop old versions as
required, explain the break in the inbox item and pull request, and do not
enable auto-merge.

Any merged `windows-sys` range change should lead to a new minor release of
`known-folders`.

## Sources

Use authoritative upstream sources:

- crates.io metadata for `windows-sys`;
- docs.rs API docs for the currently supported lower bound and the newest
  candidate version;
- the `microsoft/windows-rs` release notes, changelog, tags, and commits when
  source compatibility is unclear.

Do not rely only on semver assumptions. Verify source compatibility by building
and testing against the minimum and maximum versions in the proposed range.

## Workflow

Start by reading `Cargo.toml` and the CI matrix in `.github/workflows/ci.yaml`.
Identify the current `windows-sys` lower bound, upper bound, and the exact
versions tested in CI.

Check the newest stable `windows-sys` release. Ignore prereleases unless the
crate already depends on a prerelease or a human explicitly asks to evaluate
one.

If a newer `windows-sys` release is available, try to widen only the upper bound
first. For example, a current range like `>=0.59, <=0.61` should become
`>=0.59, <=0.62` if the source compiles against both the existing lower-bound
test versions and the newest upper-bound version.

Update the CI matrix so it continues to test:

- the oldest exact `windows-sys` version the crate claims to support;
- any intermediate exact version needed for MSRV compatibility;
- the newest supported exact version or `latest`, depending on the existing CI
  style.

If widening fails because the code no longer compiles with the current lower
bound, find the smallest compatible lower bound and update the range. Keep the
diff as small as possible and document the source incompatibility.

## Changes

If no `windows-sys` update is available, do not create a branch, commit, push,
or pull request. Open an inbox item with the checked versions and sources.

If widening the range is source compatible, create one pull request for the
range and CI updates. Add the `A-deps`, `A-release`, `C-automation`, and `codex`
labels. This is low risk and may use auto-merge when validation passes.

If source compatibility is not possible, create one pull request for the
required version bump and compatibility fixes. Add the `A-deps`, `A-release`,
`C-automation`, and `codex` labels. Do not enable auto-merge. Assign the pull
request to `lopopolo` and open an inbox item explaining what broke and why the
old versions had to be dropped.

After a `windows-sys` range change merges, open a separate release-prep pull
request that bumps the crate minor version, updates `html_root_url`, updates
README installation examples, and prepares the tag for the publish workflow. Do
not duplicate an existing open release-prep pull request.

## Validation and Summary

For `windows-sys` changes, run the relevant Windows CI commands locally or in
GitHub Actions when possible:

```sh
cargo generate-lockfile
cargo update -p windows-sys --precise <minimum-version>
cargo test --workspace
cargo update -p windows-sys --precise <maximum-version>
cargo test --workspace
cargo clippy --workspace --all-features --all-targets
pnpm run fmt:check
```

If validation cannot run locally because the host is not Windows, rely on the
pull request's GitHub Actions checks and state that in the inbox item and pull
request.

If a release-prep pull request is opened, also run `cargo package --allow-dirty`
before finalizing the pull request.

Open an inbox item after every run summarizing:

- current and newest `windows-sys` versions checked;
- source links used;
- whether the existing range could be widened;
- compatibility failures, if any;
- validation run and any skipped checks;
- pull request and auto-merge status, if a pull request was opened;
- release-prep status or required human follow-up.
