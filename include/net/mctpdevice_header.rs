/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Management Component Transport Protocol (MCTP) - device
 * definitions.
 *
 * Copyright (c) 2021 Code Construct
 * Copyright (c) 2021 Google
 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here but are not defined in this header translation.

pub struct mctp_sk_key;

#[repr(C)]
pub struct mctp_dev {
    pub dev: *mut net_device,

    pub refs: refcount_t,

    pub net: core::ffi::c_uint,
    pub binding: mctp_phys_binding,

    pub ops: *const mctp_netdev_ops,

    /* Only modified under RTNL. Reads have addrs_lock held */
    pub addrs: *mut u8,
    pub num_addrs: usize,
    pub addrs_lock: spinlock_t,

    pub rcu: rcu_head,
}

#[repr(C)]
pub struct mctp_netdev_ops {
    pub release_flow:
        Option<unsafe extern "C" fn(dev: *mut mctp_dev, key: *mut mctp_sk_key)>,
}

pub const MCTP_INITIAL_DEFAULT_NET: core::ffi::c_uint = 1;

extern "C" {
    pub fn mctp_dev_get_rtnl(dev: *const net_device) -> *mut mctp_dev;
    pub fn __mctp_dev_get(dev: *const net_device) -> *mut mctp_dev;

    pub fn mctp_register_netdev(
        dev: *mut net_device,
        ops: *const mctp_netdev_ops,
        binding: mctp_phys_binding,
    ) -> core::ffi::c_int;
    pub fn mctp_unregister_netdev(dev: *mut net_device);

    pub fn mctp_dev_hold(mdev: *mut mctp_dev);
    pub fn mctp_dev_put(mdev: *mut mctp_dev);

    pub fn mctp_dev_set_key(dev: *mut mctp_dev, key: *mut mctp_sk_key);
    pub fn mctp_dev_release_key(dev: *mut mctp_dev, key: *mut mctp_sk_key);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
