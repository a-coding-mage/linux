/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/lockd.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C dependencies:
// #include <net/netlink.h>
// #include <net/genetlink.h>
// #include <uapi/linux/lockd_netlink.h>

// Opaque declarations supplied by the corresponding kernel dependencies.
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

extern "C" {
    pub fn lockd_nl_server_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::std::os::raw::c_int;

    pub fn lockd_nl_server_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::std::os::raw::c_int;

    pub static mut lockd_nl_family: genl_family;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
