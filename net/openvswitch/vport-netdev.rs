// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2007-2012 Nicira, Inc.
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C headers and local headers are supplied by the surrounding kernel/Open vSwitch translation.

static mut ovs_netdev_vport_ops: struct_vport_ops = struct_vport_ops { ..unsafe { core::mem::zeroed() } };

/* Must be called with rcu_read_lock. */
unsafe fn netdev_port_receive(mut skb: *mut sk_buff) {
    let vport: *mut vport;

    vport = ovs_netdev_get_vport((*skb).dev);
    if unlikely(!vport.is_null()) {
        goto_error(skb);
        return;
    }

    if unlikely(skb_warn_if_lro(skb)) {
        goto_error(skb);
        return;
    }

    /* Make our own copy of the packet.  Otherwise we will mangle the
     * packet for anyone who came before us (e.g. tcpdump via AF_PACKET).
     */
    skb = skb_share_check(skb, GFP_ATOMIC);
    if unlikely(skb.is_null()) {
        return;
    }

    if (*(*skb).dev).type == ARPHRD_ETHER {
        skb_push_rcsum(skb, ETH_HLEN);
    }

    ovs_vport_receive(vport, skb, skb_tunnel_info(skb));
    return;

    unsafe fn goto_error(skb: *mut sk_buff) {
        kfree_skb(skb);
    }
}

/* Called with rcu_read_lock and bottom-halves disabled. */
unsafe fn netdev_frame_hook(pskb: *mut *mut sk_buff) -> rx_handler_result_t {
    let skb: *mut sk_buff = *pskb;

    if unlikely((*skb).pkt_type == PACKET_LOOPBACK) {
        return RX_HANDLER_PASS;
    }

    netdev_port_receive(skb);
    RX_HANDLER_CONSUMED
}

unsafe fn get_dpdev(dp: *const datapath) -> *mut net_device {
    let local: *mut vport;

    local = ovs_vport_ovsl(dp, OVSP_LOCAL);
    (*local).dev
}

unsafe fn ovs_netdev_link(vport: *mut vport) -> *mut vport {
    let mut err: i32;

    if WARN_ON_ONCE((*vport).dev.is_null()) {
        err = -ENODEV;
        goto_error_free_vport(vport, err);
        return ERR_PTR(err);
    }

    rtnl_lock();
    if (*(*vport).dev).reg_state != NETREG_REGISTERED {
        err = -ENODEV;
        goto_error_put_unlock(vport, err);
        return ERR_PTR(err);
    }

    err = netdev_master_upper_dev_link((*vport).dev, get_dpdev((*vport).dp), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if err != 0 {
        goto_error_put_unlock(vport, err);
        return ERR_PTR(err);
    }

    err = netdev_rx_handler_register((*vport).dev, Some(netdev_frame_hook), vport);
    if err != 0 {
        netdev_upper_dev_unlink((*vport).dev, get_dpdev((*vport).dp));
        netdev_put((*vport).dev, &mut (*vport).dev_tracker);
        rtnl_unlock();
        ovs_vport_free(vport);
        return ERR_PTR(err);
    }

    dev_disable_lro((*vport).dev);
    dev_set_promiscuity((*vport).dev, 1);
    (*(*vport).dev).priv_flags |= IFF_OVS_DATAPATH;
    rtnl_unlock();

    vport
}

unsafe fn netdev_create(parms: *const vport_parms) -> *mut vport {
    let vport: *mut vport;
    let mut err: i32;

    vport = ovs_vport_alloc(0, &raw mut ovs_netdev_vport_ops, parms);
    if IS_ERR(vport) {
        return vport;
    }

    (*vport).dev = dev_get_by_name(ovs_dp_get_net((*vport).dp), (*parms).name);
    if (*vport).dev.is_null() {
        err = -ENODEV;
        ovs_vport_free(vport);
        return ERR_PTR(err);
    }
    netdev_tracker_alloc((*vport).dev, &mut (*vport).dev_tracker, GFP_KERNEL);

    /* Ensure that the provided name is not an alias. */
    if strcmp((*parms).name, ovs_vport_name(vport)) != 0 {
        err = -ENODEV;
        netdev_put((*vport).dev, &mut (*vport).dev_tracker);
        ovs_vport_free(vport);
        return ERR_PTR(err);
    }

    if (*(*vport).dev).flags & IFF_LOOPBACK != 0
        || (((*(*vport).dev).type != ARPHRD_ETHER)
            && ((*(*vport).dev).type != ARPHRD_NONE))
        || ovs_is_internal_dev((*vport).dev)
    {
        err = -EINVAL;
        netdev_put((*vport).dev, &mut (*vport).dev_tracker);
        ovs_vport_free(vport);
        return ERR_PTR(err);
    }

    ovs_netdev_link(vport)
}

unsafe fn vport_netdev_free(rcu: *mut rcu_head) {
    let vport: *mut vport = container_of!(rcu, vport, rcu);

    netdev_put((*vport).dev, &mut (*vport).dev_tracker);
    ovs_vport_free(vport);
}

pub unsafe fn ovs_netdev_detach_dev(vport: *mut vport) {
    ASSERT_RTNL();
    netdev_rx_handler_unregister((*vport).dev);
    netdev_upper_dev_unlink((*vport).dev, netdev_master_upper_dev_get((*vport).dev));
    dev_set_promiscuity((*vport).dev, -1);

    /* paired with smp_mb() in netdev_destroy() */
    smp_wmb();

    (*(*vport).dev).priv_flags &= !IFF_OVS_DATAPATH;
}

unsafe fn netdev_destroy(vport: *mut vport) {
    /* When called from ovs_db_notify_wq() after a dp_device_event(), the
     * port has already been detached, so we can avoid taking the RTNL by
     * checking this first.
     */
    if netif_is_ovs_port((*vport).dev) {
        rtnl_lock();
        /* Check again while holding the lock to ensure we don't race
         * with the netdev notifier and detach twice.
         */
        if netif_is_ovs_port((*vport).dev) {
            ovs_netdev_detach_dev(vport);
        }
        rtnl_unlock();
    }

    /* paired with smp_wmb() in ovs_netdev_detach_dev() */
    smp_mb();

    call_rcu(&mut (*vport).rcu, vport_netdev_free);
}

/* Returns null if this device is not attached to a datapath. */
pub unsafe fn ovs_netdev_get_vport(dev: *mut net_device) -> *mut vport {
    if likely(netif_is_ovs_port(dev)) {
        rcu_dereference_rtnl((*dev).rx_handler_data) as *mut vport
    } else {
        core::ptr::null_mut()
    }
}

static mut ovs_netdev_vport_ops: struct_vport_ops = struct_vport_ops {
    type_: OVS_VPORT_TYPE_NETDEV,
    create: Some(netdev_create),
    destroy: Some(netdev_destroy),
    send: Some(dev_queue_xmit),
};

pub unsafe fn ovs_netdev_init() -> i32 {
    ovs_vport_ops_register(&raw mut ovs_netdev_vport_ops)
}

pub unsafe fn ovs_netdev_exit() {
    ovs_vport_ops_unregister(&raw mut ovs_netdev_vport_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
