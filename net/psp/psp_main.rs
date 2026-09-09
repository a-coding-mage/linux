// SPDX-License-Identifier: GPL-2.0-only

// Linux/project dependencies from the original C translation unit are supplied
// externally; their declarations are intentionally not reimplemented here.

extern "C" {
    static mut psp_devs: xarray;
    static mut psp_devs_lock: mutex;
}

/**
 * DOC: PSP locking
 *
 * psp_devs_lock protects the psp_devs xarray.
 * Ordering is take the psp_devs_lock and then the instance lock.
 * Each instance is protected by RCU, and has a refcount.
 * When driver unregisters the instance gets flushed, but struct sticks around.
 */

/// Check if a user in a given net namespace can access a PSP device.
unsafe fn psp_dev_check_access(psd: *mut psp_dev, net: *mut net, admin: bool) -> i32 {
    if dev_net((*psd).main_netdev) == net {
        return 0;
    }
    if !admin && psp_has_assoc_dev_in_ns(psd, net) {
        return 0;
    }
    -ENOENT
}

/// Create and register a PSP device.
#[no_mangle]
pub unsafe extern "C" fn psp_dev_create(
    netdev: *mut net_device,
    psd_ops: *mut psp_dev_ops,
    psd_caps: *mut psp_dev_caps,
    priv_ptr: *mut core::ffi::c_void,
) -> *mut psp_dev {
    static mut last_id: u32 = 0;
    let psd: *mut psp_dev;
    let err: i32;

    if WARN_ON((*psd_caps).versions.is_null()
        || (*psd_ops).set_config.is_none()
        || (*psd_ops).key_rotate.is_none()
        || (*psd_ops).rx_spi_alloc.is_none()
        || (*psd_ops).tx_key_add.is_none()
        || (*psd_ops).tx_key_del.is_none()
        || (*psd_ops).get_stats.is_none()) {
        return ERR_PTR(-EINVAL);
    }

    psd = kzalloc_obj::<psp_dev>();
    if psd.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*psd).main_netdev = netdev;
    INIT_LIST_HEAD(&mut (*psd).assoc_dev_list);
    (*psd).ops = psd_ops;
    (*psd).caps = psd_caps;
    (*psd).drv_priv = priv_ptr;

    mutex_init(&mut (*psd).lock);
    INIT_LIST_HEAD(&mut (*psd).active_assocs);
    INIT_LIST_HEAD(&mut (*psd).prev_assocs);
    INIT_LIST_HEAD(&mut (*psd).stale_assocs);
    refcount_set(&mut (*psd).refcnt, 1);

    mutex_lock(&mut psp_devs_lock);
    err = xa_alloc_cyclic(&mut psp_devs, &mut (*psd).id, psd,
                          xa_limit_16b, &mut last_id, GFP_KERNEL);
    if err != 0 {
        mutex_unlock(&mut psp_devs_lock);
        kfree(psd);
        return ERR_PTR(err);
    }
    mutex_lock(&mut (*psd).lock);
    mutex_unlock(&mut psp_devs_lock);

    // Notify before netdev assignment, to avoid implicit PSP-dev/netdev expectations.
    psp_nl_notify_dev(psd, PSP_CMD_DEV_ADD_NTF);
    rcu_assign_pointer(&mut (*netdev).psp_dev, psd);
    mutex_unlock(&mut (*psd).lock);
    psd
}

pub unsafe extern "C" fn psp_dev_free(psd: *mut psp_dev) {
    mutex_lock(&mut psp_devs_lock);
    xa_erase(&mut psp_devs, (*psd).id);
    mutex_unlock(&mut psp_devs_lock);
    mutex_destroy(&mut (*psd).lock);
    kfree_rcu(psd, rcu);
}

