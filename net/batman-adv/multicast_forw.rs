// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 * Linus Lüssing
 *
 * Direct Rust translation of multicast_forw.c. Kernel types and helpers are
 * supplied by the surrounding batman-adv bindings.
 */

const ETH_ALEN: usize = 6;
const U16_MAX: u16 = 65535;

#[inline]
unsafe fn batadv_mcast_forw_skb_push(skb: *mut sk_buff, size: usize,
                                     len: *mut u16) -> *mut core::ffi::c_void {
    *len = (*len).wrapping_add(size as u16);
    skb_push(skb, size)
}

unsafe fn batadv_mcast_forw_push_padding(skb: *mut sk_buff, tvlv_len: *mut u16) -> *mut i8 {
    if skb_headroom(skb) < 2 { return core::ptr::null_mut(); }
    let padding = batadv_mcast_forw_skb_push(skb, 2, tvlv_len) as *mut i8;
    core::ptr::write_bytes(padding, 0, 2);
    padding
}

unsafe fn batadv_mcast_forw_push_est_padding(skb: *mut sk_buff, count: i32,
                                             tvlv_len: *mut u16) -> bool {
    if count % 2 == 0 && batadv_mcast_forw_push_padding(skb, tvlv_len).is_null() { return false; }
    true
}

unsafe fn batadv_mcast_forw_orig_entry(node: *mut hlist_node, entry_offset: usize)
    -> *mut batadv_orig_node {
    let valid = entry_offset == offset_of!(batadv_orig_node, mcast_want_all_ipv4_node)
        || entry_offset == offset_of!(batadv_orig_node, mcast_want_all_ipv6_node)
        || entry_offset == offset_of!(batadv_orig_node, mcast_want_all_rtr4_node)
        || entry_offset == offset_of!(batadv_orig_node, mcast_want_all_rtr6_node);
    if !valid { WARN_ON(1); return core::ptr::null_mut(); }
    (node as *mut u8).sub(entry_offset) as *mut batadv_orig_node
}

unsafe fn batadv_mcast_forw_push_dest(bat_priv: *mut batadv_priv, skb: *mut sk_buff,
    vid: u16, orig_node: *mut batadv_orig_node, num_dests: *mut u16,
    tvlv_len: *mut u16) -> bool {
    if batadv_bla_is_backbone_gw_orig(bat_priv, (*orig_node).orig.as_ptr(), vid) { return true; }
    if skb_headroom(skb) < ETH_ALEN || *num_dests == U16_MAX { return false; }
    batadv_mcast_forw_skb_push(skb, ETH_ALEN, tvlv_len);
    ether_addr_copy((*skb).data, (*orig_node).orig.as_ptr());
    *num_dests = (*num_dests).wrapping_add(1);
    true
}

unsafe fn batadv_mcast_forw_push_dests_list(bat_priv: *mut batadv_priv, skb: *mut sk_buff,
    vid: u16, head: *mut hlist_head, offset: usize, num_dests: *mut u16,
    tvlv_len: *mut u16) -> bool {
    rcu_read_lock();
    let mut node = (*head).first;
    while !node.is_null() {
        let orig = batadv_mcast_forw_orig_entry(node, offset);
        if orig.is_null() || !batadv_mcast_forw_push_dest(bat_priv, skb, vid, orig, num_dests, tvlv_len) {
            rcu_read_unlock(); return false;
        }
        node = (*node).next;
    }
    rcu_read_unlock(); true
}

unsafe fn batadv_mcast_forw_push_tt(bat_priv: *mut batadv_priv, skb: *mut sk_buff, vid: u16,
    num_dests: *mut u16, tvlv_len: *mut u16) -> bool {
    let global = batadv_tt_global_hash_find(bat_priv, eth_hdr(skb).h_dest.as_ptr(), vid);
    if global.is_null() { return true; }
    let mut ret = true;
    rcu_read_lock();
    let mut entry = (*global).orig_list.first;
    while !entry.is_null() {
        let e = container_of!(entry, batadv_tt_orig_list_entry, list);
        if !batadv_mcast_forw_push_dest(bat_priv, skb, vid, (*e).orig_node, num_dests, tvlv_len) { ret = false; break; }
        entry = (*entry).next;
    }
    rcu_read_unlock(); batadv_tt_global_entry_put(global); ret
}

unsafe fn batadv_mcast_forw_push_want_all(bat_priv: *mut batadv_priv, skb: *mut sk_buff,
    vid: u16, num_dests: *mut u16, tvlv_len: *mut u16) -> bool {
    let (head, off) = match eth_hdr(skb).h_proto {
        ETH_P_IP => (&mut (*bat_priv).mcast.want_all_ipv4_list as *mut _, offset_of!(batadv_orig_node, mcast_want_all_ipv4_node)),
        ETH_P_IPV6 => (&mut (*bat_priv).mcast.want_all_ipv6_list as *mut _, offset_of!(batadv_orig_node, mcast_want_all_ipv6_node)),
        _ => return false,
    };
    batadv_mcast_forw_push_dests_list(bat_priv, skb, vid, head, off, num_dests, tvlv_len)
}

