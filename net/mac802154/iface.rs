// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2007-2012 Siemens AG
 *
 * Written by:
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 * Sergey Lapin <slapin@ossfans.org>
 * Maxim Gorbachyov <maxim.gorbachev@siemens.com>
 * Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

// Linux kernel and local header dependencies are supplied by the surrounding translation.

pub unsafe fn mac802154_wpan_update_llsec(dev: *mut net_device) -> c_int {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let ops = ieee802154_mlme_ops(dev);
    let wpan_dev = &mut (*sdata).wpan_dev;
    let mut rc = 0;

    if !(*ops).llsec.is_null() {
        let mut params: ieee802154_llsec_params = core::mem::zeroed();
        let mut changed = 0;
        params.pan_id = wpan_dev.pan_id;
        changed |= IEEE802154_LLSEC_PARAM_PAN_ID;
        params.hwaddr = wpan_dev.extended_addr;
        changed |= IEEE802154_LLSEC_PARAM_HWADDR;
        rc = ((*(*ops).llsec).set_params)(dev, &mut params, changed);
    }
    rc
}

unsafe fn mac802154_wpan_ioctl(dev: *mut net_device, ifr: *mut ifreq, cmd: c_int) -> c_int {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let wpan_dev = &mut (*sdata).wpan_dev;
    let sa = &mut (*ifr).ifr_addr as *mut _ as *mut sockaddr_ieee802154;
    let mut err = -ENOIOCTLCMD;
    if cmd != SIOCGIFADDR && cmd != SIOCSIFADDR { return err; }
    rtnl_lock();
    match cmd {
        SIOCGIFADDR => {
            let pan_id = le16_to_cpu(wpan_dev.pan_id);
            let short_addr = le16_to_cpu(wpan_dev.short_addr);
            if pan_id == IEEE802154_PANID_BROADCAST || short_addr == IEEE802154_ADDR_BROADCAST {
                err = -EADDRNOTAVAIL;
            } else {
                (*sa).family = AF_IEEE802154;
                (*sa).addr.addr_type = IEEE802154_ADDR_SHORT;
                (*sa).addr.pan_id = pan_id;
                (*sa).addr.short_addr = short_addr;
                err = 0;
            }
        }
        SIOCSIFADDR => {
            if netif_running(dev) { rtnl_unlock(); return -EBUSY; }
            dev_warn!(&(*dev).dev, "Using DEBUGing ioctl SIOCSIFADDR isn't recommended!\n");
            if (*sa).family != AF_IEEE802154 || (*sa).addr.addr_type != IEEE802154_ADDR_SHORT ||
                (*sa).addr.pan_id == IEEE802154_PANID_BROADCAST ||
                (*sa).addr.short_addr == IEEE802154_ADDR_BROADCAST ||
                (*sa).addr.short_addr == IEEE802154_ADDR_UNDEF {
                err = -EINVAL;
            } else {
                wpan_dev.pan_id = cpu_to_le16((*sa).addr.pan_id);
                wpan_dev.short_addr = cpu_to_le16((*sa).addr.short_addr);
                err = mac802154_wpan_update_llsec(dev);
            }
        }
        _ => {}
    }
    rtnl_unlock();
    err
}

unsafe fn mac802154_wpan_mac_addr(dev: *mut net_device, p: *mut c_void) -> c_int {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let addr = p as *mut sockaddr;
    let mut extended_addr: __le64 = 0;
    if netif_running(dev) { return -EBUSY; }
    if !(*sdata).wpan_dev.lowpan_dev.is_null() && netif_running((*sdata).wpan_dev.lowpan_dev) { return -EBUSY; }
    ieee802154_be64_to_le64(&mut extended_addr, (*addr).sa_data.as_ptr());
    if !ieee802154_is_valid_extended_unicast_addr(extended_addr) { return -EINVAL; }
    dev_addr_set(dev, (*addr).sa_data.as_ptr());
    (*sdata).wpan_dev.extended_addr = extended_addr;
    if !(*sdata).wpan_dev.lowpan_dev.is_null() { dev_addr_set((*sdata).wpan_dev.lowpan_dev, (*dev).dev_addr.as_ptr()); }
    mac802154_wpan_update_llsec(dev)
}

