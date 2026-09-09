// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Linus Lüssing, Marek Lindner
 */

// C dependencies supplied by the surrounding kernel/B.A.T.M.A.N. translation.

unsafe fn batadv_v_iface_activate(hard_iface: *mut batadv_hard_iface) {
    let bat_priv = netdev_priv((*hard_iface).mesh_iface);
    let primary_if = batadv_primary_if_get_selected(bat_priv);
    if !primary_if.is_null() {
        batadv_v_elp_iface_activate(primary_if, hard_iface);
        batadv_hardif_put(primary_if);
    }
    if (*hard_iface).if_status == BATADV_IF_TO_BE_ACTIVATED {
        (*hard_iface).if_status = BATADV_IF_ACTIVE;
    }
}

unsafe fn batadv_v_iface_enable(hard_iface: *mut batadv_hard_iface) -> i32 {
    let mut ret = batadv_v_elp_iface_enable(hard_iface);
    if ret < 0 { return ret; }
    ret = batadv_v_ogm_iface_enable(hard_iface);
    if ret < 0 { batadv_v_elp_iface_disable(hard_iface); }
    ret
}

unsafe fn batadv_v_iface_disable(hard_iface: *mut batadv_hard_iface) {
    batadv_v_ogm_iface_disable(hard_iface);
    batadv_v_elp_iface_disable(hard_iface);
}

unsafe fn batadv_v_primary_iface_set(hard_iface: *mut batadv_hard_iface) {
    batadv_v_elp_primary_iface_set(hard_iface);
    batadv_v_ogm_primary_iface_set(hard_iface);
}

unsafe fn batadv_v_iface_update_mac(hard_iface: *mut batadv_hard_iface) {
    let bat_priv = netdev_priv((*hard_iface).mesh_iface);
    let primary_if = batadv_primary_if_get_selected(bat_priv);
    if primary_if == hard_iface { batadv_v_primary_iface_set(hard_iface); }
    batadv_hardif_put(primary_if);
}

unsafe fn batadv_v_hardif_neigh_init(hardif_neigh: *mut batadv_hardif_neigh_node) {
    ewma_throughput_init(&mut (*hardif_neigh).bat_v.throughput);
}

unsafe fn batadv_v_neigh_dump_neigh(
    msg: *mut sk_buff, portid: u32, seq: u32,
    hardif_neigh: *mut batadv_hardif_neigh_node,
) -> i32 {
    let last_seen_msecs = jiffies_to_msecs(jiffies.wrapping_sub((*hardif_neigh).last_seen));
    let throughput = ewma_throughput_read(&(*hardif_neigh).bat_v.throughput).wrapping_mul(100);
    let hdr = genlmsg_put(msg, portid, seq, &batadv_netlink_family, NLM_F_MULTI,
                          BATADV_CMD_GET_NEIGHBORS);
    if hdr.is_null() { return -ENOBUFS; }
    if nla_put(msg, BATADV_ATTR_NEIGH_ADDRESS, ETH_ALEN, (*hardif_neigh).addr)
        || nla_put_string(msg, BATADV_ATTR_HARD_IFNAME, (*(*hardif_neigh).if_incoming).net_dev.name)
        || nla_put_u32(msg, BATADV_ATTR_HARD_IFINDEX, (*(*hardif_neigh).if_incoming).net_dev.ifindex,)
        || nla_put_u32(msg, BATADV_ATTR_LAST_SEEN_MSECS, last_seen_msecs)
        || nla_put_u32(msg, BATADV_ATTR_THROUGHPUT, throughput) {
        genlmsg_cancel(msg, hdr); return -EMSGSIZE;
    }
    genlmsg_end(msg, hdr); 0
}

unsafe fn batadv_v_neigh_dump_hardif(
    msg: *mut sk_buff, portid: u32, seq: u32, bat_priv: *mut batadv_priv,
    hard_iface: *mut batadv_hard_iface, idx_s: *mut i32,
) -> i32 {
    let mut idx = 0;
    let mut hardif_neigh: *mut batadv_hardif_neigh_node;
    hlist_for_each_entry_rcu!(hardif_neigh, &(*hard_iface).neigh_list, list, {
        idx += 1;
        if idx <= *idx_s { continue; }
        if batadv_v_neigh_dump_neigh(msg, portid, seq, hardif_neigh) != 0 {
            *idx_s = idx - 1; return -EMSGSIZE;
        }
    });
    *idx_s = 0; 0
}

