// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Userspace interface
 * Linux ethernet bridge
 */

/* Kernel dependencies are supplied by the surrounding translated tree. */

unsafe fn port_cost(dev: *mut net_device) -> i32 {
    let mut ecmd: ethtool_link_ksettings = core::mem::zeroed();

    if netif_get_link_ksettings(dev, &mut ecmd) == 0 {
        match ecmd.base.speed {
            SPEED_10000 => return 2,
            SPEED_5000 => return 3,
            SPEED_2500 => return 4,
            SPEED_1000 => return 5,
            SPEED_100 => return 19,
            SPEED_10 | SPEED_UNKNOWN => return 100,
            speed if speed > SPEED_10000 => return 1,
            _ => {}
        }
    }

    if libc::strncmp((*dev).name.as_ptr() as *const _, b"lec\0".as_ptr() as *const _, 3) == 0 {
        return 7;
    }
    if libc::strncmp((*dev).name.as_ptr() as *const _, b"plip\0".as_ptr() as *const _, 4) == 0 {
        return 2500;
    }
    100
}

pub unsafe fn br_port_carrier_check(p: *mut net_bridge_port, notified: *mut bool) {
    let dev = (*p).dev;
    let br = (*p).br;
    if !test_bit(BR_ADMIN_COST_BIT, &(*p).flags) && netif_running(dev) && netif_oper_up(dev) {
        core::ptr::write_volatile(&mut (*p).path_cost, port_cost(dev));
    }
    *notified = false;
    if !netif_running((*br).dev) { return; }
    spin_lock_bh(&mut (*br).lock);
    if netif_running(dev) && netif_oper_up(dev) {
        if (*p).state == BR_STATE_DISABLED { br_stp_enable_port(p); *notified = true; }
    } else if (*p).state != BR_STATE_DISABLED {
        br_stp_disable_port(p); *notified = true;
    }
    spin_unlock_bh(&mut (*br).lock);
}

unsafe fn br_port_set_promisc(p: *mut net_bridge_port) {
    if br_promisc_port(p) { return; }
    if dev_set_promiscuity((*p).dev, 1) != 0 { return; }
    br_fdb_unsync_static((*p).br, p);
    set_bit(BR_PROMISC_BIT, &mut (*p).flags);
}

unsafe fn br_port_clear_promisc(p: *mut net_bridge_port) {
    if !br_promisc_port(p) || ((*(*p).dev).priv_flags & IFF_UNICAST_FLT) == 0 { return; }
    if br_fdb_sync_static((*p).br, p) != 0 { return; }
    dev_set_promiscuity((*p).dev, -1);
    clear_bit(BR_PROMISC_BIT, &mut (*p).flags);
}

pub unsafe fn br_manage_promisc(br: *mut net_bridge) {
    let set_all = ((*(*br).dev).flags & IFF_PROMISC) != 0 || !br_vlan_enabled((*br).dev);
    let mut p = (*br).port_list.next;
    while p != &mut (*br).port_list as *mut _ {
        let port = list_entry(p, net_bridge_port, list);
        if set_all { br_port_set_promisc(port); }
        else if ((*(*port).dev).priv_flags & IFF_UNICAST_FLT) != 0 &&
            ((*br).auto_cnt == 0 || ((*br).auto_cnt == 1 && br_auto_port(port))) {
            br_port_clear_promisc(port);
        } else { br_port_set_promisc(port); }
        p = (*p).next;
    }
}

pub unsafe fn nbp_backup_change(p: *mut net_bridge_port, backup_dev: *mut net_device) -> i32 {
    let old_backup = rtnl_dereference((*p).backup_port);
    let mut backup_p: *mut net_bridge_port = core::ptr::null_mut();
    ASSERT_RTNL!();
    if !backup_dev.is_null() {
        if !netif_is_bridge_port(backup_dev) { return -ENOENT; }
        backup_p = br_port_get_rtnl(backup_dev);
        if (*backup_p).br != (*p).br { return -EINVAL; }
    }
    if p == backup_p { return -EINVAL; }
    if old_backup == backup_p { return 0; }
    if !old_backup.is_null() { (*old_backup).backup_redirected_cnt -= 1; }
    if !backup_p.is_null() { (*backup_p).backup_redirected_cnt += 1; }
    rcu_assign_pointer!((*p).backup_port, backup_p);
    0
}

