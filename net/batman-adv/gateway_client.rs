// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner
 */

// Translated from gateway_client.c. Kernel/project declarations are supplied
// by the surrounding translation unit.

const BATADV_DHCP_HTYPE_OFFSET: usize = 1;
const BATADV_DHCP_HLEN_OFFSET: usize = 2;
const BATADV_DHCP_HTYPE_ETHERNET: u8 = 0x01;
const BATADV_DHCP_CHADDR_OFFSET: usize = 28;

pub unsafe extern "C" fn batadv_gw_node_release(ref_: *mut kref) {
    let gw_node = container_of!(ref_, batadv_gw_node, refcount);
    batadv_orig_node_put((*gw_node).orig_node);
    kfree_rcu!(gw_node, rcu);
}

pub unsafe extern "C" fn batadv_gw_get_selected_gw_node(
    bat_priv: *mut batadv_priv,
) -> *mut batadv_gw_node {
    let mut gw_node: *mut batadv_gw_node;
    rcu_read_lock();
    gw_node = rcu_dereference!((*bat_priv).gw.curr_gw);
    if gw_node.is_null() {
        rcu_read_unlock();
        return gw_node;
    }
    if !kref_get_unless_zero(&mut (*gw_node).refcount) {
        gw_node = core::ptr::null_mut();
    }
    rcu_read_unlock();
    gw_node
}

pub unsafe extern "C" fn batadv_gw_get_selected_orig(
    bat_priv: *mut batadv_priv,
) -> *mut batadv_orig_node {
    let mut orig_node: *mut batadv_orig_node = core::ptr::null_mut();
    let gw_node = batadv_gw_get_selected_gw_node(bat_priv);
    if gw_node.is_null() {
        return orig_node;
    }
    rcu_read_lock();
    orig_node = (*gw_node).orig_node;
    if !orig_node.is_null() && !kref_get_unless_zero(&mut (*orig_node).refcount) {
        orig_node = core::ptr::null_mut();
    }
    rcu_read_unlock();
    batadv_gw_node_put(gw_node);
    orig_node
}

unsafe fn batadv_gw_select(bat_priv: *mut batadv_priv, new_gw_node: *mut batadv_gw_node) {
    spin_lock_bh(&mut (*bat_priv).gw.list_lock);
    if !new_gw_node.is_null() {
        kref_get(&mut (*new_gw_node).refcount);
    }
    let curr_gw_node = rcu_replace_pointer!((*bat_priv).gw.curr_gw, new_gw_node, true);
    batadv_gw_node_put(curr_gw_node);
    spin_unlock_bh(&mut (*bat_priv).gw.list_lock);
}

pub unsafe extern "C" fn batadv_gw_reselect(bat_priv: *mut batadv_priv) {
    atomic_set(&mut (*bat_priv).gw.reselect, 1);
}

pub unsafe extern "C" fn batadv_gw_check_client_stop(bat_priv: *mut batadv_priv) {
    if read_once!((*bat_priv).gw.mode) != BATADV_GW_MODE_CLIENT { return; }
    let curr_gw = batadv_gw_get_selected_gw_node(bat_priv);
    if curr_gw.is_null() { return; }
    batadv_gw_select(bat_priv, core::ptr::null_mut());
    batadv_throw_uevent(bat_priv, BATADV_UEV_GW, BATADV_UEV_DEL, core::ptr::null());
    batadv_gw_node_put(curr_gw);
}

