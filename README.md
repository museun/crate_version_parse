# crate_version_parse
[![Crates][crates_badge]][crates]
[![Documentation][docs_badge]][docs]
[![Actions][actions_badge]][actions]

A crate name and version parser

### Features
* `serde` enables [serde](https://serde.rs) support
* `std` to enable [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)

### Example
```rust
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

License: 0BSD

[crates_badge]: https://img.shields.io/crates/v/crate_version_parse
[docs_badge]: https://docs.rs/crate_version_parse/badge.svg
[actions_badge]: https://github.com/museun/crate_version_parse/workflows/Rust/badge.svg

[crates]: https://crates.io/crates/crate_version_parse
[docs]: https://docs.rs/crate_version_parse
[actions]: https://github.com/museun/crate_version_parse/actions
