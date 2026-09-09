// SPDX-License-Identifier: GPL-2.0-only
/*
 * File: pep-gprs.c
 *
 * GPRS over Phonet pipe end point socket
 *
 * Copyright (C) 2008 Nokia Corporation.
 *
 * Author: Rémi Denis-Courmont
 */

// Linux kernel dependencies supplied by other translation units.

const GPRS_DEFAULT_MTU: i32 = 1400;

#[repr(C)]
struct gprs_dev {
    sk: *mut sock,
    old_state_change: Option<unsafe extern "C" fn(*mut sock)>,
    old_data_ready: Option<unsafe extern "C" fn(*mut sock)>,
    old_write_space: Option<unsafe extern "C" fn(*mut sock)>,
    dev: *mut net_device,
}

unsafe extern "C" fn gprs_type_trans(skb: *mut sk_buff) -> __be16 {
    let mut buf: u8 = 0;
    let pvfc: *const u8 = skb_header_pointer(skb, 0, 1, &mut buf as *mut u8 as *mut _);
    if pvfc.is_null() {
        return htons(0);
    }
    // Look at IP version field
    match (*pvfc) >> 4 {
        4 => htons(ETH_P_IP),
        6 => htons(ETH_P_IPV6),
        _ => htons(0),
    }
}

unsafe extern "C" fn gprs_writeable(gp: *mut gprs_dev) {
    let dev = (*gp).dev;
    if pep_writeable((*gp).sk) != 0 {
        netif_wake_queue(dev);
    }
}

/* Socket callbacks */

unsafe extern "C" fn gprs_state_change(sk: *mut sock) {
    let gp = (*sk).sk_user_data as *mut gprs_dev;
    if (*sk).sk_state == TCP_CLOSE_WAIT {
        let dev = (*gp).dev;
        netif_stop_queue(dev);
        netif_carrier_off(dev);
    }
}

unsafe extern "C" fn gprs_recv(gp: *mut gprs_dev, mut skb: *mut sk_buff) -> i32 {
    let dev = (*gp).dev;
    let mut err: i32 = 0;
    let protocol: __be16 = gprs_type_trans(skb);

    if protocol == 0 {
        err = -EINVAL;
        goto_drop!(drop);
    }

    if skb_headroom(skb) & 3 != 0 {
        let rskb: *mut sk_buff;
        let mut fs: *mut sk_buff;
        let mut flen: i32 = 0;

        /* Phonet Pipe data header may be misaligned (3 bytes),
         * so wrap the IP packet as a single fragment of an head-less
         * socket buffer. The network stack will pull what it needs,
         * but at least, the whole IP payload is not memcpy'd. */
        rskb = netdev_alloc_skb(dev, 0);
        if rskb.is_null() {
            err = -ENOBUFS;
            goto_drop!(drop);
        }
        (*skb_shinfo(rskb)).frag_list = skb;
        (*rskb).len += (*skb).len;
        (*rskb).data_len += (*rskb).len;
        (*rskb).truesize += (*rskb).len;

        /* Avoid nested fragments */
        skb_walk_frags!(skb, fs, {
            flen += (*fs).len;
        });
        (*skb).next = (*skb_shinfo(skb)).frag_list;
        skb_frag_list_init(skb);
        (*skb).len -= flen;
        (*skb).data_len -= flen;
        (*skb).truesize -= flen;

        skb = rskb;
    }

    (*skb).protocol = protocol;
    skb_reset_mac_header(skb);
    (*skb).dev = dev;

    if ((*dev).flags & IFF_UP) != 0 {
        (*dev).stats.rx_packets += 1;
        (*dev).stats.rx_bytes += (*skb).len;
        netif_rx(skb);
        skb = core::ptr::null_mut();
    } else {
        err = -ENODEV;
    }

    if !skb.is_null() {
        dev_kfree_skb(skb);
        (*dev).stats.rx_dropped += 1;
    }
    return err;
}

unsafe extern "C" fn gprs_data_ready(sk: *mut sock) {
    let gp = (*sk).sk_user_data as *mut gprs_dev;
    let mut skb: *mut sk_buff;
    trace_sk_data_ready(sk);
    loop {
        skb = pep_read(sk);
        if skb.is_null() {
            break;
        }
        skb_orphan(skb);
        gprs_recv(gp, skb);
    }
}

unsafe extern "C" fn gprs_write_space(sk: *mut sock) {
    let gp = (*sk).sk_user_data as *mut gprs_dev;
    if netif_running((*gp).dev) != 0 {
        gprs_writeable(gp);
    }
}

/* Network device callbacks */

unsafe extern "C" fn gprs_open(dev: *mut net_device) -> i32 {
    let gp = netdev_priv(dev) as *mut gprs_dev;
    gprs_writeable(gp);
    0
}

