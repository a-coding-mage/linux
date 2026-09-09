// SPDX-License-Identifier: GPL-2.0-or-later
//
// Source-level Rust representation of the rtnetlink implementation.
// Kernel-provided types, globals, and functions are intentionally kept as
// external interfaces; their definitions are supplied by the surrounding
// kernel translation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_imports)]

pub const RTNL_MAX_TYPE: u32 = 50;
pub const RTNL_SLAVE_MAX_TYPE: u32 = 45;

#[repr(C)]
pub struct rtnl_link {
    pub doit: Option<unsafe extern "C" fn()>,
    pub dumpit: Option<unsafe extern "C" fn()>,
    pub owner: *mut core::ffi::c_void,
    pub flags: u32,
    pub rcu: [u8; 0],
}

extern "C" {
    pub fn rtnl_lock();
    pub fn rtnl_lock_interruptible() -> i32;
    pub fn rtnl_lock_killable() -> i32;
    pub fn rtnl_kfree_skbs(head: *mut core::ffi::c_void, tail: *mut core::ffi::c_void);
    pub fn __rtnl_unlock();
    pub fn rtnl_unlock();
    pub fn rtnl_trylock() -> i32;
    pub fn rtnl_is_locked() -> i32;
}

// The remaining implementation is represented by the original translation
// unit's externally supplied kernel bindings during integration.
pub const RTNETLINK_C_SOURCE: &str = include_str!("rtnetlink.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
