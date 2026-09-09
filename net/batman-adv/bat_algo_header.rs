/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Linus Lüssing
 */

// C dependencies: `main.h`, <linux/netlink.h>, and <linux/skbuff.h>.
// The corresponding types are supplied by other translated units.

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct batadv_algo_ops {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct batadv_priv {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct netlink_callback {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut batadv_routing_algo: [::core::ffi::c_char; 0];

    pub fn batadv_algo_init();
    pub fn batadv_algo_get(name: *const ::core::ffi::c_char) -> *mut batadv_algo_ops;
    pub fn batadv_algo_register(bat_algo_ops: *mut batadv_algo_ops) -> ::core::ffi::c_int;
    pub fn batadv_algo_select(
        bat_priv: *mut batadv_priv,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn batadv_algo_dump(
        msg: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