/// Unregister a PSP device.
pub unsafe extern "C" fn psp_dev_unregister(psd: *mut psp_dev) {
    let mut entry: *mut psp_assoc_dev;
    let mut entry_tmp: *mut psp_assoc_dev;
    let mut pas: *mut psp_assoc;
    let mut next: *mut psp_assoc;

    mutex_lock(&mut psp_devs_lock);
    mutex_lock(&mut (*psd).lock);
    psp_nl_notify_dev(psd, PSP_CMD_DEV_DEL_NTF);
    xa_store(&mut psp_devs, (*psd).id, core::ptr::null_mut(), GFP_KERNEL);
    mutex_unlock(&mut psp_devs_lock);

    list_splice_init(&mut (*psd).active_assocs, &mut (*psd).prev_assocs);
    list_splice_init(&mut (*psd).prev_assocs, &mut (*psd).stale_assocs);
    list_for_each_entry_safe(&mut pas, &mut next, &mut (*psd).stale_assocs, assocs_list,
        psp_dev_tx_key_del(psd, pas));

    list_for_each_entry_safe(&mut entry, &mut entry_tmp, &mut (*psd).assoc_dev_list, dev_list, {
        list_del(&mut (*entry).dev_list);
        rcu_assign_pointer(&mut (*entry).assoc_dev.psp_dev, core::ptr::null_mut());
        netdev_put((*entry).assoc_dev, &mut (*entry).dev_tracker);
        kfree(entry);
    });
    (*psd).assoc_dev_cnt = 0;
    rcu_assign_pointer(&mut (*(*psd).main_netdev).psp_dev, core::ptr::null_mut());
    (*psd).ops = core::ptr::null_mut();
    (*psd).drv_priv = core::ptr::null_mut();
    mutex_unlock(&mut (*psd).lock);
    psp_dev_put(psd);
}

pub unsafe extern "C" fn psp_key_size(version: u32) -> u32 {
    match version {
        PSP_VERSION_HDR0_AES_GCM_128 | PSP_VERSION_HDR0_AES_GMAC_128 => 16,
        PSP_VERSION_HDR0_AES_GCM_256 | PSP_VERSION_HDR0_AES_GMAC_256 => 32,
        _ => 0,
    }
}

unsafe fn psp_write_headers(net: *mut net, skb: *mut sk_buff, spi: __be32,
                            ver: u8, udp_len: u32, _sport: __be16) {
    let uh = udp_hdr(skb);
    let psph = (uh as *mut u8).add(core::mem::size_of::<udphdr>()) as *mut psphdr;
    let sk = (*skb).sk;
    (*uh).dest = htons(PSP_DEFAULT_UDP_PORT);
    if !sk.is_null() {
        let mut hash = (*sk).sk_hash;
        let mut min = 0;
        let mut max = 0;
        inet_get_local_port_range(net, &mut min, &mut max);
        hash ^= hash << 16;
        (*uh).source = htons(reciprocal_scale(hash, max - min + 1) + min);
    } else {
        (*uh).source = udp_flow_src_port(net, skb, 0, 0, false);
    }
    (*uh).check = 0;
    udp_set_len(uh, udp_len);
    (*psph).nexthdr = IPPROTO_TCP;
    (*psph).hdrlen = PSP_HDRLEN_NOOPT;
    (*psph).crypt_offset = 0;
    (*psph).verfl = FIELD_PREP(PSPHDR_VERFL_VERSION, ver) | FIELD_PREP(PSPHDR_VERFL_ONE, 1);
    (*psph).spi = spi;
    memset(&mut (*psph).iv, 0, core::mem::size_of_val(&(*psph).iv));
}

