#![doc(html_root_url = "https://docs.rs/fts-sys/0.1.0")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]

/*!
# `fts-sys`: Unsafe Rust bindings for FTS functions provided by `libc`

The FTS functions enable traversing file hierarchies.

## Versioning

This project adheres to [Semantic Versioning].
The `CHANGELOG.md` file details notable changes over time.

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
*/

use libc::dev_t;
use libc::ino_t;
use libc::nlink_t;
use libc::stat;

#[repr(C)]
#[derive(Debug)]
pub struct FTS {
    _unused: [u8; 0],
}

include!(concat!(env!("OUT_DIR"), "/fts-sys.rs"));

#[cfg(test)]
mod tests;