pub unsafe extern "C" fn batadv_gw_election(bat_priv: *mut batadv_priv) {
    let mut router_ifinfo: *mut batadv_neigh_ifinfo = core::ptr::null_mut();
    let mut router: *mut batadv_neigh_node = core::ptr::null_mut();
    let mut curr_gw: *mut batadv_gw_node = core::ptr::null_mut();
    let mut next_gw: *mut batadv_gw_node = core::ptr::null_mut();
    let mut gw_addr = [0i8; 18];
    if read_once!((*bat_priv).gw.mode) != BATADV_GW_MODE_CLIENT { return; }
    if (*bat_priv).algo_ops.is_null() || (*(*bat_priv).algo_ops).gw.get_best_gw_node.is_none() { return; }
    curr_gw = batadv_gw_get_selected_gw_node(bat_priv);
    if atomic_xchg(&mut (*bat_priv).gw.reselect, 0) == 0 && !curr_gw.is_null() {
        batadv_gw_node_put(curr_gw); return;
    }
    next_gw = ((*(*bat_priv).algo_ops).gw.get_best_gw_node.unwrap())(bat_priv);
    if curr_gw == next_gw { batadv_gw_node_put(curr_gw); batadv_gw_node_put(next_gw); return; }
    if !next_gw.is_null() {
        sprintf!(gw_addr.as_mut_ptr(), "%pM", (*(*next_gw).orig_node).orig);
        router = batadv_orig_router_get((*next_gw).orig_node, BATADV_IF_DEFAULT);
        if router.is_null() { batadv_gw_reselect(bat_priv); goto_out!(curr_gw, next_gw, router, router_ifinfo); }
        router_ifinfo = batadv_neigh_ifinfo_get(router, BATADV_IF_DEFAULT);
        if router_ifinfo.is_null() { batadv_gw_reselect(bat_priv); goto_out!(curr_gw, next_gw, router, router_ifinfo); }
    }
    if !curr_gw.is_null() && next_gw.is_null() {
        batadv_dbg!(BATADV_DBG_BATMAN, bat_priv, "Removing selected gateway - no gateway in range\n");
        batadv_throw_uevent(bat_priv, BATADV_UEV_GW, BATADV_UEV_DEL, core::ptr::null());
    } else if curr_gw.is_null() && !next_gw.is_null() {
        batadv_dbg!(BATADV_DBG_BATMAN, bat_priv, "Adding route to gateway %pM (bandwidth: %u.%u/%u.%u MBit, tq: %i)\n", (*(*next_gw).orig_node).orig, (*next_gw).bandwidth_down / 10, (*next_gw).bandwidth_down % 10, (*next_gw).bandwidth_up / 10, (*next_gw).bandwidth_up % 10, (*router_ifinfo).bat_iv.tq_avg);
        batadv_throw_uevent(bat_priv, BATADV_UEV_GW, BATADV_UEV_ADD, gw_addr.as_ptr());
    } else {
        batadv_dbg!(BATADV_DBG_BATMAN, bat_priv, "Changing route to gateway %pM (bandwidth: %u.%u/%u.%u MBit, tq: %i)\n", (*(*next_gw).orig_node).orig, (*next_gw).bandwidth_down / 10, (*next_gw).bandwidth_down % 10, (*next_gw).bandwidth_up / 10, (*next_gw).bandwidth_up % 10, (*router_ifinfo).bat_iv.tq_avg);
        batadv_throw_uevent(bat_priv, BATADV_UEV_GW, BATADV_UEV_CHANGE, gw_addr.as_ptr());
    }
    batadv_gw_select(bat_priv, next_gw);
    batadv_gw_node_put(curr_gw); batadv_gw_node_put(next_gw); batadv_neigh_node_put(router); batadv_neigh_ifinfo_put(router_ifinfo);
}

pub unsafe extern "C" fn batadv_gw_check_election(bat_priv: *mut batadv_priv, orig_node: *mut batadv_orig_node) {
    if (*bat_priv).algo_ops.is_null() || (*(*bat_priv).algo_ops).gw.is_eligible.is_none() { return; }
    let curr_gw_orig = batadv_gw_get_selected_orig(bat_priv);
    if !curr_gw_orig.is_null() && curr_gw_orig == orig_node { batadv_orig_node_put(curr_gw_orig); return; }
    if curr_gw_orig.is_null() || ((*(*bat_priv).algo_ops).gw.is_eligible.unwrap())(bat_priv, curr_gw_orig, orig_node) { batadv_gw_reselect(bat_priv); }
    batadv_orig_node_put(curr_gw_orig);
}

