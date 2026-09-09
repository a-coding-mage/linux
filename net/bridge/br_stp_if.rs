// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Spanning tree protocol; interface code
 * Linux ethernet bridge
 */

#[inline]
unsafe fn br_make_port_id(priority: u8, port_no: u16) -> port_id {
    ((priority as u16) << BR_PORT_BITS) | (port_no & ((1 << BR_PORT_BITS) - 1))
}

const BR_MAX_PORT_PRIORITY: u16 = !0u16 >> BR_PORT_BITS;

pub unsafe fn br_init_port(p: *mut net_bridge_port) {
    let mut err: i32;
    (*p).port_id = br_make_port_id((*p).priority, (*p).port_no);
    br_become_designated_port(p);
    br_set_state(p, BR_STATE_BLOCKING);
    (*p).topology_change_ack = 0;
    (*p).config_pending = 0;
    err = __set_ageing_time((*p).dev, (*(*p).br).ageing_time);
    if err != 0 { netdev_err((*p).dev, "failed to offload ageing time\n"); }
}

pub unsafe fn br_stp_enable_bridge(br: *mut net_bridge) {
    spin_lock_bh(&mut (*br).lock);
    if (*br).stp_enabled == BR_KERNEL_STP { mod_timer(&mut (*br).hello_timer, jiffies + (*br).hello_time); }
    mod_delayed_work(system_long_wq, &mut (*br).gc_work, HZ / 10);
    br_config_bpdu_generation(br);
    list_for_each_entry(p, &(*br).port_list, list, {
        if netif_running((*p).dev) && netif_oper_up((*p).dev) { br_stp_enable_port(p); }
    });
    spin_unlock_bh(&mut (*br).lock);
}

pub unsafe fn br_stp_disable_bridge(br: *mut net_bridge) {
    spin_lock_bh(&mut (*br).lock);
    list_for_each_entry(p, &(*br).port_list, list, {
        if (*p).state != BR_STATE_DISABLED { br_stp_disable_port(p); }
    });
    __br_set_topology_change(br, 0);
    (*br).topology_change_detected = 0;
    spin_unlock_bh(&mut (*br).lock);
    timer_delete_sync(&mut (*br).hello_timer);
    timer_delete_sync(&mut (*br).topology_change_timer);
    timer_delete_sync(&mut (*br).tcn_timer);
    cancel_delayed_work_sync(&mut (*br).gc_work);
}

pub unsafe fn br_stp_enable_port(p: *mut net_bridge_port) {
    br_init_port(p);
    br_port_state_selection((*p).br);
    br_ifinfo_notify(RTM_NEWLINK, core::ptr::null_mut(), p);
}

pub unsafe fn br_stp_disable_port(p: *mut net_bridge_port) {
    let br = (*p).br;
    let wasroot = br_is_root_bridge(br);
    br_become_designated_port(p);
    br_set_state(p, BR_STATE_DISABLED);
    (*p).topology_change_ack = 0;
    (*p).config_pending = 0;
    br_ifinfo_notify(RTM_NEWLINK, core::ptr::null_mut(), p);
    timer_delete(&mut (*p).message_age_timer);
    timer_delete(&mut (*p).forward_delay_timer);
    timer_delete(&mut (*p).hold_timer);
    if !rcu_access_pointer((*p).backup_port) { br_fdb_delete_by_port(br, p, 0, 0); }
    br_multicast_disable_port(p);
    br_configuration_update(br);
    br_port_state_selection(br);
    if br_is_root_bridge(br) && !wasroot { br_become_root_bridge(br); }
}

unsafe fn br_stp_call_user(br: *mut net_bridge, arg: *mut i8) -> i32 {
    let argv: [*mut i8; 4] = [BR_STP_PROG, (*br).dev.name, arg, core::ptr::null_mut()];
    let envp: [*mut i8; 1] = [core::ptr::null_mut()];
    let rc = call_usermodehelper(BR_STP_PROG, argv.as_ptr(), envp.as_ptr(), UMH_WAIT_PROC);
    if rc > 0 {
        if rc & 0xff != 0 { br_debug(br, "{} received signal {}\n", BR_STP_PROG, rc & 0x7f); }
        else { br_debug(br, "{} exited with code {}\n", BR_STP_PROG, (rc >> 8) & 0xff); }
    }
    rc
}

unsafe fn br_stp_start(br: *mut net_bridge) {
    let mut err = -ENOENT;
    if (*br).stp_mode == BR_STP_MODE_AUTO && net_eq(dev_net((*br).dev), &init_net) { err = br_stp_call_user(br, "start".as_ptr() as *mut i8); }
    if err != 0 && err != -ENOENT { br_err(br, "failed to start userspace STP ({})\n", err); }
    spin_lock_bh(&mut (*br).lock);
    if (*br).bridge_forward_delay < BR_MIN_FORWARD_DELAY { __br_set_forward_delay(br, BR_MIN_FORWARD_DELAY); }
    else if (*br).bridge_forward_delay > BR_MAX_FORWARD_DELAY { __br_set_forward_delay(br, BR_MAX_FORWARD_DELAY); }
    if (*br).stp_mode == BR_STP_MODE_USER || err == 0 {
        (*br).stp_enabled = BR_USER_STP; (*br).stp_helper_active = err == 0;
        br_debug(br, "userspace STP started\n");
    } else {
        (*br).stp_enabled = BR_KERNEL_STP; br_debug(br, "using kernel STP\n");
        if (*(*br).dev).flags & IFF_UP != 0 { mod_timer(&mut (*br).hello_timer, jiffies + (*br).hello_time); }
        br_port_state_selection(br);
    }
    spin_unlock_bh(&mut (*br).lock);
}

