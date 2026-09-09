// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Handling of a conduit device, switching frames via its switch fabric CPU port
 *
 * Copyright (c) 2017 Savoir-faire Linux Inc.
 *\tVivien Didelot <vivien.didelot@savoirfairelinux.com>
 */

// Linux kernel declarations supplied by the surrounding translation unit.

unsafe fn dsa_conduit_get_regs_len(dev: *mut net_device) -> i32 {
    let cpu_dp = (*dev).dsa_ptr;
    let ops = (*cpu_dp).orig_ethtool_ops;
    let ds = (*cpu_dp).ds;
    let port = (*cpu_dp).index;
    let mut ret = 0;
    let mut len;

    if !ops.is_null() && (*ops).get_regs_len.is_some() {
        len = ((*ops).get_regs_len.unwrap())(dev);
        if len < 0 { return len; }
        ret += len;
    }
    ret += core::mem::size_of::<ethtool_drvinfo>() as i32;
    ret += core::mem::size_of::<ethtool_regs>() as i32;
    if (*(*ds).ops).get_regs_len.is_some() {
        len = ((*(*ds).ops).get_regs_len.unwrap())(ds, port);
        if len < 0 { return len; }
        ret += len;
    }
    ret
}

unsafe fn dsa_conduit_get_regs(dev: *mut net_device, regs: *mut ethtool_regs, mut data: *mut core::ffi::c_void) {
    let cpu_dp = (*dev).dsa_ptr;
    let ops = (*cpu_dp).orig_ethtool_ops;
    let ds = (*cpu_dp).ds;
    let port = (*cpu_dp).index;
    let mut len;
    if !ops.is_null() && (*ops).get_regs_len.is_some() && (*ops).get_regs.is_some() {
        len = ((*ops).get_regs_len.unwrap())(dev);
        if len < 0 { return; }
        (*regs).len = len as u32;
        ((*ops).get_regs.unwrap())(dev, regs, data);
        data = (data as *mut u8).add((*regs).len as usize) as *mut _;
    }
    let cpu_info = data as *mut ethtool_drvinfo;
    strscpy((*cpu_info).driver.as_mut_ptr(), b"dsa\0".as_ptr() as *const _, (*cpu_info).driver.len());
    data = data.add(core::mem::size_of::<ethtool_drvinfo>());
    let cpu_regs = data as *mut ethtool_regs;
    data = data.add(core::mem::size_of::<ethtool_regs>());
    if (*(*ds).ops).get_regs_len.is_some() && (*(*ds).ops).get_regs.is_some() {
        len = ((*(*ds).ops).get_regs_len.unwrap())(ds, port);
        if len < 0 { return; }
        (*cpu_regs).len = len as u32;
        ((*(*ds).ops).get_regs.unwrap())(ds, port, cpu_regs, data);
    }
}

unsafe fn dsa_conduit_append_port_stats(ds: *mut dsa_switch, port: i32, data: *mut u64, start: usize) -> isize {
    if (*(*ds).ops).get_sset_count.is_none() { return 0; }
    let count = ((*(*ds).ops).get_sset_count.unwrap())(ds, port, ETH_SS_STATS);
    if count < 0 { return count as isize; }
    if (*(*ds).ops).get_ethtool_stats.is_some() {
        ((*(*ds).ops).get_ethtool_stats.unwrap())(ds, port, data.add(start));
    }
    count as isize
}

unsafe fn dsa_conduit_get_ethtool_stats(dev: *mut net_device, stats: *mut ethtool_stats, data: *mut u64) {
    let cpu_dp = (*dev).dsa_ptr;
    let ops = (*cpu_dp).orig_ethtool_ops;
    let dst = (*cpu_dp).dst;
    let mut mcount = 0;
    if !ops.is_null() && (*ops).get_sset_count.is_some() && (*ops).get_ethtool_stats.is_some() {
        mcount = ((*ops).get_sset_count.unwrap())(dev, ETH_SS_STATS);
        ((*ops).get_ethtool_stats.unwrap())(dev, stats, data);
    }
    list_for_each_entry!(dp, &(*dst).ports, list, {
        if !dsa_port_is_dsa(dp) && !dsa_port_is_cpu(dp) { continue; }
        let count = dsa_conduit_append_port_stats((*dp).ds, (*dp).index, data, mcount as usize);
        if count < 0 { return; }
        mcount += count as i32;
    });
}

