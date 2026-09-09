/* SPDX-License-Identifier: GPL-2.0 */

/*
 *	Generic netevent notifiers
 *
 *	Authors:
 *      Tom Tucker              <tom@opengridcomputing.com>
 *      Steve Wise              <swise@opengridcomputing.com>
 *
 * 	Changes:
 */

use core::ffi::c_void;

#[repr(C)]
pub struct dst_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct neighbour {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netevent_redirect {
    pub old: *mut dst_entry,
    pub new: *mut dst_entry,
    pub neigh: *mut neighbour,
    pub daddr: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netevent_notif_type {
    NETEVENT_NEIGH_UPDATE = 1, /* arg is struct neighbour ptr */
    NETEVENT_REDIRECT, /* arg is struct netevent_redirect ptr */
    NETEVENT_DELAY_PROBE_TIME_UPDATE, /* arg is struct neigh_parms ptr */
    NETEVENT_IPV4_MPATH_HASH_UPDATE, /* arg is struct net ptr */
    NETEVENT_IPV6_MPATH_HASH_UPDATE, /* arg is struct net ptr */
    NETEVENT_IPV4_FWD_UPDATE_PRIORITY_UPDATE, /* arg is struct net ptr */
}

unsafe extern "C" {
    pub fn register_netevent_notifier(nb: *mut notifier_block) -> i32;
    pub fn unregister_netevent_notifier(nb: *mut notifier_block) -> i32;
    pub fn call_netevent_notifiers(val: c_ulong, v: *mut c_void) -> i32;
}

type c_ulong = core::ffi::c_ulong;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
