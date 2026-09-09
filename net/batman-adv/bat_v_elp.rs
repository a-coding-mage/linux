// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 * Linus Lüssing, Marek Lindner
 */

// C headers and symbols supplied by the surrounding kernel translation.

#[repr(C)]
pub struct batadv_v_metric_queue_entry {
    pub hardif_neigh: *mut batadv_hardif_neigh_node,
    pub list: list_head,
}

unsafe fn batadv_v_elp_start_timer(hard_iface: *mut batadv_hard_iface) {
    let mut msecs = READ_ONCE((*hard_iface).bat_v.elp_interval) - BATADV_JITTER;
    msecs += get_random_u32_below(2 * BATADV_JITTER);
    queue_delayed_work(batadv_event_workqueue, &mut (*hard_iface).bat_v.elp_wq,
                       msecs_to_jiffies(msecs));
}

unsafe fn batadv_v_elp_get_throughput(neigh: *mut batadv_hardif_neigh_node,
                                      pthroughput: *mut u32) -> bool {
    let hard_iface = (*neigh).if_incoming;
    let mesh_iface = (*hard_iface).mesh_iface;
    let mut link_settings: ethtool_link_ksettings = core::mem::zeroed();
    let mut sinfo: station_info = core::mem::zeroed();
    let mut throughput = READ_ONCE((*hard_iface).bat_v.throughput_override);
    if throughput != 0 { *pthroughput = throughput; return true; }

    let wifi_flags = batadv_hardif_get_wifi_flags(hard_iface);
    if batadv_is_wifi(wifi_flags) {
        if !batadv_is_cfg80211(wifi_flags) { goto_default!(default_throughput); }
        if !rtnl_trylock() { return false; }
        let real_netdev = __batadv_get_real_netdev((*hard_iface).net_dev);
        rtnl_unlock();
        if real_netdev.is_null() { goto_default!(default_throughput); }
        let ret = cfg80211_get_station(real_netdev, (*neigh).addr, &mut sinfo);
        if ret == 0 { cfg80211_sinfo_release_content(&mut sinfo); }
        dev_put(real_netdev);
        if ret == -ENOENT { *pthroughput = 0; return true; }
        if ret != 0 { goto_default!(default_throughput); }
        if sinfo.filled & BIT(NL80211_STA_INFO_EXPECTED_THROUGHPUT) != 0 {
            *pthroughput = sinfo.expected_throughput / 100; return true;
        }
        if sinfo.filled & BIT(NL80211_STA_INFO_TX_BITRATE) != 0 {
            *pthroughput = cfg80211_calculate_bitrate(&sinfo.txrate) / 3; return true;
        }
        goto_default!(default_throughput);
    }

    if !rtnl_trylock() { return false; }
    let ret = __ethtool_get_link_ksettings((*hard_iface).net_dev, &mut link_settings);
    rtnl_unlock();
    if ret == 0 {
        if link_settings.base.duplex == DUPLEX_FULL { (*hard_iface).bat_v.flags |= BATADV_FULL_DUPLEX; }
        else { (*hard_iface).bat_v.flags &= !BATADV_FULL_DUPLEX; }
        throughput = link_settings.base.speed;
        if throughput != 0 && throughput != SPEED_UNKNOWN { *pthroughput = throughput * 10; return true; }
    }

    if (*hard_iface).bat_v.flags & BATADV_WARNING_DEFAULT == 0 {
        batadv_info(mesh_iface, "WiFi driver or ethtool info does not provide information about link speeds on interface %s, therefore defaulting to hardcoded throughput values of %u.%1u Mbps. Consider overriding the throughput manually or checking your driver.\n",
                    (*hard_iface).net_dev.name, BATADV_THROUGHPUT_DEFAULT_VALUE / 10,
                    BATADV_THROUGHPUT_DEFAULT_VALUE % 10);
        (*hard_iface).bat_v.flags |= BATADV_WARNING_DEFAULT;
    }
    *pthroughput = BATADV_THROUGHPUT_DEFAULT_VALUE; true
}

unsafe fn batadv_v_elp_throughput_metric_update(neigh: *mut batadv_hardif_neigh_node) {
    let mut throughput = 0;
    if batadv_v_elp_get_throughput(neigh, &mut throughput) { ewma_throughput_add(&mut (*neigh).bat_v.throughput, throughput); }
}

