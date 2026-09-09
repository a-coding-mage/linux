// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2018, Intel Corporation. */

/* A common module to handle registrations and notifications for paravirtual
 * drivers to enable accelerated datapath and support VF live migration.
 *
 * The notifier and event handling code is based on netvsc driver.
 */

// Kernel dependencies supplied by the surrounding tree.

static mut failover_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut failover_lock: spinlock_t = spinlock_t {};

unsafe fn failover_get_bymac(mac: *mut u8, ops: *mut *mut failover_ops) -> *mut net_device {
    let mut failover_dev: *mut net_device;
    let mut failover: *mut failover;

    spin_lock(&raw mut failover_lock);
    list_for_each_entry(failover, &raw mut failover_list, list) {
        failover_dev = rtnl_dereference((*failover).failover_dev);
        if ether_addr_equal((*failover_dev).perm_addr.as_ptr(), mac) {
            *ops = rtnl_dereference((*failover).ops);
            spin_unlock(&raw mut failover_lock);
            return failover_dev;
        }
    }
    spin_unlock(&raw mut failover_lock);
    core::ptr::null_mut()
}

/**
 * failover_slave_register - Register a slave netdev
 *
 * @slave_dev: slave netdev that is being registered
 *
 * Registers a slave device to a failover instance. Only ethernet devices
 * are supported.
 */
unsafe fn failover_slave_register(slave_dev: *mut net_device) -> i32 {
    let mut lag_upper_info: netdev_lag_upper_info = core::mem::zeroed();
    let failover_dev: *mut net_device;
    let mut fops: *mut failover_ops = core::ptr::null_mut();
    let mut err: i32;

    if (*slave_dev).type_ != ARPHRD_ETHER { return NOTIFY_DONE; }
    ASSERT_RTNL!();

    failover_dev = failover_get_bymac((*slave_dev).perm_addr.as_mut_ptr(), &mut fops);
    if failover_dev.is_null() { return NOTIFY_DONE; }

    if !(*fops).slave_pre_register.is_none()
        && ((*fops).slave_pre_register.unwrap())(slave_dev, failover_dev) != 0 { return NOTIFY_DONE; }

    err = netdev_rx_handler_register(slave_dev, (*fops).slave_handle_frame, failover_dev);
    if err != 0 {
        netdev_err!(slave_dev, "can not register failover rx handler (err = %d)\n", err);
        return NOTIFY_DONE;
    }

    lag_upper_info.tx_type = NETDEV_LAG_TX_TYPE_ACTIVEBACKUP;
    err = netdev_master_upper_dev_link(slave_dev, failover_dev, core::ptr::null_mut(), &mut lag_upper_info, core::ptr::null_mut());
    if err != 0 {
        netdev_err!(slave_dev, "can not set failover device %s (err = %d)\n", (*failover_dev).name.as_ptr(), err);
        netdev_rx_handler_unregister(slave_dev);
        return NOTIFY_DONE;
    }

    (*slave_dev).priv_flags |= IFF_FAILOVER_SLAVE | IFF_NO_ADDRCONF;

    if !(*fops).slave_register.is_none()
        && ((*fops).slave_register.unwrap())(slave_dev, failover_dev) == 0 { return NOTIFY_OK; }

    netdev_upper_dev_unlink(slave_dev, failover_dev);
    (*slave_dev).priv_flags &= !(IFF_FAILOVER_SLAVE | IFF_NO_ADDRCONF);
    netdev_rx_handler_unregister(slave_dev);
    NOTIFY_DONE
}

/**
 * failover_slave_unregister - Unregister a slave netdev
 *
 * @slave_dev: slave netdev that is being unregistered
 *
 * Unregisters a slave device from a failover instance.
 */
#[no_mangle]
pub unsafe extern "C" fn failover_slave_unregister(slave_dev: *mut net_device) -> i32 {
    let failover_dev: *mut net_device;
    let mut fops: *mut failover_ops = core::ptr::null_mut();

    if !netif_is_failover_slave(slave_dev) { return NOTIFY_DONE; }
    ASSERT_RTNL!();
    failover_dev = failover_get_bymac((*slave_dev).perm_addr.as_mut_ptr(), &mut fops);
    if failover_dev.is_null() { return NOTIFY_DONE; }
    if !(*fops).slave_pre_unregister.is_none()
        && ((*fops).slave_pre_unregister.unwrap())(slave_dev, failover_dev) != 0 { return NOTIFY_DONE; }
    netdev_rx_handler_unregister(slave_dev);
    netdev_upper_dev_unlink(slave_dev, failover_dev);
    (*slave_dev).priv_flags &= !(IFF_FAILOVER_SLAVE | IFF_NO_ADDRCONF);
    if !(*fops).slave_unregister.is_none()
        && ((*fops).slave_unregister.unwrap())(slave_dev, failover_dev) == 0 { return NOTIFY_OK; }
    NOTIFY_DONE
}

