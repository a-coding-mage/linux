// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// C dependencies supplied by the surrounding kernel/batman-adv translation.

static mut BATADV_SEND_OUTSTANDING_BCAST_PACKET: Option<unsafe extern "C" fn(*mut work_struct)> = None;

pub unsafe fn batadv_send_skb_packet(
    skb: *mut sk_buff,
    hard_iface: *mut batadv_hard_iface,
    dst_addr: *const u8,
) -> i32 {
    let ethhdr: *mut ethhdr;
    let ret: i32;

    if (*hard_iface).if_status != BATADV_IF_ACTIVE { goto_send_skb_err!(skb); }
    if (*hard_iface).net_dev.is_null() { goto_send_skb_err!(skb); }
    if ((*(*hard_iface).net_dev).flags & IFF_UP) == 0 {
        pr_warn!("Interface {} is not up - can't send packet via that interface!\n", (*(*hard_iface).net_dev).name);
        goto_send_skb_err!(skb);
    }

    if batadv_skb_head_push(skb, ETH_HLEN) < 0 { goto_send_skb_err!(skb); }
    skb_reset_mac_header(skb);
    ethhdr = eth_hdr(skb);
    ether_addr_copy((*ethhdr).h_source.as_mut_ptr(), (*(*hard_iface).net_dev).dev_addr.as_ptr());
    ether_addr_copy((*ethhdr).h_dest.as_mut_ptr(), dst_addr);
    (*ethhdr).h_proto = htons(ETH_P_BATMAN);
    skb_set_network_header(skb, ETH_HLEN);
    (*skb).protocol = htons(ETH_P_BATMAN);
    (*skb).dev = (*hard_iface).net_dev;
    ret = dev_queue_xmit(skb);
    return net_xmit_eval(ret);

    macro_rules! goto_send_skb_err { ($s:expr) => {{ kfree_skb($s); return NET_XMIT_DROP; }}; }
}

pub unsafe fn batadv_send_broadcast_skb(skb: *mut sk_buff, hard_iface: *mut batadv_hard_iface) -> i32 {
    static BROADCAST_ADDR: [u8; 6] = [0xff; 6];
    batadv_send_skb_packet(skb, hard_iface, BROADCAST_ADDR.as_ptr())
}

pub unsafe fn batadv_send_unicast_skb(skb: *mut sk_buff, neigh: *mut batadv_neigh_node) -> i32 {
    let ret = batadv_send_skb_packet(skb, (*neigh).if_incoming, (*neigh).addr.as_ptr());
    #[cfg(CONFIG_BATMAN_ADV_BATMAN_V)] {
        let hardif_neigh = batadv_hardif_neigh_get((*neigh).if_incoming, (*neigh).addr.as_ptr());
        if !hardif_neigh.is_null() && ret != NET_XMIT_DROP { (*hardif_neigh).bat_v.last_unicast_tx = jiffies; }
        batadv_hardif_neigh_put(hardif_neigh);
    }
    ret
}

pub unsafe fn batadv_send_skb_to_orig(skb: *mut sk_buff, orig_node: *mut batadv_orig_node, recv_if: *mut batadv_hard_iface) -> i32 {
    let bat_priv = (*orig_node).bat_priv;
    let neigh_node = batadv_find_router(bat_priv, orig_node, recv_if);
    if neigh_node.is_null() { kfree_skb(skb); return -EINVAL; }
    let ret;
    if READ_ONCE!((*bat_priv).fragmentation) && (*skb).len > (*(*(*neigh_node).if_incoming).net_dev).mtu as usize {
        ret = batadv_frag_send_packet(skb, orig_node, neigh_node);
    } else {
        ret = batadv_send_unicast_skb(skb, neigh_node);
    }
    batadv_neigh_node_put(neigh_node);
    kfree_skb(std::ptr::null_mut());
    ret
}

unsafe fn batadv_send_skb_push_fill_unicast(skb: *mut sk_buff, hdr_size: i32, orig_node: *mut batadv_orig_node) -> bool {
    let ttvn = READ_ONCE!((*orig_node).last_ttvn);
    if batadv_skb_head_push(skb, hdr_size) < 0 { return false; }
    let p = (*skb).data as *mut batadv_unicast_packet;
    (*p).version = BATADV_COMPAT_VERSION;
    (*p).packet_type = BATADV_UNICAST;
    (*p).ttl = BATADV_TTL;
    ether_addr_copy((*p).dest.as_mut_ptr(), (*orig_node).orig.as_ptr());
    (*p).ttvn = ttvn;
    true
}

unsafe fn batadv_send_skb_prepare_unicast(skb: *mut sk_buff, orig_node: *mut batadv_orig_node) -> bool {
    batadv_send_skb_push_fill_unicast(skb, std::mem::size_of::<batadv_unicast_packet>() as i32, orig_node)
}

