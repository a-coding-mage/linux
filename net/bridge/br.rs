// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic parts
 * Linux ethernet bridge
 */

/* Kernel headers and br_private.h provide the external types, constants, and functions used below. */

unsafe fn br_device_event(
    _unused: *mut notifier_block,
    event: c_ulong,
    ptr: *mut c_void,
) -> c_int {
    let extack = netdev_notifier_info_to_extack(ptr);
    let mut prechaddr_info: *mut netdev_notifier_pre_changeaddr_info;
    let dev = netdev_notifier_info_to_dev(ptr);
    let mut p: *mut net_bridge_port;
    let mut br: *mut net_bridge;
    let mut notified = false;
    let mut changed_addr: bool;
    let mut err: c_int;

    if netif_is_bridge_master(dev) {
        br = netdev_priv(dev) as *mut net_bridge;

        if event == NETDEV_REGISTER {
            br_fdb_change_mac_address(br, (*dev).dev_addr);
        }

        err = br_vlan_bridge_event(dev, event, ptr);
        if err != 0 { return notifier_from_errno(err); }

        if event == NETDEV_REGISTER {
            err = br_sysfs_addbr(dev);
            if err != 0 { return notifier_from_errno(err); }
            return NOTIFY_DONE;
        }
    }

    if is_vlan_dev(dev) {
        let real_dev = vlan_dev_real_dev(dev);
        if netif_is_bridge_master(real_dev) {
            br_vlan_vlan_upper_event(real_dev, dev, event);
        }
    }

    p = br_port_get_rtnl(dev);
    if p.is_null() { return NOTIFY_DONE; }
    br = (*p).br;

    match event {
        NETDEV_CHANGEMTU => br_mtu_auto_adjust(br),
        NETDEV_PRE_CHANGEADDR => {
            if (*(*br).dev).addr_assign_type == NET_ADDR_SET { return NOTIFY_DONE; }
            prechaddr_info = ptr as *mut netdev_notifier_pre_changeaddr_info;
            err = netif_pre_changeaddr_notify((*br).dev, (*prechaddr_info).dev_addr, extack);
            if err != 0 { return notifier_from_errno(err); }
        },
        NETDEV_CHANGEADDR => {
            spin_lock_bh(&mut (*br).lock);
            br_fdb_changeaddr(p, (*dev).dev_addr);
            changed_addr = br_stp_recalculate_bridge_id(br);
            spin_unlock_bh(&mut (*br).lock);
            if changed_addr { call_netdevice_notifiers(NETDEV_CHANGEADDR, (*br).dev); }
        },
        NETDEV_CHANGE => br_port_carrier_check(p, &mut notified),
        NETDEV_FEAT_CHANGE => netdev_update_features((*br).dev),
        NETDEV_DOWN => {
            spin_lock_bh(&mut (*br).lock);
            if (*(*br).dev).flags & IFF_UP != 0 {
                br_stp_disable_port(p);
                notified = true;
            }
            spin_unlock_bh(&mut (*br).lock);
        },
        NETDEV_UP => {
            if netif_running((*br).dev) && netif_oper_up(dev) {
                spin_lock_bh(&mut (*br).lock);
                br_stp_enable_port(p);
                notified = true;
                spin_unlock_bh(&mut (*br).lock);
            }
        },
        NETDEV_UNREGISTER => br_del_if(br, dev),
        NETDEV_CHANGENAME => {
            err = br_sysfs_renameif(p);
            if err != 0 { return notifier_from_errno(err); }
        },
        NETDEV_PRE_TYPE_CHANGE => return NOTIFY_BAD,
        NETDEV_RESEND_IGMP => call_netdevice_notifiers(event, (*br).dev),
        _ => {}
    }

    if event != NETDEV_UNREGISTER { br_vlan_port_event(p, event); }
    if !notified && (event == NETDEV_CHANGEADDR || event == NETDEV_UP ||
                     event == NETDEV_CHANGE || event == NETDEV_DOWN) {
        br_ifinfo_notify(RTM_NEWLINK, core::ptr::null_mut(), p);
    }
    NOTIFY_DONE
}

static mut br_device_notifier: notifier_block = notifier_block { notifier_call: br_device_event };

unsafe fn br_switchdev_event(_unused: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    let dev = switchdev_notifier_info_to_dev(ptr);
    let p = br_port_get_rtnl_rcu(dev);
    if p.is_null() { return NOTIFY_DONE; }
    let br = (*p).br;
    let mut err = NOTIFY_DONE;
    match event {
        SWITCHDEV_FDB_ADD_TO_BRIDGE => {
            let f = ptr as *mut switchdev_notifier_fdb_info;
            err = br_fdb_external_learn_add(br, p, (*f).addr, (*f).vid, (*f).locked, false);
            if err != 0 { return notifier_from_errno(err); }
            br_fdb_offloaded_set(br, p, (*f).addr, (*f).vid, (*f).offloaded);
        },
        SWITCHDEV_FDB_DEL_TO_BRIDGE => {
            let f = ptr as *mut switchdev_notifier_fdb_info;
            err = br_fdb_external_learn_del(br, p, (*f).addr, (*f).vid, false);
            if err != 0 { err = notifier_from_errno(err); }
        },
        SWITCHDEV_FDB_OFFLOADED => {
            let f = ptr as *mut switchdev_notifier_fdb_info;
            br_fdb_offloaded_set(br, p, (*f).addr, (*f).vid, (*f).offloaded);
        },
        SWITCHDEV_FDB_FLUSH_TO_BRIDGE => {
            let f = ptr as *mut switchdev_notifier_fdb_info;
            br_fdb_delete_by_port(br, p, (*f).vid, 0);
        },
        _ => {}
    }
    err
}

