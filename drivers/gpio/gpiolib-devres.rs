/* SPDX-License-Identifier: GPL-2.0 */
/*
 * devres.c - managed gpio resources
 * This file is based on kernel/irq/devres.c
 *
 * Copyright (c) 2011 John Crispin <john@phrozen.org>
 */

/* Translated from the Linux kernel C implementation. */

use core::ffi::c_void;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_descs { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_chip {
    pub parent: *mut device,
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)]
pub struct lock_class_key { _private: [u8; 0] }

pub type gpiod_flags = u32;
pub const GPIOD_FLAGS_BIT_NONEXCLUSIVE: gpiod_flags = 1 << 1;

extern "C" {
    fn gpiod_put(desc: *mut gpio_desc);
    fn gpiod_put_array(descs: *mut gpio_descs);
    fn devm_gpiod_get_index(dev: *mut device, con_id: *const i8,
                            index: u32, flags: gpiod_flags) -> *mut gpio_desc;
    fn devm_gpiod_get_index_optional(dev: *mut device, con_id: *const i8,
                                     index: u32, flags: gpiod_flags) -> *mut gpio_desc;
    fn gpiod_get_index(dev: *mut device, con_id: *const i8,
                       index: u32, flags: gpiod_flags) -> *mut gpio_desc;
    fn gpiod_find_and_request(dev: *mut device, fwnode: *mut fwnode_handle,
                              con_id: *const i8, index: i32, flags: gpiod_flags,
                              label: *const i8, lookup: bool) -> *mut gpio_desc;
    fn gpiod_get_array(dev: *mut device, con_id: *const i8,
                       flags: gpiod_flags) -> *mut gpio_descs;
    fn gpiochip_remove(gc: *mut gpio_chip);
    fn gpiochip_add_data_with_key(gc: *mut gpio_chip, data: *mut c_void,
                                  lock_key: *mut lock_class_key,
                                  request_key: *mut lock_class_key) -> i32;
    fn devm_is_action_added(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> bool;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> i32;
    fn devm_release_action(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void);
    fn devm_remove_action_nowarn(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> i32;
}

const ENOENT: i32 = 2;

unsafe fn is_err<T>(ptr: *mut T) -> bool { (ptr as isize) < 0 }
unsafe fn is_err_or_null<T>(ptr: *mut T) -> bool { ptr.is_null() || is_err(ptr) }
unsafe fn err_ptr<T>(err: i32) -> *mut T { err as isize as *mut T }
unsafe fn gpiod_not_found<T>(ptr: *mut T) -> bool { ptr.is_null() || ptr == err_ptr(-ENOENT) }
unsafe fn warn_on(condition: bool) { let _ = condition; }

unsafe extern "C" fn devm_gpiod_release(desc: *mut c_void) {
    gpiod_put(desc as *mut gpio_desc);
}

unsafe extern "C" fn devm_gpiod_release_array(descs: *mut c_void) {
    gpiod_put_array(descs as *mut gpio_descs);
}

pub unsafe fn devm_gpiod_get(dev: *mut device, con_id: *const i8,
                             flags: gpiod_flags) -> *mut gpio_desc {
    devm_gpiod_get_index(dev, con_id, 0, flags)
}

pub unsafe fn devm_gpiod_get_optional(dev: *mut device, con_id: *const i8,
                                      flags: gpiod_flags) -> *mut gpio_desc {
    devm_gpiod_get_index_optional(dev, con_id, 0, flags)
}

pub unsafe fn devm_gpiod_get_index(dev: *mut device, con_id: *const i8,
                                   idx: u32, flags: gpiod_flags) -> *mut gpio_desc {
    let desc = gpiod_get_index(dev, con_id, idx, flags);
    if is_err(desc) { return desc; }
    if flags & GPIOD_FLAGS_BIT_NONEXCLUSIVE != 0 &&
       devm_is_action_added(dev, devm_gpiod_release, desc as *mut c_void) {
        return desc;
    }
    let ret = devm_add_action_or_reset(dev, devm_gpiod_release, desc as *mut c_void);
    if ret != 0 { return err_ptr(ret); }
    desc
}

pub unsafe fn devm_fwnode_gpiod_get_index(dev: *mut device, fwnode: *mut fwnode_handle,
                                          con_id: *const i8, index: i32,
                                          flags: gpiod_flags, label: *const i8) -> *mut gpio_desc {
    let desc = gpiod_find_and_request(dev, fwnode, con_id, index, flags, label, false);
    if is_err(desc) { return desc; }
    let ret = devm_add_action_or_reset(dev, devm_gpiod_release, desc as *mut c_void);
    if ret != 0 { return err_ptr(ret); }
    desc
}

pub unsafe fn devm_gpiod_get_index_optional(dev: *mut device, con_id: *const i8,
                                            index: u32, flags: gpiod_flags) -> *mut gpio_desc {
    let desc = devm_gpiod_get_index(dev, con_id, index, flags);
    if gpiod_not_found(desc) { return core::ptr::null_mut(); }
    desc
}

pub unsafe fn devm_gpiod_get_array(dev: *mut device, con_id: *const i8,
                                   flags: gpiod_flags) -> *mut gpio_descs {
    let descs = gpiod_get_array(dev, con_id, flags);
    if is_err(descs) { return descs; }
    let ret = devm_add_action_or_reset(dev, devm_gpiod_release_array, descs as *mut c_void);
    if ret != 0 { return err_ptr(ret); }
    descs
}

pub unsafe fn devm_gpiod_get_array_optional(dev: *mut device, con_id: *const i8,
                                            flags: gpiod_flags) -> *mut gpio_descs {
    let descs = devm_gpiod_get_array(dev, con_id, flags);
    if gpiod_not_found(descs) { return core::ptr::null_mut(); }
    descs
}

pub unsafe fn devm_gpiod_put(dev: *mut device, desc: *mut gpio_desc) {
    devm_release_action(dev, devm_gpiod_release, desc as *mut c_void);
}

pub unsafe fn devm_gpiod_unhinge(dev: *mut device, desc: *mut gpio_desc) {
    if is_err_or_null(desc) { return; }
    let ret = devm_remove_action_nowarn(dev, devm_gpiod_release, desc as *mut c_void);
    if ret == -ENOENT { return; }
    warn_on(ret != 0);
}

pub unsafe fn devm_gpiod_put_array(dev: *mut device, descs: *mut gpio_descs) {
    devm_release_action(dev, devm_gpiod_release_array, descs as *mut c_void);
}

unsafe extern "C" fn devm_gpio_chip_release(data: *mut c_void) {
    gpiochip_remove(data as *mut gpio_chip);
}

pub unsafe fn devm_gpiochip_add_data_with_key(dev: *mut device, gc: *mut gpio_chip,
                                              data: *mut c_void, lock_key: *mut lock_class_key,
                                              request_key: *mut lock_class_key) -> i32 {
    if (*gc).parent.is_null() { (*gc).parent = dev; }
    let ret = gpiochip_add_data_with_key(gc, data, lock_key, request_key);
    if ret < 0 { return ret; }
    devm_add_action_or_reset(dev, devm_gpio_chip_release, gc as *mut c_void)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