pub unsafe fn batadv_send_skb_prepare_unicast_4addr(bat_priv: *mut batadv_priv, skb: *mut sk_buff, orig: *mut batadv_orig_node, packet_subtype: i32) -> bool {
    let primary_if = batadv_primary_if_get_selected(bat_priv);
    if primary_if.is_null() { return false; }
    let ok = batadv_send_skb_push_fill_unicast(skb, std::mem::size_of::<batadv_unicast_4addr_packet>() as i32, orig);
    if ok {
        let p = (*skb).data as *mut batadv_unicast_4addr_packet;
        (*p).u.packet_type = BATADV_UNICAST_4ADDR;
        ether_addr_copy((*p).src.as_mut_ptr(), (*(*primary_if).net_dev).dev_addr.as_ptr());
        (*p).subtype = packet_subtype;
        (*p).reserved = 0;
    }
    batadv_hardif_put(primary_if);
    ok
}

pub unsafe fn batadv_send_skb_unicast(bat_priv: *mut batadv_priv, skb: *mut sk_buff, packet_type: i32, packet_subtype: i32, orig_node: *mut batadv_orig_node, vid: u16) -> i32 {
    let mut ret = NET_XMIT_DROP;
    if orig_node.is_null() { kfree_skb(skb); return ret; }
    let ok = match packet_type {
        BATADV_UNICAST => batadv_send_skb_prepare_unicast(skb, orig_node),
        BATADV_UNICAST_4ADDR => batadv_send_skb_prepare_unicast_4addr(bat_priv, skb, orig_node, packet_subtype),
        _ => false,
    };
    if !ok { kfree_skb(skb); return ret; }
    let ethhdr = eth_hdr(skb);
    let packet = (*skb).data as *mut batadv_unicast_packet;
    if batadv_tt_global_client_is_roaming(bat_priv, (*ethhdr).h_dest.as_ptr(), vid) { (*packet).ttvn = (*packet).ttvn.wrapping_sub(1); }
    if batadv_send_skb_to_orig(skb, orig_node, std::ptr::null_mut()) == NET_XMIT_SUCCESS { ret = NET_XMIT_SUCCESS; }
    ret
}

pub unsafe fn batadv_send_skb_via_tt_generic(bat_priv: *mut batadv_priv, skb: *mut sk_buff, packet_type: i32, packet_subtype: i32, dst_hint: *mut u8, vid: u16) -> i32 {
    let ethhdr = (*skb).data as *mut ethhdr;
    let (src, dst) = if !dst_hint.is_null() { (std::ptr::null_mut(), dst_hint) } else { ((*ethhdr).h_source.as_mut_ptr(), (*ethhdr).h_dest.as_mut_ptr()) };
    let orig = batadv_transtable_search(bat_priv, src, dst, vid);
    let ret = batadv_send_skb_unicast(bat_priv, skb, packet_type, packet_subtype, orig, vid);
    batadv_orig_node_put(orig);
    ret
}

pub unsafe fn batadv_send_skb_via_gw(bat_priv: *mut batadv_priv, skb: *mut sk_buff, vid: u16) -> i32 {
    let orig = batadv_gw_get_selected_orig(bat_priv);
    let ret = batadv_send_skb_unicast(bat_priv, skb, BATADV_UNICAST_4ADDR, BATADV_P_DATA, orig, vid);
    batadv_orig_node_put(orig);
    ret
}

pub unsafe fn batadv_forw_packet_free(forw_packet: *mut batadv_forw_packet, dropped: bool) {
    if dropped { kfree_skb((*forw_packet).skb); } else { consume_skb((*forw_packet).skb); }
    batadv_hardif_put((*forw_packet).if_incoming); batadv_hardif_put((*forw_packet).if_outgoing);
    if !(*forw_packet).queue_left.is_null() { atomic_inc((*forw_packet).queue_left); }
    kfree(forw_packet as *mut _);
}

pub unsafe fn batadv_forw_packet_alloc(if_incoming: *mut batadv_hard_iface, if_outgoing: *mut batadv_hard_iface, queue_left: *mut atomic_t, _bat_priv: *mut batadv_priv, skb: *mut sk_buff) -> *mut batadv_forw_packet {
    if !queue_left.is_null() && !batadv_atomic_dec_not_zero(queue_left) { atomic_inc(queue_left); return std::ptr::null_mut(); }
    let p = kmalloc_obj::<batadv_forw_packet>(GFP_ATOMIC);
    if p.is_null() { if !queue_left.is_null() { atomic_inc(queue_left); } return p; }
    if !if_incoming.is_null() { kref_get(&mut (*if_incoming).refcount); }
    if !if_outgoing.is_null() { kref_get(&mut (*if_outgoing).refcount); }
    INIT_HLIST_NODE(&mut (*p).list); INIT_HLIST_NODE(&mut (*p).cleanup_list);
    (*p).skb = skb; (*p).queue_left = queue_left; (*p).if_incoming = if_incoming; (*p).if_outgoing = if_outgoing; (*p).num_packets = 1; p
}

