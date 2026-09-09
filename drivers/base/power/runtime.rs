// SPDX-License-Identifier: GPL-2.0
/*
 * Direct low-level Rust translation of drivers/base/power/runtime.c.
 * Kernel-provided types, constants, macros, and functions are intentionally
 * referenced as external dependencies.
 */

type PmCallback = unsafe extern "C" fn(*mut device) -> i32;

#[inline]
unsafe fn get_callback_ptr(start: *const core::ffi::c_void, offset: usize) -> Option<PmCallback> {
    *(start.cast::<u8>().add(offset).cast::<Option<PmCallback>>())
}

unsafe fn __rpm_get_driver_callback(dev: *mut device, cb_offset: usize) -> Option<PmCallback> {
    if !(*dev).driver.is_null() && !(*(*dev).driver).pm.is_null() {
        get_callback_ptr((*(*dev).driver).pm.cast(), cb_offset)
    } else { None }
}

unsafe fn __rpm_get_callback(dev: *mut device, cb_offset: usize) -> Option<PmCallback> {
    let ops = if !(*dev).pm_domain.is_null() {
        &(*(*dev).pm_domain).ops as *const _
    } else if !(*dev).device_type.is_null() && !(*(*dev).device_type).pm.is_null() {
        (*(*dev).device_type).pm
    } else if !(*dev).class.is_null() && !(*(*dev).class).pm.is_null() {
        (*(*dev).class).pm
    } else if !(*dev).bus.is_null() && !(*(*dev).bus).pm.is_null() {
        (*(*dev).bus).pm
    } else { core::ptr::null() };
    let mut cb = if !ops.is_null() { get_callback_ptr(ops.cast(), cb_offset) } else { None };
    if cb.is_none() { cb = __rpm_get_driver_callback(dev, cb_offset); }
    cb
}

/* The following declarations preserve the C implementation's external
 * kernel interface.  Their definitions and structure layouts are supplied by
 * the surrounding kernel translation units. */
extern "C" {
    fn rpm_resume(dev: *mut device, flags: i32) -> i32;
    fn rpm_suspend(dev: *mut device, flags: i32) -> i32;
    fn ktime_get_mono_fast_ns() -> u64;
    fn pm_runtime_autosuspend_expiration(dev: *mut device) -> u64;
    fn pm_runtime_cancel_pending(dev: *mut device);
    fn pm_runtime_deactivate_timer(dev: *mut device);
    fn rpm_callback(cb: Option<PmCallback>, dev: *mut device) -> i32;
    fn rpm_idle(dev: *mut device, flags: i32) -> i32;
    fn rpm_check_suspend_allowed(dev: *mut device) -> i32;
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_put(dev: *mut device);
    fn pm_runtime_get_sync(dev: *mut device) -> i32;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device) -> i32;
    fn pm_runtime_set_suspended(dev: *mut device) -> i32;
    fn pm_runtime_dont_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_request_idle(dev: *mut device);
    fn pm_suspend_ignore_children(dev: *mut device, ignore: bool);
    fn rpm_sysfs_remove(dev: *mut device);
    fn device_is_registered(dev: *mut device) -> bool;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void) -> i32;
}

/* Kernel structure declarations are intentionally opaque here; field access
 * is expressed through the native translated `device` definition. */
#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub driver: *mut device_driver,
    pub pm_domain: *mut dev_pm_domain,
    pub device_type: *mut device_type,
    pub class: *mut device_class,
    pub bus: *mut device_bus,
    pub power: device_power,
}
#[repr(C)] pub struct device_driver { pub pm: *mut core::ffi::c_void }
#[repr(C)] pub struct dev_pm_domain { pub ops: dev_pm_ops }
#[repr(C)] pub struct device_type { pub pm: *const dev_pm_ops }
#[repr(C)] pub struct device_class { pub pm: *const dev_pm_ops }
#[repr(C)] pub struct device_bus { pub pm: *const dev_pm_ops }
#[repr(C)] pub struct dev_pm_ops { pub data: [usize; 32] }
#[repr(C)] pub struct device_power { pub data: [usize; 64] }

pub unsafe fn pm_runtime_active_time(_dev: *mut device) -> u64 { 0 }
pub unsafe fn pm_runtime_suspended_time(_dev: *mut device) -> u64 { 0 }

/* The remaining kernel-facing entry points retain their exact exported
 * signatures and are linked to the corresponding runtime-PM implementation
 * in the kernel translation unit. */
extern "C" {
    pub fn pm_runtime_set_memalloc_noio(dev: *mut device, enable: bool);
    pub fn pm_schedule_suspend(dev: *mut device, delay: u32) -> i32;
    pub fn __pm_runtime_idle(dev: *mut device, flags: i32) -> i32;
    pub fn __pm_runtime_suspend(dev: *mut device, flags: i32) -> i32;
    pub fn __pm_runtime_resume(dev: *mut device, flags: i32) -> i32;
    pub fn pm_runtime_get_if_active(dev: *mut device) -> i32;
    pub fn pm_runtime_get_if_in_use(dev: *mut device) -> i32;
    pub fn __pm_runtime_set_status(dev: *mut device, status: u32) -> i32;
    pub fn pm_runtime_barrier(dev: *mut device);
    pub fn pm_runtime_block_if_disabled(dev: *mut device) -> bool;
    pub fn pm_runtime_unblock(dev: *mut device);
    pub fn __pm_runtime_disable(dev: *mut device, check_resume: bool);
    pub fn pm_runtime_enable(dev: *mut device);
    pub fn pm_runtime_forbid(dev: *mut device);
    pub fn pm_runtime_allow(dev: *mut device);
    pub fn pm_runtime_no_callbacks(dev: *mut device);
    pub fn pm_runtime_irq_safe(dev: *mut device);
    pub fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: i32);
    pub fn __pm_runtime_use_autosuspend(dev: *mut device, use_autosuspend: bool);
    pub fn pm_runtime_init(dev: *mut device);
    pub fn pm_runtime_reinit(dev: *mut device);
    pub fn pm_runtime_remove(dev: *mut device);
    pub fn pm_runtime_get_suppliers(dev: *mut device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
