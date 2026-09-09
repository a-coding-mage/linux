/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

/* Translated from send.h. The included declarations are supplied by other
 * translation units. */

unsafe extern "C" {
    pub fn batadv_forw_packet_free(
        forw_packet: *mut batadv_forw_packet,
        dropped: bool,
    );
    pub fn batadv_forw_packet_alloc(
        if_incoming: *mut batadv_hard_iface,
        if_outgoing: *mut batadv_hard_iface,
        queue_left: *mut atomic_t,
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
    ) -> *mut batadv_forw_packet;
    pub fn batadv_forw_packet_steal(
        packet: *mut batadv_forw_packet,
        l: *mut spinlock_t,
    ) -> bool;
    pub fn batadv_forw_packet_ogmv1_queue(
        bat_priv: *mut batadv_priv,
        forw_packet: *mut batadv_forw_packet,
        send_time: usize,
    );
    pub fn batadv_forw_packet_is_rebroadcast(
        forw_packet: *mut batadv_forw_packet,
    ) -> bool;

    pub fn batadv_send_skb_to_orig(
        skb: *mut sk_buff,
        orig_node: *mut batadv_orig_node,
        recv_if: *mut batadv_hard_iface,
    ) -> i32;
    pub fn batadv_send_skb_packet(
        skb: *mut sk_buff,
        hard_iface: *mut batadv_hard_iface,
        dst_addr: *const u8,
    ) -> i32;
    pub fn batadv_send_broadcast_skb(
        skb: *mut sk_buff,
        hard_iface: *mut batadv_hard_iface,
    ) -> i32;
    pub fn batadv_send_unicast_skb(
        skb: *mut sk_buff,
        neigh_node: *mut batadv_neigh_node,
    ) -> i32;
    pub fn batadv_forw_bcast_packet(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        delay: usize,
        own_packet: bool,
    ) -> i32;
    pub fn batadv_send_bcast_packet(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        delay: usize,
        own_packet: bool,
    );
    pub fn batadv_purge_outstanding_packets(
        bat_priv: *mut batadv_priv,
        hard_iface: *const batadv_hard_iface,
    );
    pub fn batadv_send_skb_prepare_unicast_4addr(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        orig_node: *mut batadv_orig_node,
        packet_subtype: i32,
    ) -> bool;
    pub fn batadv_send_skb_unicast(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        packet_type: i32,
        packet_subtype: i32,
        orig_node: *mut batadv_orig_node,
        vid: u16,
    ) -> i32;
    pub fn batadv_send_skb_via_tt_generic(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        packet_type: i32,
        packet_subtype: i32,
        dst_hint: *mut u8,
        vid: u16,
    ) -> i32;
    pub fn batadv_send_skb_via_gw(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        vid: u16,
    ) -> i32;
}

/// batadv_send_skb_via_tt() - send an skb via TT lookup
/// @bat_priv: the bat priv with all the mesh interface information
/// @skb: the payload to send
/// @dst_hint: can be used to override the destination contained in the skb
/// @vid: the vid to be used to search the translation table
///
/// Look up the recipient node for the destination address in the ethernet
/// header via the translation table. Wrap the given skb into a batman-adv
/// unicast header. Then send this frame to the according destination node.
///
/// Return: NET_XMIT_DROP in case of error or NET_XMIT_SUCCESS otherwise.
#[inline]
pub unsafe fn batadv_send_skb_via_tt(
    bat_priv: *mut batadv_priv,
    skb: *mut sk_buff,
    dst_hint: *mut u8,
    vid: u16,
) -> i32 {
    unsafe {
        batadv_send_skb_via_tt_generic(bat_priv, skb, BATADV_UNICAST, 0, dst_hint, vid)
    }
}

/// batadv_send_skb_via_tt_4addr() - send an skb via TT lookup
/// @bat_priv: the bat priv with all the mesh interface information
/// @skb: the payload to send
/// @packet_subtype: the unicast 4addr packet subtype to use
/// @dst_hint: can be used to override the destination contained in the skb
/// @vid: the vid to be used to search the translation table
///
/// Look up the recipient node for the destination address in the ethernet
/// header via the translation table. Wrap the given skb into a batman-adv
/// unicast-4addr header. Then send this frame to the according destination node.
///
/// Return: NET_XMIT_DROP in case of error or NET_XMIT_SUCCESS otherwise.
#[inline]
pub unsafe fn batadv_send_skb_via_tt_4addr(
    bat_priv: *mut batadv_priv,
    skb: *mut sk_buff,
    packet_subtype: i32,
    dst_hint: *mut u8,
    vid: u16,
) -> i32 {
    unsafe {
        batadv_send_skb_via_tt_generic(
            bat_priv,
            skb,
            BATADV_UNICAST_4ADDR,
            packet_subtype,
            dst_hint,
            vid,
        )
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
