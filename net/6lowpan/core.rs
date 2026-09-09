// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Authors:
 * (C) 2015 Pengutronix, Alexander Aring <aar@pengutronix.de>
 */

// Dependencies supplied by the Linux networking headers and other modules.

pub unsafe fn lowpan_register_netdevice(
    dev: *mut net_device,
    lltype: lowpan_lltypes,
) -> i32 {
    let mut ret: i32;

    match lltype {
        LOWPAN_LLTYPE_IEEE802154 => {
            (*dev).addr_len = EUI64_ADDR_LEN;
        }
        LOWPAN_LLTYPE_BTLE => {
            (*dev).addr_len = ETH_ALEN;
        }
    }

    (*dev).type_ = ARPHRD_6LOWPAN;
    (*dev).mtu = IPV6_MIN_MTU;

    (*lowpan_dev(dev)).lltype = lltype;

    spin_lock_init(&mut (*lowpan_dev(dev)).ctx.lock);
    for i in 0..LOWPAN_IPHC_CTX_TABLE_SIZE {
        (*lowpan_dev(dev)).ctx.table[i].id = i;
    }

    (*dev).ndisc_ops = &lowpan_ndisc_ops;

    ret = register_netdevice(dev);
    if ret < 0 {
        return ret;
    }

    lowpan_dev_debugfs_init(dev);

    ret
}

// EXPORT_SYMBOL(lowpan_register_netdevice)

pub unsafe fn lowpan_register_netdev(dev: *mut net_device, lltype: lowpan_lltypes) -> i32 {
    let ret: i32;

    rtnl_lock();
    ret = lowpan_register_netdevice(dev, lltype);
    rtnl_unlock();
    ret
}

// EXPORT_SYMBOL(lowpan_register_netdev)

pub unsafe fn lowpan_unregister_netdevice(dev: *mut net_device) {
    unregister_netdevice(dev);
    lowpan_dev_debugfs_exit(dev);
}

// EXPORT_SYMBOL(lowpan_unregister_netdevice)

pub unsafe fn lowpan_unregister_netdev(dev: *mut net_device) {
    rtnl_lock();
    lowpan_unregister_netdevice(dev);
    rtnl_unlock();
}

// EXPORT_SYMBOL(lowpan_unregister_netdev)

pub unsafe fn addrconf_ifid_802154_6lowpan(eui: *mut u8, dev: *mut net_device) -> i32 {
    let wpan_dev: *mut wpan_dev = (*(*lowpan_802154_dev(dev)).wdev).ieee802154_ptr;

    /* Set short_addr autoconfiguration if short_addr is present only */
    if !lowpan_802154_is_valid_src_short_addr((*wpan_dev).short_addr) {
        return -1;
    }

    /* For either address format, all zero addresses MUST NOT be used */
    if (*wpan_dev).pan_id == cpu_to_le16(0x0000) &&
       (*wpan_dev).short_addr == cpu_to_le16(0x0000) {
        return -1;
    }

    /* Alternatively, if no PAN ID is known, 16 zero bits may be used */
    if (*wpan_dev).pan_id == cpu_to_le16(IEEE802154_PAN_ID_BROADCAST) {
        memset(eui, 0, 2);
    } else {
        ieee802154_le16_to_be16(eui, &(*wpan_dev).pan_id);
    }

    /* The "Universal/Local" (U/L) bit shall be set to zero */
    *eui.add(0) &= !2;
    *eui.add(2) = 0;
    *eui.add(3) = 0xFF;
    *eui.add(4) = 0xFE;
    *eui.add(5) = 0;
    ieee802154_le16_to_be16(eui.add(6), &(*wpan_dev).short_addr);
    0
}

unsafe fn lowpan_event(
    _unused: *mut notifier_block,
    event: c_ulong,
    ptr: *mut c_void,
) -> i32 {
    let dev: *mut net_device = netdev_notifier_info_to_dev(ptr);
    let mut idev: *mut inet6_dev;
    let mut addr: in6_addr = core::mem::zeroed();
    let mut i: i32;

    if (*dev).type_ != ARPHRD_6LOWPAN {
        return NOTIFY_DONE;
    }

    idev = __in6_dev_get(dev);
    if idev.is_null() {
        return NOTIFY_DONE;
    }

    match event {
        NETDEV_UP | NETDEV_CHANGE => {
            /* (802.15.4 6LoWPAN short address slaac handling */
            if lowpan_is_ll(dev, LOWPAN_LLTYPE_IEEE802154) &&
               addrconf_ifid_802154_6lowpan(addr.s6_addr.as_mut_ptr().add(8), dev) == 0 {
                __ipv6_addr_set_half(
                    &mut addr.s6_addr32[0],
                    htonl(0xFE800000),
                    0,
                );
                addrconf_add_linklocal(idev, &mut addr, 0);
            }
        }
        NETDEV_DOWN => {
            i = 0;
            while i < LOWPAN_IPHC_CTX_TABLE_SIZE {
                clear_bit(
                    LOWPAN_IPHC_CTX_FLAG_ACTIVE,
                    &mut (*lowpan_dev(dev)).ctx.table[i as usize].flags,
                );
                i += 1;
            }
        }
        _ => return NOTIFY_DONE,
    }

    NOTIFY_OK
}

static mut lowpan_notifier: notifier_block = notifier_block {
    notifier_call: Some(lowpan_event),
};

unsafe fn lowpan_module_init() -> i32 {
    let ret: i32;

    lowpan_debugfs_init();

    ret = register_netdevice_notifier(&mut lowpan_notifier);
    if ret < 0 {
        lowpan_debugfs_exit();
        return ret;
    }

    request_module_nowait("nhc_dest");
    request_module_nowait("nhc_fragment");
    request_module_nowait("nhc_hop");
    request_module_nowait("nhc_ipv6");
    request_module_nowait("nhc_mobility");
    request_module_nowait("nhc_routing");
    request_module_nowait("nhc_udp");

    0
}

unsafe fn lowpan_module_exit() {
    lowpan_debugfs_exit();
    unregister_netdevice_notifier(&mut lowpan_notifier);
}

// module_init(lowpan_module_init)
// module_exit(lowpan_module_exit)
// MODULE_DESCRIPTION("IPv6 over Low-Power Wireless Personal Area Network core module")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
