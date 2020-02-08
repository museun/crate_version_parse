# crate_version_parse

### Features
* `serde` enables [serde](https://serde.rs) support
* `std` to enable [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)

### Example
```rust
let input = "zstd-sys-1.4.15+zstd.1.4.4";
let crate_ = try_parse(input).unwrap();
assert_eq!(crate_.name, "zstd-sys");
assert_eq!(crate_.ver, "1.4.15+zstd.1.4.4");

let input = "wasi-0.9.0+wasi-snapshot-preview1";
let crate_ = try_parse(input).unwrap();
assert_eq!(crate_.name, "wasi");
assert_eq!(crate_.ver,  "0.9.0+wasi-snapshot-preview1");

let input = "winapi-i686-pc-windows-gnu-0.4.0";
let crate_ = try_parse(input).unwrap();
assert_eq!(crate_.name, "winapi-i686-pc-windows-gnu");
assert_eq!(crate_.ver, "0.4.0");
```

License: 0BSD