unsafe fn batadv_v_elp_wifi_neigh_probe(neigh: *mut batadv_hardif_neigh_node) -> bool {
    let hard_iface = (*neigh).if_incoming;
    let bat_priv = netdev_priv((*hard_iface).mesh_iface);
    if !batadv_is_wifi_hardif(hard_iface) { return true; }
    let last_tx_diff = jiffies_to_msecs(jiffies - (*neigh).bat_v.last_unicast_tx);
    if last_tx_diff <= BATADV_ELP_PROBE_MAX_TX_DIFF { return true; }
    let probe_len = core::cmp::max(core::mem::size_of::<batadv_elp_packet>(), BATADV_ELP_MIN_PROBE_SIZE);
    for _ in 0..BATADV_ELP_PROBES_PER_NODE {
        let elp_skb_len = (*hard_iface).bat_v.elp_skb.len;
        let skb = skb_copy_expand((*hard_iface).bat_v.elp_skb, 0, probe_len - elp_skb_len, GFP_ATOMIC);
        if skb.is_null() { return false; }
        skb_put_zero(skb, probe_len - (*hard_iface).bat_v.elp_skb.len);
        batadv_dbg(BATADV_DBG_BATMAN, bat_priv, "Sending unicast (probe) ELP packet on interface %s to %pM\n", (*hard_iface).net_dev.name, (*neigh).addr);
        batadv_send_skb_packet(skb, hard_iface, (*neigh).addr);
    }
    true
}

unsafe fn batadv_v_elp_periodic_work(work: *mut work_struct) {
    let bat_v = container_of!(work, batadv_hard_iface_bat_v, elp_wq.work);
    let hard_iface = container_of!(bat_v, batadv_hard_iface, bat_v);
    let bat_priv = netdev_priv((*hard_iface).mesh_iface);
    if READ_ONCE((*bat_priv).mesh_state) == BATADV_MESH_DEACTIVATING || (*hard_iface).if_status == BATADV_IF_TO_BE_REMOVED { return; }
    if (*hard_iface).if_status != BATADV_IF_ACTIVE { batadv_v_elp_start_timer(hard_iface); return; }
    let skb = skb_copy((*hard_iface).bat_v.elp_skb, GFP_ATOMIC);
    if skb.is_null() { batadv_v_elp_start_timer(hard_iface); return; }
    let packet = (*skb).data as *mut batadv_elp_packet;
    (*packet).seqno = htonl(atomic_read(&(*hard_iface).bat_v.elp_seqno));
    let interval = READ_ONCE((*hard_iface).bat_v.elp_interval);
    (*packet).elp_interval = htonl(interval);
    batadv_dbg(BATADV_DBG_BATMAN, bat_priv, "Sending broadcast ELP packet on interface %s, seqno %u\n", (*hard_iface).net_dev.name, atomic_read(&(*hard_iface).bat_v.elp_seqno));
    batadv_send_broadcast_skb(skb, hard_iface); atomic_inc(&mut (*hard_iface).bat_v.elp_seqno);
    let mut queue: list_head = core::mem::zeroed(); INIT_LIST_HEAD(&mut queue);
    rcu_read_lock();
    hlist_for_each_entry_rcu!(hardif_neigh, &(*hard_iface).neigh_list, list, {
        if !batadv_v_elp_wifi_neigh_probe(hardif_neigh) { break; }
        if !kref_get_unless_zero(&mut (*hardif_neigh).refcount) { continue; }
        let entry = kzalloc_obj!(batadv_v_metric_queue_entry, GFP_ATOMIC);
        if entry.is_null() { batadv_hardif_neigh_put(hardif_neigh); continue; }
        (*entry).hardif_neigh = hardif_neigh; list_add(&mut (*entry).list, &mut queue);
    });
    rcu_read_unlock();
    list_for_each_entry_safe!(entry, safe, &mut queue, list, {
        batadv_v_elp_throughput_metric_update((*entry).hardif_neigh);
        batadv_hardif_neigh_put((*entry).hardif_neigh); list_del(&mut (*entry).list); kfree(entry);
    });
    batadv_v_elp_start_timer(hard_iface);
}

