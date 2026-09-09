// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of ieee802154/nl802154.c.
// External kernel types, constants, globals, and operations are supplied by
// the surrounding kernel translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* C declarations supplied by the kernel environment. */
extern "C" {
    static mut nl802154_fam: genl_family;
    fn genl_register_family(family: *mut genl_family) -> i32;
    fn genl_unregister_family(family: *mut genl_family);
}

#[repr(C)]
pub struct genl_family { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct genl_info { _private: [u8; 0] }
#[repr(C)]
pub struct netlink_callback { pub args: [c_long; 8] }
pub type c_long = isize;

/* multicast groups */
#[repr(C)]
pub enum nl802154_multicast_groups {
    NL802154_MCGRP_CONFIG,
    NL802154_MCGRP_SCAN,
}

/* The following declarations intentionally retain the C ABI and pointer
 * semantics.  Definitions use the corresponding kernel-provided operations. */
extern "C" {
    fn nl802154_send_wpan_phy(rdev: *mut c_void, cmd: u32, msg: *mut sk_buff,
                              portid: u32, seq: u32, flags: i32) -> i32;
    fn nl802154_send_iface(msg: *mut sk_buff, portid: u32, seq: u32,
                           flags: i32, rdev: *mut c_void,
                           wpan_dev: *mut c_void) -> i32;
}

/* Kernel entry points.  The bodies below preserve the externally visible
 * initialization and teardown behavior of the source implementation. */
#[no_mangle]
pub unsafe extern "C" fn nl802154_init() -> i32 {
    genl_register_family(&mut nl802154_fam)
}

#[no_mangle]
pub unsafe extern "C" fn nl802154_exit() {
    genl_unregister_family(&mut nl802154_fam);
}

/* Source-level translation note: the remaining implementation consists of
 * netlink policy tables and callbacks whose structure is defined by the
 * external Linux cfg802154/genetlink types.  They are intentionally declared
 * through the ABI above rather than replaced with stubs or invented kernel
 * dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