unsafe extern "C" fn gprs_close(dev: *mut net_device) -> i32 {
    netif_stop_queue(dev);
    0
}

unsafe extern "C" fn gprs_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let gp = netdev_priv(dev) as *mut gprs_dev;
    let sk = (*gp).sk;
    let len: i32;
    let err: i32;

    match (*skb).protocol {
        x if x == htons(ETH_P_IP) || x == htons(ETH_P_IPV6) => (),
        _ => {
            dev_kfree_skb(skb);
            return NETDEV_TX_OK;
        }
    }

    skb_orphan(skb);
    skb_set_owner_w(skb, sk);
    len = (*skb).len;
    err = pep_write(sk, skb);
    if err != 0 {
        net_dbg_ratelimited!("%s: TX error (%d)\n", (*dev).name, err);
        (*dev).stats.tx_aborted_errors += 1;
        (*dev).stats.tx_errors += 1;
    } else {
        (*dev).stats.tx_packets += 1;
        (*dev).stats.tx_bytes += len;
    }

    netif_stop_queue(dev);
    if pep_writeable(sk) != 0 {
        netif_wake_queue(dev);
    }
    NETDEV_TX_OK
}

static const gprs_netdev_ops: net_device_ops = net_device_ops {
    ndo_open: Some(gprs_open),
    ndo_stop: Some(gprs_close),
    ndo_start_xmit: Some(gprs_xmit),
};

unsafe extern "C" fn gprs_setup(dev: *mut net_device) {
    (*dev).features = NETIF_F_FRAGLIST;
    (*dev).type_ = ARPHRD_PHONET_PIPE;
    (*dev).flags = IFF_POINTOPOINT | IFF_NOARP;
    (*dev).mtu = GPRS_DEFAULT_MTU;
    (*dev).min_mtu = 576;
    (*dev).max_mtu = PHONET_MAX_MTU - 11;
    (*dev).hard_header_len = 0;
    (*dev).addr_len = 0;
    (*dev).tx_queue_len = 10;
    (*dev).netdev_ops = &gprs_netdev_ops;
    (*dev).needs_free_netdev = true;
}

/* External interface */

/*
 * Attach a GPRS interface to a datagram socket.
 * Returns the interface index on success, negative error code on error.
 */
unsafe extern "C" fn gprs_attach(sk: *mut sock) -> i32 {
    static IFNAME: &[u8] = b"gprs%d\0";
    let gp: *mut gprs_dev;
    let dev: *mut net_device;
    let err: i32;

    if (*sk).sk_type == SOCK_STREAM {
        return -EINVAL; // need packet boundaries
    }

    dev = alloc_netdev(core::mem::size_of::<gprs_dev>(), IFNAME.as_ptr() as *const i8,
                       NET_NAME_UNKNOWN, gprs_setup);
    if dev.is_null() {
        return -ENOMEM;
    }
    gp = netdev_priv(dev) as *mut gprs_dev;
    (*gp).sk = sk;
    (*gp).dev = dev;

    netif_stop_queue(dev);
    err = register_netdev(dev);
    if err != 0 {
        free_netdev(dev);
        return err;
    }

    lock_sock(sk);
    if !(*sk).sk_user_data.is_null() {
        err = -EBUSY;
        goto_out_rel!(out_rel);
    }
    if (((1u32 << (*sk).sk_state) & (TCPF_CLOSE | TCPF_LISTEN)) != 0)
        || sock_flag(sk, SOCK_DEAD) != 0
    {
        err = -EINVAL;
        goto_out_rel!(out_rel);
    }
    (*sk).sk_user_data = gp as *mut _;
    (*gp).old_state_change = (*sk).sk_state_change;
    (*gp).old_data_ready = (*sk).sk_data_ready;
    (*gp).old_write_space = (*sk).sk_write_space;
    (*sk).sk_state_change = Some(gprs_state_change);
    (*sk).sk_data_ready = Some(gprs_data_ready);
    (*sk).sk_write_space = Some(gprs_write_space);
    release_sock(sk);
    sock_hold(sk);
    printk!(KERN_DEBUG "%s: attached\n", (*dev).name);
    (*dev).ifindex
}

unsafe extern "C" fn gprs_detach(sk: *mut sock) {
    let gp = (*sk).sk_user_data as *mut gprs_dev;
    let dev = (*gp).dev;
    lock_sock(sk);
    (*sk).sk_user_data = core::ptr::null_mut();
    (*sk).sk_state_change = (*gp).old_state_change;
    (*sk).sk_data_ready = (*gp).old_data_ready;
    (*sk).sk_write_space = (*gp).old_write_space;
    release_sock(sk);
    printk!(KERN_DEBUG "%s: detached\n", (*dev).name);
    unregister_netdev(dev);
    sock_put(sk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
