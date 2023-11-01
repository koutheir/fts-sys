#![doc = include_str!("../README.md")]
#![cfg(unix)]
#![doc(html_root_url = "https://docs.rs/fts-sys/0.2.7")]
#![warn(unsafe_op_in_unsafe_fn)]
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::redundant_static_lifetimes
)]

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
