// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Forwarding decision
 *	Linux ethernet bridge
 *
 *	Authors:
 *	Lennert Buytenhek		<buytenh@gnu.org>
 */

/* Don't forward packets to originating port or forwarding disabled */
unsafe fn should_deliver(p: *const net_bridge_port, skb: *const sk_buff) -> bool {
    let vg: *mut net_bridge_vlan_group = nbp_vlan_group_rcu(p);
    (test_bit(BR_HAIRPIN_MODE_BIT, &(*p).flags) || (*skb).dev != (*p).dev)
        && (br_mst_is_enabled(p) || (*p).state == BR_STATE_FORWARDING)
        && br_allowed_egress(vg, skb)
        && nbp_switchdev_allowed_egress(p, skb)
        && !br_skb_isolated(p, skb)
}

unsafe fn br_dev_queue_push_xmit(
    net: *mut net,
    sk: *mut sock,
    mut skb: *mut sk_buff,
) -> i32 {
    skb_push(skb, ETH_HLEN);
    if !is_skb_forwardable((*skb).dev, skb) {
        goto_drop!(skb);
        return 0;
    }
    br_drop_fake_rtable(skb);
    if (*skb).ip_summed == CHECKSUM_PARTIAL && eth_type_vlan((*skb).protocol) {
        let mut depth: i32 = 0;
        if !vlan_get_protocol_and_depth(skb, (*skb).protocol, &mut depth) {
            goto_drop!(skb);
            return 0;
        }
        skb_set_network_header(skb, depth);
    }
    br_switchdev_frame_set_offload_fwd_mark(skb);
    dev_queue_xmit(skb);
    0
}

unsafe fn br_forward_finish(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    skb_clear_tstamp(skb);
    NF_HOOK(NFPROTO_BRIDGE, NF_BR_POST_ROUTING, net, sk, skb, core::ptr::null_mut(),
            (*skb).dev, br_dev_queue_push_xmit)
}

unsafe fn __br_forward(to: *const net_bridge_port, mut skb: *mut sk_buff, local_orig: bool) {
    let vg: *mut net_bridge_vlan_group;
    let mut indev: *mut net_device;
    let net: *mut net;
    let br_hook: i32;

    nbp_switchdev_frame_mark_tx_fwd_offload(to, skb);
    vg = nbp_vlan_group_rcu(to);
    skb = br_handle_vlan((*to).br, to, vg, skb);
    if skb.is_null() { return; }
    indev = (*skb).dev;
    (*skb).dev = (*to).dev;
    if !local_orig {
        if skb_warn_if_lro(skb) { kfree_skb(skb); return; }
        br_hook = NF_BR_FORWARD;
        skb_forward_csum(skb);
        net = dev_net(indev);
    } else {
        if unlikely(netpoll_tx_running((*to).br.dev)) {
            skb_push(skb, ETH_HLEN);
            if !is_skb_forwardable((*skb).dev, skb) { kfree_skb(skb); }
            else { br_netpoll_send_skb(to, skb); }
            return;
        }
        br_hook = NF_BR_LOCAL_OUT;
        net = dev_net((*skb).dev);
        indev = core::ptr::null_mut();
    }
    NF_HOOK(NFPROTO_BRIDGE, br_hook, net, core::ptr::null_mut(), skb, indev,
            (*skb).dev, br_forward_finish);
}

unsafe fn deliver_clone(prev: *const net_bridge_port, skb: *mut sk_buff, local_orig: bool) -> i32 {
    let dev = BR_INPUT_SKB_CB(skb).brdev;
    let skb = skb_clone(skb, GFP_ATOMIC);
    if skb.is_null() { DEV_STATS_INC(dev, tx_dropped); return -ENOMEM; }
    __br_forward(prev, skb, local_orig);
    0
}

/**
 * br_forward - forward a packet to a specific port
 * @to: destination port
 * @skb: packet being forwarded
 * @local_rcv: packet will be received locally after forwarding
 * @local_orig: packet is locally originated
 *
 * Should be called with rcu_read_lock.
 */
pub unsafe fn br_forward(to: *const net_bridge_port, skb: *mut sk_buff,
                         local_rcv: bool, local_orig: bool) {
    if unlikely(to.is_null()) { if !local_rcv { kfree_skb(skb); } return; }
    if !rcu_access_pointer((*to).backup_port).is_null()
        && (!netif_carrier_ok((*to).dev) || !netif_running((*to).dev)) {
        let backup_port = rcu_dereference((*to).backup_port);
        if unlikely(backup_port.is_null()) { if !local_rcv { kfree_skb(skb); } return; }
        BR_INPUT_SKB_CB(skb).backup_nhid = READ_ONCE((*to).backup_nhid);
        to = backup_port;
    }
    if should_deliver(to, skb) {
        if local_rcv { deliver_clone(to, skb, local_orig); }
        else { __br_forward(to, skb, local_orig); }
        return;
    }
    if !local_rcv { kfree_skb(skb); }
}

unsafe fn maybe_deliver(prev: *mut net_bridge_port, p: *mut net_bridge_port,
                        skb: *mut sk_buff, local_orig: bool) -> *mut net_bridge_port {
    let igmp_type = br_multicast_igmp_type(skb);
    if !should_deliver(p, skb) { return prev; }
    nbp_switchdev_frame_mark_tx_fwd_to_hwdom(p, skb);
    if !prev.is_null() {
        let err = deliver_clone(prev, skb, local_orig);
        if err != 0 { return ERR_PTR(err); }
    }
    br_multicast_count((*p).br, p, skb, igmp_type, BR_MCAST_DIR_TX);
    p
}

