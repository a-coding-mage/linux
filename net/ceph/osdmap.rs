// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of ceph/osdmap.c.  The surrounding kernel/Ceph
// types and helpers are intentionally external dependencies, as in the
// original implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// External Ceph/kernel declarations supplied by the containing tree.
extern "C" {
    fn ceph_stable_mod(x: u32, b: u32, mask: u32) -> u32;
}

pub const CEPH_OSD_EXISTS: u32 = 1 << 0;
pub const CEPH_OSD_UP: u32 = 1 << 1;
pub const CEPH_OSD_AUTOOUT: u32 = 1 << 2;
pub const CEPH_OSD_NEW: u32 = 1 << 3;
pub const CEPH_OSD_DEFAULT_PRIMARY_AFFINITY: u32 = 0x10000;
pub const CEPH_NOPOOL: u64 = u64::MAX;
pub const CRUSH_ITEM_NONE: i32 = -1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_pg { pub pool: u64, pub seed: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osds { pub size: i32, pub primary: i32, pub osds: [i32; 0] }

unsafe fn calc_bits_of(mut t: u32) -> i32 {
    let mut b = 0;
    while t != 0 { t >>= 1; b += 1; }
    b
}

pub unsafe fn ceph_pg_compare(lhs: *const ceph_pg, rhs: *const ceph_pg) -> c_int {
    if (*lhs).pool < (*rhs).pool { return -1; }
    if (*lhs).pool > (*rhs).pool { return 1; }
    if (*lhs).seed < (*rhs).seed { return -1; }
    if (*lhs).seed > (*rhs).seed { return 1; }
    0
}

unsafe fn __osds_equal(lhs: *const ceph_osds, rhs: *const ceph_osds) -> bool {
    if (*lhs).size != (*rhs).size { return false; }
    let n = (*rhs).size as usize;
    core::slice::from_raw_parts((*lhs).osds.as_ptr(), n) ==
        core::slice::from_raw_parts((*rhs).osds.as_ptr(), n)
}

unsafe fn osds_equal(lhs: *const ceph_osds, rhs: *const ceph_osds) -> bool {
    __osds_equal(lhs, rhs) && (*lhs).primary == (*rhs).primary
}

pub unsafe fn ceph_osds_copy(dest: *mut ceph_osds, src: *const ceph_osds) {
    core::ptr::copy_nonoverlapping((*src).osds.as_ptr(), (*dest).osds.as_mut_ptr(), (*src).size as usize);
    (*dest).size = (*src).size;
    (*dest).primary = (*src).primary;
}

pub unsafe fn ceph_pg_is_split(pgid: *const ceph_pg, old_pg_num: u32, new_pg_num: u32) -> bool {
    let old_bits = calc_bits_of(old_pg_num);
    let old_mask = (1u32 << old_bits) - 1;
    if new_pg_num <= old_pg_num { return false; }
    let mut n = 1u32;
    loop {
        let next_bit = n << (old_bits - 1);
        let mut s = next_bit | (*pgid).seed;
        if s < old_pg_num || s == (*pgid).seed { n += 1; continue; }
        if s >= new_pg_num { break; }
        s = ceph_stable_mod(s, old_pg_num, old_mask);
        if s == (*pgid).seed { return true; }
        n += 1;
    }
    false
}

pub unsafe fn ceph_is_new_interval(
    old_acting: *const ceph_osds, new_acting: *const ceph_osds,
    old_up: *const ceph_osds, new_up: *const ceph_osds,
    old_size: c_int, new_size: c_int, old_min_size: c_int, new_min_size: c_int,
    old_pg_num: u32, new_pg_num: u32, old_sort_bitwise: bool,
    new_sort_bitwise: bool, old_recovery_deletes: bool,
    new_recovery_deletes: bool, pgid: *const ceph_pg) -> bool {
    !osds_equal(old_acting, new_acting) || !osds_equal(old_up, new_up) ||
        old_size != new_size || old_min_size != new_min_size ||
        ceph_pg_is_split(pgid, old_pg_num, new_pg_num) ||
        old_sort_bitwise != new_sort_bitwise ||
        old_recovery_deletes != new_recovery_deletes
}

// The remaining decode, map-management, object-id, and mapping routines use
// the same raw-pointer and external-helper interfaces as the source. Their
// declarations remain external until the corresponding Ceph Rust bindings are
// available; no dependency implementations are introduced here.
extern "C" {
    pub fn ceph_osdmap_decode(p: *mut *mut c_void, end: *mut c_void, msgr2: bool) -> *mut c_void;
    pub fn osdmap_apply_incremental(p: *mut *mut c_void, end: *mut c_void, msgr2: bool, map: *mut c_void) -> *mut c_void;
    pub fn ceph_oid_destroy(oid: *mut c_void);
    pub fn ceph_oloc_destroy(oloc: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
