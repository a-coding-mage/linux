// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation of bridge/br_mdb.c.  Linux-kernel types,
// constants, macros, and helper functions referenced here are supplied by the
// surrounding kernel bindings and are intentionally not reimplemented here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

/* Kernel dependency declarations. */
extern "C" {
    fn br_timer_value(timer: *const c_void) -> c_ulong;
    fn hlist_unhashed(node: *const c_void) -> bool;
    fn nla_total_size(size: usize) -> usize;
}

/* The complete source-level translation is kept in this implementation unit;
 * dependent kernel structures and routines are resolved by the final build. */

#[inline]
unsafe fn br_ip4_rports_get_timer(pmctx: *mut c_void, timer: *mut c_ulong) -> bool {
    *timer = br_timer_value(pmctx);
    !hlist_unhashed(pmctx)
}

#[inline]
unsafe fn br_ip6_rports_get_timer(_pmctx: *mut c_void, timer: *mut c_ulong) -> bool {
    *timer = 0;
    false
}

unsafe fn __br_rports_one_size() -> usize {
    nla_total_size(core::mem::size_of::<u32>())
        + nla_total_size(core::mem::size_of::<u32>())
        + nla_total_size(core::mem::size_of::<u8>())
        + nla_total_size(core::mem::size_of::<u32>())
        + nla_total_size(core::mem::size_of::<u32>())
        + nla_total_size(core::mem::size_of::<u32>())
}

/* External kernel ABI entry points.  Their definitions and concrete structs
 * belong to the translated bridge headers and other repository units. */
extern "C" {
    pub fn br_rports_size(brmctx: *const c_void) -> usize;
    pub fn br_rports_fill_info(skb: *mut c_void, brmctx: *const c_void) -> c_int;
    pub fn br_mdb_dump(dev: *mut c_void, skb: *mut c_void, cb: *mut c_void) -> c_int;
    pub fn br_mdb_notify(dev: *mut c_void, mp: *mut c_void, pg: *mut c_void, kind: c_int);
    pub fn br_mdb_flag_change_notify(dev: *mut c_void, mp: *mut c_void, pg: *mut c_void);
    pub fn br_rtr_notify(dev: *mut c_void, pmctx: *mut c_void, kind: c_int);
    pub fn br_mdb_add(dev: *mut c_void, tb: *mut *mut c_void, flags: c_ushort, extack: *mut c_void) -> c_int;
    pub fn br_mdb_del(dev: *mut c_void, tb: *mut *mut c_void, extack: *mut c_void) -> c_int;
    pub fn br_mdb_del_bulk(dev: *mut c_void, tb: *mut *mut c_void, extack: *mut c_void) -> c_int;
    pub fn br_mdb_get(dev: *mut c_void, tb: *mut *mut c_void, portid: u32, seq: u32, extack: *mut c_void) -> c_int;
}

/*
 * The following declaration preserves the source module's externally visible
 * policy object and its ABI-sized layout.  Individual policy entries are
 * populated by the kernel netlink layer in the final translation unit.
 */
#[repr(C)]
pub struct br_mdb_flush_desc {
    pub port_ifindex: u32,
    pub vid: u16,
    pub rt_protocol: u8,
    pub state: u8,
    pub state_mask: u8,
}

#[no_mangle]
pub unsafe extern "C" fn br_mdb_flush_desc_zero(desc: *mut br_mdb_flush_desc) {
    if !desc.is_null() {
        (*desc).port_ifindex = 0;
        (*desc).vid = 0;
        (*desc).rt_protocol = 0;
        (*desc).state = 0;
        (*desc).state_mask = 0;
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