unsafe fn batadv_gw_node_add(bat_priv: *mut batadv_priv, orig_node: *mut batadv_orig_node, gateway: *mut batadv_tvlv_gateway_data) {
    lockdep_assert_held!(&(*bat_priv).gw.list_lock);
    if (*gateway).bandwidth_down == 0 { return; }
    let gw_node = kzalloc_obj!(batadv_gw_node, GFP_ATOMIC);
    if gw_node.is_null() { return; }
    kref_init(&mut (*gw_node).refcount); INIT_HLIST_NODE!(&mut (*gw_node).list);
    kref_get(&mut (*orig_node).refcount); (*gw_node).orig_node = orig_node;
    (*gw_node).bandwidth_down = ntohl((*gateway).bandwidth_down); (*gw_node).bandwidth_up = ntohl((*gateway).bandwidth_up);
    kref_get(&mut (*gw_node).refcount); hlist_add_head_rcu!(&mut (*gw_node).list, &mut (*bat_priv).gw.gateway_list); (*bat_priv).gw.generation += 1;
    batadv_dbg!(BATADV_DBG_BATMAN, bat_priv, "Found new gateway %pM -> gw bandwidth: %u.%u/%u.%u MBit\n", (*orig_node).orig, ntohl((*gateway).bandwidth_down)/10, ntohl((*gateway).bandwidth_down)%10, ntohl((*gateway).bandwidth_up)/10, ntohl((*gateway).bandwidth_up)%10);
    batadv_gw_node_put(gw_node);
}

pub unsafe extern "C" fn batadv_gw_node_get(bat_priv: *mut batadv_priv, orig_node: *mut batadv_orig_node) -> *mut batadv_gw_node {
    rcu_read_lock();
    hlist_for_each_entry_rcu!(gw_node_tmp, &(*bat_priv).gw.gateway_list, list, {
        if (*gw_node_tmp).orig_node == orig_node && kref_get_unless_zero(&mut (*gw_node_tmp).refcount) { rcu_read_unlock(); return gw_node_tmp; }
    });
    rcu_read_unlock(); core::ptr::null_mut()
}

pub unsafe extern "C" fn batadv_gw_node_update(bat_priv: *mut batadv_priv, orig_node: *mut batadv_orig_node, gateway: *mut batadv_tvlv_gateway_data) {
    let mut curr_gw: *mut batadv_gw_node = core::ptr::null_mut();
    spin_lock_bh(&mut (*bat_priv).gw.list_lock); let gw_node = batadv_gw_node_get(bat_priv, orig_node);
    if gw_node.is_null() { batadv_gw_node_add(bat_priv, orig_node, gateway); spin_unlock_bh(&mut (*bat_priv).gw.list_lock); return; }
    spin_unlock_bh(&mut (*bat_priv).gw.list_lock);
    if (*gw_node).bandwidth_down == ntohl((*gateway).bandwidth_down) && (*gw_node).bandwidth_up == ntohl((*gateway).bandwidth_up) { batadv_gw_node_put(gw_node); return; }
    (*gw_node).bandwidth_down = ntohl((*gateway).bandwidth_down); (*gw_node).bandwidth_up = ntohl((*gateway).bandwidth_up);
    if ntohl((*gateway).bandwidth_down) == 0 {
        spin_lock_bh(&mut (*bat_priv).gw.list_lock);
        if !hlist_unhashed!(&(*gw_node).list) { hlist_del_init_rcu!(&mut (*gw_node).list); batadv_gw_node_put(gw_node); (*bat_priv).gw.generation += 1; }
        spin_unlock_bh(&mut (*bat_priv).gw.list_lock);
        curr_gw = batadv_gw_get_selected_gw_node(bat_priv); if curr_gw == gw_node { batadv_gw_reselect(bat_priv); }
    }
    batadv_gw_node_put(curr_gw); batadv_gw_node_put(gw_node);
}

pub unsafe extern "C" fn batadv_gw_node_delete(bat_priv: *mut batadv_priv, orig_node: *mut batadv_orig_node) {
    let mut gateway: batadv_tvlv_gateway_data = core::mem::zeroed();
    batadv_gw_node_update(bat_priv, orig_node, &mut gateway);
}

