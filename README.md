# Memory Stats

A cross-platform-ish memory profiler for Rust.  This crate provides two metrics:

- **"Physical" Memory**, which corresponds to the _Resident Set Size_ on Linux and MacOS and the _Working Set_ on Windows.
- **"Virtual" Memory**, which corresponds to the _Virtual Size_ on Linux and MacOS and the _Pagefile Usage_ on Windows.

## License

This crate is dual-licensed under either:

- the [Apache License, Version 2.0](LICENSE-APACHE)
- the [MIT license](LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