unsafe fn batadv_forw_packet_was_stolen(p: *mut batadv_forw_packet) -> bool { !hlist_unhashed(&(*p).cleanup_list) }

pub unsafe fn batadv_forw_packet_steal(p: *mut batadv_forw_packet, lock: *mut spinlock_t) -> bool {
    spin_lock_bh(lock); if batadv_forw_packet_was_stolen(p) { spin_unlock_bh(lock); return false; }
    hlist_del_init(&mut (*p).list); hlist_add_fake(&mut (*p).cleanup_list); spin_unlock_bh(lock); true
}

unsafe fn batadv_forw_packet_list_steal(forw_list: *mut hlist_head, cleanup_list: *mut hlist_head, hard_iface: *const batadv_hard_iface) {
    hlist_for_each_entry_safe!(forw_packet, safe_tmp_node, forw_list, list, { if !hard_iface.is_null() && (*forw_packet).if_incoming != hard_iface as *mut _ && (*forw_packet).if_outgoing != hard_iface as *mut _ { continue; } hlist_del(&mut (*forw_packet).list); hlist_add_head(&mut (*forw_packet).cleanup_list, cleanup_list); });
}

unsafe fn batadv_forw_packet_list_free(head: *mut hlist_head) {
    hlist_for_each_entry_safe!(forw_packet, safe_tmp_node, head, cleanup_list, { disable_delayed_work_sync(&mut (*forw_packet).delayed_work); hlist_del(&mut (*forw_packet).cleanup_list); batadv_forw_packet_free(forw_packet, true); });
}

unsafe fn batadv_forw_packet_queue(p: *mut batadv_forw_packet, lock: *mut spinlock_t, head: *mut hlist_head, send_time: usize) {
    spin_lock_bh(lock);
    if batadv_forw_packet_was_stolen(p) { spin_unlock_bh(lock); return; }
    hlist_del_init(&mut (*p).list); hlist_add_head(&mut (*p).list, head);
    queue_delayed_work(batadv_event_workqueue, &mut (*p).delayed_work, send_time.wrapping_sub(jiffies));
    spin_unlock_bh(lock);
}

unsafe fn batadv_forw_packet_bcast_queue(bat_priv: *mut batadv_priv, p: *mut batadv_forw_packet, t: usize) { batadv_forw_packet_queue(p, &mut (*bat_priv).forw_bcast_list_lock, &mut (*bat_priv).forw_bcast_list, t); }
pub unsafe fn batadv_forw_packet_ogmv1_queue(bat_priv: *mut batadv_priv, p: *mut batadv_forw_packet, t: usize) { batadv_forw_packet_queue(p, &mut (*bat_priv).forw_bat_list_lock, &mut (*bat_priv).forw_bat_list, t); }

unsafe fn batadv_forw_bcast_packet_to_list(bat_priv: *mut batadv_priv, skb: *mut sk_buff, delay: usize, own: bool, if_in: *mut batadv_hard_iface, if_out: *mut batadv_hard_iface) -> i32 {
    let newskb = skb_clone(skb, GFP_ATOMIC); if newskb.is_null() { return NETDEV_TX_BUSY; }
    let p = batadv_forw_packet_alloc(if_in, if_out, &mut (*bat_priv).bcast_queue_left, bat_priv, newskb);
    if p.is_null() { kfree_skb(newskb); return NETDEV_TX_BUSY; }
    (*p).own = own; INIT_DELAYED_WORK!(&mut (*p).delayed_work, batadv_send_outstanding_bcast_packet);
    batadv_forw_packet_bcast_queue(bat_priv, p, jiffies.wrapping_add(if delay != 0 { delay } else { msecs_to_jiffies(5) })); NETDEV_TX_OK
}

unsafe fn batadv_forw_bcast_packet_if(bat_priv: *mut batadv_priv, skb: *mut sk_buff, delay: usize, own: bool, if_in: *mut batadv_hard_iface, if_out: *mut batadv_hard_iface) -> i32 {
    let mut num = (*if_out).num_bcasts; let mut ret = NETDEV_TX_OK;
    if delay == 0 { let n = skb_clone(skb, GFP_ATOMIC); if n.is_null() { return NETDEV_TX_BUSY; } batadv_send_broadcast_skb(n, if_out); num -= 1; }
    if num >= 1 { BATADV_SKB_CB!(skb).num_bcasts = num; ret = batadv_forw_bcast_packet_to_list(bat_priv, skb, delay, own, if_in, if_out); } ret
}

