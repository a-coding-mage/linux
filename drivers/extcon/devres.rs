// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/extcon/devres.c - EXTCON device's resource management
 *
 * Copyright (C) 2016 Samsung Electronics
 * Author: Chanwoo Choi <cw00.choi@samsung.com>
 */

// Dependency declarations supplied by extcon and device-resource code.
use core::ffi::c_void;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct extcon_dev {
    pub dev: device,
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

extern "C" {
    fn devres_alloc(release: unsafe extern "C" fn(*mut device, *mut c_void), size: usize, gfp: u32) -> *mut c_void;
    fn devres_free(res: *mut c_void);
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn devres_release(
        dev: *mut device,
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        match_fn: unsafe extern "C" fn(*mut device, *mut c_void, *mut c_void) -> i32,
        data: *mut c_void,
    ) -> i32;
    fn extcon_dev_allocate(supported_cable: *const u32) -> *mut extcon_dev;
    fn extcon_dev_free(edev: *mut extcon_dev);
    fn extcon_dev_register(edev: *mut extcon_dev) -> i32;
    fn extcon_dev_unregister(edev: *mut extcon_dev);
    fn extcon_register_notifier(edev: *mut extcon_dev, id: u32, nb: *mut notifier_block) -> i32;
    fn extcon_unregister_notifier(edev: *mut extcon_dev, id: u32, nb: *mut notifier_block);
    fn extcon_register_notifier_all(edev: *mut extcon_dev, nb: *mut notifier_block) -> i32;
    fn extcon_unregister_notifier_all(edev: *mut extcon_dev, nb: *mut notifier_block);
    fn warn_on(condition: bool) -> bool;
}

const GFP_KERNEL: u32 = 0;

unsafe extern "C" fn devm_extcon_dev_match(_dev: *mut device, res: *mut c_void, data: *mut c_void) -> i32 {
    let r = res as *mut *mut extcon_dev;

    if warn_on(r.is_null() || (*r).is_null()) {
        return 0;
    }

    if *r == data as *mut extcon_dev { 1 } else { 0 }
}

unsafe extern "C" fn devm_extcon_dev_release(_dev: *mut device, res: *mut c_void) {
    extcon_dev_free(*(res as *mut *mut extcon_dev));
}

unsafe extern "C" fn devm_extcon_dev_unreg(_dev: *mut device, res: *mut c_void) {
    extcon_dev_unregister(*(res as *mut *mut extcon_dev));
}

#[repr(C)]
struct extcon_dev_notifier_devres {
    edev: *mut extcon_dev,
    id: u32,
    nb: *mut notifier_block,
}

unsafe extern "C" fn devm_extcon_dev_notifier_unreg(_dev: *mut device, res: *mut c_void) {
    let this = &mut *(res as *mut extcon_dev_notifier_devres);

    extcon_unregister_notifier(this.edev, this.id, this.nb);
}

unsafe extern "C" fn devm_extcon_dev_notifier_all_unreg(_dev: *mut device, res: *mut c_void) {
    let this = &mut *(res as *mut extcon_dev_notifier_devres);

    extcon_unregister_notifier_all(this.edev, this.nb);
}

/**
 * devm_extcon_dev_allocate - Allocate managed extcon device
 * @dev:              the device owning the extcon device being created
 * @supported_cable:  the array of the supported external connectors
 *                    ending with EXTCON_NONE.
 *
 * This function manages automatically the memory of extcon device using
 * device resource management and simplify the control of freeing the memory
 * of extcon device.
 *
 * Returns the pointer memory of allocated extcon_dev if success
 * or ERR_PTR(err) if fail
 */
pub unsafe extern "C" fn devm_extcon_dev_allocate(
    dev: *mut device,
    supported_cable: *const u32,
) -> *mut extcon_dev {
    let ptr = devres_alloc(devm_extcon_dev_release, core::mem::size_of::<*mut extcon_dev>(), GFP_KERNEL)
        as *mut *mut extcon_dev;
    if ptr.is_null() {
        return (-12isize) as *mut extcon_dev;
    }

    let edev = extcon_dev_allocate(supported_cable);
    if (edev as isize) < 0 {
        devres_free(ptr as *mut c_void);
        return edev;
    }

    (*edev).dev.parent = dev;
    *ptr = edev;
    devres_add(dev, ptr as *mut c_void);

    edev
}

pub unsafe extern "C" fn devm_extcon_dev_free(dev: *mut device, edev: *mut extcon_dev) {
    if warn_on(devres_release(dev, devm_extcon_dev_release, devm_extcon_dev_match, edev as *mut c_void) != 0) {}
}

pub unsafe extern "C" fn devm_extcon_dev_register(dev: *mut device, edev: *mut extcon_dev) -> i32 {
    let ptr = devres_alloc(devm_extcon_dev_unreg, core::mem::size_of::<*mut extcon_dev>(), GFP_KERNEL)
        as *mut *mut extcon_dev;
    if ptr.is_null() {
        return -12;
    }

    let ret = extcon_dev_register(edev);
    if ret != 0 {
        devres_free(ptr as *mut c_void);
        return ret;
    }

    *ptr = edev;
    devres_add(dev, ptr as *mut c_void);
    0
}

pub unsafe extern "C" fn devm_extcon_dev_unregister(dev: *mut device, edev: *mut extcon_dev) {
    if warn_on(devres_release(dev, devm_extcon_dev_unreg, devm_extcon_dev_match, edev as *mut c_void) != 0) {}
}

pub unsafe extern "C" fn devm_extcon_register_notifier(
    dev: *mut device,
    edev: *mut extcon_dev,
    id: u32,
    nb: *mut notifier_block,
) -> i32 {
    let ptr = devres_alloc(devm_extcon_dev_notifier_unreg, core::mem::size_of::<extcon_dev_notifier_devres>(), GFP_KERNEL)
        as *mut extcon_dev_notifier_devres;
    if ptr.is_null() {
        return -12;
    }

    let ret = extcon_register_notifier(edev, id, nb);
    if ret != 0 {
        devres_free(ptr as *mut c_void);
        return ret;
    }

    (*ptr).edev = edev;
    (*ptr).id = id;
    (*ptr).nb = nb;
    devres_add(dev, ptr as *mut c_void);
    0
}

pub unsafe extern "C" fn devm_extcon_unregister_notifier(
    dev: *mut device,
    edev: *mut extcon_dev,
    _id: u32,
    _nb: *mut notifier_block,
) {
    if warn_on(devres_release(dev, devm_extcon_dev_notifier_unreg, devm_extcon_dev_match, edev as *mut c_void) != 0) {}
}

pub unsafe extern "C" fn devm_extcon_register_notifier_all(
    dev: *mut device,
    edev: *mut extcon_dev,
    nb: *mut notifier_block,
) -> i32 {
    let ptr = devres_alloc(devm_extcon_dev_notifier_all_unreg, core::mem::size_of::<extcon_dev_notifier_devres>(), GFP_KERNEL)
        as *mut extcon_dev_notifier_devres;
    if ptr.is_null() {
        return -12;
    }

    let ret = extcon_register_notifier_all(edev, nb);
    if ret != 0 {
        devres_free(ptr as *mut c_void);
        return ret;
    }

    (*ptr).edev = edev;
    (*ptr).nb = nb;
    devres_add(dev, ptr as *mut c_void);
    0
}

pub unsafe extern "C" fn devm_extcon_unregister_notifier_all(
    dev: *mut device,
    edev: *mut extcon_dev,
    _nb: *mut notifier_block,
) {
    if warn_on(devres_release(dev, devm_extcon_dev_notifier_all_unreg, devm_extcon_dev_match, edev as *mut c_void) != 0) {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
