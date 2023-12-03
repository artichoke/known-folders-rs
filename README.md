# known-folders-rs

[![GitHub Actions](https://github.com/artichoke/known-folders-rs/workflows/CI/badge.svg)](https://github.com/artichoke/known-folders-rs/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/known-folders.svg)](https://crates.io/crates/known-folders)
[![API](https://docs.rs/known-folders/badge.svg)](https://docs.rs/known-folders)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/known-folders-rs/known_folders/)

Retrieves the full path of a known folder identified by the folder's
**KNOWNFOLDERID** on Windows systems using `SHGetKnownFolderPath` and the [Known
Folders] API.

[known folders]:
  https://learn.microsoft.com/en-us/windows/win32/shell/known-folders

The Known Folders API first appeared in Windows Vista.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
known-folders = "1.1.0"
```

Then resolve well-known directories like this:

```rust
use known_folders::{get_known_folder_path, KnownFolder};

let profile_dir = get_known_folder_path(KnownFolder::Profile);
```

You can test this crate works on your platform by running the example:

```shell
cargo run --example get_profile_dir
```

## Implementation

known-folders-rs binds directly to `Win32` using [`windows_sys`].
Semver-incompatible `windows_sys` upgrades can be made in minor releases.

[`windows_sys`]: https://crates.io/crates/windows-sys

Note that this crate is completely empty on non-Windows platforms.

## Minimum Supported Rust Version

This crate requires at least Rust 1.58.0. This version can be bumped in minor
releases.

## License

`known-folders-rs` is distributed under the terms of either the
[MIT License](LICENSE-MIT) or the
[Apache License (Version 2.0)](LICENSE-APACHE).
