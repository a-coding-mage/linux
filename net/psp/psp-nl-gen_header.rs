/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/* Documentation/netlink/specs/psp.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// Common nested types
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined from this header.
extern "C" {
    pub static mut psp_keys_nl_policy: [nla_policy; PSP_A_KEYS_SPI as usize + 1];

    pub fn psp_device_get_locked(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_device_get_locked_admin(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_assoc_device_get_locked(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_device_get_locked_dev_assoc(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_device_unlock(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );

    pub fn psp_nl_dev_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_dev_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_dev_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_key_rotate_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_rx_assoc_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_tx_assoc_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_get_stats_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_get_stats_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_dev_assoc_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn psp_nl_dev_disassoc_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;

    pub static mut psp_nl_family: genl_family;
}

pub const PSP_NLGRP_MGMT: u32 = 0;
pub const PSP_NLGRP_USE: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