unsafe fn failover_slave_link_change(slave_dev: *mut net_device) -> i32 {
    let failover_dev: *mut net_device;
    let mut fops: *mut failover_ops = core::ptr::null_mut();
    if !netif_is_failover_slave(slave_dev) { return NOTIFY_DONE; }
    ASSERT_RTNL!();
    failover_dev = failover_get_bymac((*slave_dev).perm_addr.as_mut_ptr(), &mut fops);
    if failover_dev.is_null() || !netif_running(failover_dev) { return NOTIFY_DONE; }
    if !(*fops).slave_link_change.is_none()
        && ((*fops).slave_link_change.unwrap())(slave_dev, failover_dev) == 0 { return NOTIFY_OK; }
    NOTIFY_DONE
}

unsafe fn failover_slave_name_change(slave_dev: *mut net_device) -> i32 {
    let failover_dev: *mut net_device;
    let mut fops: *mut failover_ops = core::ptr::null_mut();
    if !netif_is_failover_slave(slave_dev) { return NOTIFY_DONE; }
    ASSERT_RTNL!();
    failover_dev = failover_get_bymac((*slave_dev).perm_addr.as_mut_ptr(), &mut fops);
    if failover_dev.is_null() || !netif_running(failover_dev) { return NOTIFY_DONE; }
    if !(*fops).slave_name_change.is_none()
        && ((*fops).slave_name_change.unwrap())(slave_dev, failover_dev) == 0 { return NOTIFY_OK; }
    NOTIFY_DONE
}

unsafe fn failover_event(_this: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> i32 {
    let event_dev = netdev_notifier_info_to_dev(ptr);
    /* Skip parent events */
    if netif_is_failover(event_dev) { return NOTIFY_DONE; }
    match event {
        NETDEV_REGISTER => failover_slave_register(event_dev),
        NETDEV_UNREGISTER => failover_slave_unregister(event_dev),
        NETDEV_UP | NETDEV_DOWN | NETDEV_CHANGE => failover_slave_link_change(event_dev),
        NETDEV_CHANGENAME => failover_slave_name_change(event_dev),
        _ => NOTIFY_DONE,
    }
}

static mut failover_notifier: notifier_block = notifier_block { notifier_call: Some(failover_event) };

unsafe fn failover_existing_slave_register(failover_dev: *mut net_device) {
    let net = dev_net(failover_dev);
    let mut dev: *mut net_device;
    rtnl_lock();
    for_each_netdev(net, dev) {
        if netif_is_failover(dev) { continue; }
        if ether_addr_equal((*failover_dev).perm_addr.as_ptr(), (*dev).perm_addr.as_ptr()) {
            netdev_lock_ops(dev);
            failover_slave_register(dev);
            netdev_unlock_ops(dev);
        }
    }
    rtnl_unlock();
}

/** Register a failover instance. */
#[no_mangle]
pub unsafe extern "C" fn failover_register(dev: *mut net_device, ops: *mut failover_ops) -> *mut failover {
    if (*dev).type_ != ARPHRD_ETHER || ops.is_null() { return ERR_PTR(-EINVAL); }
    let failover = kzalloc_obj::<failover>();
    if failover.is_null() { return ERR_PTR(-ENOMEM); }
    rcu_assign_pointer((*failover).ops, ops);
    netdev_hold(dev, &mut (*failover).dev_tracker, GFP_KERNEL);
    (*dev).priv_flags |= IFF_FAILOVER;
    rcu_assign_pointer((*failover).failover_dev, dev);
    spin_lock(&raw mut failover_lock);
    list_add_tail(&mut (*failover).list, &raw mut failover_list);
    spin_unlock(&raw mut failover_lock);
    netdev_info!(dev, "failover master:%s registered\n", (*dev).name.as_ptr());
    failover_existing_slave_register(dev);
    failover
}

/** Unregister a failover instance. */
#[no_mangle]
pub unsafe extern "C" fn failover_unregister(failover: *mut failover) {
    let failover_dev = rcu_dereference((*failover).failover_dev);
    netdev_info!(failover_dev, "failover master:%s unregistered\n", (*failover_dev).name.as_ptr());
    (*failover_dev).priv_flags &= !IFF_FAILOVER;
    netdev_put(failover_dev, &mut (*failover).dev_tracker);
    spin_lock(&raw mut failover_lock);
    list_del(&mut (*failover).list);
    spin_unlock(&raw mut failover_lock);
    kfree(failover);
}

unsafe fn failover_init() -> i32 { register_netdevice_notifier(&raw mut failover_notifier) }
unsafe fn failover_exit() { unregister_netdevice_notifier(&raw mut failover_notifier); }

module_init!(failover_init);
module_exit!(failover_exit);
MODULE_DESCRIPTION!("Generic failover infrastructure/interface");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