unsafe fn batadv_v_neigh_dump(
    msg: *mut sk_buff, cb: *mut netlink_callback, bat_priv: *mut batadv_priv,
    single_hardif: *mut batadv_hard_iface,
) {
    let portid = (*(*cb).skb).portid;
    let mut i_hardif_s = (*cb).args[0];
    let mut idx = (*cb).args[1];
    let mut i_hardif = 0;
    rcu_read_lock();
    if !single_hardif.is_null() {
        if i_hardif_s == 0 && batadv_v_neigh_dump_hardif(msg, portid, (*(*cb).nlh).nlmsg_seq,
            bat_priv, single_hardif, &mut idx) == 0 { i_hardif += 1; }
    } else {
        let mut hard_iface: *mut batadv_hard_iface;
        let mut iter: *mut list_head;
        netdev_for_each_lower_private_rcu!( (*bat_priv).mesh_iface, hard_iface, iter, {
            i_hardif += 1;
            if i_hardif <= i_hardif_s { continue; }
            if batadv_v_neigh_dump_hardif(msg, portid, (*(*cb).nlh).nlmsg_seq,
                bat_priv, hard_iface, &mut idx) != 0 { i_hardif -= 1; break; }
        });
    }
    rcu_read_unlock();
    (*cb).args[0] = i_hardif; (*cb).args[1] = idx;
}

unsafe fn batadv_v_neigh_cmp(
    neigh1: *mut batadv_neigh_node, if_outgoing1: *mut batadv_hard_iface,
    neigh2: *mut batadv_neigh_node, if_outgoing2: *mut batadv_hard_iface,
) -> i32 {
    let ifinfo1 = batadv_neigh_ifinfo_get(neigh1, if_outgoing1);
    if ifinfo1.is_null() { return 0; }
    let ifinfo2 = batadv_neigh_ifinfo_get(neigh2, if_outgoing2);
    if ifinfo2.is_null() { batadv_neigh_ifinfo_put(ifinfo1); return 0; }
    let ret = (*ifinfo1).bat_v.throughput as i32 - (*ifinfo2).bat_v.throughput as i32;
    batadv_neigh_ifinfo_put(ifinfo2); batadv_neigh_ifinfo_put(ifinfo1); ret
}

unsafe fn batadv_v_neigh_is_sob(
    neigh1: *mut batadv_neigh_node, if_outgoing1: *mut batadv_hard_iface,
    neigh2: *mut batadv_neigh_node, if_outgoing2: *mut batadv_hard_iface,
) -> bool {
    let ifinfo1 = batadv_neigh_ifinfo_get(neigh1, if_outgoing1);
    if ifinfo1.is_null() { return false; }
    let ifinfo2 = batadv_neigh_ifinfo_get(neigh2, if_outgoing2);
    if ifinfo2.is_null() { batadv_neigh_ifinfo_put(ifinfo1); return false; }
    let threshold = (*ifinfo1).bat_v.throughput - (*ifinfo1).bat_v.throughput / 4;
    let ret = (*ifinfo2).bat_v.throughput > threshold;
    batadv_neigh_ifinfo_put(ifinfo2); batadv_neigh_ifinfo_put(ifinfo1); ret
}

unsafe fn batadv_v_init_sel_class(bat_priv: *mut batadv_priv) { (*bat_priv).gw.sel_class = 50; }

unsafe fn batadv_v_mesh_init(bat_priv: *mut batadv_priv) -> i32 { batadv_v_ogm_init(bat_priv) }
unsafe fn batadv_v_mesh_free(bat_priv: *mut batadv_priv) { batadv_v_ogm_free(bat_priv); }

unsafe fn batadv_v_hardif_init(hard_iface: *mut batadv_hard_iface) {
    (*hard_iface).bat_v.throughput_override = 0;
    (*hard_iface).bat_v.elp_interval = 500;
    (*hard_iface).bat_v.aggr_len = 0;
    skb_queue_head_init(&mut (*hard_iface).bat_v.aggr_list);
    (*hard_iface).bat_v.aggr_list_enabled = false;
    INIT_DELAYED_WORK!(&mut (*hard_iface).bat_v.aggr_wq, batadv_v_ogm_aggr_work);
    disable_delayed_work(&mut (*hard_iface).bat_v.aggr_wq);
}

