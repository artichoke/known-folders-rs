// src/win/ffi.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use core::ffi::c_void;
use core::ptr;

use windows_sys::core::{PCWSTR, PWSTR};
use windows_sys::Win32::System::Com::CoTaskMemFree;

pub struct Guard(PWSTR);

impl Guard {
    /// Per upstream documentation, the last parameter to `SHGetKnownFolderPath`
    /// is a pointer to a pointer.
    ///
    /// # Parameter
    ///
    /// > `[out] ppszPath`
    /// >
    /// > Type: `PWSTR*`
    ///
    /// <https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#parameters>
    ///
    /// `PWSTR` itself is a `*mut u16`:
    ///
    /// <https://docs.rs/windows-sys/0.48.0/windows_sys/core/type.PWSTR.html>
    #[must_use]
    #[allow(non_snake_case)]
    pub fn as_out_ppszPath(&mut self) -> &mut PWSTR {
        &mut self.0
    }

    /// Access the inner wide string.
    #[must_use]
    pub fn as_pcwstr(&self) -> PCWSTR {
        self.0
    }
}

impl Default for Guard {
    fn default() -> Self {
        Self(ptr::null_mut())
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        let ptr = self.0.cast::<c_void>();
        // SAFETY: `ptr` must always be freed per the API documentation:
        //
        // > The calling process is responsible for freeing this resource
        // > once it is no longer needed by calling `CoTaskMemFree`, whether
        // > `SHGetKnownFolderPath` succeeds or not.
        //
        // Additionally, `CoTaskMemFree` has no effect if passed `NULL`, so
        // there is no issue if some future refactor creates a pathway where
        // `Guard` could be dropped before `SHGetKnownFolderPath` is called.
        unsafe {
            CoTaskMemFree(ptr);
        }
    }
}
