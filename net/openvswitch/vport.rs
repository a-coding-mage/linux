// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2007-2014 Nicira, Inc.
 */

// Linux/repository dependencies supplied by the surrounding translation.

static mut VPORT_OPS_LIST: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
/* Protected by RCU read lock for reading, ovs_mutex for writing. */
static mut DEV_TABLE: *mut HlistHead = core::ptr::null_mut();
const VPORT_HASH_BUCKETS: usize = 1024;

pub unsafe fn ovs_vport_init() -> i32 {
    DEV_TABLE = kzalloc_objs::<HlistHead>(VPORT_HASH_BUCKETS);
    if DEV_TABLE.is_null() { return -ENOMEM; }
    0
}

pub unsafe fn ovs_vport_exit() { kfree(DEV_TABLE as *mut core::ffi::c_void); }

unsafe fn hash_bucket(net: *const Net, name: *const i8) -> *mut HlistHead {
    let hash = jhash(name, strlen(name), net as usize);
    DEV_TABLE.add(hash & (VPORT_HASH_BUCKETS - 1))
}

pub unsafe fn ovs_vport_ops_register(ops: *mut VportOps) -> i32 {
    let mut err = -EEXIST;
    let mut o: *mut VportOps = core::ptr::null_mut();
    ovs_lock();
    list_for_each_entry(&mut o, &mut VPORT_OPS_LIST, (*VportOps).list) {
        if (*ops).type_ == (*o).type_ { ovs_unlock(); return err; }
    }
    list_add_tail(&mut (*ops).list, &mut VPORT_OPS_LIST);
    err = 0;
    ovs_unlock();
    err
}

pub unsafe fn ovs_vport_ops_unregister(ops: *mut VportOps) {
    ovs_lock();
    list_del(&mut (*ops).list);
    ovs_unlock();
}

pub unsafe fn ovs_vport_locate(net: *const Net, name: *const i8) -> *mut Vport {
    let bucket = hash_bucket(net, name);
    let mut vport: *mut Vport = core::ptr::null_mut();
    hlist_for_each_entry_rcu(&mut vport, bucket, (*Vport).hash_node, lockdep_ovsl_is_held()) {
        if strcmp(name, ovs_vport_name(vport)) == 0 && net_eq(ovs_dp_get_net((*vport).dp), net) { return vport; }
    }
    core::ptr::null_mut()
}

pub unsafe fn ovs_vport_alloc(priv_size: i32, ops: *const VportOps, parms: *const VportParms) -> *mut Vport {
    let mut alloc_size = core::mem::size_of::<Vport>();
    if priv_size != 0 { alloc_size = align(alloc_size, VPORT_ALIGN); alloc_size += priv_size as usize; }
    let vport = kzalloc(alloc_size, GFP_KERNEL) as *mut Vport;
    if vport.is_null() { return ERR_PTR(-ENOMEM); }
    (*vport).upcall_stats = netdev_alloc_pcpu_stats::<VportUpcallStatsPercpu>();
    if (*vport).upcall_stats.is_null() { kfree(vport as *mut _); return ERR_PTR(-ENOMEM); }
    (*vport).dp = (*parms).dp;
    (*vport).port_no = (*parms).port_no;
    (*vport).ops = ops;
    INIT_HLIST_NODE(&mut (*vport).dp_hash_node);
    if ovs_vport_set_upcall_portids(vport, (*parms).upcall_portids) != 0 {
        free_percpu((*vport).upcall_stats); kfree(vport as *mut _); return ERR_PTR(-EINVAL);
    }
    vport
}

pub unsafe fn ovs_vport_free(vport: *mut Vport) {
    /* vport is freed from RCU callback or error path, Therefore it is safe to use raw dereference. */
    kfree(rcu_dereference_raw((*vport).upcall_portids) as *mut _);
    free_percpu((*vport).upcall_stats);
    kfree(vport as *mut _);
}

