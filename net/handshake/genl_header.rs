/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*\tDocumentation/netlink/specs/handshake.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C dependencies: <net/netlink.h>, <net/genetlink.h>,
// <uapi/linux/handshake.h>, and <linux/err.h>.

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genl_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genl_family {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn handshake_nl_accept_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::std::os::raw::c_int;
    pub fn handshake_nl_done_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::std::os::raw::c_int;

    pub static mut handshake_nl_family: genl_family;
}

pub const HANDSHAKE_NLGRP_NONE: ::std::os::raw::c_uint = 0;
pub const HANDSHAKE_NLGRP_TLSHD: ::std::os::raw::c_uint = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
