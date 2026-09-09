/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*\tDocumentation/netlink/specs/dev-energymodel.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

/* Dependencies supplied by the corresponding netlink and uapi headers. */

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genl_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_callback {
    _private: [u8; 0],
}

extern "C" {
    pub fn dev_energymodel_nl_get_perf_domains_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;

    pub fn dev_energymodel_nl_get_perf_domains_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;

    pub fn dev_energymodel_nl_get_perf_table_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
}

pub const DEV_ENERGYMODEL_NLGRP_EVENT: ::core::ffi::c_uint = 0;

extern "C" {
    pub static mut dev_energymodel_nl_family: genl_family;
}

#[repr(C)]
pub struct genl_family {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
