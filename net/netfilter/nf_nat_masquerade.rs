// SPDX-License-Identifier: GPL-2.0

// Translated from nf_nat_masquerade.c. Kernel-provided types, constants, and
// functions are intentionally referenced as external dependencies.

#[repr(C)]
struct MasqDevWork {
    work: work_struct,
    net: *mut net,
    ns_tracker: netns_tracker,
    addr: nf_inet_addr,
    ifindex: i32,
    iter: Option<unsafe extern "C" fn(*mut nf_conn, *mut core::ffi::c_void) -> i32>,
}

const MAX_MASQ_WORKER_COUNT: i32 = 16;

static mut masq_mutex: mutex = unsafe { core::mem::zeroed() };
static mut masq_refcnt: u32 = 0;
static mut masq_worker_count: atomic_t = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn nf_nat_masquerade_ipv4(
    skb: *mut sk_buff,
    hooknum: u32,
    range: *const nf_nat_range2,
    out: *const net_device,
) -> u32 {
    let mut ctinfo = core::mem::zeroed::<ip_conntrack_info>();
    let ct = nf_ct_get(skb, &mut ctinfo);
    WARN_ON(hooknum != NF_INET_POST_ROUTING);
    WARN_ON(!( !ct.is_null() && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED || ctinfo == IP_CT_RELATED_REPLY)));

    if (*(*ct).tuplehash.as_ptr().add(IP_CT_DIR_ORIGINAL as usize)).tuple.src.u3.ip == 0 {
        return NF_ACCEPT;
    }

    let rt = skb_rtable(skb);
    let nh = rt_nexthop(rt, (*ip_hdr(skb)).daddr);
    let newsrc = inet_select_addr(out, nh, RT_SCOPE_UNIVERSE);
    if newsrc == 0 {
        pr_info!("%s ate my IP address\n", (*out).name);
        return NF_DROP;
    }

    let nat = nf_ct_nat_ext_add(ct);
    if !nat.is_null() { (*nat).masq_index = (*out).ifindex; }

    let mut newrange = core::mem::zeroed::<nf_nat_range2>();
    newrange.flags = (*range).flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr.ip = newsrc;
    newrange.max_addr.ip = newsrc;
    newrange.min_proto = (*range).min_proto;
    newrange.max_proto = (*range).max_proto;
    nf_nat_setup_info(ct, &mut newrange, NF_NAT_MANIP_SRC)
}

unsafe extern "C" fn iterate_cleanup_work(work: *mut work_struct) {
    let w = container_of!(work, MasqDevWork, work);
    let mut iter_data = core::mem::zeroed::<nf_ct_iter_data>();
    (*iter_data).net = (*w).net;
    (*iter_data).data = w as *mut core::ffi::c_void;
    nf_ct_iterate_cleanup_net((*w).iter, &mut iter_data);
    put_net_track((*w).net, &mut (*w).ns_tracker);
    kfree(w as *mut core::ffi::c_void);
    atomic_dec(&mut masq_worker_count);
    module_put(THIS_MODULE);
}

unsafe extern "C" fn nf_nat_masq_schedule(
    mut net: *mut net, addr: *mut nf_inet_addr, ifindex: i32,
    iter: Option<unsafe extern "C" fn(*mut nf_conn, *mut core::ffi::c_void) -> i32>,
    gfp_flags: gfp_t,
) {
    if atomic_read(&masq_worker_count) > MAX_MASQ_WORKER_COUNT { return; }
    net = maybe_get_net(net);
    if net.is_null() { return; }
    if !try_module_get(THIS_MODULE) { put_net(net); return; }
    let w = kzalloc_obj::<MasqDevWork>(gfp_flags);
    if !w.is_null() {
        atomic_inc(&mut masq_worker_count);
        INIT_WORK(&mut (*w).work, Some(iterate_cleanup_work));
        (*w).ifindex = ifindex;
        (*w).net = net;
        netns_tracker_alloc(net, &mut (*w).ns_tracker, gfp_flags);
        (*w).iter = iter;
        if !addr.is_null() { (*w).addr = *addr; }
        schedule_work(&mut (*w).work);
        return;
    }
    module_put(THIS_MODULE);
    put_net(net);
}

unsafe extern "C" fn device_cmp(i: *mut nf_conn, arg: *mut core::ffi::c_void) -> i32 {
    let nat = nfct_nat(i);
    if nat.is_null() { return 0; }
    ((*nat).masq_index == (*(arg as *mut MasqDevWork)).ifindex) as i32
}

unsafe extern "C" fn masq_device_event(_: *mut notifier_block, event: u64, ptr: *mut core::ffi::c_void) -> i32 {
    let dev = netdev_notifier_info_to_dev(ptr);
    if event == NETDEV_DOWN { nf_nat_masq_schedule(dev_net(dev), core::ptr::null_mut(), (*dev).ifindex, Some(device_cmp), GFP_KERNEL); }
    NOTIFY_DONE
}

unsafe extern "C" fn inet_cmp(ct: *mut nf_conn, ptr: *mut core::ffi::c_void) -> i32 {
    if device_cmp(ct, ptr) == 0 { return 0; }
    let w = ptr as *mut MasqDevWork;
    let tuple = &(*ct).tuplehash[IP_CT_DIR_REPLY as usize].tuple;
    nf_inet_addr_cmp(&(*w).addr, &tuple.dst.u3) as i32
}