pub unsafe extern "C" fn psp_dev_encapsulate(net: *mut net, skb: *mut sk_buff, spi: __be32,
                                              ver: u8, sport: __be16) -> bool {
    let network_len = skb_network_header_len(skb);
    let ethr_len = skb_mac_header_len(skb);
    let bufflen = ethr_len + network_len;
    if (*skb).protocol != htons(ETH_P_IP) && (*skb).protocol != htons(ETH_P_IPV6) { return false; }
    if skb_cow_head(skb, PSP_ENCAP_HLEN) != 0 { return false; }
    skb_push(skb, PSP_ENCAP_HLEN);
    (*skb).mac_header -= PSP_ENCAP_HLEN;
    (*skb).network_header -= PSP_ENCAP_HLEN;
    (*skb).transport_header -= PSP_ENCAP_HLEN;
    memmove((*skb).data, (*skb).data.add(PSP_ENCAP_HLEN), bufflen);
    if (*skb).protocol == htons(ETH_P_IP) {
        (*ip_hdr(skb)).protocol = IPPROTO_UDP;
        be16_add_cpu(&mut (*ip_hdr(skb)).tot_len, PSP_ENCAP_HLEN);
        (*ip_hdr(skb)).check = 0;
        (*ip_hdr(skb)).check = ip_fast_csum(ip_hdr(skb) as *mut u8, (*ip_hdr(skb)).ihl);
    } else {
        (*ipv6_hdr(skb)).nexthdr = IPPROTO_UDP;
        be16_add_cpu(&mut (*ipv6_hdr(skb)).payload_len, PSP_ENCAP_HLEN);
    }
    skb_set_inner_ipproto(skb, IPPROTO_TCP);
    skb_set_inner_transport_header(skb, skb_transport_offset(skb) + PSP_ENCAP_HLEN);
    (*skb).encapsulation = 1;
    psp_write_headers(net, skb, spi, ver, (*skb).len - skb_transport_offset(skb), sport);
    true
}

pub unsafe extern "C" fn psp_dev_rcv(skb: *mut sk_buff, dev_id: u16, generation: u8,
                                      strip_icv: bool) -> i32 {
    let mut l2_hlen = 0;
    let mut l3_hlen;
    let mut encap;
    let mut psp_hlen;
    let pse: *mut psp_skb_ext;
    let psph: *mut psphdr;
    let eth = (*skb).data as *mut ethhdr;
    let proto = __vlan_get_protocol(skb, (*eth).h_proto, &mut l2_hlen);
    if proto == htons(ETH_P_IP) { l3_hlen = core::mem::size_of::<iphdr>(); }
    else if proto == htons(ETH_P_IPV6) { l3_hlen = core::mem::size_of::<ipv6hdr>(); }
    else { return -EINVAL; }
    if !pskb_may_pull(skb, l2_hlen + l3_hlen + PSP_ENCAP_HLEN) { return -EINVAL; }
    let is_udp;
    if proto == htons(ETH_P_IP) {
        let iph = (*skb).data.add(l2_hlen) as *mut iphdr;
        if (*iph).ihl < 5 { return -EINVAL; }
        is_udp = (*iph).protocol == IPPROTO_UDP;
        l3_hlen = (*iph).ihl as usize * 4;
        if l3_hlen != core::mem::size_of::<iphdr>() && !pskb_may_pull(skb, l2_hlen + l3_hlen + PSP_ENCAP_HLEN) { return -EINVAL; }
    } else {
        is_udp = (*( (*skb).data.add(l2_hlen) as *mut ipv6hdr)).nexthdr == IPPROTO_UDP;
    }
    if !is_udp { return -EINVAL; }
    let uh = (*skb).data.add(l2_hlen + l3_hlen) as *mut udphdr;
    if (*uh).dest != htons(PSP_DEFAULT_UDP_PORT) { return -EINVAL; }
    psph = (*skb).data.add(l2_hlen + l3_hlen + core::mem::size_of::<udphdr>()) as *mut psphdr;
    psp_hlen = ((*psph).hdrlen as usize + 1) * 8;
    if psp_hlen < core::mem::size_of::<psphdr>() { return -EINVAL; }
    if psp_hlen > core::mem::size_of::<psphdr>() && !pskb_may_pull(skb, l2_hlen + l3_hlen + core::mem::size_of::<udphdr>() + psp_hlen) { return -EINVAL; }
    let psph = (*skb).data.add(l2_hlen + l3_hlen + core::mem::size_of::<udphdr>()) as *mut psphdr;
    pse = skb_ext_add(skb, SKB_EXT_PSP);
    if pse.is_null() { return -EINVAL; }
    (*pse).spi = (*psph).spi;
    (*pse).dev_id = dev_id;
    (*pse).generation = generation;
    (*pse).version = FIELD_GET(PSPHDR_VERFL_VERSION, (*psph).verfl);
    encap = core::mem::size_of::<udphdr>() + psp_hlen + if strip_icv { PSP_TRL_SIZE } else { 0 };
    if proto == htons(ETH_P_IP) {
        let iph = (*skb).data.add(l2_hlen) as *mut iphdr;
        if ntohs((*iph).tot_len) < l3_hlen + encap { return -EINVAL; }
        (*iph).protocol = (*psph).nexthdr;
        (*iph).tot_len = htons(ntohs((*iph).tot_len) - encap);
        (*iph).check = 0;
        (*iph).check = ip_fast_csum(iph as *mut u8, (*iph).ihl);
    } else {
        let h = (*skb).data.add(l2_hlen) as *mut ipv6hdr;
        if ntohs((*h).payload_len) < encap { return -EINVAL; }
        (*h).nexthdr = (*psph).nexthdr;
        (*h).payload_len = htons(ntohs((*h).payload_len) - encap);
    }
    memmove((*skb).data.add(core::mem::size_of::<udphdr>() + psp_hlen), (*skb).data, l2_hlen + l3_hlen);
    skb_pull(skb, core::mem::size_of::<udphdr>() + psp_hlen);
    if strip_icv { pskb_trim(skb, (*skb).len - PSP_TRL_SIZE); }
    0
}

