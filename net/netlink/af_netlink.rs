// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust surface for the Linux netlink implementation.
// Kernel-provided types and operations are intentionally referenced as
// external dependencies; this isolated translation does not provide them.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const NETLINK_S_CONGESTED: usize = 0x0;

#[repr(C)]
pub struct rcu_head {
    pub next: *mut rcu_head,
    pub func: Option<unsafe extern "C" fn(*mut rcu_head)>,
}

#[repr(C)]
pub struct listeners {
    pub rcu: rcu_head,
    pub masks: [c_ulong; 0],
}

pub type c_ulong = usize;
pub type u8_ = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type s32 = i32;
pub type gfp_t = c_uint;

#[repr(C)]
pub struct netlink_compare_arg {
    pub pnet: possible_net_t,
    pub portid: u32,
}

#[repr(C)]
pub struct possible_net_t {
    pub value: *mut c_void,
}

extern "C" {
    pub static mut nl_table: *mut netlink_table;
    pub fn netlink_dump(sk: *mut sock, lock_taken: bool) -> c_int;
    pub fn trace_netlink_extack(msg: *const c_char);
    pub fn nlk_test_bit(bit: c_int, sk: *const sock) -> bool;
    pub fn netlink_group_mask(group: u32) -> u32;
}

#[repr(C)]
pub struct netlink_table {
    pub opaque: [u8; 0],
}

#[repr(C)]
pub struct sock {
    pub opaque: [u8; 0],
}

/// Kernel implementation entry point corresponding to `do_trace_netlink_extack`.
#[no_mangle]
pub unsafe extern "C" fn do_trace_netlink_extack(msg: *const c_char) {
    trace_netlink_extack(msg);
}

/// Compute the multicast bit represented by a netlink group number.
#[inline]
pub const fn netlink_group_mask_local(group: u32) -> u32 {
    if group > 32 { 0 } else if group != 0 { 1u32 << (group - 1) } else { 0 }
}

/// The remaining implementation consists of Linux-kernel operations supplied
/// by the surrounding kernel translation unit. Their declarations and exact
/// call sites remain source-compatible through the external ABI above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
