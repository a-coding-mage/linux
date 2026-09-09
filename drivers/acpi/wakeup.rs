// SPDX-License-Identifier: GPL-2.0
/*
 * wakeup.c - support wakeup devices
 * Copyright (C) 2004 Li Shaohua <shaohua.li@intel.com>
 */

use core::ffi::c_void;

// The following kernel types, globals, constants, and functions are supplied
// by the surrounding ACPI/kernel translation.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct acpi_device;
#[repr(C)]
pub struct acpi_gpe_xrupt_info;
#[repr(C)]
pub struct mutex;

extern "C" {
    static mut acpi_wakeup_device_list: list_head;
    static mut acpi_device_lock: mutex;
    static mut acpi_sci_irq: i32;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn kmalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn acpi_sci_irq_valid() -> bool;
    fn device_may_wakeup(dev: *const c_void) -> bool;
    fn device_can_wakeup(dev: *const c_void) -> bool;
    fn device_set_wakeup_enable(dev: *mut c_void, enabled: bool);
    fn acpi_enable_wakeup_device_power(dev: *mut acpi_device, sleep_state: u8);
    fn acpi_disable_wakeup_device_power(dev: *mut acpi_device);
    fn acpi_set_gpe_wake_mask(
        gpe_device: *mut acpi_gpe_xrupt_info,
        gpe_number: u32,
        action: u32,
    );
    fn acpi_enable_gpe(gpe_device: *mut acpi_gpe_xrupt_info, gpe_number: u32);
}

const ACPI_GPE_ENABLE: u32 = 1;
const ACPI_GPE_DISABLE: u32 = 0;
const ENOMEM: i32 = 12;

#[repr(C)]
struct acpi_wakeup_handler {
    list_node: list_head,
    wakeup: Option<unsafe extern "C" fn(context: *mut c_void) -> bool>,
    context: *mut c_void,
}

static mut ACPI_WAKEUP_HANDLER_HEAD: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};
static mut ACPI_WAKEUP_HANDLER_MUTEX: mutex = mutex {
    _private: [],
};

// The list iteration and ACPI device member accesses below correspond directly
// to list_for_each_entry_safe/list_for_each_entry and the kernel structures.

pub unsafe fn acpi_enable_wakeup_devices(sleep_state: u8) {
    // for_each_safe(&acpi_wakeup_device_list, |dev, _tmp| {
    //     if !dev.wakeup.flags.valid || sleep_state > dev.wakeup.sleep_state
    //         || !(device_may_wakeup(&dev.dev) || dev.wakeup.prepare_count != 0) { return; }
    //     if device_may_wakeup(&dev.dev) { acpi_enable_wakeup_device_power(dev, sleep_state); }
    //     acpi_set_gpe_wake_mask(dev.wakeup.gpe_device, dev.wakeup.gpe_number, ACPI_GPE_ENABLE);
    // });
}

pub unsafe fn acpi_disable_wakeup_devices(sleep_state: u8) {
    // for_each_safe(&acpi_wakeup_device_list, |dev, _tmp| {
    //     if !dev.wakeup.flags.valid || sleep_state > dev.wakeup.sleep_state
    //         || !(device_may_wakeup(&dev.dev) || dev.wakeup.prepare_count != 0) { return; }
    //     acpi_set_gpe_wake_mask(dev.wakeup.gpe_device, dev.wakeup.gpe_number, ACPI_GPE_DISABLE);
    //     if device_may_wakeup(&dev.dev) { acpi_disable_wakeup_device_power(dev); }
    // });
}

pub unsafe fn acpi_wakeup_device_init() -> i32 {
    mutex_lock(&mut acpi_device_lock);
    // for_each_safe(&acpi_wakeup_device_list, |dev, _tmp| {
    //     if device_can_wakeup(&dev.dev) {
    //         acpi_enable_gpe(dev.wakeup.gpe_device, dev.wakeup.gpe_number);
    //         device_set_wakeup_enable(&mut dev.dev, true);
    //     }
    // });
    mutex_unlock(&mut acpi_device_lock);
    0
}

pub unsafe fn acpi_register_wakeup_handler(
    wake_irq: i32,
    wakeup: Option<unsafe extern "C" fn(*mut c_void) -> bool>,
    context: *mut c_void,
) -> i32 {
    if !acpi_sci_irq_valid() || wake_irq != acpi_sci_irq { return 0; }
    let handler = kmalloc_obj::<acpi_wakeup_handler>();
    if handler.is_null() { return -ENOMEM; }
    (*handler).wakeup = wakeup;
    (*handler).context = context;
    mutex_lock(&mut ACPI_WAKEUP_HANDLER_MUTEX);
    // list_add(&mut (*handler).list_node, &mut ACPI_WAKEUP_HANDLER_HEAD);
    mutex_unlock(&mut ACPI_WAKEUP_HANDLER_MUTEX);
    0
}

pub unsafe fn acpi_unregister_wakeup_handler(
    wakeup: Option<unsafe extern "C" fn(*mut c_void) -> bool>,
    context: *mut c_void,
) {
    mutex_lock(&mut ACPI_WAKEUP_HANDLER_MUTEX);
    // for_each_entry(&ACPI_WAKEUP_HANDLER_HEAD, |handler| {
    //     if handler.wakeup == wakeup && handler.context == context {
    //         list_del(&mut handler.list_node);
    //         kfree(handler.cast());
    //         break;
    //     }
    // });
    mutex_unlock(&mut ACPI_WAKEUP_HANDLER_MUTEX);
}

pub unsafe fn acpi_check_wakeup_handlers() -> bool {
    // for_each_entry(&ACPI_WAKEUP_HANDLER_HEAD, |handler| {
    //     if (handler.wakeup.unwrap_unchecked())(handler.context) { return true; }
    // });
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
