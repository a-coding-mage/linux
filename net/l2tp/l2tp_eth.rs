// SPDX-License-Identifier: GPL-2.0-or-later
/* L2TPv3 ethernet pseudowire driver
 *
 * Copyright (c) 2008,2009,2010 Katalix Systems Ltd
 */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel/Rust bindings.  Build-time configuration such as CONFIG_L2TP_DEBUGFS
// is intentionally preserved at its use site.

const L2TP_ETH_DEV_NAME: &str = "l2tpeth%d";

#[repr(C)]
struct l2tp_eth {
    session: *mut l2tp_session,
}

#[repr(C)]
struct l2tp_eth_sess {
    dev: *mut net_device,
}

unsafe fn l2tp_eth_dev_init(dev: *mut net_device) -> i32 {
    eth_hw_addr_random(dev);
    eth_broadcast_addr((*dev).broadcast.as_mut_ptr());
    netdev_lockdep_set_classes(dev);
    0
}

unsafe fn l2tp_eth_dev_uninit(dev: *mut net_device) {
    let priv_ = netdev_priv::<l2tp_eth>(dev);
    let spriv = l2tp_session_priv::<l2tp_eth_sess>((*priv_).session);
    rcu_assign_pointer((*spriv).dev, core::ptr::null_mut());
    // No need for synchronize_net() here: unregister_netdev*() synchronizes.
}

unsafe fn l2tp_eth_dev_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let priv_ = netdev_priv::<l2tp_eth>(dev);
    let session = (*priv_).session;
    let len = (*skb).len;
    let ret = l2tp_xmit_skb(session, skb);

    if likely(ret == NET_XMIT_SUCCESS) {
        dev_dstats_tx_add(dev, len);
    } else {
        dev_dstats_tx_dropped(dev);
    }
    NETDEV_TX_OK
}

static l2tp_eth_netdev_ops: net_device_ops = net_device_ops {
    ndo_init: Some(l2tp_eth_dev_init),
    ndo_uninit: Some(l2tp_eth_dev_uninit),
    ndo_start_xmit: Some(l2tp_eth_dev_xmit),
    ndo_set_mac_address: Some(eth_mac_addr),
};

static l2tpeth_type: device_type = device_type { name: "l2tpeth" };

unsafe fn l2tp_eth_dev_setup(dev: *mut net_device) {
    SET_NETDEV_DEVTYPE(dev, &l2tpeth_type);
    ether_setup(dev);
    (*dev).priv_flags &= !IFF_TX_SKB_SHARING;
    (*dev).lltx = true;
    (*dev).netdev_ops = &l2tp_eth_netdev_ops;
    (*dev).needs_free_netdev = true;
    (*dev).pcpu_stat_type = NETDEV_PCPU_STAT_DSTATS;
}

unsafe fn l2tp_eth_dev_recv(session: *mut l2tp_session, skb: *mut sk_buff, data_len: i32) {
    let spriv = l2tp_session_priv::<l2tp_eth_sess>(session);
    let dev;

    if !pskb_may_pull(skb, ETH_HLEN) { goto_error(skb); return; }
    secpath_reset(skb);
    (*skb).ip_summed = CHECKSUM_NONE;
    skb_clear_hash(skb);
    skb_dst_drop(skb);
    nf_reset_ct(skb);

    rcu_read_lock();
    dev = rcu_dereference((*spriv).dev);
    if dev.is_null() {
        rcu_read_unlock();
        goto_error(skb);
        return;
    }
    if dev_forward_skb(dev, skb) == NET_RX_SUCCESS {
        dev_dstats_rx_add(dev, data_len as u32);
    } else {
        DEV_STATS_INC(dev, rx_errors);
    }
    rcu_read_unlock();
    return;
}

unsafe fn goto_error(skb: *mut sk_buff) { kfree_skb(skb); }

unsafe fn l2tp_eth_delete(session: *mut l2tp_session) {
    if !session.is_null() {
        let spriv = l2tp_session_priv::<l2tp_eth_sess>(session);
        rtnl_lock();
        let dev = rtnl_dereference((*spriv).dev);
        if !dev.is_null() {
            unregister_netdevice(dev);
            rtnl_unlock();
            module_put(THIS_MODULE);
        } else { rtnl_unlock(); }
    }
}

unsafe fn l2tp_eth_show(m: *mut seq_file, arg: *mut core::ffi::c_void) {
    let session = arg as *mut l2tp_session;
    let spriv = l2tp_session_priv::<l2tp_eth_sess>(session);
    rcu_read_lock();
    let dev = rcu_dereference((*spriv).dev);
    if dev.is_null() { rcu_read_unlock(); return; }
    dev_hold(dev);
    rcu_read_unlock();
    seq_printf(m, "   interface %s\n", (*dev).name.as_ptr());
    dev_put(dev);
}

