// SPDX-License-Identifier: GPL-2.0
/* Copyright 2011-2014 Autronica Fire and Security AS
 *
 * Author(s):
 *	2011-2014 Arvid Brodin, arvid.brodin@alten.se
 *
 * Frame handler other utility functions for HSR and PRP.
 */

// C dependencies supplied by the surrounding kernel/project translation.

pub unsafe fn hsr_invalid_dan_ingress_frame(protocol: __be16) -> bool {
    protocol != htons(ETH_P_PRP) && protocol != htons(ETH_P_HSR)
}

unsafe fn hsr_handle_frame(pskb: *mut *mut sk_buff) -> rx_handler_result_t {
    let skb = *pskb;
    let mut port: *mut hsr_port;
    let hsr: *mut hsr_priv;
    let protocol: __be16;

    /* Packets from dev_loopback_xmit() do not have L2 header, bail out */
    if unlikely((*skb).pkt_type == PACKET_LOOPBACK) {
        return RX_HANDLER_PASS;
    }

    if !skb_mac_header_was_set(skb) {
        WARN_ONCE(true, "%s: skb invalid", __func__);
        return RX_HANDLER_PASS;
    }

    port = hsr_port_get_rcu((*skb).dev);
    if port.is_null() {
        return RX_HANDLER_PASS;
    }
    hsr = (*port).hsr;

    if hsr_addr_is_self((*port).hsr, (*eth_hdr(skb)).h_source) {
        /* Directly kill frames sent by ourselves */
        kfree_skb(skb);
        return RX_HANDLER_CONSUMED;
    }

    /* For HSR, only tagged frames are expected (unless the device offloads
     * HSR tag removal), but for PRP there could be non tagged frames as
     * well from Single attached nodes (SANs).
     */
    protocol = (*eth_hdr(skb)).h_proto;

    if ((*(*port).dev).features & NETIF_F_HW_HSR_TAG_RM) == 0
        && (*port).type != HSR_PT_INTERLINK
        && (*(*hsr).proto_ops).invalid_dan_ingress_frame.is_some()
        && !(*(*hsr).proto_ops).invalid_dan_ingress_frame.unwrap()(protocol)
    {
        return RX_HANDLER_PASS;
    }

    skb_push(skb, ETH_HLEN);
    skb_reset_mac_header(skb);
    if ((!(*hsr).prot_version && protocol == htons(ETH_P_PRP))
        || protocol == htons(ETH_P_HSR)
    {
        if !pskb_may_pull(skb, ETH_HLEN + HSR_HLEN) {
            kfree_skb(skb);
            return RX_HANDLER_CONSUMED;
        }

        skb_set_network_header(skb, ETH_HLEN + HSR_HLEN);
    }
    skb_reset_mac_len(skb);

    /* Only the frames received over the interlink port will assign a
     * sequence number and require synchronisation vs other sender.
     */
    if (*port).type == HSR_PT_INTERLINK {
        spin_lock_bh(&mut (*hsr).seqnr_lock);
        hsr_forward_skb(skb, port);
        spin_unlock_bh(&mut (*hsr).seqnr_lock);
    } else {
        hsr_forward_skb(skb, port);
    }

    RX_HANDLER_CONSUMED
}

pub unsafe fn hsr_port_exists(dev: *const net_device) -> bool {
    rcu_access_pointer((*dev).rx_handler) == Some(hsr_handle_frame)
}

unsafe fn hsr_check_dev_ok(dev: *mut net_device, extack: *mut netlink_ext_ack) -> i32 {
    /* Don't allow HSR on non-ethernet like devices */
    if ((*dev).flags & IFF_LOOPBACK) != 0 || (*dev).type_ != ARPHRD_ETHER
        || (*dev).addr_len != ETH_ALEN
    {
        NL_SET_ERR_MSG_MOD(extack, "Cannot use loopback or non-ethernet device as HSR slave.");
        return -EINVAL;
    }

    /* Don't allow enslaving hsr devices */
    if is_hsr_master(dev) {
        NL_SET_ERR_MSG_MOD(extack, "Cannot create trees of HSR devices.");
        return -EINVAL;
    }

    if hsr_port_exists(dev) {
        NL_SET_ERR_MSG_MOD(extack, "This device is already a HSR slave.");
        return -EINVAL;
    }

    if is_vlan_dev(dev) {
        NL_SET_ERR_MSG_MOD(extack, "HSR on top of VLAN is not yet supported in this driver.");
        return -EINVAL;
    }

    if ((*dev).priv_flags & IFF_DONT_BRIDGE) != 0 {
        NL_SET_ERR_MSG_MOD(extack, "This device does not support bridging.");
        return -EOPNOTSUPP;
    }

    /* HSR over bonded devices has not been tested, but I'm not sure it
     * won't work...
     */
    0
}

/* Setup device to be added to the HSR bridge. */
unsafe fn hsr_portdev_setup(
    hsr: *mut hsr_priv,
    dev: *mut net_device,
    port: *mut hsr_port,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut lag_upper_info: netdev_lag_upper_info = core::mem::zeroed();
    let hsr_dev: *mut net_device;
    let master: *mut hsr_port;
    let mut res: i32;

    /* Don't use promiscuous mode for offload since L2 frame forward
     * happens at the offloaded hardware.
     */
    if !(*(*port).hsr).fwd_offloaded {
        res = dev_set_promiscuity(dev, 1);
        if res != 0 { return res; }
    }

    master = hsr_port_get_hsr(hsr, HSR_PT_MASTER);
    hsr_dev = (*master).dev;
    (*(&mut lag_upper_info)).tx_type = NETDEV_LAG_TX_TYPE_BROADCAST;
    (*(&mut lag_upper_info)).hash_type = NETDEV_LAG_HASH_UNKNOWN;
    res = netdev_master_upper_dev_link(dev, hsr_dev, core::ptr::null_mut(), &mut lag_upper_info, extack);
    if res != 0 { goto_fail_upper_dev_link(res, port, dev, hsr_dev); }
    res = netdev_rx_handler_register(dev, Some(hsr_handle_frame), port);
    if res != 0 {
        netdev_upper_dev_unlink(dev, hsr_dev);
        goto_fail_upper_dev_link(res, port, dev, hsr_dev);
    }
    dev_disable_lro(dev);
    0
}

unsafe fn goto_fail_upper_dev_link(res: i32, port: *mut hsr_port, dev: *mut net_device, hsr_dev: *mut net_device) -> i32 {
    if !(*(*port).hsr).fwd_offloaded { dev_set_promiscuity(dev, -1); }
    res
}

pub unsafe fn hsr_add_port(hsr: *mut hsr_priv, dev: *mut net_device, type_: hsr_port_type, extack: *mut netlink_ext_ack) -> i32 {
    let mut port: *mut hsr_port;
    let master: *mut hsr_port;
    let mut res: i32;
    if type_ != HSR_PT_MASTER {
        res = hsr_check_dev_ok(dev, extack);
        if res != 0 { return res; }
    }
    port = hsr_port_get_hsr(hsr, type_);
    if !port.is_null() { return -EBUSY; }
    port = kzalloc_obj();
    if port.is_null() { return -ENOMEM; }
    (*port).hsr = hsr;
    (*port).dev = dev;
    (*port).type = type_;
    ether_addr_copy((*port).original_macaddress.as_mut_ptr(), (*dev).dev_addr);
    list_add_tail_rcu(&mut (*port).port_list, &mut (*hsr).ports);
    if type_ != HSR_PT_MASTER {
        res = hsr_portdev_setup(hsr, dev, port, extack);
        if res != 0 { list_del_rcu(&mut (*port).port_list); kfree_rcu(port); return res; }
    }
    master = hsr_port_get_hsr(hsr, HSR_PT_MASTER);
    netdev_update_features((*master).dev);
    dev_set_mtu((*master).dev, hsr_get_max_mtu(hsr));
    0
}

pub unsafe fn hsr_del_port(port: *mut hsr_port) {
    let hsr = (*port).hsr;
    let master = hsr_port_get_hsr(hsr, HSR_PT_MASTER);
    list_del_rcu(&mut (*port).port_list);
    if port != master {
        netdev_update_features((*master).dev);
        dev_set_mtu((*master).dev, hsr_get_max_mtu(hsr));
        netdev_rx_handler_unregister((*port).dev);
        if !(*hsr).fwd_offloaded { dev_set_promiscuity((*port).dev, -1); }
        if (*port).type == HSR_PT_SLAVE_A || (*port).type == HSR_PT_SLAVE_B {
            vlan_vids_del_by_dev((*port).dev, (*master).dev);
        }
        netdev_upper_dev_unlink((*port).dev, (*master).dev);
        if (*hsr).prot_version == PRP_V1 && (*port).type == HSR_PT_SLAVE_B {
            eth_hw_addr_set((*port).dev, (*port).original_macaddress.as_ptr());
            call_netdevice_notifiers(NETDEV_CHANGEADDR, (*port).dev);
        }
    }
    kfree_rcu(port);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
