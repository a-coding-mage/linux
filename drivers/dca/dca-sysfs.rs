// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright(c) 2007 - 2009 Intel Corporation. All rights reserved.
 */

// Kernel declarations supplied by the surrounding translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct class {
    pub name: *const u8,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct idr;
#[repr(C)]
pub struct spinlock_t;

#[repr(C)]
pub struct dca_provider {
    pub cd: *mut device,
    pub id: i32,
}

extern "C" {
    fn device_create(
        class: *const class,
        parent: *mut device,
        devt: u32,
        drvdata: *mut c_void,
        fmt: *const u8,
        ...,
    ) -> *mut device;
    fn device_destroy(class: *const class, devt: u32);
    fn device_unregister(dev: *mut device);
    fn class_register(class: *const class) -> i32;
    fn class_unregister(class: *const class);
    fn idr_preload(gfp_mask: u32);
    fn idr_preload_end();
    fn idr_alloc(
        idr: *mut idr,
        ptr: *mut c_void,
        start: i32,
        end: i32,
        gfp_mask: u32,
    ) -> i32;
    fn idr_remove(idr: *mut idr, id: i32);
    fn idr_init(idr: *mut idr);
    fn idr_destroy(idr: *mut idr);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn ptr_err_or_zero(ptr: *mut device) -> i32;
    fn ptr_err(ptr: *mut device) -> i32;
    fn is_err(ptr: *mut device) -> bool;
}

const GFP_KERNEL: u32 = 0;
const GFP_NOWAIT: u32 = 0;

const fn mkdev(major: u32, minor: i32) -> u32 {
    ((major & 0xfff) << 20) | ((minor as u32) & 0xfffff)
}

static DCA_CLASS: class = class {
    name: b"dca\0".as_ptr(),
};
static mut DCA_IDR: idr = unsafe { core::mem::zeroed() };
static mut DCA_IDR_LOCK: spinlock_t = unsafe { core::mem::zeroed() };

pub unsafe extern "C" fn dca_sysfs_add_req(
    dca: *mut dca_provider,
    _dev: *mut device,
    slot: i32,
) -> i32 {
    let cd: *mut device;
    static mut REQ_COUNT: i32 = 0;

    cd = device_create(
        &DCA_CLASS,
        (*dca).cd,
        mkdev(0, slot + 1),
        core::ptr::null_mut(),
        b"requester%d\0".as_ptr(),
        REQ_COUNT,
    );
    REQ_COUNT = REQ_COUNT.wrapping_add(1);
    ptr_err_or_zero(cd)
}

pub unsafe extern "C" fn dca_sysfs_remove_req(_dca: *mut dca_provider, slot: i32) {
    device_destroy(&DCA_CLASS, mkdev(0, slot + 1));
}

pub unsafe extern "C" fn dca_sysfs_add_provider(
    dca: *mut dca_provider,
    dev: *mut device,
) -> i32 {
    let cd: *mut device;
    let ret: i32;

    idr_preload(GFP_KERNEL);
    spin_lock(&mut DCA_IDR_LOCK);

    ret = idr_alloc(
        &mut DCA_IDR,
        dca.cast::<c_void>(),
        0,
        0,
        GFP_NOWAIT,
    );
    if ret >= 0 {
        (*dca).id = ret;
    }

    spin_unlock(&mut DCA_IDR_LOCK);
    idr_preload_end();
    if ret < 0 {
        return ret;
    }

    cd = device_create(
        &DCA_CLASS,
        dev,
        mkdev(0, 0),
        core::ptr::null_mut(),
        b"dca%d\0".as_ptr(),
        (*dca).id,
    );
    if is_err(cd) {
        spin_lock(&mut DCA_IDR_LOCK);
        idr_remove(&mut DCA_IDR, (*dca).id);
        spin_unlock(&mut DCA_IDR_LOCK);
        return ptr_err(cd);
    }
    (*dca).cd = cd;
    0
}

pub unsafe extern "C" fn dca_sysfs_remove_provider(dca: *mut dca_provider) {
    device_unregister((*dca).cd);
    (*dca).cd = core::ptr::null_mut();
    spin_lock(&mut DCA_IDR_LOCK);
    idr_remove(&mut DCA_IDR, (*dca).id);
    spin_unlock(&mut DCA_IDR_LOCK);
}

pub unsafe extern "C" fn dca_sysfs_init() -> i32 {
    let err: i32;

    idr_init(&mut DCA_IDR);
    spin_lock_init(&mut DCA_IDR_LOCK);

    err = class_register(&DCA_CLASS);
    if err != 0 {
        idr_destroy(&mut DCA_IDR);
        return err;
    }
    0
}

pub unsafe extern "C" fn dca_sysfs_exit() {
    class_unregister(&DCA_CLASS);
    idr_destroy(&mut DCA_IDR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