unsafe fn ieee802154_setup_hw(sdata: *mut ieee802154_sub_if_data) -> c_int {
    let local = (*sdata).local;
    let wpan_dev = &mut (*sdata).wpan_dev;
    let mut ret;
    (*sdata).required_filtering = (*sdata).iface_default_filtering;
    if (*local).hw.flags & IEEE802154_HW_AFILT != 0 {
        (*local).addr_filt.pan_id = wpan_dev.pan_id;
        (*local).addr_filt.ieee_addr = wpan_dev.extended_addr;
        (*local).addr_filt.short_addr = wpan_dev.short_addr;
    }
    if (*local).hw.flags & IEEE802154_HW_LBT != 0 { ret = drv_set_lbt_mode(local, wpan_dev.lbt); if ret < 0 { return ret; } }
    if (*local).hw.flags & IEEE802154_HW_CSMA_PARAMS != 0 { ret = drv_set_csma_params(local, wpan_dev.min_be, wpan_dev.max_be, wpan_dev.csma_retries); if ret < 0 { return ret; } }
    if (*local).hw.flags & IEEE802154_HW_FRAME_RETRIES != 0 { ret = drv_set_max_frame_retries(local, wpan_dev.frame_retries); if ret < 0 { return ret; } }
    0
}

unsafe fn mac802154_slave_open(dev: *mut net_device) -> c_int {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let local = (*sdata).local;
    let mut res;
    ASSERT_RTNL!();
    set_bit(SDATA_STATE_RUNNING, &mut (*sdata).state);
    if (*local).open_count == 0 {
        res = ieee802154_setup_hw(sdata); if res != 0 { clear_bit(SDATA_STATE_RUNNING, &mut (*sdata).state); return res; }
        res = drv_start(local, (*sdata).required_filtering, &mut (*local).addr_filt); if res != 0 { clear_bit(SDATA_STATE_RUNNING, &mut (*sdata).state); return res; }
    }
    (*local).open_count += 1;
    netif_start_queue(dev);
    0
}

unsafe fn ieee802154_check_mac_settings(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, nsdata: *mut ieee802154_sub_if_data) -> c_int {
    let nwpan_dev = &(*nsdata).wpan_dev;
    let wpan_dev = &(*sdata).wpan_dev;
    ASSERT_RTNL!();
    if (*sdata).iface_default_filtering != (*nsdata).iface_default_filtering { return -EBUSY; }
    if (*local).hw.flags & IEEE802154_HW_AFILT != 0 && (wpan_dev.pan_id != nwpan_dev.pan_id || wpan_dev.short_addr != nwpan_dev.short_addr || wpan_dev.extended_addr != nwpan_dev.extended_addr) { return -EBUSY; }
    if (*local).hw.flags & IEEE802154_HW_CSMA_PARAMS != 0 && (wpan_dev.min_be != nwpan_dev.min_be || wpan_dev.max_be != nwpan_dev.max_be || wpan_dev.csma_retries != nwpan_dev.csma_retries) { return -EBUSY; }
    if (*local).hw.flags & IEEE802154_HW_FRAME_RETRIES != 0 && wpan_dev.frame_retries != nwpan_dev.frame_retries { return -EBUSY; }
    if (*local).hw.flags & IEEE802154_HW_LBT != 0 && wpan_dev.lbt != nwpan_dev.lbt { return -EBUSY; }
    0
}

unsafe fn ieee802154_check_concurrent_iface(sdata: *mut ieee802154_sub_if_data, _iftype: nl802154_iftype) -> c_int {
    let local = (*sdata).local;
    list_for_each_entry!(nsdata, &(*local).interfaces, list, {
        if nsdata != sdata && ieee802154_sdata_running(nsdata) {
            if (*sdata).wpan_dev.iftype != NL802154_IFTYPE_MONITOR && (*nsdata).wpan_dev.iftype != NL802154_IFTYPE_MONITOR { return -EBUSY; }
            let ret = ieee802154_check_mac_settings(local, sdata, nsdata); if ret < 0 { return ret; }
        }
    });
    0
}

