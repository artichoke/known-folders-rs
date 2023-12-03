// src/lib.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Retrieves the full path of a known folder identified by the folder's
//! **KNOWNFOLDERID** on Windows systems using `SHGetKnownFolderPath` and the
//! [Known Folders] API.
//!
//! # Platform Support
//!
//! The Known Folders API first appeared in Windows Vista.
//!
//! Note that this crate is completely empty on non-Windows platforms.
//!
//! ## Linkage
//!
//! The Known Folders API is provided by Win32, which is linked into every
//! binary on Windows platforms.
//!
//! # Examples
//!
#![cfg_attr(windows, doc = "```")]
#![cfg_attr(not(windows), doc = "```compile_fail")]
//! use known_folders::{get_known_folder_path, KnownFolder};
//!
//! let profile_dir = get_known_folder_path(KnownFolder::Profile);
//! ```
//!
//! [Known Folders]: https://learn.microsoft.com/en-us/windows/win32/shell/known-folders

#![doc(html_root_url = "https://docs.rs/known-folders/1.1.0")]

// Ensure code blocks in `README.md` compile
#[cfg(all(doctest, windows))]
#[doc = include_str!("../README.md")]
mod readme {}

#[cfg(windows)]
#[allow(clippy::too_many_lines)]
mod win;

#[cfg(windows)]
pub use self::win::*;

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let profile_dir = get_known_folder_path(KnownFolder::Profile).unwrap();
        assert!(profile_dir.is_dir());
        assert!(profile_dir.exists());
    }
}