unsafe extern "C" fn masq_inet_event(_: *mut notifier_block, event: u64, ptr: *mut core::ffi::c_void) -> i32 {
    if event != NETDEV_DOWN { return NOTIFY_DONE; }
    let ifa = ptr as *mut in_ifaddr;
    let idev = (*ifa).ifa_dev;
    if (*idev).dead { return NOTIFY_DONE; }
    let mut addr = core::mem::zeroed::<nf_inet_addr>();
    addr.ip = (*ifa).ifa_address;
    let dev = (*idev).dev;
    nf_nat_masq_schedule(dev_net(dev), &mut addr, (*dev).ifindex, Some(inet_cmp), GFP_KERNEL);
    NOTIFY_DONE
}

static mut masq_dev_notifier: notifier_block = notifier_block { notifier_call: Some(masq_device_event) };
static mut masq_inet_notifier: notifier_block = notifier_block { notifier_call: Some(masq_inet_event) };

#[cfg(feature = "CONFIG_IPV6")]
#[no_mangle]
pub unsafe extern "C" fn nf_nat_masquerade_ipv6(skb: *mut sk_buff, range: *const nf_nat_range2, out: *const net_device) -> u32 {
    let mut ctinfo = core::mem::zeroed::<ip_conntrack_info>();
    let ct = nf_ct_get(skb, &mut ctinfo);
    WARN_ON(!( !ct.is_null() && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED || ctinfo == IP_CT_RELATED_REPLY)));
    let mut src = core::mem::zeroed::<in6_addr>();
    if ipv6_dev_get_saddr(nf_ct_net(ct), out, &(*ipv6_hdr(skb)).daddr, 0, &mut src) < 0 { return NF_DROP; }
    let nat = nf_ct_nat_ext_add(ct);
    if !nat.is_null() { (*nat).masq_index = (*out).ifindex; }
    let mut newrange = core::mem::zeroed::<nf_nat_range2>();
    newrange.flags = (*range).flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr.in6 = src; newrange.max_addr.in6 = src;
    newrange.min_proto = (*range).min_proto; newrange.max_proto = (*range).max_proto;
    nf_nat_setup_info(ct, &mut newrange, NF_NAT_MANIP_SRC)
}

#[cfg(feature = "CONFIG_IPV6")]
unsafe extern "C" fn masq_inet6_event(_: *mut notifier_block, event: u64, ptr: *mut core::ffi::c_void) -> i32 {
    if event != NETDEV_DOWN { return NOTIFY_DONE; }
    let ifa = ptr as *mut inet6_ifaddr;
    let dev = (*(*ifa).idev).dev;
    let mut addr = core::mem::zeroed::<nf_inet_addr>();
    addr.in6 = (*ifa).addr;
    nf_nat_masq_schedule(dev_net(dev), &mut addr, (*dev).ifindex, Some(inet_cmp), GFP_ATOMIC);
    NOTIFY_DONE
}

#[cfg(feature = "CONFIG_IPV6")]
static mut masq_inet6_notifier: notifier_block = notifier_block { notifier_call: Some(masq_inet6_event) };

unsafe extern "C" fn nf_nat_masquerade_ipv6_register_notifier() -> i32 {
    #[cfg(feature = "CONFIG_IPV6")] { return register_inet6addr_notifier(&mut masq_inet6_notifier); }
    #[cfg(not(feature = "CONFIG_IPV6"))] { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_masquerade_inet_register_notifiers() -> i32 {
    let mut ret = 0;
    mutex_lock(&mut masq_mutex);
    if WARN_ON_ONCE(masq_refcnt == u32::MAX) { ret = -EOVERFLOW; mutex_unlock(&mut masq_mutex); return ret; }
    masq_refcnt += 1;
    if masq_refcnt > 1 { mutex_unlock(&mut masq_mutex); return ret; }
    ret = register_netdevice_notifier(&mut masq_dev_notifier);
    if ret != 0 { masq_refcnt -= 1; mutex_unlock(&mut masq_mutex); return ret; }
    ret = register_inetaddr_notifier(&mut masq_inet_notifier);
    if ret != 0 { unregister_netdevice_notifier(&mut masq_dev_notifier); masq_refcnt -= 1; mutex_unlock(&mut masq_mutex); return ret; }
    ret = nf_nat_masquerade_ipv6_register_notifier();
    if ret != 0 { unregister_inetaddr_notifier(&mut masq_inet_notifier); unregister_netdevice_notifier(&mut masq_dev_notifier); masq_refcnt -= 1; }
    mutex_unlock(&mut masq_mutex); ret
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_masquerade_inet_unregister_notifiers() {
    mutex_lock(&mut masq_mutex);
    masq_refcnt -= 1;
    if masq_refcnt > 0 { mutex_unlock(&mut masq_mutex); return; }
    unregister_netdevice_notifier(&mut masq_dev_notifier);
    unregister_inetaddr_notifier(&mut masq_inet_notifier);
    #[cfg(feature = "CONFIG_IPV6")] unregister_inet6addr_notifier(&mut masq_inet6_notifier);
    mutex_unlock(&mut masq_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