pub unsafe extern "C" fn batadv_gw_node_free(bat_priv: *mut batadv_priv) {
    spin_lock_bh(&mut (*bat_priv).gw.list_lock);
    let curr_gw = rcu_replace_pointer!((*bat_priv).gw.curr_gw, core::ptr::null_mut(), true); batadv_gw_node_put(curr_gw);
    hlist_for_each_entry_safe!(gw_node, node_tmp, &mut (*bat_priv).gw.gateway_list, list, {
        hlist_del_init_rcu!(&mut (*gw_node).list); batadv_gw_node_put(gw_node); (*bat_priv).gw.generation += 1;
    });
    spin_unlock_bh(&mut (*bat_priv).gw.list_lock);
}

pub unsafe extern "C" fn batadv_gw_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let mesh_iface = batadv_netlink_get_meshif(cb); if IS_ERR!(mesh_iface) { return PTR_ERR!(mesh_iface); }
    let bat_priv = netdev_priv(mesh_iface); let primary_if = batadv_primary_if_get_selected(bat_priv);
    let ret: c_int;
    if primary_if.is_null() || (*primary_if).if_status != BATADV_IF_ACTIVE { ret = -ENOENT; }
    else if (*bat_priv).algo_ops.is_null() || (*(*bat_priv).algo_ops).gw.dump.is_none() { ret = -EOPNOTSUPP; }
    else { ((*(*bat_priv).algo_ops).gw.dump.unwrap())(msg, cb, bat_priv); ret = (*msg).len; }
    batadv_hardif_put(primary_if); dev_put(mesh_iface); ret
}

pub unsafe extern "C" fn batadv_gw_dhcp_recipient_get(skb: *mut sk_buff, header_len: *mut c_uint, chaddr: *mut u8) -> batadv_dhcp_recipient {
    let mut ret = BATADV_DHCP_NO; let mut proto: __be16; let mut chaddr_offset: usize; let p: *mut u8;
    if !pskb_may_pull(skb, *header_len as usize + ETH_HLEN) { return ret; }
    let ethhdr = eth_hdr(skb); proto = (*ethhdr).h_proto; *header_len += ETH_HLEN as c_uint;
    if proto == htons(ETH_P_8021Q) { if !pskb_may_pull(skb, *header_len as usize + VLAN_HLEN) { return ret; } let vhdr = vlan_eth_hdr(skb); proto = (*vhdr).h_vlan_encapsulated_proto; *header_len += VLAN_HLEN as c_uint; }
    match proto {
        x if x == htons(ETH_P_IP) => { if !pskb_may_pull(skb, *header_len as usize + core::mem::size_of::<iphdr>()) { return ret; } let iphdr = ( (*skb).data.add(*header_len as usize) as *mut iphdr); *header_len += ((*iphdr).ihl as c_uint) * 4; if (*iphdr).protocol != IPPROTO_UDP { return ret; } },
        x if x == htons(ETH_P_IPV6) => { if !pskb_may_pull(skb, *header_len as usize + core::mem::size_of::<ipv6hdr>()) { return ret; } let ipv6hdr = (*skb).data.add(*header_len as usize) as *mut ipv6hdr; *header_len += core::mem::size_of::<ipv6hdr>() as c_uint; if (*ipv6hdr).nexthdr != IPPROTO_UDP { return ret; } },
        _ => return ret,
    }
    if !pskb_may_pull(skb, *header_len as usize + core::mem::size_of::<udphdr>()) { return ret; }
    let udphdr = (*skb).data.add(*header_len as usize) as *mut udphdr; *header_len += core::mem::size_of::<udphdr>() as c_uint;
    if proto == htons(ETH_P_IP) { if (*udphdr).dest == htons(67) { ret = BATADV_DHCP_TO_SERVER; } else if (*udphdr).source == htons(67) { ret = BATADV_DHCP_TO_CLIENT; } }
    else if (*udphdr).dest == htons(547) { ret = BATADV_DHCP_TO_SERVER; } else if (*udphdr).source == htons(547) { ret = BATADV_DHCP_TO_CLIENT; }
    chaddr_offset = *header_len as usize + BATADV_DHCP_CHADDR_OFFSET;
    if ret == BATADV_DHCP_TO_CLIENT { if !pskb_may_pull(skb, chaddr_offset + ETH_ALEN) { return BATADV_DHCP_NO; } p = (*skb).data.add(*header_len as usize + BATADV_DHCP_HTYPE_OFFSET); if *p != BATADV_DHCP_HTYPE_ETHERNET { return BATADV_DHCP_NO; } p = (*skb).data.add(*header_len as usize + BATADV_DHCP_HLEN_OFFSET); if *p != ETH_ALEN as u8 { return BATADV_DHCP_NO; } ether_addr_copy(chaddr, (*skb).data.add(chaddr_offset)); }
    ret
}

