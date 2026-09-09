// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2007-2012 Nicira, Inc.
 */

// Linux and Open vSwitch dependencies supplied by other translation units.

#[repr(C)]
struct internal_dev {
    vport: *mut vport,
}

// Forward declaration of ovs_internal_vport_ops; its definition appears below.

unsafe fn internal_dev_priv(netdev: *mut net_device) -> *mut internal_dev {
    netdev_priv(netdev)
}

/* Called with rcu_read_lock_bh. */
unsafe fn internal_dev_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t {
    // store len value because skb can be freed inside ovs_vport_receive()
    let len: i32 = (*skb).len;

    rcu_read_lock();
    let err = ovs_vport_receive((*internal_dev_priv(netdev)).vport, skb, core::ptr::null_mut());
    rcu_read_unlock();

    if likely(err == 0) {
        dev_sw_netstats_tx_add(netdev, 1, len);
    } else {
        (*netdev).stats.tx_errors += 1;
    }

    NETDEV_TX_OK
}

unsafe fn internal_dev_open(netdev: *mut net_device) -> i32 {
    netif_start_queue(netdev);
    0
}

unsafe fn internal_dev_stop(netdev: *mut net_device) -> i32 {
    netif_stop_queue(netdev);
    0
}

unsafe fn internal_dev_getinfo(netdev: *mut net_device, info: *mut ethtool_drvinfo) {
    strscpy((*info).driver.as_mut_ptr(), b"openvswitch\0".as_ptr(), core::mem::size_of_val(&(*info).driver));
}

static internal_dev_ethtool_ops: ethtool_ops = ethtool_ops {
    get_drvinfo: Some(internal_dev_getinfo),
    get_link: Some(ethtool_op_get_link),
};

unsafe fn internal_dev_destructor(dev: *mut net_device) {
    let vport = ovs_internal_dev_get_vport(dev);
    ovs_vport_free(vport);
}

static internal_dev_netdev_ops: net_device_ops = net_device_ops {
    ndo_open: Some(internal_dev_open),
    ndo_stop: Some(internal_dev_stop),
    ndo_start_xmit: Some(internal_dev_xmit),
    ndo_set_mac_address: Some(eth_mac_addr),
};

static mut internal_dev_link_ops: rtnl_link_ops = rtnl_link_ops {
    kind: b"openvswitch\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn do_setup(netdev: *mut net_device) {
    ether_setup(netdev);

    (*netdev).max_mtu = ETH_MAX_MTU;
    (*netdev).netdev_ops = &internal_dev_netdev_ops;
    (*netdev).priv_flags &= !IFF_TX_SKB_SHARING;
    (*netdev).priv_flags |= IFF_LIVE_ADDR_CHANGE | IFF_OPENVSWITCH | IFF_NO_QUEUE;
    (*netdev).lltx = true;
    (*netdev).needs_free_netdev = true;
    (*netdev).priv_destructor = None;
    (*netdev).ethtool_ops = &internal_dev_ethtool_ops;
    (*netdev).rtnl_link_ops = &internal_dev_link_ops;

    (*netdev).features = NETIF_F_SG | NETIF_F_FRAGLIST | NETIF_F_HIGHDMA |
        NETIF_F_HW_CSUM | NETIF_F_GSO_SOFTWARE | NETIF_F_GSO_ENCAP_ALL;
    (*netdev).vlan_features = (*netdev).features;
    (*netdev).hw_enc_features = (*netdev).features;
    (*netdev).features |= NETIF_F_HW_VLAN_CTAG_TX | NETIF_F_HW_VLAN_STAG_TX;
    (*netdev).hw_features = (*netdev).features;
    eth_hw_addr_random(netdev);
}

unsafe fn internal_dev_create(parms: *const vport_parms) -> *mut vport {
    let mut err: i32;
    let vport = ovs_vport_alloc(0, &mut ovs_internal_vport_ops, parms);
    if IS_ERR(vport) {
        err = PTR_ERR(vport);
        return ERR_PTR(err);
    }

    let dev = alloc_netdev(core::mem::size_of::<internal_dev>(), (*parms).name, NET_NAME_USER, do_setup);
    (*vport).dev = dev;
    if dev.is_null() {
        err = -ENOMEM;
        ovs_vport_free(vport);
        return ERR_PTR(err);
    }
    (*dev).pcpu_stat_type = NETDEV_PCPU_STAT_TSTATS;
    dev_net_set((*vport).dev, ovs_dp_get_net((*vport).dp));
    (*dev).ifindex = (*parms).desired_ifindex;
    (*internal_dev_priv((*vport).dev)).vport = vport;

    // Restrict bridge port to current netns.
    if (*vport).port_no == OVSP_LOCAL {
        (*vport).dev.netns_immutable = true;
    }

    rtnl_lock();
    err = register_netdevice((*vport).dev);
    if err != 0 {
        rtnl_unlock();
        free_netdev(dev);
        ovs_vport_free(vport);
        return ERR_PTR(err);
    }
    (*vport).dev.priv_destructor = Some(internal_dev_destructor);
    dev_set_promiscuity((*vport).dev, 1);
    rtnl_unlock();
    netif_start_queue((*vport).dev);
    vport
}

unsafe fn internal_dev_destroy(vport: *mut vport) {
    netif_stop_queue((*vport).dev);
    rtnl_lock();
    dev_set_promiscuity((*vport).dev, -1);
    // unregister_netdevice() waits for an RCU grace period.
    unregister_netdevice((*vport).dev);
    rtnl_unlock();
}

unsafe fn internal_dev_recv(skb: *mut sk_buff) -> i32 {
    let netdev = (*skb).dev;
    if ((*netdev).flags & IFF_UP) == 0 {
        kfree_skb(skb);
        (*netdev).stats.rx_dropped += 1;
        return NETDEV_TX_OK;
    }
    skb_dst_drop(skb);
    nf_reset_ct(skb);
    (*skb).pkt_type = PACKET_HOST;
    (*skb).protocol = eth_type_trans(skb, netdev);
    skb_postpull_rcsum(skb, eth_hdr(skb), ETH_HLEN);
    dev_sw_netstats_rx_add(netdev, (*skb).len);
    netif_rx(skb);
    NETDEV_TX_OK
}

static mut ovs_internal_vport_ops: vport_ops = vport_ops {
    type_: OVS_VPORT_TYPE_INTERNAL,
    create: Some(internal_dev_create),
    destroy: Some(internal_dev_destroy),
    send: Some(internal_dev_recv),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn ovs_is_internal_dev(netdev: *const net_device) -> i32 {
    ((*netdev).netdev_ops == &internal_dev_netdev_ops) as i32
}

unsafe fn ovs_internal_dev_get_vport(netdev: *mut net_device) -> *mut vport {
    if ovs_is_internal_dev(netdev) == 0 { return core::ptr::null_mut(); }
    (*internal_dev_priv(netdev)).vport
}

unsafe fn ovs_internal_dev_rtnl_link_register() -> i32 {
    let mut err = rtnl_link_register(&mut internal_dev_link_ops);
    if err < 0 { return err; }
    err = ovs_vport_ops_register(&mut ovs_internal_vport_ops);
    if err < 0 { rtnl_link_unregister(&mut internal_dev_link_ops); }
    err
}

unsafe fn ovs_internal_dev_rtnl_link_unregister() {
    ovs_vport_ops_unregister(&mut ovs_internal_vport_ops);
    rtnl_link_unregister(&mut internal_dev_link_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
