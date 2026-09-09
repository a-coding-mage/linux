// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Device handling code
 *	Linux ethernet bridge
 *
 *	Authors:
 *	Lennert Buytenhek		<buytenh@gnu.org>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const COMMON_FEATURES: netdev_features_t = NETIF_F_SG | NETIF_F_FRAGLIST |
    NETIF_F_HIGHDMA | NETIF_F_GSO_MASK | NETIF_F_HW_CSUM;

pub static mut nf_br_ops: *const nf_br_ops = core::ptr::null();

/* net device transmit always called with BH disabled */
pub unsafe fn br_dev_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let reason = pskb_may_pull_reason(skb, ETH_HLEN);
    let mut pmctx_null: *mut net_bridge_mcast_port = core::ptr::null_mut();
    let br = netdev_priv(dev);
    let brmctx = &mut (*br).multicast_ctx;
    let mut dst: *mut net_bridge_fdb_entry;
    let mut mdst: *mut net_bridge_mdb_entry;
    let nf_ops: *const nf_br_ops;
    let mut state: u8 = BR_STATE_FORWARDING;
    let mut vlan: *mut net_bridge_vlan = core::ptr::null_mut();
    let dest: *const u8;
    let mut vid: u16 = 0;

    if reason != SKB_NOT_DROPPED_YET {
        kfree_skb_reason(skb, reason);
        return NETDEV_TX_OK;
    }

    core::ptr::write_bytes((*skb).cb.as_mut_ptr(), 0, core::mem::size_of::<br_input_skb_cb>());
    br_tc_skb_miss_set(skb, false);

    rcu_read_lock();
    nf_ops = rcu_dereference(nf_br_ops);
    if !nf_ops.is_null() && ((*nf_ops).br_dev_xmit_hook)(skb) {
        rcu_read_unlock();
        return NETDEV_TX_OK;
    }

    dev_sw_netstats_tx_add(dev, 1, (*skb).len);
    br_switchdev_frame_unmark(skb);
    BR_INPUT_SKB_CB(skb).brdev = dev;
    BR_INPUT_SKB_CB(skb).frag_max_size = 0;
    skb_reset_mac_header(skb);
    skb_pull(skb, ETH_HLEN);

    if !br_allowed_ingress(br, br_vlan_group_rcu(br), skb, &mut vid, &mut state, &mut vlan) {
        rcu_read_unlock();
        return NETDEV_TX_OK;
    }

    if IS_ENABLED(CONFIG_INET) &&
        ((*eth_hdr(skb)).h_proto == htons(ETH_P_ARP) || (*eth_hdr(skb)).h_proto == htons(ETH_P_RARP)) &&
        br_opt_get(br, BROPT_NEIGH_SUPPRESS_ENABLED) {
        br_do_proxy_suppress_arp(skb, br, vid, core::ptr::null_mut());
    } else if ipv6_mod_enabled() && (*skb).protocol == htons(ETH_P_IPV6) &&
        br_opt_get(br, BROPT_NEIGH_SUPPRESS_ENABLED) &&
        pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<nd_msg>()) &&
        (*ipv6_hdr(skb)).nexthdr == IPPROTO_ICMPV6 {
        let msg = br_is_nd_neigh_msg(skb);
        if !msg.is_null() { br_do_suppress_nd(skb, br, vid, core::ptr::null_mut(), msg); }
    }

    dest = (*eth_hdr(skb)).h_dest.as_ptr();
    if is_broadcast_ether_addr(dest) {
        br_flood(br, skb, BR_PKT_BROADCAST, false, true, vid);
    } else if is_multicast_ether_addr(dest) {
        if netpoll_tx_running(dev) {
            br_flood(br, skb, BR_PKT_MULTICAST, false, true, vid);
            rcu_read_unlock(); return NETDEV_TX_OK;
        }
        if br_multicast_rcv(brmctx, &mut pmctx_null, vlan, skb, vid) {
            kfree_skb(skb); rcu_read_unlock(); return NETDEV_TX_OK;
        }
        mdst = br_mdb_entry_skb_get(brmctx, skb, vid);
        if (!mdst.is_null() || BR_INPUT_SKB_CB_MROUTERS_ONLY(skb) {
            && br_multicast_querier_exists(brmctx, eth_hdr(skb), mdst) {
            br_multicast_flood(mdst, skb, brmctx, false, true);
        } else { br_flood(br, skb, BR_PKT_MULTICAST, false, true, vid); }
    } else if { dst = br_fdb_find_rcu(br, dest, vid); !dst.is_null() } {
        br_forward((*dst).dst.read(), skb, false, true);
    } else { br_flood(br, skb, BR_PKT_UNICAST, false, true, vid); }
    rcu_read_unlock();
    NETDEV_TX_OK
}

unsafe fn br_dev_init(dev: *mut net_device) -> c_int {
    let br = netdev_priv(dev); let mut err = br_fdb_hash_init(br); if err != 0 { return err; }
    err = br_mdb_hash_init(br); if err != 0 { br_fdb_hash_fini(br); return err; }
    err = br_vlan_init(br); if err != 0 { br_mdb_hash_fini(br); br_fdb_hash_fini(br); return err; }
    err = br_multicast_init_stats(br); if err != 0 { br_vlan_flush(br); br_mdb_hash_fini(br); br_fdb_hash_fini(br); return err; }
    netdev_lockdep_set_classes(dev); 0
}

unsafe fn br_dev_uninit(dev: *mut net_device) { let br = netdev_priv(dev); br_multicast_dev_del(br); br_multicast_uninit_stats(br); br_vlan_flush(br); br_mdb_hash_fini(br); br_fdb_hash_fini(br); }
unsafe fn br_dev_open(dev: *mut net_device) -> c_int { let br = netdev_priv(dev); netdev_update_features(dev); netif_start_queue(dev); br_stp_enable_bridge(br); br_multicast_open(br); if br_opt_get(br, BROPT_MULTICAST_ENABLED) { br_multicast_join_snoopers(br); } 0 }
unsafe fn br_dev_set_multicast_list(_dev: *mut net_device) {}
unsafe fn br_dev_change_rx_flags(dev: *mut net_device, change: c_int) { if change & IFF_PROMISC != 0 { br_manage_promisc(netdev_priv(dev)); } }
unsafe fn br_dev_stop(dev: *mut net_device) -> c_int { let br = netdev_priv(dev); br_stp_disable_bridge(br); br_multicast_stop(br); if br_opt_get(br, BROPT_MULTICAST_ENABLED) { br_multicast_leave_snoopers(br); } netif_stop_queue(dev); 0 }

unsafe fn br_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int { let br = netdev_priv(dev); WRITE_ONCE!((*dev).mtu, new_mtu); br_opt_toggle(br, BROPT_MTU_SET_BY_USER, true); /* CONFIG_BRIDGE_NETFILTER: dst_metric_set(&br->fake_rtable.dst, RTAX_MTU, new_mtu); */ 0 }
unsafe fn br_set_mac_address(dev: *mut net_device, p: *mut c_void) -> c_int { let br = netdev_priv(dev); let addr = p as *mut sockaddr; if !is_valid_ether_addr((*addr).sa_data.as_ptr()) { return -EADDRNOTAVAIL; } if (*dev).reg_state != NETREG_REGISTERED { return -EBUSY; } spin_lock_bh(&mut (*br).lock); if !ether_addr_equal((*dev).dev_addr.as_ptr(), (*addr).sa_data.as_ptr()) { br_stp_change_bridge_id(br, (*addr).sa_data.as_ptr()); } spin_unlock_bh(&mut (*br).lock); 0 }

unsafe fn br_getinfo(dev: *mut net_device, info: *mut ethtool_drvinfo) { strscpy((*info).driver.as_mut_ptr(), b"bridge\0".as_ptr(), (*info).driver.len()); strscpy((*info).version.as_mut_ptr(), BR_VERSION.as_ptr(), (*info).version.len()); strscpy((*info).fw_version.as_mut_ptr(), b"N/A\0".as_ptr(), (*info).fw_version.len()); strscpy((*info).bus_info.as_mut_ptr(), b"N/A\0".as_ptr(), (*info).bus_info.len()); }
unsafe fn br_get_link_ksettings(dev: *mut net_device, cmd: *mut ethtool_link_ksettings) -> c_int { let br = netdev_priv(dev); (*cmd).base.duplex = DUPLEX_UNKNOWN; (*cmd).base.port = PORT_OTHER; (*cmd).base.speed = SPEED_UNKNOWN; list_for_each_entry!((*br).port_list, p, { let mut ecmd = core::mem::zeroed::<ethtool_link_ksettings>(); let pdev = (*p).dev; if !netif_running(pdev) || !netif_oper_up(pdev) { continue; } if __ethtool_get_link_ksettings(pdev, &mut ecmd) != 0 || ecmd.base.speed == SPEED_UNKNOWN { continue; } if (*cmd).base.speed == SPEED_UNKNOWN || (*cmd).base.speed < ecmd.base.speed { (*cmd).base.speed = ecmd.base.speed; } }); 0 }
unsafe fn br_fix_features(dev: *mut net_device, features: netdev_features_t) -> netdev_features_t { br_features_recompute(netdev_priv(dev), features) }

// CONFIG_NET_POLL_CONTROLLER conditionally supplies the following netpoll hooks.
#[cfg(feature = "CONFIG_NET_POLL_CONTROLLER")]
unsafe fn br_poll_controller(_dev: *mut net_device) {}

unsafe fn br_add_slave(dev: *mut net_device, slave_dev: *mut net_device, extack: *mut netlink_ext_ack) -> c_int { br_add_if(netdev_priv(dev), slave_dev, extack) }
unsafe fn br_del_slave(dev: *mut net_device, slave_dev: *mut net_device) -> c_int { br_del_if(netdev_priv(dev), slave_dev) }

unsafe fn br_fill_forward_path(ctx: *mut net_device_path_ctx, path: *mut net_device_path) -> c_int {
    if netif_is_bridge_port((*ctx).dev) { return -1; }
    let br = netdev_priv((*ctx).dev); br_vlan_fill_forward_path_pvid(br, ctx, path);
    let f = br_fdb_find_rcu(br, (*ctx).daddr, (*path).bridge.vlan_id); if f.is_null() { return -1; }
    let dst = (*f).dst.read(); if dst.is_null() || br_vlan_fill_forward_path_mode(br, dst, path) != 0 { return -1; }
    (*path).type_ = DEV_PATH_BRIDGE; (*path).dev = (*dst).br.dev; (*ctx).dev = (*dst).dev;
    match (*path).bridge.vlan_mode { DEV_PATH_BR_VLAN_TAG => { if (*ctx).num_vlans >= ARRAY_SIZE!((*ctx).vlan) { return -ENOSPC; } (*ctx).vlan[(*ctx).num_vlans].id = (*path).bridge.vlan_id; (*ctx).vlan[(*ctx).num_vlans].proto = (*path).bridge.vlan_proto; (*ctx).num_vlans += 1; }, DEV_PATH_BR_VLAN_UNTAG_HW | DEV_PATH_BR_VLAN_UNTAG => (*ctx).num_vlans -= 1, DEV_PATH_BR_VLAN_KEEP => {} , _ => {} }
    0
}

pub unsafe fn br_dev_setup(dev: *mut net_device) {
    let br = netdev_priv(dev); eth_hw_addr_random(dev); ether_setup(dev);
    (*dev).netdev_ops = &br_netdev_ops; (*dev).needs_free_netdev = true; (*dev).ethtool_ops = &br_ethtool_ops; SET_NETDEV_DEVTYPE!(dev, &br_type); (*dev).priv_flags = IFF_EBRIDGE | IFF_NO_QUEUE; (*dev).lltx = true; (*dev).netns_immutable = true;
    (*dev).features = COMMON_FEATURES | NETIF_F_HW_VLAN_CTAG_TX | NETIF_F_HW_VLAN_STAG_TX; (*dev).hw_features = (*dev).features; (*dev).vlan_features = COMMON_FEATURES; (*dev).pcpu_stat_type = NETDEV_PCPU_STAT_TSTATS; (*br).dev = dev; spin_lock_init(&mut (*br).lock); INIT_LIST_HEAD!(&mut (*br).port_list); INIT_HLIST_HEAD!(&mut (*br).fdb_list); INIT_HLIST_HEAD!(&mut (*br).frame_type_list); spin_lock_init(&mut (*br).hash_lock);
    (*br).bridge_id.prio[0] = 0x80; (*br).bridge_id.prio[1] = 0x00; ether_addr_copy((*br).group_addr.as_mut_ptr(), eth_stp_addr); (*br).stp_enabled = BR_NO_STP; (*br).stp_mode = BR_STP_MODE_AUTO; (*br).group_fwd_mask = BR_GROUPFWD_DEFAULT; (*br).group_fwd_mask_required = BR_GROUPFWD_DEFAULT; (*br).designated_root = (*br).bridge_id; (*br).bridge_max_age = 20 * HZ; (*br).max_age = (*br).bridge_max_age; (*br).bridge_hello_time = 2 * HZ; (*br).hello_time = (*br).bridge_hello_time; (*br).bridge_forward_delay = 15 * HZ; (*br).forward_delay = (*br).bridge_forward_delay; (*br).bridge_ageing_time = BR_DEFAULT_AGEING_TIME; (*br).ageing_time = (*br).bridge_ageing_time; (*dev).max_mtu = ETH_MAX_MTU; br_netfilter_rtable_init(br); br_stp_timer_init(br); br_multicast_init(br); INIT_DELAYED_WORK!(&mut (*br).gc_work, br_fdb_cleanup);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
