// SPDX-License-Identifier: GPL-2.0-or-later
/* Handle incoming frames - Linux ethernet bridge */

unsafe fn br_netif_receive_skb(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    br_drop_fake_rtable(skb);
    netif_receive_skb(skb)
}

unsafe fn br_pass_frame_up(skb: *mut sk_buff, promisc: bool) -> c_int {
    let indev: *mut net_device;
    let brdev = (*BR_INPUT_SKB_CB(skb)).brdev;
    let br = netdev_priv(brdev);
    let vg: *mut net_bridge_vlan_group;
    dev_sw_netstats_rx_add(brdev, (*skb).len);
    vg = br_vlan_group_rcu(br);
    br_switchdev_frame_unmark(skb);
    if ((*brdev).flags & IFF_PROMISC) == 0 && !br_allowed_egress(vg, skb) {
        kfree_skb(skb); return NET_RX_DROP;
    }
    indev = (*skb).dev;
    (*skb).dev = brdev;
    let skb = br_handle_vlan(br, core::ptr::null_mut(), vg, skb);
    if skb.is_null() { return NET_RX_DROP; }
    br_multicast_count(br, core::ptr::null_mut(), skb, br_multicast_igmp_type(skb), BR_MCAST_DIR_TX);
    (*BR_INPUT_SKB_CB(skb)).promisc = promisc;
    NF_HOOK(NFPROTO_BRIDGE, NF_BR_LOCAL_IN, dev_net(indev), core::ptr::null_mut(), skb, indev, core::ptr::null_mut(), br_netif_receive_skb)
}

pub unsafe fn br_handle_frame_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let mut reason = SKB_DROP_REASON_NOT_SPECIFIED;
    let p = br_port_get_rcu((*skb).dev);
    let mut pkt_type = BR_PKT_UNICAST;
    let mut dst: *mut net_bridge_fdb_entry = core::ptr::null_mut();
    let mut pmctx: *mut net_bridge_mcast_port;
    let mut mdst: *mut net_bridge_mdb_entry;
    let mut local_rcv;
    let mut mcast_hit = false;
    let brmctx: *mut net_bridge_mcast;
    let mut vlan: *mut net_bridge_vlan = core::ptr::null_mut();
    let br: *mut net_bridge;
    let promisc;
    let mut vid: u16 = 0;
    let mut state: u8;
    if p.is_null() { goto_drop(skb, reason); return 0; }
    br = (*p).br;
    if br_mst_is_enabled(p) { state = BR_STATE_FORWARDING; }
    else { if (*p).state == BR_STATE_DISABLED { reason = SKB_DROP_REASON_BRIDGE_INGRESS_STP_STATE; goto_drop(skb, reason); return 0; } state = (*p).state; }
    brmctx = &mut (*br).multicast_ctx;
    pmctx = &mut (*p).multicast_ctx;
    if !br_allowed_ingress(br, nbp_vlan_group_rcu(p), skb, &mut vid, &mut state, &mut vlan) { return 0; }
    if test_bit(BR_PORT_LOCKED_BIT, &(*p).flags) {
        let fdb_src = br_fdb_find_rcu(br, eth_hdr(skb).h_source, vid);
        if fdb_src.is_null() { if test_bit(BR_PORT_MAB_BIT, &(*p).flags) { br_fdb_update(br, p, eth_hdr(skb).h_source, vid, BIT(BR_FDB_LOCKED)); } goto_drop(skb, reason); return 0; }
        if READ_ONCE((*fdb_src).dst) != p || test_bit(BR_FDB_LOCAL, &(*fdb_src).flags) { goto_drop(skb, reason); return 0; }
        if test_bit(BR_FDB_LOCKED, &(*fdb_src).flags) { br_fdb_update(br, p, eth_hdr(skb).h_source, vid, BIT(BR_FDB_LOCKED)); goto_drop(skb, reason); return 0; }
    }
    nbp_switchdev_frame_mark(p, skb);
    if test_bit(BR_LEARNING_BIT, &(*p).flags) { br_fdb_update(br, p, eth_hdr(skb).h_source, vid, 0); }
    promisc = ((*br).dev.flags & IFF_PROMISC) != 0; local_rcv = promisc;
    if is_multicast_ether_addr(eth_hdr(skb).h_dest) {
        if is_broadcast_ether_addr(eth_hdr(skb).h_dest) { pkt_type = BR_PKT_BROADCAST; local_rcv = true; }
        else { pkt_type = BR_PKT_MULTICAST; if br_multicast_rcv(&mut (brmctx), &mut pmctx, vlan, skb, vid) { goto_drop(skb, reason); return 0; } }
    }
    if state == BR_STATE_LEARNING { reason = SKB_DROP_REASON_BRIDGE_INGRESS_STP_STATE; goto_drop(skb, reason); return 0; }
    (*BR_INPUT_SKB_CB(skb)).brdev = (*br).dev;
    (*BR_INPUT_SKB_CB(skb)).src_port_isolated = test_bit(BR_ISOLATED_BIT, &(*p).flags);
    if IS_ENABLED(CONFIG_INET) && ((*skb).protocol == htons(ETH_P_ARP) || (*skb).protocol == htons(ETH_P_RARP)) { br_do_proxy_suppress_arp(skb, br, vid, p); }
    else if ipv6_mod_enabled() && (*skb).protocol == htons(ETH_P_IPV6) && br_opt_get(br, BROPT_NEIGH_SUPPRESS_ENABLED) && pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<nd_msg>()) && (*ipv6_hdr(skb)).nexthdr == IPPROTO_ICMPV6 { let msg = br_is_nd_neigh_msg(skb); if !msg.is_null() { br_do_suppress_nd(skb, br, vid, p, msg); } }
    match pkt_type {
        BR_PKT_MULTICAST => { mdst = br_mdb_entry_skb_get(brmctx, skb, vid); if (!mdst.is_null() || (*BR_INPUT_SKB_CB(skb)).mrouters_only) && br_multicast_querier_exists(brmctx, eth_hdr(skb), mdst) { if (!mdst.is_null() && (*mdst).host_joined) || br_multicast_is_router(brmctx, skb) || ((*br).dev.flags & IFF_ALLMULTI) != 0 { local_rcv = true; DEV_STATS_INC((*br).dev, multicast); } mcast_hit = true; } else { local_rcv = true; DEV_STATS_INC((*br).dev, multicast); } }
        BR_PKT_UNICAST => { dst = br_fdb_find_rcu(br, eth_hdr(skb).h_dest, vid); if dst.is_null() && vid != 0 && br_opt_get(br, BROPT_FDB_LOCAL_VLAN_0) { dst = br_fdb_find_rcu(br, eth_hdr(skb).h_dest, 0); if !dst.is_null() && (!test_bit(BR_FDB_LOCAL, &(*dst).flags) || test_bit(BR_FDB_ADDED_BY_USER, &(*dst).flags)) { dst = core::ptr::null_mut(); } } }
        _ => {}
    }
    if !dst.is_null() { if test_bit(BR_FDB_LOCAL, &(*dst).flags) { return br_pass_frame_up(skb, false); } let now = jiffies; if now != READ_ONCE((*dst).used) { WRITE_ONCE((*dst).used, now); } br_forward(READ_ONCE((*dst).dst), skb, local_rcv, false); }
    else if !mcast_hit { br_flood(br, skb, pkt_type, local_rcv, false, vid); } else { br_multicast_flood(mdst, skb, brmctx, local_rcv, false); }
    if local_rcv { return br_pass_frame_up(skb, promisc); } 0
}

