#![cfg(all(test, unix))]

use std::os::raw::c_char;
use std::ptr;

#[test]
fn fts_open() {
    let paths: [*const c_char; 2] = ["/\0".as_ptr().cast(), ptr::null()];
    let fts = unsafe { super::fts_open(paths.as_ptr().cast(), super::FTS_PHYSICAL, None) };
    assert_ne!(fts, ptr::null_mut());

    unsafe { super::fts_close(fts) };
}

#[test]
fn fts_read() {
    let paths: [*const c_char; 2] = ["/\0".as_ptr().cast(), ptr::null()];
    let fts = unsafe { super::fts_open(paths.as_ptr().cast(), super::FTS_PHYSICAL, None) };
    assert_ne!(fts, ptr::null_mut());

    let entry = unsafe { super::fts_read(fts) };
    assert_ne!(entry, ptr::null_mut());
    if let Some(entry) = unsafe { entry.as_ref() } {
        assert_ne!(entry.fts_name.as_ptr(), ptr::null_mut());
    }

    unsafe { super::fts_close(fts) };
}