unsafe fn br_stp_stop(br: *mut net_bridge) {
    if (*br).stp_enabled == BR_USER_STP {
        if (*br).stp_helper_active {
            let err = br_stp_call_user(br, "stop".as_ptr() as *mut i8);
            if err != 0 { br_err(br, "failed to stop userspace STP ({})\n", err); }
            (*br).stp_helper_active = false;
        }
        spin_lock_bh(&mut (*br).lock); br_port_state_selection(br); spin_unlock_bh(&mut (*br).lock);
    }
    (*br).stp_enabled = BR_NO_STP;
}

pub unsafe fn br_stp_set_enabled(br: *mut net_bridge, val: usize, extack: *mut netlink_ext_ack) -> i32 {
    ASSERT_RTNL();
    if br_mrp_enabled(br) { NL_SET_ERR_MSG_MOD(extack, "STP can't be enabled if MRP is already enabled"); return -EINVAL; }
    if val != 0 { if (*br).stp_enabled == BR_NO_STP { br_stp_start(br); } }
    else if (*br).stp_enabled != BR_NO_STP { br_stp_stop(br); }
    0
}

pub unsafe fn br_stp_change_bridge_id(br: *mut net_bridge, addr: *const u8) {
    let mut oldaddr_aligned = [0u16; ETH_ALEN >> 1];
    let oldaddr = oldaddr_aligned.as_mut_ptr() as *mut u8;
    let wasroot = br_is_root_bridge(br);
    br_fdb_change_mac_address(br, addr);
    memcpy(oldaddr, (*br).bridge_id.addr.as_mut_ptr(), ETH_ALEN);
    memcpy((*br).bridge_id.addr.as_mut_ptr(), addr, ETH_ALEN);
    eth_hw_addr_set((*br).dev, addr);
    list_for_each_entry(p, &(*br).port_list, list, {
        if ether_addr_equal((*p).designated_bridge.addr.as_ptr(), oldaddr) { memcpy((*p).designated_bridge.addr.as_mut_ptr(), addr, ETH_ALEN); }
        if ether_addr_equal((*p).designated_root.addr.as_ptr(), oldaddr) { memcpy((*p).designated_root.addr.as_mut_ptr(), addr, ETH_ALEN); }
    });
    br_configuration_update(br); br_port_state_selection(br);
    if br_is_root_bridge(br) && !wasroot { br_become_root_bridge(br); }
}

static BR_MAC_ZERO_ALIGNED: [u16; ETH_ALEN >> 1] = [0; ETH_ALEN >> 1];

pub unsafe fn br_stp_recalculate_bridge_id(br: *mut net_bridge) -> bool {
    let br_mac_zero = BR_MAC_ZERO_ALIGNED.as_ptr() as *const u8;
    let mut addr = br_mac_zero;
    if (*(*br).dev).addr_assign_type == NET_ADDR_SET { return false; }
    list_for_each_entry(p, &(*br).port_list, list, {
        if addr == br_mac_zero || memcmp((*p).dev.dev_addr.as_ptr(), addr, ETH_ALEN) < 0 { addr = (*p).dev.dev_addr.as_ptr(); }
    });
    if ether_addr_equal((*br).bridge_id.addr.as_ptr(), addr) { return false; }
    br_stp_change_bridge_id(br, addr); true
}

pub unsafe fn br_stp_set_bridge_priority(br: *mut net_bridge, newprio: u16) {
    spin_lock_bh(&mut (*br).lock);
    let wasroot = br_is_root_bridge(br);
    list_for_each_entry(p, &(*br).port_list, list, {
        if (*p).state != BR_STATE_DISABLED && br_is_designated_port(p) { (*p).designated_bridge.prio[0] = (newprio >> 8) as u8; (*p).designated_bridge.prio[1] = newprio as u8; }
    });
    (*br).bridge_id.prio[0] = (newprio >> 8) as u8; (*br).bridge_id.prio[1] = newprio as u8;
    br_configuration_update(br); br_port_state_selection(br);
    if br_is_root_bridge(br) && !wasroot { br_become_root_bridge(br); }
    spin_unlock_bh(&mut (*br).lock);
}

pub unsafe fn br_stp_set_port_priority(p: *mut net_bridge_port, newprio: usize) -> i32 {
    if newprio > BR_MAX_PORT_PRIORITY as usize { return -ERANGE; }
    let new_port_id = br_make_port_id(newprio as u8, (*p).port_no);
    if br_is_designated_port(p) { (*p).designated_port = new_port_id; }
    (*p).port_id = new_port_id; (*p).priority = newprio as u8;
    if memcmp(&(*(*p).br).bridge_id as *const _ as *const u8, &(*p).designated_bridge as *const _ as *const u8, 8) == 0 && (*p).port_id < (*p).designated_port { br_become_designated_port(p); br_port_state_selection((*p).br); }
    0
}

pub unsafe fn br_stp_set_path_cost(p: *mut net_bridge_port, path_cost: usize) -> i32 {
    if path_cost < BR_MIN_PATH_COST || path_cost > BR_MAX_PATH_COST { return -ERANGE; }
    set_bit(BR_ADMIN_COST_BIT, &mut (*p).flags); (*p).path_cost = path_cost; br_configuration_update((*p).br); br_port_state_selection((*p).br); 0
}

pub unsafe fn br_show_bridge_id(buf: *mut i8, id: *const bridge_id) -> isize {
    sysfs_emit(buf, "{:02x}{:02x}.{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}\n", (*id).prio[0], (*id).prio[1], (*id).addr[0], (*id).addr[1], (*id).addr[2], (*id).addr[3], (*id).addr[4], (*id).addr[5])
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
