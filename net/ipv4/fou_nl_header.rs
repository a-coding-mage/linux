/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/fou.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// Dependencies supplied by the surrounding kernel translation.

/* Global operation policy for fou */
extern "C" {
    pub static fou_nl_policy: [nla_policy; FOU_ATTR_IFINDEX as usize + 1];

    /* Ops table for fou */
    pub static fou_nl_ops: [genl_small_ops; 3];

    pub fn fou_nl_add_doit(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn fou_nl_del_doit(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn fou_nl_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn fou_nl_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
