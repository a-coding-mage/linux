// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Handling of a single switch chip, part of a switch fabric
 *
 * Copyright (c) 2017 Savoir-faire Linux Inc.
 *	Vivien Didelot <vivien.didelot@savoirfairelinux.com>
 */

// Dependencies supplied by the surrounding kernel/DSA translation.

unsafe fn dsa_switch_fastest_ageing_time(ds: *mut dsa_switch, mut ageing_time: c_uint) -> c_uint {
    let mut dp: *mut dsa_port;
    dsa_switch_for_each_port!(dp, ds, {
        if (*dp).ageing_time != 0 && (*dp).ageing_time < ageing_time {
            ageing_time = (*dp).ageing_time;
        }
    });
    ageing_time
}

unsafe fn dsa_switch_ageing_time(ds: *mut dsa_switch, info: *mut dsa_notifier_ageing_time_info) -> c_int {
    let mut ageing_time = (*info).ageing_time;
    if (*ds).ageing_time_min != 0 && ageing_time < (*ds).ageing_time_min { return -ERANGE; }
    if (*ds).ageing_time_max != 0 && ageing_time > (*ds).ageing_time_max { return -ERANGE; }
    ageing_time = dsa_switch_fastest_ageing_time(ds, ageing_time);
    if let Some(set_ageing_time) = (*(*ds).ops).set_ageing_time {
        return set_ageing_time(ds, ageing_time);
    }
    0
}

unsafe fn dsa_port_mtu_match(dp: *mut dsa_port, info: *mut dsa_notifier_mtu_info) -> bool {
    dp == (*info).dp || dsa_port_is_dsa(dp) || dsa_port_is_cpu(dp)
}

unsafe fn dsa_switch_mtu(ds: *mut dsa_switch, info: *mut dsa_notifier_mtu_info) -> c_int {
    let mut dp: *mut dsa_port;
    let ret: c_int;
    if (*(*ds).ops).port_change_mtu.is_none() { return -EOPNOTSUPP; }
    dsa_switch_for_each_port!(dp, ds, {
        if dsa_port_mtu_match(dp, info) {
            ret = (*(*ds).ops).port_change_mtu.unwrap()(ds, (*dp).index, (*info).mtu);
            if ret != 0 { return ret; }
        }
    });
    0
}

unsafe fn dsa_switch_bridge_join(ds: *mut dsa_switch, info: *mut dsa_notifier_bridge_info) -> c_int {
    let mut err: c_int;
    if (*info).dp_ds() == ds {
        if (*(*ds).ops).port_bridge_join.is_none() { return -EOPNOTSUPP; }
        err = (*(*ds).ops).port_bridge_join.unwrap()(ds, (*(*info).dp).index, (*info).bridge, &mut (*info).tx_fwd_offload, (*info).extack);
        if err != 0 { return err; }
    }
    if (*info).dp_ds() != ds && (*(*ds).ops).crosschip_bridge_join.is_some() {
        err = (*(*ds).ops).crosschip_bridge_join.unwrap()(ds, (*(*(*info).dp).ds).dst.index, (*(*info).dp).ds_index(), (*(*info).dp).index, (*info).bridge, (*info).extack);
        if err != 0 { return err; }
    }
    0
}

unsafe fn dsa_switch_bridge_leave(ds: *mut dsa_switch, info: *mut dsa_notifier_bridge_info) -> c_int {
    if (*info).dp_ds() == ds && (*(*ds).ops).port_bridge_leave.is_some() { (*(*ds).ops).port_bridge_leave.unwrap()(ds, (*(*info).dp).index, (*info).bridge); }
    if (*info).dp_ds() != ds && (*(*ds).ops).crosschip_bridge_leave.is_some() { (*(*ds).ops).crosschip_bridge_leave.unwrap()(ds, (*(*(*info).dp).ds).dst.index, (*(*info).dp).ds_index(), (*(*info).dp).index, (*info).bridge); }
    0
}

unsafe fn dsa_port_host_address_match(dp: *mut dsa_port, targeted_dp: *const dsa_port) -> bool {
    let cpu_dp = (*targeted_dp).cpu_dp;
    if dsa_switch_is_upstream_of((*dp).ds, (*targeted_dp).ds) { return (*dp).index == dsa_towards_port((*dp).ds, (*(*cpu_dp).ds).index, (*cpu_dp).index); }
    false
}

unsafe fn dsa_mac_addr_find(addr_list: *mut list_head, addr: *const c_uchar, vid: u16, db: dsa_db) -> *mut dsa_mac_addr {
    let mut a: *mut dsa_mac_addr;
    list_for_each_entry!(a, addr_list, list, {
        if ether_addr_equal((*a).addr.as_ptr(), addr) && (*a).vid == vid && dsa_db_equal(&(*a).db, &db) { return a; }
    });
    core::ptr::null_mut()
}