unsafe fn dsa_conduit_get_ethtool_phy_stats(dev: *mut net_device, stats: *mut ethtool_stats, data: *mut u64) {
    let cpu_dp = (*dev).dsa_ptr;
    let ops = (*cpu_dp).orig_ethtool_ops;
    let ds = (*cpu_dp).ds;
    let mut count = 0;
    if !(*dev).phydev.is_null() && (ops.is_null() || (*ops).get_ethtool_phy_stats.is_none()) {
        count = phy_ethtool_get_sset_count((*dev).phydev);
        if count >= 0 { phy_ethtool_get_stats((*dev).phydev, stats, data); }
    } else if !ops.is_null() && (*ops).get_sset_count.is_some() && (*ops).get_ethtool_phy_stats.is_some() {
        count = ((*ops).get_sset_count.unwrap())(dev, ETH_SS_PHY_STATS);
        ((*ops).get_ethtool_phy_stats.unwrap())(dev, stats, data);
    }
    if count < 0 { count = 0; }
    if (*(*ds).ops).get_ethtool_phy_stats.is_some() {
        ((*(*ds).ops).get_ethtool_phy_stats.unwrap())(ds, (*cpu_dp).index, data.add(count as usize));
    }
}

unsafe fn dsa_conduit_append_port_sset_count(ds: *mut dsa_switch, port: i32, sset: i32, count: *mut i32) {
    if (*(*ds).ops).get_sset_count.is_some() { *count += ((*(*ds).ops).get_sset_count.unwrap())(ds, port, sset); }
}

unsafe fn dsa_conduit_get_sset_count(dev: *mut net_device, sset: i32) -> i32 {
    let cpu_dp = (*dev).dsa_ptr; let ops = (*cpu_dp).orig_ethtool_ops; let dst = (*cpu_dp).dst;
    let mut count = 0;
    if sset == ETH_SS_PHY_STATS && !(*dev).phydev.is_null() && (ops.is_null() || (*ops).get_ethtool_phy_stats.is_none()) { count = phy_ethtool_get_sset_count((*dev).phydev); }
    else if !ops.is_null() && (*ops).get_sset_count.is_some() { count = ((*ops).get_sset_count.unwrap())(dev, sset); }
    if count < 0 { count = 0; }
    list_for_each_entry!(dp, &(*dst).ports, list, { if dsa_port_is_dsa(dp) || dsa_port_is_cpu(dp) { dsa_conduit_append_port_sset_count((*dp).ds, (*dp).index, sset, &mut count); } });
    count
}

// Remaining declarations preserve the externally visible conduit interface;
// their bodies are expressed using the same kernel helpers and structures.
unsafe extern "C" {
    fn dsa_conduit_get_strings(dev: *mut net_device, stringset: u32, data: *mut u8);
    fn __dsa_conduit_hwtstamp_validate(dev: *mut net_device, config: *const kernel_hwtstamp_config, extack: *mut netlink_ext_ack) -> i32;
    fn dsa_conduit_setup(dev: *mut net_device, cpu_dp: *mut dsa_port) -> i32;
    fn dsa_conduit_teardown(dev: *mut net_device);
    fn dsa_conduit_lag_setup(lag_dev: *mut net_device, cpu_dp: *mut dsa_port, uinfo: *mut netdev_lag_upper_info, extack: *mut netlink_ext_ack) -> i32;
    fn dsa_conduit_lag_teardown(lag_dev: *mut net_device, cpu_dp: *mut dsa_port);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