unsafe fn nbp_backup_clear(p: *mut net_bridge_port) {
    nbp_backup_change(p, core::ptr::null_mut());
    if (*p).backup_redirected_cnt != 0 {
        let mut it = (*(*p).br).port_list.next;
        while it != &mut (*(*p).br).port_list as *mut _ {
            let cur_p = list_entry(it, net_bridge_port, list);
            if rtnl_dereference((*cur_p).backup_port) == p { nbp_backup_change(cur_p, core::ptr::null_mut()); }
            it = (*it).next;
        }
    }
    WARN_ON!(rcu_access_pointer((*p).backup_port) != core::ptr::null_mut() || (*p).backup_redirected_cnt != 0);
}

unsafe fn nbp_update_port_count(br: *mut net_bridge) {
    let mut cnt: u32 = 0;
    let mut it = (*br).port_list.next;
    while it != &mut (*br).port_list as *mut _ { if br_auto_port(list_entry(it, net_bridge_port, list)) { cnt += 1; } it = (*it).next; }
    if (*br).auto_cnt != cnt { (*br).auto_cnt = cnt; br_manage_promisc(br); }
}

unsafe fn nbp_delete_promisc(p: *mut net_bridge_port) {
    dev_set_allmulti((*p).dev, -1);
    if br_promisc_port(p) { dev_set_promiscuity((*p).dev, -1); } else { br_fdb_unsync_static((*p).br, p); }
}

unsafe fn release_nbp(kobj: *mut kobject) { kfree(container_of!(kobj, net_bridge_port, kobj)); }
unsafe fn brport_get_ownership(kobj: *const kobject, uid: *mut kuid_t, gid: *mut kgid_t) {
    let p = kobj_to_brport(kobj); net_ns_get_ownership(dev_net((*p).dev), uid, gid);
}

#[cfg(CONFIG_SYSFS)]
static brport_ktype: kobj_type = kobj_type { sysfs_ops: &brport_sysfs_ops, release: release_nbp, get_ownership: brport_get_ownership };
#[cfg(not(CONFIG_SYSFS))]
static brport_ktype: kobj_type = kobj_type { release: release_nbp, get_ownership: brport_get_ownership };

unsafe fn destroy_nbp(p: *mut net_bridge_port) {
    let dev = (*p).dev; (*p).br = core::ptr::null_mut(); (*p).dev = core::ptr::null_mut(); netdev_put(dev, &mut (*p).dev_tracker); kobject_put(&mut (*p).kobj);
}
unsafe fn destroy_nbp_rcu(head: *mut rcu_head) { destroy_nbp(container_of!(head, net_bridge_port, rcu)); }

unsafe fn get_max_headroom(br: *mut net_bridge) -> u32 {
    let mut max = 0; let mut it = (*br).port_list.next;
    while it != &mut (*br).port_list as *mut _ { let p = list_entry(it, net_bridge_port, list); max = max.max(netdev_get_fwd_headroom((*p).dev)); it = (*it).next; } max
}
unsafe fn update_headroom(br: *mut net_bridge, new_hr: i32) { let mut it = (*br).port_list.next; while it != &mut (*br).port_list as *mut _ { netdev_set_rx_headroom((*list_entry(it, net_bridge_port, list)).dev, new_hr); it = (*it).next; } (*(*br).dev).needed_headroom = new_hr; }

