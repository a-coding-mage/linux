/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

/* Translated from routing.h. The declarations below depend on symbols from
 * the included kernel and BATMAN-adv headers. */

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct batadv_hard_iface {
    _private: [u8; 0],
}

#[repr(C)]
pub struct batadv_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct batadv_orig_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct batadv_neigh_node {
    _private: [u8; 0],
}

extern "C" {
    pub fn batadv_check_management_packet(
        skb: *mut sk_buff,
        hard_iface: *mut batadv_hard_iface,
        header_len: ::core::ffi::c_int,
    ) -> bool;

    pub fn batadv_update_route(
        bat_priv: *mut batadv_priv,
        orig_node: *mut batadv_orig_node,
        recv_if: *mut batadv_hard_iface,
        neigh_node: *mut batadv_neigh_node,
    );

    pub fn batadv_recv_icmp_packet(
        skb: *mut sk_buff,
        recv_if: *mut batadv_hard_iface,
    ) -> ::core::ffi::c_int;

    pub fn batadv_recv_unicast_packet(
        skb: *mut sk_buff,
        recv_if: *mut batadv_hard_iface,
    ) -> ::core::ffi::c_int;

    pub fn batadv_recv_frag_packet(
        skb: *mut sk_buff,
        iface: *mut batadv_hard_iface,
    ) -> ::core::ffi::c_int;

    pub fn batadv_recv_bcast_packet(
        skb: *mut sk_buff,
        recv_if: *mut batadv_hard_iface,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")]
    pub fn batadv_recv_mcast_packet(
        skb: *mut sk_buff,
        recv_if: *mut batadv_hard_iface,
    ) -> ::core::ffi::c_int;

    pub fn batadv_recv_unicast_tvlv(
        skb: *mut sk_buff,
        recv_if: *mut batadv_hard_iface,
    ) -> ::core::ffi::c_int;

    pub fn batadv_recv_unhandled_unicast_packet(
        skb: *mut sk_buff,
        recv_if: *mut batadv_hard_iface,
    ) -> ::core::ffi::c_int;

    pub fn batadv_find_router(
        bat_priv: *mut batadv_priv,
        orig_node: *mut batadv_orig_node,
        recv_if: *mut batadv_hard_iface,
    ) -> *mut batadv_neigh_node;

    pub fn batadv_window_protected(
        bat_priv: *mut batadv_priv,
        seq_num_diff: i32,
        seq_old_max_diff: i32,
        last_reset: *mut ::core::ffi::c_ulong,
        protection_started: *mut bool,
    ) -> bool;
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
#[inline]
pub unsafe fn batadv_recv_mcast_packet(
    skb: *mut sk_buff,
    _recv_if: *mut batadv_hard_iface,
) -> ::core::ffi::c_int {
    kfree_skb(skb);
    NET_RX_DROP
}

/* Supplied by the included kernel headers/dependencies. */
extern "C" {
    fn kfree_skb(skb: *mut sk_buff);
}

/* Supplied by the included kernel headers/dependencies. */
extern {
    static NET_RX_DROP: ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