unsafe fn ovs_vport_lookup(parms: *const VportParms) -> *mut Vport {
    let mut ops: *mut VportOps = core::ptr::null_mut();
    list_for_each_entry(&mut ops, &mut VPORT_OPS_LIST, (*VportOps).list) {
        if (*ops).type_ == (*parms).type_ { return ops; }
    }
    core::ptr::null_mut()
}

pub unsafe fn ovs_vport_add(parms: *const VportParms) -> *mut Vport {
    let ops = ovs_vport_lookup(parms);
    if !ops.is_null() {
        let vport = ((*ops).create)(parms);
        if IS_ERR(vport) { return vport; }
        let bucket = hash_bucket(ovs_dp_get_net((*vport).dp), ovs_vport_name(vport));
        hlist_add_head_rcu(&mut (*vport).hash_node, bucket);
        return vport;
    }
    ERR_PTR(-EAFNOSUPPORT)
}

pub unsafe fn ovs_vport_del(vport: *mut Vport) {
    hlist_del_rcu(&mut (*vport).hash_node);
    ((*(*vport).ops).destroy)(vport);
}

pub unsafe fn ovs_vport_get_stats(vport: *mut Vport, stats: *mut OvsVportStats) {
    let mut temp = core::mem::MaybeUninit::<RtnlLinkStats64>::uninit();
    let dev_stats = dev_get_stats((*vport).dev, temp.as_mut_ptr());
    (*stats).rx_errors = (*dev_stats).rx_errors; (*stats).tx_errors = (*dev_stats).tx_errors;
    (*stats).tx_dropped = (*dev_stats).tx_dropped; (*stats).rx_dropped = (*dev_stats).rx_dropped;
    (*stats).rx_bytes = (*dev_stats).rx_bytes; (*stats).rx_packets = (*dev_stats).rx_packets;
    (*stats).tx_bytes = (*dev_stats).tx_bytes; (*stats).tx_packets = (*dev_stats).tx_packets;
}

pub unsafe fn ovs_vport_get_upcall_stats(vport: *mut Vport, skb: *mut SkBuff) -> i32 {
    let (mut tx_success, mut tx_fail) = (0u64, 0u64);
    for i in for_each_possible_cpu() {
        let stats = per_cpu_ptr((*vport).upcall_stats, i);
        let (n_success, n_fail);
        loop {
            let start = u64_stats_fetch_begin(&(*stats).syncp);
            n_success = u64_stats_read(&(*stats).n_success); n_fail = u64_stats_read(&(*stats).n_fail);
            if !u64_stats_fetch_retry(&(*stats).syncp, start) { break; }
        }
        tx_success += n_success; tx_fail += n_fail;
    }
    let nla = nla_nest_start_noflag(skb, OVS_VPORT_ATTR_UPCALL_STATS);
    if nla.is_null() { return -EMSGSIZE; }
    if nla_put_u64_64bit(skb, OVS_VPORT_UPCALL_ATTR_SUCCESS, tx_success, OVS_VPORT_ATTR_PAD) != 0 { nla_nest_cancel(skb, nla); return -EMSGSIZE; }
    if nla_put_u64_64bit(skb, OVS_VPORT_UPCALL_ATTR_FAIL, tx_fail, OVS_VPORT_ATTR_PAD) != 0 { nla_nest_cancel(skb, nla); return -EMSGSIZE; }
    nla_nest_end(skb, nla); 0
}

pub unsafe fn ovs_vport_set_upcall_portids(vport: *mut Vport, ids: *const Nlattr) -> i32 {
    let len = nla_len(ids);
    if len == 0 || len % core::mem::size_of::<u32>() != 0 || len / 4 > nr_cpu_ids() { return -EINVAL; }
    let old = ovsl_dereference((*vport).upcall_portids);
    let p = kmalloc(core::mem::size_of::<VportPortids>() + len, GFP_KERNEL) as *mut VportPortids;
    if p.is_null() { return -ENOMEM; }
    (*p).n_ids = len / 4; (*p).rn_ids = reciprocal_value((*p).n_ids); nla_memcpy((*p).ids.as_mut_ptr(), ids, len);
    rcu_assign_pointer(&mut (*vport).upcall_portids, p);
    if !old.is_null() { kfree_rcu(old, (*VportPortids).rcu); } 0
}