pub unsafe extern "C" fn batadv_gw_out_of_range(bat_priv: *mut batadv_priv, skb: *mut sk_buff) -> bool {
    let mut orig_dst_node = core::ptr::null_mut(); let mut neigh_curr = core::ptr::null_mut(); let mut neigh_old = core::ptr::null_mut(); let mut gw_node = core::ptr::null_mut(); let mut curr_gw = core::ptr::null_mut(); let mut out_of_range = false;
    let vid = batadv_get_vid(skb, 0); let ethhdr = (*skb).data as *mut ethhdr; if is_multicast_ether_addr((*ethhdr).h_dest.as_ptr()) { return false; }
    orig_dst_node = batadv_transtable_search(bat_priv, (*ethhdr).h_source.as_ptr(), (*ethhdr).h_dest.as_ptr(), vid); if orig_dst_node.is_null() { return false; }
    gw_node = batadv_gw_node_get(bat_priv, orig_dst_node); if gw_node.is_null() { batadv_orig_node_put(orig_dst_node); return false; }
    let curr_tq_avg: u8;
    match read_once!((*bat_priv).gw.mode) { BATADV_GW_MODE_SERVER => curr_tq_avg = BATADV_TQ_MAX_VALUE, BATADV_GW_MODE_CLIENT => { curr_gw = batadv_gw_get_selected_gw_node(bat_priv); if curr_gw.is_null() || (*curr_gw).orig_node == orig_dst_node { goto_cleanup!(orig_dst_node, gw_node, curr_gw, neigh_old, neigh_curr, out_of_range); } neigh_curr = batadv_find_router(bat_priv, (*curr_gw).orig_node, core::ptr::null_mut()); if neigh_curr.is_null() { goto_cleanup!(orig_dst_node, gw_node, curr_gw, neigh_old, neigh_curr, out_of_range); } let curr_ifinfo = batadv_neigh_ifinfo_get(neigh_curr, BATADV_IF_DEFAULT); if curr_ifinfo.is_null() { goto_cleanup!(orig_dst_node, gw_node, curr_gw, neigh_old, neigh_curr, out_of_range); } curr_tq_avg = (*curr_ifinfo).bat_iv.tq_avg; batadv_neigh_ifinfo_put(curr_ifinfo); }, _ => goto_cleanup!(orig_dst_node, gw_node, curr_gw, neigh_old, neigh_curr, out_of_range) }
    neigh_old = batadv_find_router(bat_priv, orig_dst_node, core::ptr::null_mut()); if neigh_old.is_null() { goto_cleanup!(orig_dst_node, gw_node, curr_gw, neigh_old, neigh_curr, out_of_range); }
    let old_ifinfo = batadv_neigh_ifinfo_get(neigh_old, BATADV_IF_DEFAULT); if old_ifinfo.is_null() { goto_cleanup!(orig_dst_node, gw_node, curr_gw, neigh_old, neigh_curr, out_of_range); }
    if curr_tq_avg.wrapping_sub((*old_ifinfo).bat_iv.tq_avg) > BATADV_GW_THRESHOLD { out_of_range = true; } batadv_neigh_ifinfo_put(old_ifinfo);
    batadv_orig_node_put(orig_dst_node); batadv_gw_node_put(curr_gw); batadv_gw_node_put(gw_node); batadv_neigh_node_put(neigh_old); batadv_neigh_node_put(neigh_curr); out_of_range
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
