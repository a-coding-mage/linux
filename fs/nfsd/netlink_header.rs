/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/* Documentation/netlink/specs/nfsd.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

/* Dependencies supplied by the corresponding kernel networking headers. */

#[repr(C)]
pub struct nla_policy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_callback {
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

/* Common nested types */
unsafe extern "C" {
    pub static nfsd_auth_flavor_nl_policy:
        [nla_policy; NFSD_A_AUTH_FLAVOR_FLAGS as usize + 1];
    pub static nfsd_expkey_nl_policy:
        [nla_policy; NFSD_A_EXPKEY_PATH as usize + 1];
    pub static nfsd_fslocation_nl_policy:
        [nla_policy; NFSD_A_FSLOCATION_PATH as usize + 1];
    pub static nfsd_fslocations_nl_policy:
        [nla_policy; NFSD_A_FSLOCATIONS_LOCATION as usize + 1];
    pub static nfsd_sock_nl_policy:
        [nla_policy; NFSD_A_SOCK_TRANSPORT_NAME as usize + 1];
    pub static nfsd_svc_export_nl_policy:
        [nla_policy; NFSD_A_SVC_EXPORT_FSID as usize + 1];
    pub static nfsd_version_nl_policy:
        [nla_policy; NFSD_A_VERSION_ENABLED as usize + 1];

    pub fn nfsd_nl_rpc_status_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_threads_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_threads_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_version_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_version_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_listener_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_listener_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_pool_mode_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_pool_mode_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_svc_export_get_reqs_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_svc_export_set_reqs_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_expkey_get_reqs_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_expkey_set_reqs_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_cache_flush_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_unlock_ip_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_unlock_filesystem_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_unlock_export_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn nfsd_nl_server_stats_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;

    pub static mut nfsd_nl_family: genl_family;
}

pub const NFSD_NLGRP_NONE: ::core::ffi::c_uint = 0;
pub const NFSD_NLGRP_EXPORTD: ::core::ffi::c_uint = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
