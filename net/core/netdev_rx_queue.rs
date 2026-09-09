// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel headers and symbols referenced below are supplied by other translated units.

pub unsafe fn netdev_rx_queue_lease(
    rxq_dst: *mut netdev_rx_queue,
    rxq_src: *mut netdev_rx_queue,
) {
    netdev_assert_locked((*rxq_src).dev);
    netdev_assert_locked((*rxq_dst).dev);
    netdev_hold((*rxq_src).dev, &mut (*rxq_src).lease_tracker, GFP_KERNEL);
    core::ptr::write_volatile(&mut (*rxq_src).lease, rxq_dst);
    core::ptr::write_volatile(&mut (*rxq_dst).lease, rxq_src);
}

pub unsafe fn netdev_rx_queue_unlease(
    rxq_dst: *mut netdev_rx_queue,
    rxq_src: *mut netdev_rx_queue,
) {
    netdev_assert_locked((*rxq_dst).dev);
    netdev_assert_locked((*rxq_src).dev);
    netif_rxq_cleanup_unlease(rxq_src, rxq_dst);
    core::ptr::write_volatile(&mut (*rxq_src).lease, core::ptr::null_mut());
    core::ptr::write_volatile(&mut (*rxq_dst).lease, core::ptr::null_mut());
    netdev_put((*rxq_src).dev, &mut (*rxq_src).lease_tracker);
}

pub unsafe fn netif_rxq_is_leased(dev: *mut net_device, rxq_idx: u32) -> bool {
    if rxq_idx < (*dev).real_num_rx_queues {
        return !core::ptr::read_volatile(&(*__netif_get_rx_queue(dev, rxq_idx)).lease).is_null();
    }
    false
}

/* Virtual devices eligible for leasing have no dev->dev.parent, while
 * physical devices always have one. Use this to enforce the correct
 * lease traversal direction.
 */
unsafe fn netif_lease_dir_ok(dev: *const net_device, dir: netif_lease_dir) -> bool {
    if dir == NETIF_VIRT_TO_PHYS && (*dev).dev.parent.is_null() { return true; }
    if dir == NETIF_PHYS_TO_VIRT && !(*dev).dev.parent.is_null() { return true; }
    false
}

pub unsafe fn netif_is_queue_leasee(dev: *const net_device) -> bool {
    netif_lease_dir_ok(dev, NETIF_VIRT_TO_PHYS)
}

pub unsafe fn __netif_get_rx_queue_lease(
    dev: *mut *mut net_device,
    rxq_idx: *mut u32,
    dir: netif_lease_dir,
) -> *mut netdev_rx_queue {
    let orig_dev = *dev;
    let mut rxq = __netif_get_rx_queue(orig_dev, *rxq_idx);
    if !(*rxq).lease.is_null() {
        if !netif_lease_dir_ok(orig_dev, dir) { return core::ptr::null_mut(); }
        rxq = (*rxq).lease;
        *rxq_idx = get_netdev_rx_queue_index(rxq);
        *dev = (*rxq).dev;
    }
    rxq
}

/* See also page_pool_is_unreadable() */
pub unsafe fn netif_rxq_has_unreadable_mp(dev: *mut net_device, rxq_idx: u32) -> bool {
    if rxq_idx < (*dev).real_num_rx_queues {
        return !(*__netif_get_rx_queue(dev, rxq_idx)).mp_params.mp_ops.is_null();
    }
    false
}

pub unsafe fn netif_rxq_has_mp(dev: *mut net_device, rxq_idx: u32) -> bool {
    if rxq_idx < (*dev).real_num_rx_queues {
        return !(*__netif_get_rx_queue(dev, rxq_idx)).mp_params.mp_priv.is_null();
    }
    false
}