pub unsafe fn ovs_vport_get_upcall_portids(vport: *const Vport, skb: *mut SkBuff) -> i32 {
    let ids = rcu_dereference_ovsl((*vport).upcall_portids);
    if (*(*vport).dp).user_features & OVS_DP_F_VPORT_PIDS != 0 { nla_put(skb, OVS_VPORT_ATTR_UPCALL_PID, (*ids).n_ids * 4, (*ids).ids.as_ptr() as *const _ ) } else { nla_put_u32(skb, OVS_VPORT_ATTR_UPCALL_PID, (*ids).ids[0]) }
}

pub unsafe fn ovs_vport_find_upcall_portid(vport: *const Vport, skb: *mut SkBuff) -> u32 {
    let ids = rcu_dereference((*vport).upcall_portids);
    if (*ids).n_ids == 1 { return (*ids).ids[0]; }
    let hash = skb_get_hash(skb); let index = hash - (*ids).n_ids as u32 * reciprocal_divide(hash, (*ids).rn_ids);
    (*ids).ids[index as usize]
}

pub unsafe fn ovs_vport_receive(vport: *mut Vport, skb: *mut SkBuff, tun_info: *const IpTunnelInfo) -> i32 {
    let cb = OVS_CB(skb); (*cb).input_vport = vport; (*cb).mru = 0; (*cb).cutlen = u32::MAX; (*cb).probability = 0; (*cb).upcall_pid = 0;
    let mut tun_info = tun_info;
    if dev_net((*skb).dev) != ovs_dp_get_net((*vport).dp) { let mark = (*skb).mark; skb_scrub_packet(skb, true); (*skb).mark = mark; tun_info = core::ptr::null(); }
    let mut key = core::mem::MaybeUninit::<SwFlowKey>::uninit(); let error = ovs_flow_key_extract(tun_info, skb, key.as_mut_ptr());
    if error != 0 { kfree_skb(skb); return error; } ovs_dp_process_packet(skb, key.as_mut_ptr()); 0
}

unsafe fn packet_length(skb: *const SkBuff, dev: *mut NetDevice) -> i32 {
    let mut length = (*skb).len - (*dev).hard_header_len;
    if !skb_vlan_tag_present(skb) && eth_type_vlan((*skb).protocol) { length -= VLAN_HLEN; }
    if length > 0 { length } else { 0 }
}

pub unsafe fn ovs_vport_send(vport: *mut Vport, skb: *mut SkBuff, mac_proto: u8) {
    let mtu = (*(*vport).dev).mtu;
    match (*(*vport).dev).type_ {
        ARPHRD_NONE => { if mac_proto == MAC_PROTO_ETHERNET { skb_reset_network_header(skb); skb_reset_mac_len(skb); (*skb).protocol = htons(ETH_P_TEB); } else if mac_proto != MAC_PROTO_NONE { WARN_ON_ONCE(1); return drop_skb(skb); } }
        ARPHRD_ETHER => { if mac_proto != MAC_PROTO_ETHERNET { return drop_skb(skb); } }
        _ => return drop_skb(skb),
    }
    if packet_length(skb, (*vport).dev) > mtu && !skb_is_gso(skb) { (*(*vport).dev).stats.tx_errors += 1; if (*(*vport).dev).flags & IFF_UP != 0 { net_warn_ratelimited(); } return drop_skb(skb); }
    (*skb).dev = (*vport).dev; skb_clear_tstamp(skb); ((*(*vport).ops).send)(skb);
}

unsafe fn drop_skb(skb: *mut SkBuff) { kfree_skb(skb); }

// External types, constants, and functions are provided by the translated kernel/repository dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