static mut br_switchdev_notifier: notifier_block = notifier_block { notifier_call: br_switchdev_event };

unsafe fn br_switchdev_blocking_event(_nb: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    let extack = netdev_notifier_info_to_extack(ptr);
    let dev = switchdev_notifier_info_to_dev(ptr);
    let p = br_port_get_rtnl(dev);
    if p.is_null() { return NOTIFY_DONE; }
    let mut err = NOTIFY_DONE;
    match event {
        SWITCHDEV_BRPORT_OFFLOADED => {
            let i = ptr as *mut switchdev_notifier_brport_info;
            let b = &(*i).brport;
            err = br_switchdev_port_offload(p, b.dev, b.ctx, b.atomic_nb, b.blocking_nb, b.tx_fwd_offload, extack);
            err = notifier_from_errno(err);
        },
        SWITCHDEV_BRPORT_UNOFFLOADED => {
            let i = ptr as *mut switchdev_notifier_brport_info;
            let b = &(*i).brport;
            br_switchdev_port_unoffload(p, b.ctx, b.atomic_nb, b.blocking_nb);
        },
        SWITCHDEV_BRPORT_REPLAY => {
            let i = ptr as *mut switchdev_notifier_brport_info;
            let b = &(*i).brport;
            err = br_switchdev_port_replay(p, b.dev, b.ctx, b.atomic_nb, b.blocking_nb, extack);
            err = notifier_from_errno(err);
        },
        _ => {}
    }
    err
}

static mut br_switchdev_blocking_notifier: notifier_block = notifier_block { notifier_call: br_switchdev_blocking_event };

unsafe fn br_toggle_fdb_local_vlan_0(br: *mut net_bridge, on: bool, extack: *mut netlink_ext_ack) -> c_int {
    if br_opt_get(br, BROPT_FDB_LOCAL_VLAN_0) == on as c_int { return 0; }
    let err = br_fdb_toggle_local_vlan_0(br, on, extack);
    if err != 0 { return err; }
    br_opt_toggle(br, BROPT_FDB_LOCAL_VLAN_0, on);
    0
}

pub unsafe fn br_boolopt_toggle(br: *mut net_bridge, opt: br_boolopt_id, on: bool, extack: *mut netlink_ext_ack) -> c_int {
    let mut err = 0;
    match opt {
        BR_BOOLOPT_NO_LL_LEARN => br_opt_toggle(br, BROPT_NO_LL_LEARN, on),
        BR_BOOLOPT_MCAST_VLAN_SNOOPING => err = br_multicast_toggle_vlan_snooping(br, on, extack),
        BR_BOOLOPT_MST_ENABLE => err = br_mst_set_enabled(br, on, extack),
        BR_BOOLOPT_MDB_OFFLOAD_FAIL_NOTIFICATION => br_opt_toggle(br, BROPT_MDB_OFFLOAD_FAIL_NOTIFICATION, on),
        BR_BOOLOPT_FDB_LOCAL_VLAN_0 => err = br_toggle_fdb_local_vlan_0(br, on, extack),
        _ => { WARN_ON(1); }
    }
    err
}

pub unsafe fn br_boolopt_get(br: *const net_bridge, opt: br_boolopt_id) -> c_int {
    match opt {
        BR_BOOLOPT_NO_LL_LEARN => br_opt_get(br, BROPT_NO_LL_LEARN),
        BR_BOOLOPT_MCAST_VLAN_SNOOPING => br_opt_get(br, BROPT_MCAST_VLAN_SNOOPING_ENABLED),
        BR_BOOLOPT_MST_ENABLE => br_opt_get(br, BROPT_MST_ENABLED),
        BR_BOOLOPT_MDB_OFFLOAD_FAIL_NOTIFICATION => br_opt_get(br, BROPT_MDB_OFFLOAD_FAIL_NOTIFICATION),
        BR_BOOLOPT_FDB_LOCAL_VLAN_0 => br_opt_get(br, BROPT_FDB_LOCAL_VLAN_0),
        _ => { WARN_ON(1); 0 }
    }
}