unsafe fn netdev_rx_queue_reconfig(
    dev: *mut net_device,
    rxq_idx: u32,
    qcfg_old: *mut netdev_queue_config,
    qcfg_new: *mut netdev_queue_config,
) -> i32 {
    let rxq = __netif_get_rx_queue(dev, rxq_idx);
    let qops = (*dev).queue_mgmt_ops;
    let mut new_mem: *mut core::ffi::c_void;
    let mut old_mem: *mut core::ffi::c_void;
    let mut err: i32;
    if qops.is_null() || (*qops).ndo_queue_stop.is_none() || (*qops).ndo_queue_mem_free.is_none()
        || (*qops).ndo_queue_mem_alloc.is_none() || (*qops).ndo_queue_start.is_none() { return -EOPNOTSUPP; }
    netdev_assert_locked(dev);
    new_mem = kvzalloc((*qops).ndo_queue_mem_size, GFP_KERNEL);
    if new_mem.is_null() { return -ENOMEM; }
    old_mem = kvzalloc((*qops).ndo_queue_mem_size, GFP_KERNEL);
    if old_mem.is_null() { kvfree(new_mem); return -ENOMEM; }
    err = ((*qops).ndo_queue_mem_alloc.unwrap())(dev, qcfg_new, new_mem, rxq_idx);
    if err != 0 { kvfree(old_mem); kvfree(new_mem); return err; }
    err = page_pool_check_memory_provider(dev, rxq);
    if err != 0 { ((*qops).ndo_queue_mem_free.unwrap())(dev, new_mem); kvfree(old_mem); kvfree(new_mem); return err; }
    if netif_running(dev) {
        err = ((*qops).ndo_queue_stop.unwrap())(dev, old_mem, rxq_idx);
        if err != 0 { ((*qops).ndo_queue_mem_free.unwrap())(dev, new_mem); kvfree(old_mem); kvfree(new_mem); return err; }
        err = ((*qops).ndo_queue_start.unwrap())(dev, qcfg_new, new_mem, rxq_idx);
        if err != 0 {
            if ((*qops).ndo_queue_start.unwrap())(dev, qcfg_old, old_mem, rxq_idx) != 0 {
                WARN(1, "Failed to restart old queue in error path. RX queue %d may be unhealthy.", rxq_idx);
                ((*qops).ndo_queue_mem_free.unwrap())(dev, old_mem);
            }
            ((*qops).ndo_queue_mem_free.unwrap())(dev, new_mem); kvfree(old_mem); kvfree(new_mem); return err;
        }
    } else { core::mem::swap(&mut new_mem, &mut old_mem); }
    ((*qops).ndo_queue_mem_free.unwrap())(dev, old_mem);
    kvfree(old_mem); kvfree(new_mem); 0
}

pub unsafe fn netdev_rx_queue_restart(dev: *mut net_device, rxq_idx: u32) -> i32 {
    let mut qcfg = core::mem::MaybeUninit::<netdev_queue_config>::uninit();
    netdev_queue_config(dev, rxq_idx, qcfg.as_mut_ptr());
    netdev_rx_queue_reconfig(dev, rxq_idx, qcfg.as_mut_ptr(), qcfg.as_mut_ptr())
}

unsafe fn __netif_mp_open_rxq(dev: *mut net_device, rxq_idx: u32, p: *const pp_memory_provider_params, extack: *mut netlink_ext_ack) -> i32 {
    let qops = (*dev).queue_mgmt_ops;
    let mut qcfg: [netdev_queue_config; 2] = core::mem::MaybeUninit::uninit().assume_init();
    let rxq = __netif_get_rx_queue(dev, rxq_idx);
    if qops.is_null() { return -EOPNOTSUPP; }
    if (*(*dev).cfg).hds_config != ETHTOOL_TCP_DATA_SPLIT_ENABLED { NL_SET_ERR_MSG(extack, "tcp-data-split is disabled"); return -EINVAL; }
    if (*(*dev).cfg).hds_thresh != 0 { NL_SET_ERR_MSG(extack, "hds-thresh is not zero"); return -EINVAL; }
    if dev_xdp_prog_count(dev) != 0 { NL_SET_ERR_MSG(extack, "unable to custom memory provider to device with XDP program attached"); return -EEXIST; }
    if (*p).rx_page_size != 0 && (*qops).supported_params & QCFG_RX_PAGE_SIZE == 0 { NL_SET_ERR_MSG(extack, "device does not support: rx_page_size"); return -EOPNOTSUPP; }
    if !(*rxq).mp_params.mp_ops.is_null() { NL_SET_ERR_MSG(extack, "designated queue already memory provider bound"); return -EEXIST; }
    #[cfg(CONFIG_XDP_SOCKETS)]
    if !(*rxq).pool.is_null() { NL_SET_ERR_MSG(extack, "designated queue already in use by AF_XDP"); return -EBUSY; }
    netdev_queue_config(dev, rxq_idx, &mut qcfg[0]); (*rxq).mp_params = *p;
    let ret = netdev_queue_config_validate(dev, rxq_idx, &mut qcfg[1], extack);
    if ret != 0 { core::ptr::write_bytes(&mut (*rxq).mp_params, 0, 1); return ret; }
    let ret = netdev_rx_queue_reconfig(dev, rxq_idx, &mut qcfg[0], &mut qcfg[1]);
    if ret != 0 { core::ptr::write_bytes(&mut (*rxq).mp_params, 0, 1); }
    ret
}