unsafe fn mac802154_wpan_open(dev: *mut net_device) -> c_int {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let rc = ieee802154_check_concurrent_iface(sdata, (*sdata).wpan_dev.iftype);
    if rc < 0 { return rc; }
    mac802154_slave_open(dev)
}

unsafe fn mac802154_slave_close(dev: *mut net_device) -> c_int {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev); let local = (*sdata).local;
    ASSERT_RTNL!();
    if mac802154_is_scanning(local) { mac802154_abort_scan_locked(local, sdata); }
    if mac802154_is_beaconing(local) { mac802154_stop_beacons_locked(local, sdata); }
    netif_stop_queue(dev); (*local).open_count -= 1; clear_bit(SDATA_STATE_RUNNING, &mut (*sdata).state);
    if (*local).open_count == 0 { ieee802154_stop_device(local); }
    0
}

unsafe fn mac802154_set_header_security(sdata: *mut ieee802154_sub_if_data, hdr: *mut ieee802154_hdr, cb: *const ieee802154_mac_cb) -> c_int {
    let mut params: ieee802154_llsec_params = core::mem::zeroed(); let mut level;
    mac802154_llsec_get_params(&(*sdata).sec, &mut params);
    if !params.enabled && (*cb).secen_override && (*cb).secen { return -EINVAL; }
    if !params.enabled || ((*cb).secen_override && !(*cb).secen) || !params.out_level { return 0; }
    if (*cb).seclevel_override && !(*cb).seclevel { return -EINVAL; }
    level = if (*cb).seclevel_override { (*cb).seclevel } else { params.out_level };
    (*hdr).fc.security_enabled = 1; (*hdr).sec.level = level; (*hdr).sec.key_id_mode = params.out_key.mode;
    if params.out_key.mode == IEEE802154_SCF_KEY_SHORT_INDEX { (*hdr).sec.short_src = params.out_key.short_source; } else if params.out_key.mode == IEEE802154_SCF_KEY_HW_INDEX { (*hdr).sec.extended_src = params.out_key.extended_source; }
    (*hdr).sec.key_id = params.out_key.id; 0
}

unsafe fn ieee802154_header_create(skb: *mut sk_buff, dev: *mut net_device, daddr: *const ieee802154_addr, saddr: *const ieee802154_addr, len: c_uint) -> c_int {
    if daddr.is_null() { return -EINVAL; }
    let mut hdr: ieee802154_hdr = core::mem::zeroed(); let sdata = IEEE802154_DEV_TO_SUB_IF(dev); let wpan_dev = &(*sdata).wpan_dev; let cb = mac_cb(skb);
    hdr.fc.type_ = (*cb).type_; hdr.fc.security_enabled = (*cb).secen; hdr.fc.ack_request = (*cb).ackreq; hdr.seq = (atomic_inc_return(&mut (*(*dev).ieee802154_ptr).dsn) & 0xff) as _;
    if mac802154_set_header_security(sdata, &mut hdr, cb) < 0 { return -EINVAL; }
    if saddr.is_null() {
        if wpan_dev.short_addr == cpu_to_le16(IEEE802154_ADDR_BROADCAST) || wpan_dev.short_addr == cpu_to_le16(IEEE802154_ADDR_UNDEF) || wpan_dev.pan_id == cpu_to_le16(IEEE802154_PANID_BROADCAST) { hdr.source.mode = IEEE802154_ADDR_LONG; hdr.source.extended_addr = wpan_dev.extended_addr; } else { hdr.source.mode = IEEE802154_ADDR_SHORT; hdr.source.short_addr = wpan_dev.short_addr; }
        hdr.source.pan_id = wpan_dev.pan_id;
    } else { hdr.source = *saddr; }
    hdr.dest = *daddr;
    let hlen = ieee802154_hdr_push(skb, &mut hdr); if hlen < 0 { return -EINVAL; }
    skb_reset_mac_header(skb); (*skb).mac_len = hlen as _;
    if len > ieee802154_max_payload(&hdr) { return -EMSGSIZE; } hlen
}