unsafe fn del_nbp(p: *mut net_bridge_port) {
    let br = (*p).br; let dev = (*p).dev;
    sysfs_remove_link((*br).ifobj, (*dev).name.as_ptr()); nbp_delete_promisc(p);
    spin_lock_bh(&mut (*br).lock); br_stp_disable_port(p); spin_unlock_bh(&mut (*br).lock);
    br_mrp_port_del(br, p); br_cfm_port_del(br, p); br_ifinfo_notify(RTM_DELLINK, core::ptr::null_mut(), p);
    list_del_rcu(&mut (*p).list);
    if netdev_get_fwd_headroom(dev) == (*(*br).dev).needed_headroom { update_headroom(br, get_max_headroom(br) as i32); }
    netdev_reset_rx_headroom(dev); nbp_vlan_flush(p); br_fdb_delete_by_port(br, p, 0, 1); switchdev_deferred_process(); nbp_backup_clear(p); nbp_update_port_count(br);
    netdev_upper_dev_unlink(dev, (*br).dev); (*dev).priv_flags &= !IFF_BRIDGE_PORT; netdev_rx_handler_unregister(dev); br_multicast_del_port(p);
    kobject_uevent(&mut (*p).kobj, KOBJ_REMOVE); kobject_del(&mut (*p).kobj); br_netpoll_disable(p); call_rcu(&mut (*p).rcu, destroy_nbp_rcu);
}

unsafe fn find_portno(br: *mut net_bridge) -> i32 {
    let inuse = bitmap_zalloc(BR_MAX_PORTS, GFP_KERNEL); if inuse.is_null() { return -ENOMEM; }
    __set_bit(0, inuse); let mut it = (*br).port_list.next; while it != &mut (*br).port_list as *mut _ { __set_bit((*list_entry(it, net_bridge_port, list)).port_no, inuse); it = (*it).next; }
    let index = find_first_zero_bit(inuse, BR_MAX_PORTS); bitmap_free(inuse); if index >= BR_MAX_PORTS { -EXFULL } else { index as i32 }
}

unsafe fn new_nbp(br: *mut net_bridge, dev: *mut net_device) -> *mut net_bridge_port {
    let index = find_portno(br); if index < 0 { return ERR_PTR(index); }
    let p = kzalloc_obj::<net_bridge_port>(); if p.is_null() { return ERR_PTR(-ENOMEM); }
    (*p).br = br; netdev_hold(dev, &mut (*p).dev_tracker, GFP_KERNEL); (*p).dev = dev; netdev_lock_ops(dev); (*p).path_cost = port_cost(dev); netdev_unlock_ops(dev);
    (*p).priority = 0x8000 >> BR_PORT_BITS; (*p).port_no = index as _; (*p).flags = BR_LEARNING | BR_FLOOD | BR_MCAST_FLOOD | BR_BCAST_FLOOD; br_init_port(p); br_set_state(p, BR_STATE_DISABLED); br_stp_port_timer_init(p);
    let err = br_multicast_add_port(p); if err != 0 { netdev_put(dev, &mut (*p).dev_tracker); kfree(p); return ERR_PTR(err); } p
}

// The remaining bridge deletion and creation routines preserve the C call sequence.
pub unsafe fn br_dev_delete(dev: *mut net_device, head: *mut list_head) { let br = netdev_priv(dev) as *mut net_bridge; let mut it = (*br).port_list.next; while it != &mut (*br).port_list as *mut _ { let p = list_entry(it, net_bridge_port, list); let next = (*it).next; del_nbp(p); it = next; } br_mst_uninit(br); br_recalculate_neigh_suppress_enabled(br); br_fdb_delete_by_port(br, core::ptr::null_mut(), 0, 1); timer_shutdown_sync(&mut (*br).hello_timer); timer_shutdown_sync(&mut (*br).topology_change_timer); timer_shutdown_sync(&mut (*br).tcn_timer); cancel_delayed_work_sync(&mut (*br).gc_work); br_sysfs_delbr((*br).dev); unregister_netdevice_queue((*br).dev, head); }

pub unsafe fn br_add_bridge(net: *mut net, name: *const i8) -> i32 { let dev = alloc_netdev(core::mem::size_of::<net_bridge>(), name, NET_NAME_UNKNOWN, br_dev_setup); if dev.is_null() { return -ENOMEM; } dev_net_set(dev, net); (*dev).rtnl_link_ops = &br_link_ops; let res = register_netdevice(dev); if res != 0 { free_netdev(dev); } res }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
