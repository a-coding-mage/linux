/* SPDX-License-Identifier: GPL-2.0-or-later */

// Declarations supplied by the kernel lockdep, netdevice, and rtnetlink APIs
// are intentionally referenced here rather than redefined.

pub unsafe fn netdev_trylock(dev: *mut net_device) -> bool {
    mutex_trylock(unsafe { &mut (*dev).lock })
}

pub unsafe fn netdev_assert_locked(dev: *const net_device) {
    lockdep_assert_held(unsafe { &(*dev).lock });
}

pub unsafe fn netdev_assert_locked_or_invisible(dev: *const net_device) {
    if unsafe { (*dev).reg_state == NETREG_REGISTERED
        || (*dev).reg_state == NETREG_UNREGISTERING }
    {
        netdev_assert_locked(dev);
    }
}

pub unsafe fn netdev_need_ops_lock(dev: *const net_device) -> bool {
    let mut ret = unsafe { (*dev).request_ops_lock || !(*dev).queue_mgmt_ops.is_null() };

    // CONFIG_NET_SHAPER is a build-time configuration condition.
    #[cfg(feature = "CONFIG_NET_SHAPER")]
    {
        ret |= unsafe { !(*(*dev).netdev_ops).net_shaper_ops.is_null() };
    }

    ret
}

pub unsafe fn netdev_lock_ops(dev: *mut net_device) {
    if netdev_need_ops_lock(dev) {
        netdev_lock(dev);
    }
}

pub unsafe fn netdev_unlock_ops(dev: *mut net_device) {
    if netdev_need_ops_lock(dev) {
        netdev_unlock(dev);
    }
}

pub unsafe fn netdev_lock_ops_to_full(dev: *mut net_device) {
    if netdev_need_ops_lock(dev) {
        netdev_assert_locked(dev);
    } else {
        netdev_lock(dev);
    }
}

pub unsafe fn netdev_unlock_full_to_ops(dev: *mut net_device) {
    if netdev_need_ops_lock(dev) {
        netdev_assert_locked(dev);
    } else {
        netdev_unlock(dev);
    }
}

pub unsafe fn netdev_assert_locked_ops_compat(dev: *const net_device) {
    if netdev_need_ops_lock(dev) {
        lockdep_assert_held(unsafe { &(*dev).lock });
    } else {
        ASSERT_RTNL();
    }
}

pub unsafe fn netdev_assert_locked_ops_compat_or_invisible(dev: *const net_device) {
    if unsafe { (*dev).reg_state == NETREG_REGISTERED
        || (*dev).reg_state == NETREG_UNREGISTERING }
    {
        netdev_assert_locked_ops_compat(dev);
    }
}

pub unsafe fn netdev_assert_locked_ops(dev: *const net_device) {
    if netdev_need_ops_lock(dev) {
        netdev_assert_locked(dev);
    }
}

pub unsafe fn netdev_lock_ops_compat(dev: *mut net_device) {
    if netdev_need_ops_lock(dev) {
        netdev_lock(dev);
    } else {
        rtnl_lock();
    }
}

pub unsafe fn netdev_unlock_ops_compat(dev: *mut net_device) {
    if netdev_need_ops_lock(dev) {
        netdev_unlock(dev);
    } else {
        rtnl_unlock();
    }
}

/* Matching "ops protected" category from netdevice.h */
pub unsafe fn netdev_is_locked_ops_compat(dev: *const net_device) -> i32 {
    if netdev_need_ops_lock(dev) {
        return lockdep_is_held(unsafe { &(*dev).lock });
    }
    lockdep_rtnl_is_held()
}

pub unsafe fn netdev_lock_cmp_fn(
    a: *const lockdep_map,
    b: *const lockdep_map,
) -> i32 {
    if a == b {
        return 0;
    }

    /* Allow locking multiple devices only under rtnl_lock,
     * the exact order doesn't matter.
     * Note that upper devices don't lock their ops, so nesting
     * mostly happens in batched device removal for now.
     */
    if lockdep_rtnl_is_held() != 0 { -1 } else { 1 }
}

#[macro_export]
macro_rules! netdev_lockdep_set_classes {
    ($dev:expr) => {{
        static mut qdisc_tx_busylock_key: lock_class_key = lock_class_key::new();
        static mut qdisc_xmit_lock_key: lock_class_key = lock_class_key::new();
        static mut dev_addr_list_lock_key: lock_class_key = lock_class_key::new();
        static mut dev_instance_lock_key: lock_class_key = lock_class_key::new();
        let mut i: u32 = 0;

        unsafe {
            (*$dev).qdisc_tx_busylock = &mut qdisc_tx_busylock_key;
            lockdep_set_class(&mut (*$dev).addr_list_lock, &mut dev_addr_list_lock_key);
            lockdep_set_class(&mut (*$dev).lock, &mut dev_instance_lock_key);
            lock_set_cmp_fn(&mut (*$dev).lock, netdev_lock_cmp_fn, core::ptr::null_mut());
            while i < (*$dev).num_tx_queues {
                lockdep_set_class(&mut (*$dev)._tx[i as usize]._xmit_lock,
                                  &mut qdisc_xmit_lock_key);
                i += 1;
            }
        }
    }};
}

#[macro_export]
macro_rules! netdev_lock_dereference {
    ($p:expr, $dev:expr) => {
        rcu_dereference_protected($p, lockdep_is_held(unsafe { &(*$dev).lock }))
    };
}

#[macro_export]
macro_rules! netdev_ops_lock_dereference {
    ($p:expr, $dev:expr) => {
        rcu_dereference_protected($p, netdev_is_locked_ops_compat($dev))
    };
}

extern "C" {
    pub fn netdev_debug_event(nb: *mut notifier_block, event: u64, ptr: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