unsafe fn batadv_send_no_broadcast(bat_priv: *mut batadv_priv, skb: *mut sk_buff, own: bool, if_out: *mut batadv_hard_iface) -> bool {
    let mut neigh = std::ptr::null_mut(); if !own { neigh = batadv_hardif_neigh_get(if_out, (*eth_hdr(skb)).h_source.as_ptr()); }
    let p = (*skb).data as *mut batadv_bcast_packet; let orig = if !neigh.is_null() { (*neigh).orig.as_ptr() } else { std::ptr::null() };
    let ret = batadv_hardif_no_broadcast(if_out, (*p).orig.as_ptr(), orig); batadv_hardif_neigh_put(neigh); if ret == 0 { return false; }
    batadv_dbg!(BATADV_DBG_BATMAN, bat_priv, "BCAST packet from orig %pM on %s suppressed\n", (*p).orig.as_ptr(), (*(*if_out).net_dev).name); true
}

unsafe fn __batadv_forw_bcast_packet(bat_priv: *mut batadv_priv, skb: *mut sk_buff, delay: usize, own: bool) -> i32 {
    let primary = batadv_primary_if_get_selected(bat_priv); if primary.is_null() { return NETDEV_TX_BUSY; }
    rcu_read_lock(); netdev_for_each_lower_private_rcu!( (*bat_priv).mesh_iface, hard_iface, iter, {
        if !kref_get_unless_zero(&mut (*hard_iface).refcount) { continue; }
        if batadv_send_no_broadcast(bat_priv, skb, own, hard_iface) { batadv_hardif_put(hard_iface); continue; }
        let ret = batadv_forw_bcast_packet_if(bat_priv, skb, delay, own, primary, hard_iface); batadv_hardif_put(hard_iface); if ret == NETDEV_TX_BUSY { break; }
    }); rcu_read_unlock(); batadv_hardif_put(primary); NETDEV_TX_OK
}

pub unsafe fn batadv_forw_bcast_packet(bat_priv: *mut batadv_priv, skb: *mut sk_buff, delay: usize, own: bool) -> i32 { __batadv_forw_bcast_packet(bat_priv, skb, delay, own) }
pub unsafe fn batadv_send_bcast_packet(bat_priv: *mut batadv_priv, skb: *mut sk_buff, delay: usize, own: bool) { __batadv_forw_bcast_packet(bat_priv, skb, delay, own); consume_skb(skb); }

unsafe fn batadv_forw_packet_bcasts_left(p: *mut batadv_forw_packet) -> bool { BATADV_SKB_CB!( (*p).skb ).num_bcasts != 0 }
unsafe fn batadv_forw_packet_bcasts_dec(p: *mut batadv_forw_packet) { BATADV_SKB_CB!( (*p).skb ).num_bcasts -= 1; }
pub unsafe fn batadv_forw_packet_is_rebroadcast(p: *mut batadv_forw_packet) -> bool { BATADV_SKB_CB!( (*p).skb ).num_bcasts != (*p).if_outgoing.num_bcasts }

unsafe fn batadv_send_outstanding_bcast_packet(work: *mut work_struct) {
    let delayed = to_delayed_work(work); let p = container_of!(delayed, batadv_forw_packet, delayed_work); let bat_priv = netdev_priv((*(*p).if_incoming).mesh_iface);
    let mut dropped = false; if READ_ONCE!((*bat_priv).mesh_state) == BATADV_MESH_DEACTIVATING || batadv_dat_drop_broadcast_packet(bat_priv, p) { dropped = true; }
    if !dropped { let skb1 = skb_clone((*p).skb, GFP_ATOMIC); if !skb1.is_null() { batadv_send_broadcast_skb(skb1, (*p).if_outgoing); batadv_forw_packet_bcasts_dec(p); if batadv_forw_packet_bcasts_left(p) { batadv_forw_packet_bcast_queue(bat_priv, p, jiffies + msecs_to_jiffies(5)); return; } } }
    if batadv_forw_packet_steal(p, &mut (*bat_priv).forw_bcast_list_lock) { batadv_forw_packet_free(p, dropped); }
}

pub unsafe fn batadv_purge_outstanding_packets(bat_priv: *mut batadv_priv, hard_iface: *const batadv_hard_iface) {
    let mut head = HLIST_HEAD_INIT;
    spin_lock_bh(&mut (*bat_priv).forw_bcast_list_lock); batadv_forw_packet_list_steal(&mut (*bat_priv).forw_bcast_list, &mut head, hard_iface); spin_unlock_bh(&mut (*bat_priv).forw_bcast_list_lock);
    spin_lock_bh(&mut (*bat_priv).forw_bat_list_lock); batadv_forw_packet_list_steal(&mut (*bat_priv).forw_bat_list, &mut head, hard_iface); spin_unlock_bh(&mut (*bat_priv).forw_bat_list_lock);
    batadv_forw_packet_list_free(&mut head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
