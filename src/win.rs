// src/win.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::c_void;
use std::mem::size_of;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::slice;

use windows_sys::core::{GUID, PWSTR};
use windows_sys::Win32::{
    Foundation::{E_FAIL, E_INVALIDARG, HANDLE, S_OK},
    Globalization::lstrlenW,
    System::Com::CoTaskMemFree,
    UI::Shell::{FOLDERID_Profile, FOLDERID_RoamingAppData, SHGetKnownFolderPath, KF_FLAG_DEFAULT},
};

mod guard {
    pub struct FreeGuard(PWSTR);

    impl FreeGuard {
        /// Per upstream documentation, the last parameter to
        /// `SHGetKnownFolderPath` is a pointer to a pointer.
        ///
        /// # Parameter
        ///
        /// > `[out] ppszPath`
        /// >
        /// > Type: `PWSTR*`
        ///
        /// https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#parameters
        ///
        /// `PWSTR` itself is a `*mut u16`:
        ///
        /// https://docs.rs/windows-sys/0.48.0/windows_sys/core/type.PWSTR.html
        pub fn as_out_ppszPath(&mut self) -> &mut PWSTR {
            &mut self.0
        }

        /// Access the inner wide string.
        pub fn as_pwstr(&self) -> PWSTR {
            self.0
        }
    }

    impl Default for FreeGuard {
        fn default() -> Self {
            let ptr = ptr::null_mut::<PWSTR>();
            Self(ptr)
        }
    }

    impl Drop for FreeGuard {
        fn drop(&mut self) {
            let ptr = self.0.cast::<c_void>();
            // SAFETY: `ptr` must always be freed per the API documentation:
            //
            // > The calling process is responsible for freeing this resource
            // > once it is no longer needed by calling `CoTaskMemFree`, whether
            // > `SHGetKnownFolderPath` succeeds or not.
            unsafe {
                CoTaskMemFree(ptr);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KnownFolder {
    Profile,
    AppDataRoaming,
}

impl KnownFolder {
    fn to_guid(self) -> &'static GUID {
        match self {
            Self::Profile => &FOLDERID_Profile,
            Self::AppDataRoaming => &FOLDERID_RoamingAppData,
        }
    }
}

pub fn get_known_folder_path(known_folder: KnownFolder) -> Option<PathBuf> {
    // This guard ensures `CoTaskMemFree` is always called after invoking
    // `SHGetKnownFolderPath`, which is required regardless of the return
    // value.
    //
    // See `ppszPath` out parameter description:
    //
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#parameters
    let mut guard = guard::FreeGuard::default();

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
            KF_FLAG_DEFAULT,
            HANDLE::default(),
            guard.as_out_ppszPath(),
        )
    } {
        S_OK => {
            let path_ptr = guard.as_pwstr();

            // SAFETY: on success, the out pointer is guaranteed to be a valid,
            // NUL-terminated wide string.
            //
            // > When `SHGetKnownFolderPath` returns, contains the address of a
            // > pointer to a null-terminated Unicode string that specifies the
            // > path of the known folder
            let len = unsafe {
                let len = lstrlenW(path_ptr);
                usize::try_from(len).ok()?;
            };

            // SAFETY: `path_ptr` is valid for `len` bytes in a single string
            // allocation, per windows-sys APIs. `lstrlenW` returns `i32` on
            // 64-bit platforms. The `match` below guarantees the size of the
            // allocation is no larger than `isize::MAX`.
            let path = unsafe {
                match isize::try_from(len) {
                    Ok(len) if len < 0 => return None,
                    Ok(len) if len.checked_mul(size_of::<u16>() as isize).is_some() => {}
                    Ok(_) | Err(_) => return None,
                };

                slice::from_raw_parts(path_ptr, len)
            };

            let os_str = OsString::from_wide(path);
            Some(os_str)
        }
        E_FAIL | E_INVALIDARG => {
            // Expected return codes. See:
            //
            // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#return-value
            None
        }
        _ => {
            // Unexpected return code.
            None
        }
    }
}
