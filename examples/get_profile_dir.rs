// examples/get_profile_dir.rs
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
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
#![allow(unknown_lints)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

//! Example demonstrating the sysdir well-known directory enumeration API.
//!
//! # Usage
//!
//! ```shell
//! cargo run --example enumerate_system_dirs
//! ```

use std::io::{self, Write as _};
use std::process;

fn main() {
    match platform::try_main() {
        Ok(()) => {}
        Err(err) => {
            let _ignore = writeln!(io::stderr(), "{err}");
            process::exit(1);
        }
    }
}

#[cfg(not(windows))]
mod platform {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, Clone, Copy)]
    struct PlatformNotSupported;

    impl fmt::Display for PlatformNotSupported {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("The Known Folders API is not supported on this platform. The Known Folders API is only available on Windows Vista and later.")
        }
    }

    impl Error for PlatformNotSupported {}

    pub fn try_main() -> Result<(), Box<dyn Error>> {
        return Err(Box::new(PlatformNotSupported));
    }
}

#[cfg(windows)]
mod platform {
    use std::error::Error;
    use std::fmt;
    use std::io::{self, Write as _};

    use known_folders::{get_known_folder_path, KnownFolder};

    #[derive(Debug, Clone, Copy)]
    struct PlatformError;

    impl fmt::Display for PlatformError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("An unknown error occurred when resolving the known folder.")
        }
    }

    impl Error for PlatformError {}

    #[derive(Debug, Clone, Copy)]
    struct UnsupportedPathEncoding;

    impl fmt::Display for UnsupportedPathEncoding {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("The known folder path was not UTF-8 encoded.")
        }
    }

    impl Error for UnsupportedPathEncoding {}

    pub fn try_main() -> Result<(), Box<dyn Error>> {
        let profile_dir = get_known_folder_path(KnownFolder::Profile).ok_or(PlatformError)?;

        let display = profile_dir
            .into_os_string()
            .into_string()
            .map_err(|_| UnsupportedPathEncoding)?;

        writeln!(io::stdout(), "Profile directory: {display}")?;
        Ok(())
    }
}