pub unsafe fn netif_mp_open_rxq(dev: *mut net_device, mut rxq_idx: u32, p: *const pp_memory_provider_params, extack: *mut netlink_ext_ack) -> i32 {
    if !netdev_need_ops_lock(dev) { return -EOPNOTSUPP; }
    if rxq_idx >= (*dev).real_num_rx_queues { NL_SET_ERR_MSG(extack, "rx queue index out of range"); return -ERANGE; }
    rxq_idx = array_index_nospec(rxq_idx, (*dev).real_num_rx_queues);
    if !netif_rxq_is_leased(dev, rxq_idx) { return __netif_mp_open_rxq(dev, rxq_idx, p, extack); }
    if __netif_get_rx_queue_lease(&mut dev, &mut rxq_idx, NETIF_VIRT_TO_PHYS).is_null() { NL_SET_ERR_MSG(extack, "rx queue leased to a virtual netdev"); return -EBUSY; }
    if (*dev).dev.parent.is_null() { NL_SET_ERR_MSG(extack, "rx queue belongs to a virtual netdev"); return -EOPNOTSUPP; }
    netdev_lock(dev); let ret = __netif_mp_open_rxq(dev, rxq_idx, p, extack); netdev_unlock(dev); ret
}

unsafe fn __netif_mp_close_rxq(dev: *mut net_device, rxq_idx: u32, old_p: *const pp_memory_provider_params) {
    let mut qcfg: [netdev_queue_config; 2] = core::mem::MaybeUninit::uninit().assume_init();
    let rxq = __netif_get_rx_queue(dev, rxq_idx);
    if (*dev).reg_state > NETREG_REGISTERED && (*rxq).mp_params.mp_ops.is_null() { return; }
    if WARN_ON_ONCE((*rxq).mp_params.mp_ops != (*old_p).mp_ops || (*rxq).mp_params.mp_priv != (*old_p).mp_priv) { return; }
    netdev_queue_config(dev, rxq_idx, &mut qcfg[0]); core::ptr::write_bytes(&mut (*rxq).mp_params, 0, 1); netdev_queue_config(dev, rxq_idx, &mut qcfg[1]);
    let err = netdev_rx_queue_reconfig(dev, rxq_idx, &mut qcfg[0], &mut qcfg[1]); WARN_ON(err != 0 && err != -ENETDOWN);
}

pub unsafe fn netif_mp_close_rxq(dev: *mut net_device, rxq_idx: u32, old_p: *const pp_memory_provider_params) {
    if WARN_ON_ONCE(rxq_idx >= (*dev).real_num_rx_queues) { return; }
    if !netif_rxq_is_leased(dev, rxq_idx) { return __netif_mp_close_rxq(dev, rxq_idx, old_p); }
    if __netif_get_rx_queue_lease(&mut dev, &mut (rxq_idx as u32), NETIF_VIRT_TO_PHYS).is_null() { WARN_ON_ONCE(true); return; }
    netdev_lock(dev); __netif_mp_close_rxq(dev, rxq_idx, old_p); netdev_unlock(dev);
}

pub unsafe fn __netif_mp_uninstall_rxq(rxq: *mut netdev_rx_queue, p: *const pp_memory_provider_params) {
    if !(*p).mp_ops.is_null() && (*(*p).mp_ops).uninstall.is_some() { ((*(*p).mp_ops).uninstall.unwrap())((*p).mp_priv, rxq); }
}

/* Clean up memory provider state when a queue lease is torn down. */
pub unsafe fn netif_rxq_cleanup_unlease(phys_rxq: *mut netdev_rx_queue, virt_rxq: *mut netdev_rx_queue) {
    let rxq_idx = get_netdev_rx_queue_index(phys_rxq);
    let p = (*phys_rxq).mp_params;
    if p.mp_ops.is_null() { return; }
    __netif_mp_close_rxq((*phys_rxq).dev, rxq_idx, &p);
    __netif_mp_uninstall_rxq(virt_rxq, &p);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
