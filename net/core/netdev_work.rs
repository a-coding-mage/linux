// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel dependencies and build-time declarations are supplied by other translated files.

static mut NETDEV_WORK_LIST: list_head = LIST_HEAD_INIT;
static mut NETDEV_WORK_LOCK: spinlock_t = SPINLOCK_INIT;
static mut NETDEV_WORK: work_struct = DECLARE_WORK_INIT(netdev_work_proc);

unsafe fn netdev_work_enqueue(
    dev: *mut net_device,
    events: c_ulong,
    core: c_ulong,
) {
    if events == 0 && core == 0 {
        return;
    }

    spin_lock_bh(&raw mut NETDEV_WORK_LOCK);
    if !dev_isalive(dev) {
        spin_unlock_bh(&raw mut NETDEV_WORK_LOCK);
        return;
    }
    if list_empty(&(*dev).work_node) {
        list_add_tail(&mut (*dev).work_node, &raw mut NETDEV_WORK_LIST);
        netdev_hold(dev, &mut (*dev).work_tracker, GFP_ATOMIC);
    }
    (*dev).work_pending |= events;
    (*dev).work_core_pending |= core;
    spin_unlock_bh(&raw mut NETDEV_WORK_LOCK);

    schedule_work(&raw mut NETDEV_WORK);
}

unsafe fn netdev_work_dequeue(
    dev: *mut net_device,
    pending: *mut c_ulong,
    mask: c_ulong,
) -> c_ulong {
    let events: c_ulong;

    spin_lock_bh(&raw mut NETDEV_WORK_LOCK);
    events = *pending & mask;
    *pending &= !events;
    if !list_empty(&(*dev).work_node)
        && (*dev).work_pending == 0
        && (*dev).work_core_pending == 0
    {
        list_del_init(&mut (*dev).work_node);
        netdev_put(dev, &mut (*dev).work_tracker);
    }
    spin_unlock_bh(&raw mut NETDEV_WORK_LOCK);

    events
}

#[no_mangle]
pub unsafe extern "C" fn netdev_work_cancel_all(dev: *mut net_device) {
    spin_lock_bh(&raw mut NETDEV_WORK_LOCK);
    (*dev).work_pending = 0;
    (*dev).work_core_pending = 0;
    if !list_empty(&(*dev).work_node) {
        list_del_init(&mut (*dev).work_node);
        netdev_put(dev, &mut (*dev).work_tracker);
    }
    spin_unlock_bh(&raw mut NETDEV_WORK_LOCK);
}

#[no_mangle]
pub unsafe extern "C" fn netdev_work_sched(dev: *mut net_device, events: c_ulong) {
    netdev_work_enqueue(dev, events, 0);
}

// EXPORT_SYMBOL(netdev_work_sched)

/**
 * netdev_work_cancel() - cancel selected work for a netdev
 * @dev: net_device
 * @mask: events to cancel
 *
 * Clear @mask from the device's work pending mask. If no work is left pending
 * the device is dequeued and its ndo_work won't be called.
 *
 * No expectations on locking, but also no guarantees provided. If the caller
 * wants to touch @dev afterwards (e.g. call the work that got canceled)
 * they have to ensure @dev does not get freed.
 *
 * Returns: the subset of @mask that was actually pending, so the caller can run
 * those events inline.
 */
#[no_mangle]
pub unsafe extern "C" fn netdev_work_cancel(
    dev: *mut net_device,
    mask: c_ulong,
) -> c_ulong {
    netdev_work_dequeue(dev, &mut (*dev).work_pending, mask)
}

// EXPORT_SYMBOL(netdev_work_cancel)

#[no_mangle]
pub unsafe extern "C" fn __netdev_work_core_sched(
    dev: *mut net_device,
    events: c_ulong,
) {
    netdev_work_enqueue(dev, 0, events);
}

#[no_mangle]
pub unsafe extern "C" fn __netdev_work_core_cancel(
    dev: *mut net_device,
    mask: c_ulong,
) -> c_ulong {
    netdev_work_dequeue(dev, &mut (*dev).work_core_pending, mask)
}

unsafe fn netdev_work_run(dev: *mut net_device, events: c_ulong, core: c_ulong) {
    if !netif_device_present(dev) {
        return;
    }

    if core & NETDEV_WORK_RX_MODE != 0 {
        netif_rx_mode_run(dev);
    }
    if events != 0 && !(*dev).netdev_ops.is_null()
        && (*(*dev).netdev_ops).ndo_work.is_some()
    {
        ((*(*dev).netdev_ops).ndo_work.unwrap())(dev, events);
    }
}

unsafe extern "C" fn netdev_work_proc(_work: *mut work_struct) {
    rtnl_lock();

    loop {
        let mut events: c_ulong = 0;
        let mut core: c_ulong = 0;
        let mut tracker: netdevice_tracker = core::mem::zeroed();
        let dev: *mut net_device;

        spin_lock_bh(&raw mut NETDEV_WORK_LOCK);
        if list_empty(&raw mut NETDEV_WORK_LIST) {
            spin_unlock_bh(&raw mut NETDEV_WORK_LOCK);
            break;
        }
        dev = list_first_entry(&raw mut NETDEV_WORK_LIST, net_device, work_node);
        /* Take a temporary reference so @dev can't be freed while we
         * drop the lock to grab its ops lock; the work reference is
         * only released once we claim the work below.
         * The re-locking dance is to ensure that ops lock is enough
         * to ensure canceling work is not racy with dequeue.
         */
        netdev_hold(dev, &mut tracker, GFP_ATOMIC);
        spin_unlock_bh(&raw mut NETDEV_WORK_LOCK);

        netdev_lock_ops(dev);
        spin_lock_bh(&raw mut NETDEV_WORK_LOCK);
        if !list_empty(&(*dev).work_node) {
            list_del_init(&mut (*dev).work_node);
            core = (*dev).work_core_pending;
            (*dev).work_core_pending = 0;
            events = (*dev).work_pending;
            (*dev).work_pending = 0;
            /* We took another ref above */
            netdev_put(dev, &mut (*dev).work_tracker);

            if !dev_isalive(dev) {
                core = 0;
                events = 0;
            }
        }
        spin_unlock_bh(&raw mut NETDEV_WORK_LOCK);

        netdev_work_run(dev, events, core);
        netdev_unlock_ops(dev);

        netdev_put(dev, &mut tracker);
    }

    rtnl_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
