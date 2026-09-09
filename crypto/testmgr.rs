// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation boundary for the kernel crypto test
// manager.  The implementation relies on the kernel interfaces supplied by
// the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const XBUFSIZE: usize = 8;
pub const ENCRYPT: c_uint = 1;
pub const DECRYPT: c_uint = 0;
pub const TESTMGR_POISON_BYTE: u8 = 0xfe;
pub const TESTMGR_POISON_LEN: usize = 16;
pub const TEST_SG_TOTAL: c_uint = 10000;

#[repr(C)]
pub struct aead_test_suite {
    pub vecs: *const c_void,
    pub count: c_uint,
    pub einval_allowed: c_uint,
    pub aad_iv: c_uint,
}

#[repr(C)]
pub struct cipher_test_suite {
    pub vecs: *const c_void,
    pub count: c_uint,
}

#[repr(C)]
pub struct test_sg_division {
    pub proportion_of_total: c_uint,
    pub offset: c_uint,
    pub offset_relative_to_alignmask: bool,
    pub flush_type: flush_type,
    pub nosimd: bool,
}

#[repr(C)]
pub struct testvec_config {
    pub name: *const c_char,
    pub inplace_mode: inplace_mode,
    pub req_flags: u32,
    pub src_divs: [test_sg_division; XBUFSIZE],
    pub dst_divs: [test_sg_division; XBUFSIZE],
    pub iv_offset: c_uint,
    pub key_offset: c_uint,
    pub iv_offset_relative_to_alignmask: bool,
    pub key_offset_relative_to_alignmask: bool,
    pub finalization_type: finalization_type,
    pub nosimd: bool,
    pub nosimd_setkey: bool,
}

#[repr(C)]
pub struct test_sglist {
    pub bufs: [*mut c_char; XBUFSIZE],
    pub sgl: [*mut c_void; XBUFSIZE],
    pub sgl_saved: [*mut c_void; XBUFSIZE],
    pub sgl_ptr: *mut c_void,
    pub nents: c_uint,
}

#[repr(C)]
pub struct cipher_test_sglists {
    pub src: test_sglist,
    pub dst: test_sglist,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum flush_type {
    FLUSH_TYPE_NONE = 0,
    FLUSH_TYPE_FLUSH = 1,
    FLUSH_TYPE_REIMPORT = 2,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum finalization_type {
    FINALIZATION_TYPE_FINAL = 0,
    FINALIZATION_TYPE_FINUP = 1,
    FINALIZATION_TYPE_DIGEST = 2,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum inplace_mode {
    OUT_OF_PLACE = 0,
    INPLACE_ONE_SGLIST = 1,
    INPLACE_TWO_SGLISTS = 2,
}

#[inline]
pub unsafe fn testmgr_poison(addr: *mut c_void, len: usize) {
    core::ptr::write_bytes(addr, TESTMGR_POISON_BYTE, len);
}

/* The remaining kernel-specific test-manager routines are declaration-only
 * dependencies in this isolated translation unit; their definitions and
 * associated crypto vector types are supplied by the surrounding kernel port.
 */
extern "C" {
    pub fn alg_test(driver: *const c_char, alg: *const c_char,
                    type_: u32, mask: u32) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
