// src/win/ffi.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::c_void;
use std::ptr;

use windows_sys::core::PWSTR;
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
    pub fn as_out_ppszPath(&mut self) -> &mut PWSTR {
        &mut self.0
    }

    /// Access the inner wide string.
    pub fn as_pwstr(&self) -> PWSTR {
        self.0
    }
}

impl Default for Guard {
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