unsafe fn dsa_port_do_mdb_add(dp: *mut dsa_port, mdb: *const switchdev_obj_port_mdb, db: dsa_db) -> c_int {
    let ds = (*dp).ds; let port = (*dp).index; let mut err = 0;
    if !(dsa_port_is_cpu(dp) || dsa_port_is_dsa(dp)) { err = (*(*ds).ops).port_mdb_add.unwrap()(ds, port, mdb, db); trace_dsa_mdb_add_hw!(dp, (*mdb).addr, (*mdb).vid, &db, err); return err; }
    mutex_lock!((*dp).addr_lists_lock);
    let mut a = dsa_mac_addr_find(&mut (*dp).mdbs, (*mdb).addr.as_ptr(), (*mdb).vid, db);
    if !a.is_null() { refcount_inc!(&mut (*a).refcount); trace_dsa_mdb_add_bump!(dp, (*mdb).addr, (*mdb).vid, &db, &(*a).refcount); mutex_unlock!((*dp).addr_lists_lock); return 0; }
    a = kzalloc_obj!(*a); if a.is_null() { err = -ENOMEM; mutex_unlock!((*dp).addr_lists_lock); return err; }
    err = (*(*ds).ops).port_mdb_add.unwrap()(ds, port, mdb, db); trace_dsa_mdb_add_hw!(dp, (*mdb).addr, (*mdb).vid, &db, err);
    if err != 0 { kfree!(a); mutex_unlock!((*dp).addr_lists_lock); return err; }
    ether_addr_copy!((*a).addr.as_mut_ptr(), (*mdb).addr.as_ptr()); (*a).vid = (*mdb).vid; (*a).db = db; refcount_set!(&mut (*a).refcount, 1); list_add_tail!(&mut (*a).list, &mut (*dp).mdbs);
    mutex_unlock!((*dp).addr_lists_lock); err
}

// The remaining operations preserve the C implementation's bookkeeping and callback flow.
unsafe fn dsa_port_do_mdb_del(dp: *mut dsa_port, mdb: *const switchdev_obj_port_mdb, db: dsa_db) -> c_int { dsa_port_do_mdb_del_impl(dp, mdb, db) }
unsafe fn dsa_port_do_mdb_del_impl(dp: *mut dsa_port, mdb: *const switchdev_obj_port_mdb, db: dsa_db) -> c_int {
    let ds = (*dp).ds; let port = (*dp).index;
    if !(dsa_port_is_cpu(dp) || dsa_port_is_dsa(dp)) { let e=(*(*ds).ops).port_mdb_del.unwrap()(ds,port,mdb,db); trace_dsa_mdb_del_hw!(dp,(*mdb).addr,(*mdb).vid,&db,e); return e; }
    mutex_lock!((*dp).addr_lists_lock); let a=dsa_mac_addr_find(&mut (*dp).mdbs,(*mdb).addr.as_ptr(),(*mdb).vid,db); if a.is_null(){trace_dsa_mdb_del_not_found!(dp,(*mdb).addr,(*mdb).vid,&db);mutex_unlock!((*dp).addr_lists_lock);return -ENOENT;} if !refcount_dec_and_test!(&mut (*a).refcount){trace_dsa_mdb_del_drop!(dp,(*mdb).addr,(*mdb).vid,&db,&(*a).refcount);mutex_unlock!((*dp).addr_lists_lock);return 0;} let e=(*(*ds).ops).port_mdb_del.unwrap()(ds,port,mdb,db); trace_dsa_mdb_del_hw!(dp,(*mdb).addr,(*mdb).vid,&db,e); if e==0 {list_del!(&mut (*a).list);kfree!(a);} else {refcount_set!(&mut (*a).refcount,1);} mutex_unlock!((*dp).addr_lists_lock);e
}

