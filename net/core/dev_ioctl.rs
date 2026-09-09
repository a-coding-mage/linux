// SPDX-License-Identifier: GPL-2.0
// Dependencies are supplied by the surrounding kernel translation.

/*
 * Map an interface index to its name (SIOCGIFNAME)
 */

/*
 * We need this ioctl for efficient implementation of the
 * if_indextoname() function required by the IPv6 API. Without
 * it, we would have to search all the interfaces to find a
 * match. --pb
 */
unsafe fn dev_ifname(net: *mut net, ifr: *mut ifreq) -> i32 {
    (*ifr).ifr_name[IFNAMSIZ - 1] = 0;
    netdev_get_name(net, (*ifr).ifr_name.as_mut_ptr(), (*ifr).ifr_ifindex)
}

/*
 * Perform a SIOCGIFCONF call. This structure will change
 * size eventually, and there is nothing I can do about it.
 * Thus we will need a 'compatibility mode'.
 */
pub unsafe fn dev_ifconf(net: *mut net, uifc: *mut ifconf) -> i32 {
    let mut pos: *mut core::ffi::c_void;
    let size: usize;
    let len: i32;
    let mut total: i32 = 0;
    let mut done: i32;

    /* both the ifconf and the ifreq structures are slightly different */
    if in_compat_syscall() {
        let mut ifc32: compat_ifconf = core::mem::zeroed();
        if copy_from_user(&mut ifc32 as *mut _ as *mut _, uifc as *const _, core::mem::size_of::<compat_ifconf>()) != 0 {
            return -EFAULT;
        }
        pos = compat_ptr(ifc32.ifcbuf);
        len = ifc32.ifc_len;
        size = core::mem::size_of::<compat_ifreq>();
    } else {
        let mut ifc: ifconf = core::mem::zeroed();
        if copy_from_user(&mut ifc as *mut _ as *mut _, uifc as *const _, core::mem::size_of::<ifconf>()) != 0 {
            return -EFAULT;
        }
        pos = ifc.ifc_buf;
        len = ifc.ifc_len;
        size = core::mem::size_of::<ifreq>();
    }

    /* Loop over the interfaces, and write an info block for each. */
    rtnl_net_lock(net);
    for_each_netdev!(net, dev, {
        if !pos.is_null() {
            done = inet_gifconf(dev, pos.add(total as usize), len - total, size);
        } else {
            done = inet_gifconf(dev, core::ptr::null_mut(), 0, size);
        }
        if done < 0 {
            rtnl_net_unlock(net);
            return -EFAULT;
        }
        total += done;
    });
    rtnl_net_unlock(net);

    put_user(total, &mut (*uifc).ifc_len)
}

unsafe fn dev_getifmap(dev: *mut net_device, ifr: *mut ifreq) -> i32 {
    let ifmap: *mut ifmap = &mut (*ifr).ifr_map;
    if in_compat_syscall() {
        let cifmap = ifmap as *mut compat_ifmap;
        (*cifmap).mem_start = (*dev).mem_start;
        (*cifmap).mem_end = (*dev).mem_end;
        (*cifmap).base_addr = (*dev).base_addr;
        (*cifmap).irq = (*dev).irq;
        (*cifmap).dma = (*dev).dma;
        (*cifmap).port = (*dev).if_port;
        return 0;
    }
    (*ifmap).mem_start = (*dev).mem_start;
    (*ifmap).mem_end = (*dev).mem_end;
    (*ifmap).base_addr = (*dev).base_addr;
    (*ifmap).irq = (*dev).irq;
    (*ifmap).dma = (*dev).dma;
    (*ifmap).port = (*dev).if_port;
    0
}

unsafe fn netif_setifmap(dev: *mut net_device, ifr: *mut ifreq) -> i32 {
    let cifmap = &(*ifr).ifr_map as *const ifmap as *const compat_ifmap;
    if (*(*dev).netdev_ops).ndo_set_config.is_none() { return -EOPNOTSUPP; }
    if in_compat_syscall() {
        let ifmap = ifmap { mem_start: (*cifmap).mem_start, mem_end: (*cifmap).mem_end, base_addr: (*cifmap).base_addr, irq: (*cifmap).irq, dma: (*cifmap).dma, port: (*cifmap).port };
        return ((*(*dev).netdev_ops).ndo_set_config.unwrap())(dev, &ifmap);
    }
    ((*(*dev).netdev_ops).ndo_set_config.unwrap())(dev, &(*ifr).ifr_map)
}