pub unsafe fn br_boolopt_multi_toggle(br: *mut net_bridge, bm: *mut br_boolopt_multi, extack: *mut netlink_ext_ack) -> c_int {
    let mut bitmap = (*bm).optmask;
    let mut err = 0;
    let opt_id = find_next_bit(&mut bitmap, BITS_PER_LONG, BR_BOOLOPT_MAX);
    if opt_id != BITS_PER_LONG {
        NL_SET_ERR_MSG_FMT_MOD(extack, "Unknown boolean option %d", opt_id);
        return -EINVAL;
    }
    for_each_set_bit!(opt_id, &mut bitmap, BR_BOOLOPT_MAX, {
        let on = ((*bm).optval & BIT(opt_id)) != 0;
        err = br_boolopt_toggle(br, opt_id, on, extack);
        if err != 0 { br_debug(br, "boolopt multi-toggle error: option: %d current: %d new: %d error: %d\\n", opt_id, br_boolopt_get(br, opt_id), on, err); }
    });
    err
}

pub unsafe fn br_boolopt_multi_get(br: *const net_bridge, bm: *mut br_boolopt_multi) {
    let mut optval: u32 = 0;
    for opt_id in 0..BR_BOOLOPT_MAX { optval |= (br_boolopt_get(br, opt_id) << opt_id) as u32; }
    (*bm).optval = optval;
    (*bm).optmask = GENMASK(BR_BOOLOPT_MAX - 1, 0);
}

pub unsafe fn br_opt_toggle(br: *mut net_bridge, opt: net_bridge_opts, on: bool) {
    let cur = br_opt_get(br, opt) != 0;
    br_debug(br, "toggle option: %d state: %d -> %d\\n", opt, cur, on);
    if cur == on { return; }
    if on { set_bit(opt, &mut (*br).options); } else { clear_bit(opt, &mut (*br).options); }
}

unsafe fn br_net_exit_rtnl(net: *mut net, dev_to_kill: *mut list_head) {
    ASSERT_RTNL_NET(net);
    for_each_netdev!(net, dev, {
        if netif_is_bridge_master(dev) { br_dev_delete(dev, dev_to_kill); }
    });
}

static mut br_net_ops: pernet_operations = pernet_operations { exit_rtnl: br_net_exit_rtnl };
static br_stp_proto: stp_proto = stp_proto { rcv: br_stp_rcv };

unsafe fn br_init() -> c_int {
    BUILD_BUG_ON!(core::mem::size_of::<br_input_skb_cb>() > core::mem::size_of::<sk_buff_cb>());
    let mut err = stp_proto_register(&br_stp_proto);
    if err < 0 { pr_err!("bridge: can't register sap for STP\\n"); return err; }
    err = br_fdb_init(); if err != 0 { stp_proto_unregister(&br_stp_proto); return err; }
    err = register_pernet_subsys(&br_net_ops); if err != 0 { br_fdb_fini(); stp_proto_unregister(&br_stp_proto); return err; }
    err = br_nf_core_init(); if err != 0 { unregister_pernet_subsys(&br_net_ops); br_fdb_fini(); stp_proto_unregister(&br_stp_proto); return err; }
    err = register_netdevice_notifier(&br_device_notifier); if err != 0 { br_nf_core_fini(); unregister_pernet_subsys(&br_net_ops); br_fdb_fini(); stp_proto_unregister(&br_stp_proto); return err; }
    err = register_switchdev_notifier(&br_switchdev_notifier); if err != 0 { unregister_netdevice_notifier(&br_device_notifier); br_nf_core_fini(); unregister_pernet_subsys(&br_net_ops); br_fdb_fini(); stp_proto_unregister(&br_stp_proto); return err; }
    err = register_switchdev_blocking_notifier(&br_switchdev_blocking_notifier); if err != 0 { unregister_switchdev_notifier(&br_switchdev_notifier); unregister_netdevice_notifier(&br_device_notifier); br_nf_core_fini(); unregister_pernet_subsys(&br_net_ops); br_fdb_fini(); stp_proto_unregister(&br_stp_proto); return err; }
    err = br_netlink_init(); if err != 0 { unregister_switchdev_blocking_notifier(&br_switchdev_blocking_notifier); unregister_switchdev_notifier(&br_switchdev_notifier); unregister_netdevice_notifier(&br_device_notifier); br_nf_core_fini(); unregister_pernet_subsys(&br_net_ops); br_fdb_fini(); stp_proto_unregister(&br_stp_proto); return err; }
    brioctl_set(br_ioctl_stub);
    0
}

unsafe fn br_deinit() {
    stp_proto_unregister(&br_stp_proto);
    br_netlink_fini();
    unregister_switchdev_blocking_notifier(&br_switchdev_blocking_notifier);
    unregister_switchdev_notifier(&br_switchdev_notifier);
    unregister_netdevice_notifier(&br_device_notifier);
    brioctl_set(core::ptr::null_mut());
    unregister_pernet_subsys(&br_net_ops);
    rcu_barrier();
    br_nf_core_fini();
    br_fdb_fini();
}

module_init!(br_init);
module_exit!(br_deinit);
MODULE_LICENSE!("GPL");
MODULE_VERSION!(BR_VERSION);
MODULE_ALIAS_RTNL_LINK!("bridge");
MODULE_DESCRIPTION!("Ethernet bridge driver");
MODULE_IMPORT_NS!("NETDEV_INTERNAL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