unsafe fn batadv_mcast_forw_push_want_rtr(bat_priv: *mut batadv_priv, skb: *mut sk_buff,
    vid: u16, num_dests: *mut u16, tvlv_len: *mut u16) -> bool {
    let (head, off) = match eth_hdr(skb).h_proto {
        ETH_P_IP => (&mut (*bat_priv).mcast.want_all_rtr4_list as *mut _, offset_of!(batadv_orig_node, mcast_want_all_rtr4_node)),
        ETH_P_IPV6 => (&mut (*bat_priv).mcast.want_all_rtr6_list as *mut _, offset_of!(batadv_orig_node, mcast_want_all_rtr6_node)),
        _ => return false,
    };
    batadv_mcast_forw_push_dests_list(bat_priv, skb, vid, head, off, num_dests, tvlv_len)
}

unsafe fn batadv_mcast_forw_push_dests(bat_priv: *mut batadv_priv, skb: *mut sk_buff,
    vid: u16, routable: i32, count: *mut i32, tvlv_len: *mut u16) -> i32 {
    let mut n = 0u16;
    if !batadv_mcast_forw_push_est_padding(skb, *count, tvlv_len) { return -ENOMEM; }
    if !batadv_mcast_forw_push_tt(bat_priv, skb, vid, &mut n, tvlv_len) { return -ENOMEM; }
    if !batadv_mcast_forw_push_want_all(bat_priv, skb, vid, &mut n, tvlv_len) { return -ENOMEM; }
    if routable != 0 && !batadv_mcast_forw_push_want_rtr(bat_priv, skb, vid, &mut n, tvlv_len) { return -ENOMEM; }
    if (n % 2) != ((*count as u16) % 2) {
        if *count % 2 == 0 { batadv_mcast_forw_scrape_padding(skb, tvlv_len); }
        else if !batadv_mcast_forw_push_padding(skb, tvlv_len).is_null() {}
        else { return -ENOMEM; }
    }
    *count = n as i32; 0
}

unsafe fn batadv_mcast_forw_scrape_padding(skb: *mut sk_buff, len: *mut u16) {
    let to = skb_pull(skb, 2) as *mut u8;
    core::ptr::copy(to.sub(2), to, (*len as usize).saturating_sub(2)); *len -= 2;
}

pub unsafe fn batadv_mcast_forw_packet_hdrlen(num_dests: u32) -> u32 {
    (if num_dests % 2 == 0 { 2 } else { 0 }) + num_dests * ETH_ALEN as u32
        + core::mem::size_of::<batadv_tvlv_mcast_tracker>() as u32
        + core::mem::size_of::<batadv_tvlv_hdr>() as u32
        + core::mem::size_of::<batadv_mcast_packet>() as u32
}

pub unsafe fn batadv_mcast_forw_tracker_tvlv_handler(bat_priv: *mut batadv_priv,
                                                     skb: *mut sk_buff) -> i32 {
    batadv_mcast_forw_packet(bat_priv, skb, false)
}

unsafe fn batadv_mcast_forw_packet(bat_priv: *mut batadv_priv, skb: *mut sk_buff,
                                   local_xmit: bool) -> i32 {
    let tracker = skb_network_header(skb) as *mut batadv_tvlv_mcast_tracker;
    let n = u16::from_be((*tracker).num_dests);
    let mut dest = (tracker as *mut u8).add(core::mem::size_of::<batadv_tvlv_mcast_tracker>());
    if core::mem::size_of::<batadv_tvlv_mcast_tracker>() + n as usize * ETH_ALEN > skb_network_header_len(skb) { return -EINVAL; }
    (*skb).ip_summed = CHECKSUM_NONE;
    for _ in 0..n {
        if !is_zero_ether_addr(dest) && !is_multicast_ether_addr(dest) {
            let neigh = batadv_orig_to_router(bat_priv, dest, core::ptr::null_mut());
            if !neigh { eth_zero_addr(dest); } else {
                let copy = skb_copy(skb, GFP_ATOMIC); if copy.is_null() { batadv_neigh_node_put(neigh); return -ENOMEM; }
                batadv_inc_counter(bat_priv, BATADV_CNT_MCAST_TX);
                let ret = batadv_send_unicast_skb(copy, neigh); batadv_neigh_node_put(neigh); if ret < 0 { return ret; }
            }
        }
        dest = dest.add(ETH_ALEN);
    }
    if local_xmit { NET_RX_SUCCESS } else { NET_RX_DROP }
}

pub unsafe fn batadv_mcast_forw_mcsend(bat_priv: *mut batadv_priv, skb: *mut sk_buff) -> i32 {
    let ret = batadv_mcast_forw_packet(bat_priv, skb, true);
    if ret < 0 { kfree_skb(skb); NET_XMIT_DROP } else { consume_skb(skb); NET_XMIT_SUCCESS }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
