// SPDX-License-Identifier: GPL-2.0-only
//! Secondary ordinary ABI fixture: verbatim generated structs, actual ffi crate.
#![no_std]
pub use ffi;
/// Verbatim type declarations from native bindings; not a kernel substitute.
#[allow(non_camel_case_types, missing_docs)]
pub mod bindings {
use crate::ffi;
pub type __u8 = ffi::c_uchar;
#[repr(C)]
pub struct guid_t {
    pub b: [__u8; 16usize],
}
#[repr(C)]
pub struct uuid_t {
    pub b: [__u8; 16usize],
}
unsafe extern "C" {
    pub fn get_random_bytes(buf: *mut ffi::c_void, len: usize);
}
}