static ieee802154_header_ops: wpan_dev_header_ops = wpan_dev_header_ops { create: Some(ieee802154_header_create) };

unsafe fn mac802154_header_create(skb: *mut sk_buff, dev: *mut net_device, _type: c_ushort, daddr: *const c_void, saddr: *const c_void, len: c_uint) -> c_int {
    if daddr.is_null() { return -EINVAL; }
    let mut hdr: ieee802154_hdr = core::mem::zeroed(); let sdata = IEEE802154_DEV_TO_SUB_IF(dev); let wpan_dev = &(*sdata).wpan_dev; let cb: ieee802154_mac_cb = core::mem::zeroed();
    hdr.fc.type_ = IEEE802154_FC_TYPE_DATA; hdr.fc.ack_request = wpan_dev.ackreq; hdr.seq = (atomic_inc_return(&mut (*(*dev).ieee802154_ptr).dsn) & 0xff) as _;
    if mac802154_set_header_security(sdata, &mut hdr, &cb) < 0 { return -EINVAL; }
    hdr.dest.pan_id = wpan_dev.pan_id; hdr.dest.mode = IEEE802154_ADDR_LONG; ieee802154_be64_to_le64(&mut hdr.dest.extended_addr, daddr as *const _);
    hdr.source.pan_id = hdr.dest.pan_id; hdr.source.mode = IEEE802154_ADDR_LONG;
    if saddr.is_null() { hdr.source.extended_addr = wpan_dev.extended_addr; } else { ieee802154_be64_to_le64(&mut hdr.source.extended_addr, saddr as *const _); }
    let hlen = ieee802154_hdr_push(skb, &mut hdr); if hlen < 0 { return -EINVAL; }
    skb_reset_mac_header(skb); (*skb).mac_len = hlen as _; if len > ieee802154_max_payload(&hdr) { return -EMSGSIZE; } hlen
}

unsafe fn mac802154_header_parse(skb: *const sk_buff, _dev: *const net_device, haddr: *mut u8) -> c_int {
    let mut hdr: ieee802154_hdr = core::mem::zeroed();
    if ieee802154_hdr_peek_addrs(skb, &mut hdr) < 0 { pr_debug!("malformed packet\n"); return 0; }
    if hdr.source.mode == IEEE802154_ADDR_LONG { ieee802154_le64_to_be64(haddr, &hdr.source.extended_addr); return IEEE802154_EXTENDED_ADDR_LEN; } 0
}

static mac802154_header_ops: header_ops = header_ops { create: Some(mac802154_header_create), parse: Some(mac802154_header_parse) };
static mac802154_wpan_ops: net_device_ops = net_device_ops { ndo_open: Some(mac802154_wpan_open), ndo_stop: Some(mac802154_slave_close), ndo_start_xmit: Some(ieee802154_subif_start_xmit), ndo_do_ioctl: Some(mac802154_wpan_ioctl), ndo_set_mac_address: Some(mac802154_wpan_mac_addr) };
static mac802154_monitor_ops: net_device_ops = net_device_ops { ndo_open: Some(mac802154_wpan_open), ndo_stop: Some(mac802154_slave_close), ndo_start_xmit: Some(ieee802154_monitor_start_xmit) };

unsafe fn mac802154_wpan_free(dev: *mut net_device) { let sdata = IEEE802154_DEV_TO_SUB_IF(dev); mac802154_llsec_destroy(&mut (*sdata).sec); }

