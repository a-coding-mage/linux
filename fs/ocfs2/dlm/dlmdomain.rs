// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of ocfs2/dlm/dlmdomain.c.
// Kernel and OCFS2 declarations are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::{ffi::c_void, ptr};

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;

#[repr(C)]
pub struct dlm_protocol_version { pub pv_major: u8, pub pv_minor: u8 }

extern "C" {
    static mut dlm_domain_lock: c_void;
    static mut dlm_domains: c_void;
    fn test_bit(nr: usize, addr: *const c_void) -> bool;
    fn set_bit(nr: usize, addr: *mut c_void);
    fn clear_bit(nr: usize, addr: *mut c_void);
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
    fn strlen(s: *const i8) -> usize;
}

#[inline]
unsafe fn byte_set_bit(nr: u8, map: *mut u8) { *map.add((nr >> 3) as usize) |= 1u8 << (nr & 7); }
#[inline]
unsafe fn byte_test_bit(nr: u8, map: *const u8) -> bool { (*map.add((nr >> 3) as usize) & (1u8 << (nr & 7))) != 0 }
#[inline]
unsafe fn byte_copymap(dmap: *mut u8, smap: *const c_void, sz: u32) {
    if sz == 0 { return; }
    memset(dmap as *mut c_void, 0, ((sz + 7) >> 3) as usize);
    for nn in 0..sz { if test_bit(nn as usize, smap) { byte_set_bit(nn as u8, dmap); } }
}

// The following declarations retain the source interfaces; their definitions use
// the kernel list, locking, allocation, networking, and DLM types supplied by the
// companion translated units.
extern "C" {
    pub fn __dlm_unhash_lockres(dlm: *mut c_void, res: *mut c_void);
    pub fn __dlm_insert_lockres(dlm: *mut c_void, res: *mut c_void);
    pub fn __dlm_lookup_lockres_full(dlm: *mut c_void, name: *const i8, len: u32, hash: u32) -> *mut c_void;
    pub fn __dlm_lookup_lockres(dlm: *mut c_void, name: *const i8, len: u32, hash: u32) -> *mut c_void;
    pub fn dlm_lookup_lockres(dlm: *mut c_void, name: *const i8, len: u32) -> *mut c_void;
    pub fn dlm_unregister_domain(dlm: *mut c_void);
    pub fn dlm_register_domain(domain: *const i8, key: u32, proto: *mut dlm_protocol_version) -> *mut c_void;
    pub fn dlm_fire_domain_eviction_callbacks(dlm: *mut c_void, node_num: i32);
    pub fn dlm_setup_eviction_cb(cb: *mut c_void, f: *const c_void, data: *mut c_void);
    pub fn dlm_register_eviction_cb(dlm: *mut c_void, cb: *mut c_void);
    pub fn dlm_unregister_eviction_cb(cb: *mut c_void);
}

#[inline]
unsafe fn dlm_protocol_compare(existing: *mut dlm_protocol_version, request: *mut dlm_protocol_version) -> i32 {
    if (*existing).pv_major != (*request).pv_major { return 1; }
    if (*existing).pv_minor > (*request).pv_minor { return 1; }
    if (*existing).pv_minor < (*request).pv_minor { (*request).pv_minor = (*existing).pv_minor; }
    0
}

// Module setup/teardown are intentionally declarations: cache and handler
// implementations belong to the surrounding OCFS2 translation units.
extern "C" {
    fn dlm_init() -> i32;
    fn dlm_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
