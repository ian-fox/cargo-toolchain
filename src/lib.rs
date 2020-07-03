//! # cargo-toolchain
//!
//! `cargo-toolchain` is a utility to get the currently active and default
//! [rustup toolchains](https://doc.rust-lang.org/stable/book/appendix-07-nightly-rust.html#rustup-and-the-role-of-rust-nightly).
//!
//! It requires that [rustup](https://rustup.rs/) is installed.
//!
//! ## Usage as a CLI
//!
//! ```shell
//! cargo install cargo-toolchain
//!
//! cargo toolchain # prints the currently active cargo toolchain, e.g. 'stable'
//!
//! cargo toolchain -d # prints the default toolchain for the directory
//!
//! cargo toolchain -h # print help message
//! ```

use anyhow::Result;
use std::env;
use std::process::Command;

const RUSTUP_TOOLCHAIN_VAR: &str = "RUSTUP_TOOLCHAIN";
const DEFAULT_PROFILES: [&str; 3] = ["stable", "beta", "nightly"];

/// Get the profile that was used to call the current executable.
///
/// Affected by directory overrides and command line flags (e.g. `cargo +nightly ...`)
///
/// # Errors
///
/// If the `RUSTUP_TOOLCHAIN` variable is not set, `get_active_toolchain` falls back to [`get_directory_toolchain`](./fn.get_directory_toolchain.html).
/// That function returns an error if running `rustup` or parsing its output as utf-8 fails.
pub fn get_active_toolchain() -> Result<String> {
    match env::var(RUSTUP_TOOLCHAIN_VAR) {
        Ok(toolchain) => Ok(truncate_toolchain(toolchain)),
        _ => get_directory_toolchain(),
    }
}

/// Get the default rustup toolchain for the current directory.
///
/// Affected by directory overrides, but not command line flags (e.g. `cargo +nightly ...`)
///
/// # Errors
///
/// If the `rustup` command fails or its output cannot be interpreted as utf-8,
/// an error is returned.
pub fn get_directory_toolchain() -> Result<String> {
    let output = Command::new("rustup")
        .args(&["show", "active-toolchain"])
        .env_remove(RUSTUP_TOOLCHAIN_VAR)
        .output()?;
    let toolchain = String::from_utf8(output.stdout)?;

    Ok(truncate_toolchain(toolchain))
}

/// Truncate the triple from a toolchain name if it's one of the defaults.
///
/// # Examples
///
/// ```rust
/// let toolchain = cargo_toolchain::truncate_toolchain(String::from("nightly-x86_64-unknown-linux-gnu"));
///
/// assert_eq!(toolchain, "nightly");
///
/// let toolchain = cargo_toolchain::truncate_toolchain(String::from("my-custom-toolchain"));
///
/// assert_eq!(toolchain, "my-custom-toolchain");
/// ```
///
pub fn truncate_toolchain(toolchain: String) -> String {
    for default in &DEFAULT_PROFILES {
        if toolchain.starts_with(default) {
            return String::from(*default);
        }
    }

    toolchain
}

/// Wrapper around [`get_active_toolchain`](./fn.get_active_toolchain.html)
/// and [`get_directory_toolchain`](./fn.get_directory_toolchain.html)
pub fn get_toolchain(directory_default: bool) -> Result<String> {
    if directory_default {
        return get_directory_toolchain();
    }

    get_active_toolchain()
}
