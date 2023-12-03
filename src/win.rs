// src/win.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use core::mem::size_of;
use core::slice;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows_sys::Win32::{
    Foundation::{E_FAIL, E_INVALIDARG, HANDLE, S_OK},
    Globalization::lstrlenW,
    UI::Shell::{SHGetKnownFolderPath, KF_FLAG_DEFAULT},
};

mod ffi;
mod known_folder;

pub use known_folder::KnownFolder;

/// Retrieve the full path of a known folder identified by the folder's
/// [`KNOWNFOLDERID`].
///
/// A safe wrapper around the [`SHGetKnownFolderPath`] Win32 API function on
/// Windows.
///
/// See [`KnownFolder`] for the types of known folders this function can
/// retrieve.
///
/// # Errors
///
/// If an error occurs when calling the underlying Windows APIs or the given
/// Known Folder ID is not present on the system (for example, if the ID was
/// introduced in a newer OS version), [`None`] is returned.
///
/// # Examples
///
/// ```
/// use known_folders::{get_known_folder_path, KnownFolder};
///
/// let profile_dir = get_known_folder_path(KnownFolder::Profile);
/// ```
///
/// [`KNOWNFOLDERID`]: KnownFolder
/// [`SHGetKnownFolderPath`]: https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath
#[must_use]
#[allow(clippy::match_same_arms)]
#[allow(clippy::cast_possible_wrap)]
pub fn get_known_folder_path(known_folder: KnownFolder) -> Option<PathBuf> {
    // This guard ensures `CoTaskMemFree` is always called after invoking
    // `SHGetKnownFolderPath`, which is required regardless of the return
    // value.
    //
    // See `ppszPath` out parameter description:
    //
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#parameters
    let mut guard = ffi::Guard::default();

    // Upstream docs:
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath
    //
    // `SHGetKnownFolderPath` replaces `SHGetFolderPathW` as of Windows Vista:
    //
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetfolderpathw
    //
    // SAFETY: this invocation meets the preconditions defined in the API
    // documentation:
    //
    // - `rfid` is a reference to a known folder ID, provided by `windows-sys`.
    // - `dwFlags` can be `0` per the documentation, we have no special retrieval
    //   requirements, so use the default defined in `windows-sys`.
    //   The `KNOWN_FOLDER_FLAG` enum is documented here:
    //   https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/ne-shlobj_core-known_folder_flag
    // - `hToken` is "an access token that represents a particular user. If this
    //   parameter is `NULL`, which is the most common usage, the function
    //   requests the known folder for the current user. We want the known folder
    //   for the current user, so use `HANDLE::default()`.
    // - `ppszPath` is an out parameter and should be a NULL pointer to a PWSTR.
    //
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#parameters
    match unsafe {
        SHGetKnownFolderPath(
            known_folder.to_guid(),
            KF_FLAG_DEFAULT as _,
            HANDLE::default(),
            guard.as_out_ppszPath(),
        )
    } {
        S_OK => {
            let path_ptr = guard.as_pcwstr();

            // SAFETY: on success, the out pointer is guaranteed to be a valid,
            // NUL-terminated wide string.
            //
            // > When `SHGetKnownFolderPath` returns, contains the address of a
            // > pointer to a null-terminated Unicode string that specifies the
            // > path of the known folder
            let len = unsafe {
                let len = lstrlenW(path_ptr);
                usize::try_from(len).ok()?
            };

            // SAFETY: `path_ptr` is valid for `len` "characters" in a single
            // string allocation, per windows-sys APIs. "Characters" are `WCHAR`
            // values. Additionally, `lstrlenW` returns `i32` on 64-bit
            // platforms. The `match` below guarantees the size of the
            // allocation is no larger than `isize::MAX`.
            let path = unsafe {
                match isize::try_from(len) {
                    Ok(len) if len < 0 => return None,
                    Ok(len) if len.checked_mul(size_of::<u16>() as isize).is_some() => {}
                    Ok(_) | Err(_) => return None,
                };

                // NOTE: this slice must go out of scope before `guard` above is
                // dropped. This invariant holds since the guard is constructed
                // outside the scope of this `match` block.
                slice::from_raw_parts(path_ptr, len)
            };

            let os_str = OsString::from_wide(path);
            Some(os_str.into())
        }
        // Expected return codes. See:
        //
        // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#return-value
        E_FAIL | E_INVALIDARG => None,
        // Unexpected return code.
        _ => None,
    }
}