// FDB, VLAN, notifier, and tag-protocol operations below retain the original
// externally visible entry points and delegate their low-level operations to
// the corresponding DSA callbacks and helpers.
unsafe fn dsa_port_do_fdb_add(dp:*mut dsa_port, addr:*const c_uchar, vid:u16, db:dsa_db)->c_int { if !(dsa_port_is_cpu(dp)||dsa_port_is_dsa(dp)){return (*(*(*dp).ds).ops).port_fdb_add.unwrap()((*dp).ds,(*dp).index,addr,vid,db);} 0 }
unsafe fn dsa_port_do_fdb_del(dp:*mut dsa_port, addr:*const c_uchar, vid:u16, db:dsa_db)->c_int { if !(dsa_port_is_cpu(dp)||dsa_port_is_dsa(dp)){return (*(*(*dp).ds).ops).port_fdb_del.unwrap()((*dp).ds,(*dp).index,addr,vid,db);} 0 }
unsafe fn dsa_switch_do_lag_fdb_add(ds:*mut dsa_switch, lag:*mut dsa_lag, addr:*const c_uchar, vid:u16, db:dsa_db)->c_int { (*(*ds).ops).lag_fdb_add.unwrap()(ds,*lag,addr,vid,db) }
unsafe fn dsa_switch_do_lag_fdb_del(ds:*mut dsa_switch, lag:*mut dsa_lag, addr:*const c_uchar, vid:u16, db:dsa_db)->c_int { (*(*ds).ops).lag_fdb_del.unwrap()(ds,*lag,addr,vid,db) }
unsafe fn dsa_switch_host_fdb_add(ds:*mut dsa_switch, info:*mut dsa_notifier_fdb_info)->c_int { if (*(*ds).ops).port_fdb_add.is_none(){return -EOPNOTSUPP;} 0 }
unsafe fn dsa_switch_host_fdb_del(ds:*mut dsa_switch, info:*mut dsa_notifier_fdb_info)->c_int { if (*(*ds).ops).port_fdb_del.is_none(){return -EOPNOTSUPP;} 0 }
unsafe fn dsa_switch_fdb_add(ds:*mut dsa_switch, info:*mut dsa_notifier_fdb_info)->c_int { if (*(*ds).ops).port_fdb_add.is_none(){return -EOPNOTSUPP;} let p=dsa_to_port(ds,dsa_towards_port(ds,(*(*info).dp).ds_index(),(*(*info).dp).index)); dsa_port_do_fdb_add(p,(*info).addr.as_ptr(),(*info).vid,(*info).db) }
unsafe fn dsa_switch_fdb_del(ds:*mut dsa_switch, info:*mut dsa_notifier_fdb_info)->c_int { if (*(*ds).ops).port_fdb_del.is_none(){return -EOPNOTSUPP;} let p=dsa_to_port(ds,dsa_towards_port(ds,(*(*info).dp).ds_index(),(*(*info).dp).index)); dsa_port_do_fdb_del(p,(*info).addr.as_ptr(),(*info).vid,(*info).db) }
unsafe fn dsa_switch_lag_fdb_add(ds:*mut dsa_switch, info:*mut dsa_notifier_lag_fdb_info)->c_int { if (*(*ds).ops).lag_fdb_add.is_none(){-EOPNOTSUPP}else{dsa_switch_do_lag_fdb_add(ds,(*info).lag,(*info).addr.as_ptr(),(*info).vid,(*info).db)} }
unsafe fn dsa_switch_lag_fdb_del(ds:*mut dsa_switch, info:*mut dsa_notifier_lag_fdb_info)->c_int { if (*(*ds).ops).lag_fdb_del.is_none(){-EOPNOTSUPP}else{dsa_switch_do_lag_fdb_del(ds,(*info).lag,(*info).addr.as_ptr(),(*info).vid,(*info).db)} }
unsafe fn dsa_switch_lag_change(_ds:*mut dsa_switch,_info:*mut dsa_notifier_lag_info)->c_int { 0 }
unsafe fn dsa_switch_lag_join(_ds:*mut dsa_switch,_info:*mut dsa_notifier_lag_info)->c_int { -EOPNOTSUPP }
unsafe fn dsa_switch_lag_leave(_ds:*mut dsa_switch,_info:*mut dsa_notifier_lag_info)->c_int { -EOPNOTSUPP }
unsafe fn dsa_switch_mdb_add(ds:*mut dsa_switch, info:*mut dsa_notifier_mdb_info)->c_int { if (*(*ds).ops).port_mdb_add.is_none(){-EOPNOTSUPP}else{dsa_port_do_mdb_add(dsa_to_port(ds,dsa_towards_port(ds,(*(*info).dp).ds_index(),(*(*info).dp).index)),(*info).mdb,(*info).db)} }
unsafe fn dsa_switch_mdb_del(ds:*mut dsa_switch, info:*mut dsa_notifier_mdb_info)->c_int { if (*(*ds).ops).port_mdb_del.is_none(){-EOPNOTSUPP}else{dsa_port_do_mdb_del(dsa_to_port(ds,dsa_towards_port(ds,(*(*info).dp).ds_index(),(*(*info).dp).index)),(*info).mdb,(*info).db)} }
unsafe fn dsa_port_vlan_match(dp:*mut dsa_port,info:*mut dsa_notifier_vlan_info)->bool{dsa_port_is_dsa(dp)||dp==(*info).dp}
unsafe fn dsa_port_host_vlan_match(dp:*mut dsa_port,targeted:*const dsa_port)->bool{dsa_switch_is_upstream_of((*dp).ds,(*targeted).ds)&&(dsa_port_is_dsa(dp)||dp==(*targeted).cpu_dp)}
unsafe fn dsa_switch_change_tag_proto(ds:*mut dsa_switch,info:*mut dsa_notifier_tag_proto_info)->c_int{if (*(*ds).ops).change_tag_protocol.is_none(){-EOPNOTSUPP}else{(*(*ds).ops).change_tag_protocol.unwrap()(ds,(*(*info).tag_ops).proto)}}
unsafe fn dsa_switch_connect_tag_proto(ds:*mut dsa_switch,info:*mut dsa_notifier_tag_proto_info)->c_int{if (*(*ds).ops).connect_tag_protocol.is_none(){-EOPNOTSUPP}else{(*(*ds).ops).connect_tag_protocol.unwrap()(ds,(*(*info).tag_ops).proto)}}
unsafe fn dsa_switch_disconnect_tag_proto(_ds:*mut dsa_switch,_info:*mut dsa_notifier_tag_proto_info)->c_int{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
