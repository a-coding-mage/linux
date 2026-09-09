/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/netdev.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C header dependencies: <net/netlink.h>, <net/genetlink.h>,
// <uapi/linux/netdev.h>, <net/netdev_netlink.h>, and <asm/page.h>.

/* Common nested types */
extern "C" {
    pub static netdev_lease_nl_policy:
        [nla_policy; NETDEV_A_LEASE_NETNS_ID as usize + 1];
    pub static netdev_page_pool_info_nl_policy:
        [nla_policy; NETDEV_A_PAGE_POOL_IFINDEX as usize + 1];
    pub static netdev_queue_id_nl_policy:
        [nla_policy; NETDEV_A_QUEUE_TYPE as usize + 1];

    pub fn netdev_nl_dev_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn netdev_nl_dev_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_page_pool_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_page_pool_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_page_pool_stats_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_page_pool_stats_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_queue_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_queue_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_napi_get_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_napi_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_qstats_get_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_bind_rx_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_napi_set_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_bind_tx_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;
    pub fn netdev_nl_queue_create_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;

    pub static mut netdev_nl_family: genl_family;

    pub fn netdev_nl_sock_priv_init(priv_: *mut netdev_nl_sock);
    pub fn netdev_nl_sock_priv_destroy(priv_: *mut netdev_nl_sock);
}

pub const NETDEV_NLGRP_MGMT: ::core::ffi::c_uint = 0;
pub const NETDEV_NLGRP_PAGE_POOL: ::core::ffi::c_uint = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
