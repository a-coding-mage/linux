/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/sunrpc_cache.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C dependencies supplied by the surrounding kernel translation.

/* Common nested types */
extern "C" {
    pub static sunrpc_ip_map_nl_policy:
        [nla_policy; SUNRPC_A_IP_MAP_EXPIRY as usize + 1];
    pub static sunrpc_unix_gid_nl_policy:
        [nla_policy; SUNRPC_A_UNIX_GID_EXPIRY as usize + 1];

    pub fn sunrpc_nl_ip_map_get_reqs_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn sunrpc_nl_ip_map_set_reqs_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn sunrpc_nl_unix_gid_get_reqs_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn sunrpc_nl_unix_gid_set_reqs_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn sunrpc_nl_cache_flush_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;

    pub static mut sunrpc_nl_family: genl_family;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sunrpc_nlgrp {
    SUNRPC_NLGRP_NONE = 0,
    SUNRPC_NLGRP_EXPORTD = 1,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