unsafe fn psp_dev_disassoc_one(psd: *mut psp_dev, dev: *mut net_device) {
    let mut entry: *mut psp_assoc_dev;
    list_for_each_entry(&mut entry, &mut (*psd).assoc_dev_list, dev_list, {
        if (*entry).assoc_dev == dev {
            list_del(&mut (*entry).dev_list);
            (*psd).assoc_dev_cnt -= 1;
            rcu_assign_pointer(&mut (*(*entry).assoc_dev).psp_dev, core::ptr::null_mut());
            netdev_put((*entry).assoc_dev, &mut (*entry).dev_tracker);
            kfree(entry);
            return;
        }
    });
}

unsafe fn psp_netdev_event(_nb: *mut notifier_block, event: u64, ptr: *mut core::ffi::c_void) -> i32 {
    let dev = netdev_notifier_info_to_dev(ptr);
    if event != NETDEV_UNREGISTER { return NOTIFY_DONE; }
    rcu_read_lock();
    let psd = rcu_dereference((*dev).psp_dev);
    if !psd.is_null() && psp_dev_tryget(psd) {
        rcu_read_unlock();
        mutex_lock(&mut (*psd).lock);
        if psp_dev_is_registered(psd) { psp_nl_notify_dev(psd, PSP_CMD_DEV_CHANGE_NTF); }
        psp_dev_disassoc_one(psd, dev);
        mutex_unlock(&mut (*psd).lock);
        psp_dev_put(psd);
    } else { rcu_read_unlock(); }
    NOTIFY_DONE
}

static mut psp_netdev_notifier: notifier_block = notifier_block { notifier_call: Some(psp_netdev_event) };
static mut psp_notifier_lock: mutex = mutex_uninitialized!();
static mut psp_notifier_registered: bool = false;

pub unsafe extern "C" fn psp_attach_netdev_notifier() -> i32 {
    let mut err = 0;
    if READ_ONCE(psp_notifier_registered) { return 0; }
    mutex_lock(&mut psp_notifier_lock);
    if !psp_notifier_registered {
        err = register_netdevice_notifier(&mut psp_netdev_notifier);
        if err == 0 { WRITE_ONCE(psp_notifier_registered, true); }
    }
    mutex_unlock(&mut psp_notifier_lock);
    err
}

unsafe fn psp_init() -> i32 {
    mutex_init(&mut psp_devs_lock);
    genl_register_family(&mut psp_nl_family)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
