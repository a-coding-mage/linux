// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains all networking devres helpers.
 */

// Dependencies corresponding to <linux/device.h>, <linux/etherdevice.h>, and
// <linux/netdevice.h> are supplied by other files.

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct net_device {
    pub dev: device,
}

#[repr(C)]
struct net_device_devres {
    ndev: *mut net_device,
}

extern "C" {
    fn free_netdev(ndev: *mut net_device);
    fn alloc_etherdev_mqs(
        sizeof_priv: c_int,
        txqs: c_uint,
        rxqs: c_uint,
    ) -> *mut net_device;
    fn register_netdev(ndev: *mut net_device) -> c_int;
    fn unregister_netdev(ndev: *mut net_device);

    fn devres_alloc(
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        size: usize,
        gfp: c_uint,
    ) -> *mut c_void;
    fn devres_free(dr: *mut c_void);
    fn devres_add(dev: *mut device, dr: *mut c_void);
    fn devres_find(
        dev: *mut device,
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        match_fn: unsafe extern "C" fn(*mut device, *mut c_void, *mut c_void) -> c_int,
        match_data: *mut c_void,
    ) -> *mut c_void;
    fn warn_on(condition: bool) -> bool;
}

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

unsafe extern "C" fn devm_free_netdev(_dev: *mut device, this: *mut c_void) {
    let res = this as *mut net_device_devres;

    free_netdev((*res).ndev);
}

#[no_mangle]
pub unsafe extern "C" fn devm_alloc_etherdev_mqs(
    dev: *mut device,
    sizeof_priv: c_int,
    txqs: c_uint,
    rxqs: c_uint,
) -> *mut net_device {
    let mut dr: *mut net_device_devres;

    dr = devres_alloc(
        devm_free_netdev,
        core::mem::size_of::<net_device_devres>(),
        GFP_KERNEL,
    ) as *mut net_device_devres;
    if dr.is_null() {
        return core::ptr::null_mut();
    }

    (*dr).ndev = alloc_etherdev_mqs(sizeof_priv, txqs, rxqs);
    if (*dr).ndev.is_null() {
        devres_free(dr as *mut c_void);
        return core::ptr::null_mut();
    }

    devres_add(dev, dr as *mut c_void);

    (*dr).ndev
}

unsafe extern "C" fn devm_unregister_netdev(_dev: *mut device, this: *mut c_void) {
    let res = this as *mut net_device_devres;

    unregister_netdev((*res).ndev);
}

unsafe extern "C" fn netdev_devres_match(
    _dev: *mut device,
    this: *mut c_void,
    match_data: *mut c_void,
) -> c_int {
    let res = this as *mut net_device_devres;
    let ndev = match_data as *mut net_device;

    (ndev == (*res).ndev) as c_int
}

/**
 *\tdevm_register_netdev - resource managed variant of register_netdev()
 *\t@dev: managing device for this netdev - usually the parent device
 *\t@ndev: device to register
 *
 *\tThis is a devres variant of register_netdev() for which the unregister
 *\tfunction will be called automatically when the managing device is
 *\tdetached. Note: the net_device used must also be resource managed by
 *\tthe same struct device.
 */
#[no_mangle]
pub unsafe extern "C" fn devm_register_netdev(
    dev: *mut device,
    ndev: *mut net_device,
) -> c_int {
    let dr: *mut net_device_devres;
    let ret: c_int;

    /* struct net_device must itself be managed. For now a managed netdev
     * can only be allocated by devm_alloc_etherdev_mqs() so the check is
     * straightforward.
     */
    if warn_on(
        devres_find(
            dev,
            devm_free_netdev,
            netdev_devres_match,
            ndev as *mut c_void,
        )
        .is_null(),
    ) {
        return -EINVAL;
    }

    dr = devres_alloc(
        devm_unregister_netdev,
        core::mem::size_of::<net_device_devres>(),
        GFP_KERNEL,
    ) as *mut net_device_devres;
    if dr.is_null() {
        return -ENOMEM;
    }

    ret = register_netdev(ndev);
    if ret != 0 {
        devres_free(dr as *mut c_void);
        return ret;
    }

    (*dr).ndev = ndev;
    devres_add((*ndev).dev.parent, dr as *mut c_void);

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
