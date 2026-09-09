/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/dpll.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

/* Common nested types */
extern "C" {
    pub static dpll_pin_parent_device_nl_policy:
        [nla_policy; DPLL_A_PIN_OPERSTATE as usize + 1];
    pub static dpll_pin_parent_pin_nl_policy:
        [nla_policy; DPLL_A_PIN_STATE as usize + 1];
    pub static dpll_reference_sync_nl_policy:
        [nla_policy; DPLL_A_PIN_STATE as usize + 1];
}

extern "C" {
    pub fn dpll_lock_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn dpll_pre_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn dpll_pin_pre_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn dpll_unlock_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );
    pub fn dpll_post_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );
    pub fn dpll_pin_post_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );

    pub fn dpll_nl_device_id_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn dpll_nl_device_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn dpll_nl_device_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn dpll_nl_device_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn dpll_nl_pin_id_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn dpll_nl_pin_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn dpll_nl_pin_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn dpll_nl_pin_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
}

pub const DPLL_NLGRP_MONITOR: usize = 0;

extern "C" {
    pub static mut dpll_nl_family: genl_family;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
