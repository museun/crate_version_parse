#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

/*! A crate name and version parser

## Features
* `serde` enables [serde](https://serde.rs) support
* `std` to enable [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)

## Example
```rust
# use crate_version_parse::CrateVersion;
let input = "zstd-sys-1.4.15+zstd.1.4.4";
let crate_ = CrateVersion::try_parse(input).unwrap();
assert_eq!(crate_.name, "zstd-sys");
assert_eq!(crate_.version, "1.4.15+zstd.1.4.4");

let input = "wasi-0.9.0+wasi-snapshot-preview1";
let crate_ = CrateVersion::try_parse(input).unwrap();
assert_eq!(crate_.name, "wasi");
assert_eq!(crate_.version,  "0.9.0+wasi-snapshot-preview1");

let input = "winapi-i686-pc-windows-gnu-0.4.0";
let crate_ = CrateVersion::try_parse(input).unwrap();
assert_eq!(crate_.name, "winapi-i686-pc-windows-gnu");
assert_eq!(crate_.version, "0.4.0");
```
*/

/// An error found while parsing
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[non_exhaustive]
pub enum Error {
    /// Name is missing
    MissingName {
        /// Position parser ended up at
        pos: usize,
    },
    /// Version is missing
    MissingVersion {
        /// Position parser ended up at
        pos: usize,
    },
}

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingName { pos } => write!(f, "missing name at approx: {}", pos),
            Self::MissingVersion { pos } => write!(f, "missing version at approx: {}", pos),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// A Crate + semver version
#[derive(Debug, Copy, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CrateVersion<'a> {
    /// Name of the crate
    pub name: &'a str,
    /// Semver for the crate
    pub version: &'a str,
}

impl<'a> CrateVersion<'a> {
    /// Tries to parse a crate `name` and `version` from a `$crate-$vers` format
    ///
    /// # Example
    /// ```rust
    /// # use crate_version_parse::CrateVersion;
    /// let input = "wasi-0.9.0+wasi-snapshot-preview1";
    ///
    /// let version = CrateVersion::try_parse(input).unwrap();
    /// assert_eq!(version, CrateVersion {
    ///     name: "wasi",
    ///     version: "0.9.0+wasi-snapshot-preview1",
    /// });
    /// ```
    pub fn try_parse(input: &'a str) -> Result<Self, Error> {
        let mut split = 0;
        let mut iter = input
            .chars()
            .rev()
            .map(|c| {
                split += 1;
                c
            })
            .peekable();

        while let Some(c) = iter.next() {
            match iter.peek() {
                Some('-') if c.is_numeric() => break,
                _ => {}
            }
        }

        let midpoint = input.len() - split;

        let name = input
            .get(..midpoint)
            .ok_or_else(|| Error::MissingName { pos: midpoint })?;

        let version = input
            .get(midpoint + 1..)
            .ok_or_else(|| Error::MissingVersion { pos: midpoint + 1 })?;

        Ok(CrateVersion { name, version })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let tests = &[
            (
                "zstd-sys-1.4.15+zstd.1.4.4",
                CrateVersion {
                    name: "zstd-sys",
                    version: "1.4.15+zstd.1.4.4",
                },
            ),
            (
                "winapi-i686-pc-windows-gnu-0.4.0",
                CrateVersion {
                    name: "winapi-i686-pc-windows-gnu",
                    version: "0.4.0",
                },
            ),
            (
                "wasi-0.9.0+wasi-snapshot-preview1",
                CrateVersion {
                    name: "wasi",
                    version: "0.9.0+wasi-snapshot-preview1",
                },
            ),
            (
                "ppv-lite86-0.2.5",
                CrateVersion {
                    name: "ppv-lite86",
                    version: "0.2.5",
                },
            ),
            (
                "log-0.4.8",
                CrateVersion {
                    name: "log",
                    version: "0.4.8",
                },
            ),
        ];

        for (input, expected) in tests {
            let crate_ = CrateVersion::try_parse(input).unwrap();
            assert_eq!(crate_, *expected);
        }
    }
}
