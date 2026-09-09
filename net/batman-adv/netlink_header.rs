/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Matthias Schiffer
 */

// Dependency intent from main.h, <linux/netlink.h>, and <linux/types.h> is
// preserved through the externally supplied C-compatible types below.

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_callback {
    _private: [u8; 0],
}

#[repr(C)]
pub struct batadv_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genl_family {
    _private: [u8; 0],
}

extern "C" {
    pub fn batadv_netlink_register() -> ::core::ffi::c_int;
    pub fn batadv_netlink_unregister();
    pub fn batadv_netlink_get_meshif(cb: *mut netlink_callback) -> *mut net_device;
    pub fn batadv_netlink_get_hardif(
        bat_priv: *mut batadv_priv,
        cb: *mut netlink_callback,
    ) -> *mut net_device;

    pub fn batadv_netlink_tpmeter_notify(
        bat_priv: *mut batadv_priv,
        dst: *const u8,
        result: u8,
        test_time: u32,
        total_bytes: u64,
        cookie: u32,
    ) -> ::core::ffi::c_int;

    pub static mut batadv_netlink_family: genl_family;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