pub unsafe fn batadv_v_elp_iface_enable(hard_iface: *mut batadv_hard_iface) -> i32 {
    let size = ETH_HLEN + NET_IP_ALIGN + BATADV_ELP_HLEN + core::mem::size_of::<u32>();
    (*hard_iface).bat_v.elp_skb = dev_alloc_skb(size);
    if (*hard_iface).bat_v.elp_skb.is_null() { return -ENOMEM; }
    skb_reserve((*hard_iface).bat_v.elp_skb, ETH_HLEN + NET_IP_ALIGN);
    let packet = skb_put_zero((*hard_iface).bat_v.elp_skb, BATADV_ELP_HLEN + core::mem::size_of::<u32>()) as *mut batadv_elp_packet;
    (*packet).packet_type = BATADV_ELP; (*packet).version = BATADV_COMPAT_VERSION;
    let mut seqno = 0; get_random_bytes(&mut seqno, core::mem::size_of::<u32>()); atomic_set(&mut (*hard_iface).bat_v.elp_seqno, seqno);
    (*hard_iface).bat_v.flags |= BATADV_FULL_DUPLEX; (*hard_iface).bat_v.flags &= !BATADV_WARNING_DEFAULT;
    if batadv_is_wifi_hardif(hard_iface) { (*hard_iface).bat_v.flags &= !BATADV_FULL_DUPLEX; }
    INIT_DELAYED_WORK!(&mut (*hard_iface).bat_v.elp_wq, batadv_v_elp_periodic_work); batadv_v_elp_start_timer(hard_iface); 0
}

pub unsafe fn batadv_v_elp_iface_disable(hard_iface: *mut batadv_hard_iface) { disable_delayed_work_sync(&mut (*hard_iface).bat_v.elp_wq); dev_kfree_skb((*hard_iface).bat_v.elp_skb); (*hard_iface).bat_v.elp_skb = core::ptr::null_mut(); }

pub unsafe fn batadv_v_elp_iface_activate(primary_iface: *mut batadv_hard_iface, hard_iface: *mut batadv_hard_iface) {
    if (*hard_iface).bat_v.elp_skb.is_null() { return; }
    ether_addr_copy(((*(*hard_iface).bat_v.elp_skb).data as *mut batadv_elp_packet).as_mut().unwrap().orig.as_mut_ptr(), (*primary_iface).net_dev.dev_addr);
}

pub unsafe fn batadv_v_elp_primary_iface_set(primary_iface: *mut batadv_hard_iface) { rcu_read_lock(); netdev_for_each_lower_private_rcu!((*primary_iface).mesh_iface, hard_iface, iter, batadv_v_elp_iface_activate(primary_iface, hard_iface)); rcu_read_unlock(); }

unsafe fn batadv_v_elp_neigh_update(bat_priv: *mut batadv_priv, neigh_addr: *mut u8, if_incoming: *mut batadv_hard_iface, packet: *mut batadv_elp_packet) {
    let orig = batadv_v_ogm_orig_get(bat_priv, (*packet).orig.as_mut_ptr()); if orig.is_null() { return; }
    let neigh = batadv_neigh_node_get_or_create(orig, if_incoming, neigh_addr); if neigh.is_null() { batadv_orig_node_put(orig); return; }
    let hardif = batadv_hardif_neigh_get(if_incoming, neigh_addr); if hardif.is_null() { batadv_neigh_node_put(neigh); batadv_orig_node_put(orig); return; }
    let diff = ntohl((*packet).seqno) as i32 - (*hardif).bat_v.elp_latest_seqno;
    if diff >= 1 || diff <= -BATADV_ELP_MAX_AGE { (*neigh).last_seen = jiffies; (*hardif).last_seen = jiffies; (*hardif).bat_v.elp_latest_seqno = ntohl((*packet).seqno); (*hardif).bat_v.elp_interval = ntohl((*packet).elp_interval); }
    batadv_hardif_neigh_put(hardif); batadv_neigh_node_put(neigh); batadv_orig_node_put(orig);
}

pub unsafe fn batadv_v_elp_packet_recv(skb: *mut sk_buff, if_incoming: *mut batadv_hard_iface) -> i32 {
    let bat_priv = netdev_priv((*if_incoming).mesh_iface); let eth = eth_hdr(skb); let mut ret = NET_RX_DROP;
    if !batadv_check_management_packet(skb, if_incoming, BATADV_ELP_HLEN) || batadv_is_my_mac(bat_priv, (*eth).h_source.as_mut_ptr()) || strcmp((*bat_priv).algo_ops).name, "BATMAN_V") != 0 { kfree_skb(skb); return ret; }
    let packet = (*skb).data as *mut batadv_elp_packet; let primary = batadv_primary_if_get_selected(bat_priv); if primary.is_null() { kfree_skb(skb); return ret; }
    batadv_v_elp_neigh_update(bat_priv, (*eth).h_source.as_mut_ptr(), if_incoming, packet); ret = NET_RX_SUCCESS; batadv_hardif_put(primary);
    if ret == NET_RX_SUCCESS { consume_skb(skb); } else { kfree_skb(skb); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