/* called under rcu_read_lock */
pub unsafe fn br_flood(br: *mut net_bridge, skb: *mut sk_buff, pkt_type: br_pkt_type,
                       local_rcv: bool, local_orig: bool, vid: u16) {
    let mut reason = SKB_DROP_REASON_NO_TX_TARGET;
    let mut prev: *mut net_bridge_port = core::ptr::null_mut();
    let mut p: *mut net_bridge_port;
    br_tc_skb_miss_set(skb, pkt_type != BR_PKT_BROADCAST);
    list_for_each_entry_rcu!(p, &(*br).port_list, list) {
        match pkt_type {
            BR_PKT_UNICAST if !test_bit(BR_FLOOD_BIT, &(*p).flags) => continue,
            BR_PKT_MULTICAST if !test_bit(BR_MCAST_FLOOD_BIT, &(*p).flags) && (*skb).dev != (*br).dev => continue,
            BR_PKT_BROADCAST if !test_bit(BR_BCAST_FLOOD_BIT, &(*p).flags) && (*skb).dev != (*br).dev => continue,
            _ => {}
        }
        if test_bit(BR_PROXYARP_BIT, &(*p).flags) { continue; }
        if BR_INPUT_SKB_CB(skb).proxyarp_replied {
            if test_bit(BR_PROXYARP_WIFI_BIT, &(*p).flags) { continue; }
            if br_is_neigh_suppress_enabled(p, vid)
                && (!BR_INPUT_SKB_CB(skb).grat_arp || !br_is_neigh_forward_grat_enabled(p, vid)) { continue; }
        }
        prev = maybe_deliver(prev, p, skb, local_orig);
        if IS_ERR(prev) { reason = if PTR_ERR(prev) == -ENOMEM { SKB_DROP_REASON_NOMEM } else { SKB_DROP_REASON_NOT_SPECIFIED }; break; }
    }
    if !prev.is_null() { if local_rcv { deliver_clone(prev, skb, local_orig); } else { __br_forward(prev, skb, local_orig); } }
    else if !local_rcv { kfree_skb_reason(skb, reason); }
}

// CONFIG_BRIDGE_IGMP_SNOOPING retains the conditional implementation below.

#[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
unsafe fn maybe_deliver_addr(p: *mut net_bridge_port, mut skb: *mut sk_buff,
                             addr: *const u8, local_orig: bool) {
    let dev = BR_INPUT_SKB_CB(skb).brdev;
    let src = eth_hdr(skb).h_source.as_ptr();
    if !should_deliver(p, skb) { return; }
    /* Even with hairpin, no soliloquies - prevent breaking IPv6 DAD */
    if (*skb).dev == (*p).dev && ether_addr_equal(src, addr) { return; }
    __skb_push(skb, ETH_HLEN);
    let nskb = pskb_copy(skb, GFP_ATOMIC);
    __skb_pull(skb, ETH_HLEN);
    if nskb.is_null() { DEV_STATS_INC(dev, tx_dropped); return; }
    skb = nskb;
    __skb_pull(skb, ETH_HLEN);
    if !is_broadcast_ether_addr(addr) {
        memcpy(eth_hdr(skb).h_dest.as_mut_ptr(), addr, ETH_ALEN);
    }
    __br_forward(p, skb, local_orig);
}

/* called with rcu_read_lock */
#[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
pub unsafe fn br_multicast_flood(mdst: *mut net_bridge_mdb_entry,
                                 skb: *mut sk_buff, brmctx: *mut net_bridge_mcast,
                                 local_rcv: bool, local_orig: bool) {
    let mut reason = SKB_DROP_REASON_NO_TX_TARGET;
    let mut prev: *mut net_bridge_port = core::ptr::null_mut();
    let mut p: *mut net_bridge_port_group;
    let mut allow_mode_include = true;
    let mut rp: *mut hlist_node = br_multicast_get_first_rport_node(brmctx, skb);
    if !mdst.is_null() {
        p = rcu_dereference((*mdst).ports);
        if br_multicast_should_handle_mode(brmctx, (*mdst).addr.proto)
            && br_multicast_is_star_g(&(*mdst).addr) { allow_mode_include = false; }
    } else { p = core::ptr::null_mut(); br_tc_skb_miss_set(skb, true); }
    while !p.is_null() || !rp.is_null() {
        let lport = if !p.is_null() { (*p).key.port } else { core::ptr::null_mut() };
        let rport = br_multicast_rport_from_node_skb(rp, skb);
        let port;
        if (lport as usize) > (rport as usize) {
            port = lport;
            if test_bit(BR_MULTICAST_TO_UNICAST_BIT, &(*port).flags) {
                maybe_deliver_addr(lport, skb, (*p).eth_addr.as_ptr(), local_orig);
            } else if (!allow_mode_include && (*p).filter_mode == MCAST_INCLUDE)
                || ((*p).flags & MDB_PG_FLAGS_BLOCKED) != 0 { }
            else { prev = maybe_deliver(prev, port, skb, local_orig); }
        } else { port = rport; prev = maybe_deliver(prev, port, skb, local_orig); }
        if IS_ERR(prev) { reason = if PTR_ERR(prev) == -ENOMEM { SKB_DROP_REASON_NOMEM } else { SKB_DROP_REASON_NOT_SPECIFIED }; break; }
        if (lport as usize) >= (port as usize) { p = rcu_dereference((*p).next); }
        if (rport as usize) >= (port as usize) { rp = rcu_dereference(hlist_next_rcu(rp)); }
    }
    if !prev.is_null() { if local_rcv { deliver_clone(prev, skb, local_orig); } else { __br_forward(prev, skb, local_orig); } }
    else if !local_rcv { kfree_skb_reason(skb, reason); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