/* Perform the SIOCxIFxxx calls, inside rcu_read_lock() */
unsafe fn dev_ifsioc_locked(net: *mut net, ifr: *mut ifreq, cmd: u32) -> i32 {
    let dev = dev_get_by_name_rcu(net, (*ifr).ifr_name.as_ptr());
    if dev.is_null() { return -ENODEV; }
    match cmd {
        SIOCGIFFLAGS => { (*ifr).ifr_flags = netif_get_flags(dev) as i16; 0 }
        SIOCGIFMETRIC => { (*ifr).ifr_metric = 0; 0 }
        SIOCGIFMTU => { (*ifr).ifr_mtu = (*dev).mtu; 0 }
        SIOCGIFSLAVE => -EINVAL,
        SIOCGIFMAP => dev_getifmap(dev, ifr),
        SIOCGIFINDEX => { (*ifr).ifr_ifindex = (*dev).ifindex; 0 }
        SIOCGIFTXQLEN => { (*ifr).ifr_qlen = (*dev).tx_queue_len; 0 }
        _ => { WARN_ON!(true); -ENOTTY }
    }
}

pub unsafe fn net_hwtstamp_validate(cfg: *const kernel_hwtstamp_config) -> i32 {
    let tx_type = (*cfg).tx_type;
    let rx_filter = (*cfg).rx_filter;
    let mut tx_type_valid = false;
    let mut rx_filter_valid = false;
    if (*cfg).flags & !HWTSTAMP_FLAG_MASK != 0 { return -EINVAL; }
    match tx_type { HWTSTAMP_TX_OFF | HWTSTAMP_TX_ON | HWTSTAMP_TX_ONESTEP_SYNC | HWTSTAMP_TX_ONESTEP_P2P => tx_type_valid = true, __HWTSTAMP_TX_CNT => {} _ => {} }
    match rx_filter {
        HWTSTAMP_FILTER_NONE | HWTSTAMP_FILTER_ALL | HWTSTAMP_FILTER_SOME |
        HWTSTAMP_FILTER_PTP_V1_L4_EVENT | HWTSTAMP_FILTER_PTP_V1_L4_SYNC |
        HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ | HWTSTAMP_FILTER_PTP_V2_L4_EVENT |
        HWTSTAMP_FILTER_PTP_V2_L4_SYNC | HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ |
        HWTSTAMP_FILTER_PTP_V2_L2_EVENT | HWTSTAMP_FILTER_PTP_V2_L2_SYNC |
        HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ | HWTSTAMP_FILTER_PTP_V2_EVENT |
        HWTSTAMP_FILTER_PTP_V2_SYNC | HWTSTAMP_FILTER_PTP_V2_DELAY_REQ |
        HWTSTAMP_FILTER_NTP_ALL => rx_filter_valid = true,
        __HWTSTAMP_FILTER_CNT => {}, _ => {}
    }
    if !tx_type_valid || !rx_filter_valid { return -ERANGE; }
    0
}

// The remaining helpers are translated with their original API and ordering.
pub unsafe fn dev_get_hwtstamp_phylib(dev: *mut net_device, cfg: *mut kernel_hwtstamp_config) -> i32 {
    let hwprov = netdev_ops_lock_dereference((*dev).hwprov, dev);
    if !hwprov.is_null() {
        (*cfg).qualifier = (*hwprov).desc.qualifier;
        if (*hwprov).source == HWTSTAMP_SOURCE_PHYLIB && !(*hwprov).phydev.is_null() { return phy_hwtstamp_get((*hwprov).phydev, cfg); }
        if (*hwprov).source == HWTSTAMP_SOURCE_NETDEV { return ((*(*dev).netdev_ops).ndo_hwtstamp_get.unwrap())(dev, cfg); }
        return -EOPNOTSUPP;
    }
    if phy_is_default_hwtstamp((*dev).phydev) { return phy_hwtstamp_get((*dev).phydev, cfg); }
    ((*(*dev).netdev_ops).ndo_hwtstamp_get.unwrap())(dev, cfg)
}

unsafe fn dev_get_hwtstamp(dev: *mut net_device, ifr: *mut ifreq) -> i32 {
    let ops = (*dev).netdev_ops;
    let mut kernel_cfg: kernel_hwtstamp_config = core::mem::zeroed();
    let mut cfg: hwtstamp_config = core::mem::zeroed();
    if (*ops).ndo_hwtstamp_get.is_none() { return -EOPNOTSUPP; }
    if !netif_device_present(dev) { return -ENODEV; }
    kernel_cfg.ifr = ifr;
    netdev_lock_ops(dev);
    let err = dev_get_hwtstamp_phylib(dev, &mut kernel_cfg);
    netdev_unlock_ops(dev);
    if err != 0 { return err; }
    if !kernel_cfg.copied_to_user {
        hwtstamp_config_from_kernel(&mut cfg, &kernel_cfg);
        if copy_to_user((*ifr).ifr_data, &cfg as *const _ as *const _, core::mem::size_of::<hwtstamp_config>()) != 0 { return -EFAULT; }
    }
    0
}