unsafe fn l2tp_eth_adjust_mtu(tunnel: *mut l2tp_tunnel, session: *mut l2tp_session, dev: *mut net_device) {
    let mut overhead: u32 = 0;
    let mut l3_overhead: u32 = 0;
    if (*tunnel).encap == L2TP_ENCAPTYPE_UDP {
        overhead += core::mem::size_of::<udphdr>() as u32;
        (*dev).needed_headroom += core::mem::size_of::<udphdr>() as u32;
    }
    lock_sock((*tunnel).sock);
    l3_overhead = kernel_sock_ip_overhead((*tunnel).sock);
    release_sock((*tunnel).sock);
    if l3_overhead == 0 { return; }
    overhead += (*session).hdr_len + ETH_HLEN as u32 + l3_overhead;
    let mtu = l2tp_tunnel_dst_mtu(tunnel) - overhead;
    (*dev).mtu = if mtu < (*dev).min_mtu || mtu > (*dev).max_mtu { ETH_DATA_LEN as u32 - overhead } else { mtu };
    (*dev).needed_headroom += (*session).hdr_len;
}

unsafe fn l2tp_eth_create(net: *mut net, tunnel: *mut l2tp_tunnel, session_id: u32, peer_session_id: u32, cfg: *mut l2tp_session_cfg) -> i32 {
    let mut name = [0i8; IFNAMSIZ];
    let name_assign_type;
    if !(*cfg).ifname.is_null() { strscpy(name.as_mut_ptr(), (*cfg).ifname); name_assign_type = NET_NAME_USER; }
    else { strscpy(name.as_mut_ptr(), L2TP_ETH_DEV_NAME.as_ptr() as *const i8); name_assign_type = NET_NAME_ENUM; }
    let session = l2tp_session_create(core::mem::size_of::<l2tp_eth_sess>(), tunnel, session_id, peer_session_id, cfg);
    if IS_ERR(session) { return PTR_ERR(session); }
    let dev = alloc_netdev(core::mem::size_of::<l2tp_eth>(), name.as_mut_ptr(), name_assign_type, l2tp_eth_dev_setup);
    if dev.is_null() { l2tp_session_put(session); return -ENOMEM; }
    dev_net_set(dev, net); (*dev).min_mtu = 0; (*dev).max_mtu = ETH_MAX_MTU;
    l2tp_eth_adjust_mtu(tunnel, session, dev);
    netdev_priv::<l2tp_eth>(dev).write(l2tp_eth { session });
    (*session).recv_skb = Some(l2tp_eth_dev_recv); (*session).session_close = Some(l2tp_eth_delete);
    // CONFIG_L2TP_DEBUGFS conditionally assigns session->show = l2tp_eth_show.
    let spriv = l2tp_session_priv::<l2tp_eth_sess>(session);
    refcount_inc(&mut (*session).ref_count); rtnl_lock();
    let mut rc = l2tp_session_register(session, tunnel);
    if rc < 0 { rtnl_unlock(); l2tp_session_put(session); free_netdev(dev); l2tp_session_put(session); return rc; }
    rc = register_netdevice(dev);
    if rc < 0 { rtnl_unlock(); l2tp_session_delete(session); l2tp_session_put(session); free_netdev(dev); return rc; }
    strscpy((*session).ifname.as_mut_ptr(), (*dev).name.as_ptr(), IFNAMSIZ);
    rcu_assign_pointer((*spriv).dev, dev); rtnl_unlock(); l2tp_session_put(session); __module_get(THIS_MODULE); 0
}

static l2tp_eth_nl_cmd_ops: l2tp_nl_cmd_ops = l2tp_nl_cmd_ops { session_create: Some(l2tp_eth_create), session_delete: Some(l2tp_session_delete) };

unsafe fn l2tp_eth_init() -> i32 {
    let err = l2tp_nl_register_ops(L2TP_PWTYPE_ETH, &l2tp_eth_nl_cmd_ops);
    if err != 0 { return err; }
    pr_info!("L2TP ethernet pseudowire support (L2TPv3)\n"); 0
}

unsafe fn l2tp_eth_exit() { l2tp_nl_unregister_ops(L2TP_PWTYPE_ETH); }

// module_init(l2tp_eth_init); module_exit(l2tp_eth_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("James Chapman <jchapman@katalix.com>");
// MODULE_DESCRIPTION("L2TP ethernet pseudowire driver");
// MODULE_VERSION("1.0");
// MODULE_ALIAS_L2TP_PWTYPE(5);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
