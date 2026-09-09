/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Linus Lüssing
 */

// Translated from multicast.h. Declarations supplied by included headers remain
// external dependencies of this translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum batadv_forw_mode {
    /// Forward the packet to all nodes via a batman-adv broadcast packet.
    BATADV_FORW_BCAST,
    /// Forward the packet to some nodes via one or more batman-adv unicast packets.
    BATADV_FORW_UCASTS,
    /// Forward the packet to some nodes via a batman-adv multicast packet.
    BATADV_FORW_MCAST,
    /// Do not forward; drop it.
    BATADV_FORW_NONE,
}

#[repr(C)]
pub struct batadv_priv {
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
pub struct batadv_orig_node {
    _private: [u8; 0],
}

extern "C" {
    pub fn kfree_skb(skb: *mut sk_buff);
}

// CONFIG_BATMAN_ADV_MCAST selects the declarations below in the kernel build.
// The cfg feature preserves that build-time condition for Rust consumers.
#[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")]
extern "C" {
    pub fn batadv_mcast_forw_mode(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        vid: u16,
        is_routable: *mut i32,
    ) -> batadv_forw_mode;

    pub fn batadv_mcast_forw_send(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        vid: u16,
        is_routable: i32,
    ) -> i32;

    pub fn batadv_mcast_init(bat_priv: *mut batadv_priv);

    pub fn batadv_mcast_mesh_info_put(
        msg: *mut sk_buff,
        bat_priv: *mut batadv_priv,
    ) -> i32;

    pub fn batadv_mcast_flags_dump(
        msg: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> i32;

    pub fn batadv_mcast_free(bat_priv: *mut batadv_priv);

    pub fn batadv_mcast_purge_orig(orig_node: *mut batadv_orig_node);

    /* multicast_forw.c */
    pub fn batadv_mcast_forw_tracker_tvlv_handler(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
    ) -> i32;

    pub fn batadv_mcast_forw_packet_hdrlen(num_dests: u32) -> u32;

    pub fn batadv_mcast_forw_push(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        vid: u16,
        is_routable: i32,
        count: i32,
    ) -> bool;

    pub fn batadv_mcast_forw_mcsend(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
pub unsafe fn batadv_mcast_forw_mode(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
    _vid: u16,
    _is_routable: *mut i32,
) -> batadv_forw_mode {
    batadv_forw_mode::BATADV_FORW_BCAST
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
pub unsafe fn batadv_mcast_forw_send(
    _bat_priv: *mut batadv_priv,
    skb: *mut sk_buff,
    _vid: u16,
    _is_routable: i32,
) -> i32 {
    kfree_skb(skb);
    NET_XMIT_DROP
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
pub unsafe fn batadv_mcast_init(_bat_priv: *mut batadv_priv) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
pub unsafe fn batadv_mcast_mesh_info_put(
    _msg: *mut sk_buff,
    _bat_priv: *mut batadv_priv,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
pub unsafe fn batadv_mcast_flags_dump(
    _msg: *mut sk_buff,
    _cb: *mut netlink_callback,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
pub unsafe fn batadv_mcast_free(_bat_priv: *mut batadv_priv) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
pub unsafe fn batadv_mcast_purge_orig(_orig_node: *mut batadv_orig_node) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_MCAST"))]
pub unsafe fn batadv_mcast_forw_mcsend(
    _bat_priv: *mut batadv_priv,
    skb: *mut sk_buff,
) -> i32 {
    kfree_skb(skb);
    NET_XMIT_DROP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