unsafe fn dev_set_hwtstamp_phylib(dev: *mut net_device, cfg: *mut kernel_hwtstamp_config, extack: *mut netlink_ext_ack) -> i32 {
    let ops = (*dev).netdev_ops;
    let mut old_cfg: kernel_hwtstamp_config = core::mem::zeroed();
    let hwprov = netdev_ops_lock_dereference((*dev).hwprov, dev);
    let mut phydev: *mut phy_device = core::ptr::null_mut();
    let mut phy_ts = false;
    let mut changed = false;
    if !hwprov.is_null() {
        if (*hwprov).source == HWTSTAMP_SOURCE_PHYLIB && !(*hwprov).phydev.is_null() { phy_ts = true; phydev = (*hwprov).phydev; }
        else if (*hwprov).source != HWTSTAMP_SOURCE_NETDEV { return -EOPNOTSUPP; }
        (*cfg).qualifier = (*hwprov).desc.qualifier;
    } else { phy_ts = phy_is_default_hwtstamp((*dev).phydev); if phy_ts { phydev = (*dev).phydev; } }
    (*cfg).source = if phy_ts { HWTSTAMP_SOURCE_PHYLIB } else { HWTSTAMP_SOURCE_NETDEV };
    if phy_ts && (*dev).see_all_hwtstamp_requests { let e = ((*ops).ndo_hwtstamp_get.unwrap())(dev, &mut old_cfg); if e != 0 { return e; } }
    if !phy_ts || (*dev).see_all_hwtstamp_requests {
        let e = ((*ops).ndo_hwtstamp_set.unwrap())(dev, cfg, extack);
        if e != 0 { if !(*extack)._msg.is_null() { netdev_err!(dev, "%s\\n", (*extack)._msg); } return e; }
    }
    if phy_ts && (*dev).see_all_hwtstamp_requests { changed = kernel_hwtstamp_config_changed(&old_cfg, cfg); }
    if phy_ts { let e = phy_hwtstamp_set(phydev, cfg, extack); if e != 0 { if changed { ((*ops).ndo_hwtstamp_set.unwrap())(dev, &mut old_cfg, core::ptr::null_mut()); } return e; } }
    0
}

unsafe fn dev_set_hwtstamp(dev: *mut net_device, ifr: *mut ifreq) -> i32 {
    let mut cfg: hwtstamp_config = core::mem::zeroed();
    let mut kernel_cfg: kernel_hwtstamp_config = core::mem::zeroed();
    let mut extack: netlink_ext_ack = core::mem::zeroed();
    if copy_from_user(&mut cfg as *mut _ as *mut _, (*ifr).ifr_data, core::mem::size_of::<hwtstamp_config>()) != 0 { return -EFAULT; }
    hwtstamp_config_to_kernel(&mut kernel_cfg, &cfg); kernel_cfg.ifr = ifr;
    let mut err = net_hwtstamp_validate(&kernel_cfg); if err != 0 { return err; }
    err = dsa_conduit_hwtstamp_validate(dev, &mut kernel_cfg, &mut extack); if err != 0 { return err; }
    if (*(*dev).netdev_ops).ndo_hwtstamp_set.is_none() { return -EOPNOTSUPP; }
    if !netif_device_present(dev) { return -ENODEV; }
    netdev_lock_ops(dev); err = dev_set_hwtstamp_phylib(dev, &mut kernel_cfg, &mut extack); netdev_unlock_ops(dev);
    if err != 0 { return err; }
    if !kernel_cfg.copied_to_user { hwtstamp_config_from_kernel(&mut cfg, &kernel_cfg); if copy_to_user((*ifr).ifr_data, &cfg as *const _ as *const _, core::mem::size_of::<hwtstamp_config>()) != 0 { return -EFAULT; } }
    0
}

pub unsafe fn generic_hwtstamp_get_lower(dev: *mut net_device, cfg: *mut kernel_hwtstamp_config) -> i32 {
    if !netif_device_present(dev) { return -ENODEV; }
    if (*(*dev).netdev_ops).ndo_hwtstamp_get.is_none() { return -EOPNOTSUPP; }
    netdev_lock_ops(dev); let err = dev_get_hwtstamp_phylib(dev, cfg); netdev_unlock_ops(dev); err
}

pub unsafe fn generic_hwtstamp_set_lower(dev: *mut net_device, cfg: *mut kernel_hwtstamp_config, extack: *mut netlink_ext_ack) -> i32 {
    if !netif_device_present(dev) { return -ENODEV; }
    if (*(*dev).netdev_ops).ndo_hwtstamp_set.is_none() { return -EOPNOTSUPP; }
    netdev_lock_ops(dev); let err = dev_set_hwtstamp_phylib(dev, cfg, extack); netdev_unlock_ops(dev); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
