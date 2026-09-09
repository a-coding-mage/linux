/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/net_shaper.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

/* C dependencies: <net/netlink.h>, <net/genetlink.h>, and
 * <uapi/linux/net_shaper.h>. */

pub const NET_SHAPER_MAX_HANDLE_ID: u32 = 67108862;

/* Common nested types */
extern "C" {
    pub static net_shaper_handle_nl_policy: [nla_policy; NET_SHAPER_A_HANDLE_ID as usize + 1];
    pub static net_shaper_leaf_info_nl_policy: [nla_policy; NET_SHAPER_A_WEIGHT as usize + 1];

    pub fn net_shaper_nl_pre_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> i32;
    pub fn net_shaper_nl_pre_doit_write(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> i32;
    pub fn net_shaper_nl_cap_pre_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> i32;
    pub fn net_shaper_nl_post_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );
    pub fn net_shaper_nl_post_doit_write(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );
    pub fn net_shaper_nl_cap_post_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );
    pub fn net_shaper_nl_pre_dumpit(cb: *mut netlink_callback) -> i32;
    pub fn net_shaper_nl_cap_pre_dumpit(cb: *mut netlink_callback) -> i32;
    pub fn net_shaper_nl_post_dumpit(cb: *mut netlink_callback) -> i32;
    pub fn net_shaper_nl_cap_post_dumpit(cb: *mut netlink_callback) -> i32;

    pub fn net_shaper_nl_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn net_shaper_nl_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> i32;
    pub fn net_shaper_nl_set_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn net_shaper_nl_delete_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn net_shaper_nl_group_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn net_shaper_nl_cap_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn net_shaper_nl_cap_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> i32;

    pub static mut net_shaper_nl_family: genl_family;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
