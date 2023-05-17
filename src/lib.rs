// src/lib.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

//! Retrieves the full path of a known folder identified by the folder's
//! **KNOWNFOLDERID** on Windows systems using `SHGetKnownFolderPath` and the
//! [Known Folders] API.
//!
//! # Platform Support
//!
//! The Known Folders API first appeared in Windows Vista.
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

#![no_std]
#![doc(html_root_url = "https://docs.rs/known-folders/0.1.0")]

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