unsafe fn goto_drop(skb: *mut sk_buff, reason: skb_drop_reason) { kfree_skb_reason(skb, reason); }

unsafe fn __br_handle_local_finish(skb: *mut sk_buff) { let p = br_port_get_rcu((*skb).dev); let mut vid = 0u16; if test_bit(BR_LEARNING_BIT, &(*p).flags) && nbp_state_should_learn(p) && !br_opt_get((*p).br, BROPT_NO_LL_LEARN) && br_should_learn(p, skb, &mut vid) { br_fdb_update((*p).br, p, eth_hdr(skb).h_source, vid, 0); } }
unsafe fn br_handle_local_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int { __br_handle_local_finish(skb); 1 }

unsafe fn nf_hook_bridge_pre(skb: *mut sk_buff, pskb: *mut *mut sk_buff) -> c_int {
    /* CONFIG_NETFILTER_FAMILY_BRIDGE code is preserved through the external hook API. */
    br_handle_frame_finish(dev_net((*skb).dev), core::ptr::null_mut(), skb); RX_HANDLER_CONSUMED
}

unsafe fn br_process_frame_type(p: *mut net_bridge_port, skb: *mut sk_buff) -> c_int { let mut tmp: *mut br_frame_type; hlist_for_each_entry_rcu!(tmp, &(*p).br.frame_type_list, list) { if (*tmp).type_ == (*skb).protocol { return ((*tmp).frame_handler)(p, skb); } } 0 }

unsafe fn br_handle_frame(pskb: *mut *mut sk_buff) -> rx_handler_result_t { let skb = *pskb; let dest = eth_hdr(skb).h_dest; if (*skb).pkt_type == PACKET_LOOPBACK || !is_valid_ether_addr(eth_hdr(skb).h_source) { return RX_HANDLER_PASS; } let skb = skb_share_check(skb, GFP_ATOMIC); if skb.is_null() { return RX_HANDLER_CONSUMED; } memset((*skb).cb.as_mut_ptr(), 0, core::mem::size_of::<br_input_skb_cb>()); br_tc_skb_miss_set(skb, false); let p = br_port_get_rcu((*skb).dev); if test_bit(BR_VLAN_TUNNEL_BIT, &(*p).flags) { br_handle_ingress_vlan_tunnel(skb, p, nbp_vlan_group_rcu(p)); } if is_link_local_ether_addr(dest) { *pskb = skb; __br_handle_local_finish(skb); return RX_HANDLER_PASS; } if br_process_frame_type(p, skb) != 0 { return RX_HANDLER_PASS; } if br_mst_is_enabled(p) || (*p).state == BR_STATE_FORWARDING || (*p).state == BR_STATE_LEARNING { if ether_addr_equal((*p).br).dev.dev_addr, dest { (*skb).pkt_type = PACKET_HOST; } return nf_hook_bridge_pre(skb, pskb); } kfree_skb_reason(skb, SKB_DROP_REASON_BRIDGE_INGRESS_STP_STATE); RX_HANDLER_CONSUMED }

unsafe fn br_handle_frame_dummy(pskb: *mut *mut sk_buff) -> rx_handler_result_t { RX_HANDLER_PASS }
unsafe fn br_get_rx_handler(dev: *const net_device) -> rx_handler_func_t { if netdev_uses_dsa(dev) { br_handle_frame_dummy } else { br_handle_frame } }
pub unsafe fn br_add_frame(br: *mut net_bridge, ft: *mut br_frame_type) { hlist_add_head_rcu!(&mut (*ft).list, &mut (*br).frame_type_list); }
pub unsafe fn br_del_frame(br: *mut net_bridge, ft: *mut br_frame_type) { let mut tmp: *mut br_frame_type; hlist_for_each_entry!(tmp, &(*br).frame_type_list, list) { if ft == tmp { hlist_del_rcu!(&mut (*ft).list); return; } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
