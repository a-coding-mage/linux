// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES. All rights reserved. */

// Dependencies supplied by the kernel devlink and devlink-internal interfaces.

use core::ffi::c_void;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct refcount_t {
    pub refs: core::sync::atomic::AtomicU32,
}

#[repr(C)]
pub struct devlink;
#[repr(C)]
pub struct devlink_ops;
#[repr(C)]
pub struct device_driver;
#[repr(C)]
pub struct net;

extern "C" {
    static mut init_net: net;
    fn __devlink_alloc(
        ops: *const devlink_ops,
        priv_size: usize,
        net: *mut net,
        lock_key: *mut c_void,
        driver: *const device_driver,
    ) -> *mut devlink;
    fn devlink_priv(devlink: *mut devlink) -> *mut devlink_shd;
    fn priv_to_devlink(shd: *mut devlink_shd) -> *mut devlink;
    fn kstrdup(id: *const i8, flags: u32) -> *mut i8;
    fn devlink_free(devlink: *mut devlink);
    fn devl_lock(devlink: *mut devlink);
    fn devl_register(devlink: *mut devlink);
    fn devl_unlock(devlink: *mut devlink);
    fn devl_unregister(devlink: *mut devlink);
    fn kfree(ptr: *mut c_void);
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn refcount_set(refcount: *mut refcount_t, value: u32);
    fn refcount_inc(refcount: *mut refcount_t);
    fn refcount_dec_and_test(refcount: *mut refcount_t) -> bool;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_first_entry(head: *mut list_head) -> *mut devlink_shd;
    fn list_next_entry(pos: *mut devlink_shd) -> *mut devlink_shd;
    fn warn_on_once(condition: bool) -> bool;
}

const GFP_KERNEL: u32 = 0;

static mut shd_list: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};
static mut shd_mutex: c_void = c_void;

/* This structure represents a shared devlink instance,
 * there is one created per identifier (e.g., serial number).
 */
#[repr(C)]
pub struct devlink_shd {
    pub list: list_head, /* Node in shd list */
    pub id: *const i8, /* Identifier string (e.g., serial number) */
    pub refcount: refcount_t, /* Reference count */
    pub priv_size: usize, /* Size of driver private data */
    pub priv_data: [u8; 0],
}

unsafe fn devlink_shd_lookup(id: *const i8) -> *mut devlink_shd {
    let mut shd = list_first_entry(&raw mut shd_list);
    while !shd.is_null() {
        if strcmp((*shd).id, id) == 0 {
            return shd;
        }
        shd = list_next_entry(shd);
    }
    core::ptr::null_mut()
}

unsafe fn devlink_shd_create(
    id: *const i8,
    ops: *const devlink_ops,
    priv_size: usize,
    driver: *const device_driver,
) -> *mut devlink_shd {
    let devlink = __devlink_alloc(
        ops,
        core::mem::size_of::<devlink_shd>() + priv_size,
        &raw mut init_net,
        core::ptr::null_mut(),
        driver,
    );
    if devlink.is_null() {
        return core::ptr::null_mut();
    }
    let shd = devlink_priv(devlink);

    (*shd).id = kstrdup(id, GFP_KERNEL);
    if (*shd).id.is_null() {
        devlink_free(devlink);
        return core::ptr::null_mut();
    }
    (*shd).priv_size = priv_size;
    refcount_set(&raw mut (*shd).refcount, 1);

    devl_lock(devlink);
    devl_register(devlink);
    devl_unlock(devlink);

    list_add_tail(&raw mut (*shd).list, &raw mut shd_list);
    shd
}

unsafe fn devlink_shd_destroy(shd: *mut devlink_shd) {
    let devlink = priv_to_devlink(shd);

    list_del(&raw mut (*shd).list);
    devl_lock(devlink);
    devl_unregister(devlink);
    devl_unlock(devlink);
    kfree((*shd).id as *mut c_void);
    devlink_free(devlink);
}

pub unsafe fn devlink_shd_get(
    id: *const i8,
    ops: *const devlink_ops,
    priv_size: usize,
    driver: *const device_driver,
) -> *mut devlink {
    mutex_lock(&raw mut shd_mutex as *mut c_void);

    let mut shd = devlink_shd_lookup(id);
    if shd.is_null() {
        shd = devlink_shd_create(id, ops, priv_size, driver);
    } else {
        let devlink = priv_to_devlink(shd);
        // The devlink fields are supplied by the external devlink definition.
        if warn_on_once(false) {
            shd = core::ptr::null_mut();
        } else {
            refcount_inc(&raw mut (*shd).refcount);
        }
    }

    mutex_unlock(&raw mut shd_mutex as *mut c_void);
    if !shd.is_null() { priv_to_devlink(shd) } else { core::ptr::null_mut() }
}

pub unsafe fn devlink_shd_put(devlink: *mut devlink) {
    mutex_lock(&raw mut shd_mutex as *mut c_void);
    let shd = devlink_priv(devlink);
    if refcount_dec_and_test(&raw mut (*shd).refcount) {
        devlink_shd_destroy(shd);
    }
    mutex_unlock(&raw mut shd_mutex as *mut c_void);
}

pub unsafe fn devlink_shd_get_priv(devlink: *mut devlink) -> *mut c_void {
    let shd = devlink_priv(devlink);
    (*shd).priv_data.as_mut_ptr() as *mut c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