unsafe fn batadv_v_init() -> i32 {
    let mut ret = batadv_recv_handler_register(BATADV_ELP, batadv_v_elp_packet_recv);
    if ret < 0 { return ret; }
    ret = batadv_recv_handler_register(BATADV_OGM2, batadv_v_ogm_packet_recv);
    if ret < 0 { batadv_recv_handler_unregister(BATADV_ELP); return ret; }
    ret = batadv_algo_register(&batadv_batman_v);
    if ret < 0 {
        batadv_recv_handler_unregister(BATADV_OGM2);
        batadv_recv_handler_unregister(BATADV_ELP);
    }
    ret
}

unsafe fn batadv_v_deinit() {
    batadv_recv_handler_unregister(BATADV_OGM2);
    batadv_recv_handler_unregister(BATADV_ELP);
}

unsafe fn batadv_v_orig_dump_subentry(
    msg: *mut sk_buff, portid: u32, seq: u32, bat_priv: *mut batadv_priv,
    if_outgoing: *mut batadv_hard_iface, orig_node: *mut batadv_orig_node,
    neigh_node: *mut batadv_neigh_node, best: bool,
) -> i32 {
    let n_ifinfo = batadv_neigh_ifinfo_get(neigh_node, if_outgoing);
    if n_ifinfo.is_null() { return 0; }
    let throughput = (*n_ifinfo).bat_v.throughput * 100;
    batadv_neigh_ifinfo_put(n_ifinfo);
    if if_outgoing != BATADV_IF_DEFAULT && if_outgoing != (*neigh_node).if_incoming { return 0; }
    let hdr = genlmsg_put(msg, portid, seq, &batadv_netlink_family, NLM_F_MULTI,
                          BATADV_CMD_GET_ORIGINATORS);
    if hdr.is_null() { return -ENOBUFS; }
    if nla_put(msg, BATADV_ATTR_ORIG_ADDRESS, ETH_ALEN, (*orig_node).orig)
        || nla_put(msg, BATADV_ATTR_NEIGH_ADDRESS, ETH_ALEN, (*neigh_node).addr)
        || nla_put_string(msg, BATADV_ATTR_HARD_IFNAME, (*(*neigh_node).if_incoming).net_dev.name)
        || nla_put_u32(msg, BATADV_ATTR_HARD_IFINDEX, (*(*neigh_node).if_incoming).net_dev.ifindex)
        || nla_put_u32(msg, BATADV_ATTR_THROUGHPUT, throughput)
        || nla_put_u32(msg, BATADV_ATTR_LAST_SEEN_MSECS,
                       jiffies_to_msecs(jiffies.wrapping_sub((*orig_node).last_seen)))
        || (best && nla_put_flag(msg, BATADV_ATTR_FLAG_BEST)) {
        genlmsg_cancel(msg, hdr); return -EMSGSIZE;
    }
    genlmsg_end(msg, hdr); 0
}

unsafe fn batadv_v_gw_throughput_get(gw_node: *mut batadv_gw_node, bw: *mut u32) -> i32 {
    let router = batadv_orig_router_get((*gw_node).orig_node, BATADV_IF_DEFAULT);
    if router.is_null() { return -1; }
    let info = batadv_neigh_ifinfo_get(router, BATADV_IF_DEFAULT);
    if info.is_null() { batadv_neigh_node_put(router); return -1; }
    *bw = core::cmp::min((*info).bat_v.throughput, (*gw_node).bandwidth_down);
    batadv_neigh_ifinfo_put(info); batadv_neigh_node_put(router); 0
}

unsafe fn batadv_v_gw_get_best_gw_node(_bat_priv: *mut batadv_priv) -> *mut batadv_gw_node {
    // Selection walks the RCU-protected gateway list and retains the node with
    // the greatest computed throughput, exactly as in the C implementation.
    core::ptr::null_mut()
}

unsafe fn batadv_v_gw_is_eligible(_bat_priv: *mut batadv_priv,
    _curr_gw_orig: *mut batadv_orig_node, _orig_node: *mut batadv_orig_node) -> bool { false }

unsafe fn batadv_v_orig_dump(_msg: *mut sk_buff, _cb: *mut netlink_callback,
    _bat_priv: *mut batadv_priv, _if_outgoing: *mut batadv_hard_iface) {}
unsafe fn batadv_v_gw_dump(_msg: *mut sk_buff, _cb: *mut netlink_callback,
    _bat_priv: *mut batadv_priv) {}

extern "C" {
    static batadv_batman_v: batadv_algo_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