unsafe fn ieee802154_if_setup(dev: *mut net_device) {
    (*dev).addr_len = IEEE802154_EXTENDED_ADDR_LEN; core::ptr::write_bytes((*dev).broadcast.as_mut_ptr(), 0xff, IEEE802154_EXTENDED_ADDR_LEN as usize);
    (*dev).hard_header_len = IEEE802154_MIN_HEADER_LEN - 1; (*dev).needed_tailroom = IEEE802154_MAX_AUTH_TAG_LEN + IEEE802154_FCS_LEN; (*dev).mtu = IEEE802154_MTU - IEEE802154_FCS_LEN - (*dev).hard_header_len; (*dev).tx_queue_len = 300; (*dev).flags = IFF_NOARP | IFF_BROADCAST;
}

unsafe fn ieee802154_setup_sdata(sdata: *mut ieee802154_sub_if_data, type_: nl802154_iftype) -> c_int {
    let wpan_dev = &mut (*sdata).wpan_dev; (*sdata).wpan_dev.iftype = type_; let mut tmp: u8 = 0;
    get_random_bytes(&mut tmp as *mut _ as *mut c_void, core::mem::size_of::<u8>()); atomic_set(&mut wpan_dev.bsn, tmp as _); get_random_bytes(&mut tmp as *mut _ as *mut c_void, 1); atomic_set(&mut wpan_dev.dsn, tmp as _);
    wpan_dev.min_be = 3; wpan_dev.max_be = 5; wpan_dev.csma_retries = 4; wpan_dev.frame_retries = 3; wpan_dev.pan_id = cpu_to_le16(IEEE802154_PANID_BROADCAST); wpan_dev.short_addr = cpu_to_le16(IEEE802154_ADDR_BROADCAST);
    match type_ {
        NL802154_IFTYPE_COORD | NL802154_IFTYPE_NODE => { ieee802154_be64_to_le64(&mut wpan_dev.extended_addr, (*sdata).dev.as_ref().unwrap().dev_addr.as_ptr()); (*sdata).dev.as_mut().unwrap().header_ops = &mac802154_header_ops; (*sdata).dev.as_mut().unwrap().needs_free_netdev = true; (*sdata).dev.as_mut().unwrap().priv_destructor = Some(mac802154_wpan_free); (*sdata).dev.as_mut().unwrap().netdev_ops = &mac802154_wpan_ops; (*sdata).dev.as_mut().unwrap().ml_priv = &mac802154_mlme_wpan as *const _ as *mut _; (*sdata).iface_default_filtering = IEEE802154_FILTERING_4_FRAME_FIELDS; wpan_dev.header_ops = &ieee802154_header_ops; mutex_init(&mut (*sdata).sec_mtx); mac802154_llsec_init(&mut (*sdata).sec); let ret = mac802154_wpan_update_llsec((*sdata).dev); if ret < 0 { return ret; } }
        NL802154_IFTYPE_MONITOR => { (*sdata).dev.as_mut().unwrap().needs_free_netdev = true; (*sdata).dev.as_mut().unwrap().netdev_ops = &mac802154_monitor_ops; (*sdata).iface_default_filtering = IEEE802154_FILTERING_NONE; }
        _ => { BUG!(); }
    } 0
}

pub unsafe fn ieee802154_if_add(local: *mut ieee802154_local, name: *const c_char, name_assign_type: u8, type_: nl802154_iftype, extended_addr: __le64) -> *mut net_device {
    let mut addr = [0u8; IEEE802154_EXTENDED_ADDR_LEN as usize]; let mut ndev = alloc_netdev(core::mem::size_of::<ieee802154_sub_if_data>(), name, name_assign_type, ieee802154_if_setup); if ndev.is_null() { return ERR_PTR(-ENOMEM); }
    (*ndev).needed_headroom = (*local).hw.extra_tx_headroom + IEEE802154_MAX_HEADER_LEN; let mut ret = dev_alloc_name(ndev, (*ndev).name.as_mut_ptr()); if ret < 0 { free_netdev(ndev); return ERR_PTR(ret); }
    ieee802154_le64_to_be64((*ndev).perm_addr.as_mut_ptr(), &(*local).hw.phy.as_ref().unwrap().perm_extended_addr);
    match type_ { NL802154_IFTYPE_COORD | NL802154_IFTYPE_NODE => { (*ndev).type_ = ARPHRD_IEEE802154; if ieee802154_is_valid_extended_unicast_addr(extended_addr) { ieee802154_le64_to_be64(addr.as_mut_ptr(), &extended_addr); dev_addr_set(ndev, addr.as_ptr()); } else { dev_addr_set(ndev, (*ndev).perm_addr.as_ptr()); } }, NL802154_IFTYPE_MONITOR => (*ndev).type_ = ARPHRD_IEEE802154_MONITOR, _ => { ret = -EINVAL; free_netdev(ndev); return ERR_PTR(ret); } }
    SET_NETDEV_DEV!(ndev, &(*local).phy.dev); dev_net_set(ndev, wpan_phy_net((*local).hw.phy)); let sdata = netdev_priv(ndev) as *mut ieee802154_sub_if_data; (*ndev).ieee802154_ptr = &mut (*sdata).wpan_dev; core::ptr::copy_nonoverlapping((*ndev).name.as_ptr(), (*sdata).name.as_mut_ptr(), IFNAMSIZ); (*sdata).dev = ndev; (*sdata).wpan_dev.wpan_phy = (*local).hw.phy; (*sdata).local = local; INIT_LIST_HEAD!(&mut (*sdata).wpan_dev.list);
    ret = ieee802154_setup_sdata(sdata, type_); if ret != 0 { free_netdev(ndev); return ERR_PTR(ret); } ret = register_netdevice(ndev); if ret < 0 { free_netdev(ndev); return ERR_PTR(ret); }
    mutex_lock(&mut (*local).iflist_mtx); list_add_tail_rcu!(&mut (*sdata).list, &mut (*local).interfaces); mutex_unlock(&mut (*local).iflist_mtx); ndev
}

pub unsafe fn ieee802154_if_remove(sdata: *mut ieee802154_sub_if_data) { ASSERT_RTNL!(); mutex_lock(&mut (*(*sdata).local).iflist_mtx); if list_empty(&(*(*sdata).local).interfaces) { mutex_unlock(&mut (*(*sdata).local).iflist_mtx); return; } list_del_rcu!(&mut (*sdata).list); mutex_unlock(&mut (*(*sdata).local).iflist_mtx); synchronize_rcu(); unregister_netdevice((*sdata).dev); }

pub unsafe fn ieee802154_remove_interfaces(local: *mut ieee802154_local) { mutex_lock(&mut (*local).iflist_mtx); list_for_each_entry_safe!(sdata, tmp, &(*local).interfaces, list, { list_del_rcu!(&mut (*sdata).list); unregister_netdevice((*sdata).dev); }); mutex_unlock(&mut (*local).iflist_mtx); }

unsafe fn netdev_notify(_nb: *mut notifier_block, state: c_ulong, ptr: *mut c_void) -> c_int { let dev = netdev_notifier_info_to_dev(ptr); if state != NETDEV_CHANGENAME || (*dev).ieee802154_ptr.is_null() || (*(*dev).ieee802154_ptr).wpan_phy.is_null() || (*(*dev).ieee802154_ptr).wpan_phy.as_ref().unwrap().privid != mac802154_wpan_phy_privid { return NOTIFY_DONE; } let sdata = IEEE802154_DEV_TO_SUB_IF(dev); core::ptr::copy_nonoverlapping((*dev).name.as_ptr(), (*sdata).name.as_mut_ptr(), IFNAMSIZ); NOTIFY_OK }

static mut mac802154_netdev_notifier: notifier_block = notifier_block { notifier_call: Some(netdev_notify) };
pub unsafe fn ieee802154_iface_init() -> c_int { register_netdevice_notifier(&mut mac802154_netdev_notifier) }
pub unsafe fn ieee802154_iface_exit() { unregister_netdevice_notifier(&mut mac802154_netdev_notifier); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
