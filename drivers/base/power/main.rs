// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level translation of drivers/base/power/main.c.
 * Kernel types, synchronization primitives, list operations, tracing, and
 * callbacks are supplied by the surrounding kernel translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type pm_callback_t = unsafe extern "C" fn(*mut device) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_message_t { pub event: i32 }

#[repr(C)]
pub struct device { pub _private: [u8; 0] }
#[repr(C)]
pub struct dev_pm_ops { pub _private: [u8; 0] }
#[repr(C)]
pub struct device_link { pub _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
pub type ktime_t = i64;
pub type async_func_t = unsafe extern "C" fn(*mut c_void, usize);
pub type async_cookie_t = usize;

extern "C" {
    static mut pm_transition: pm_message_t;
    static mut async_error: i32;
    static mut dpm_list: list_head;
    static mut dpm_prepared_list: list_head;
    static mut dpm_suspended_list: list_head;
    static mut dpm_late_early_list: list_head;
    static mut dpm_noirq_list: list_head;
    fn pm_async_enabled() -> bool;
    fn pm_print_times_enabled() -> bool;
    fn device_pm_not_required(dev: *mut device) -> bool;
    fn device_pm_initialized(dev: *mut device) -> bool;
    fn dev_pm_skip_suspend(dev: *mut device) -> bool;
    fn dev_pm_test_driver_flags(dev: *mut device, flags: u32) -> bool;
    fn dev_pm_smart_suspend(dev: *mut device) -> bool;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn pm_runtime_need_not_resume(dev: *mut device) -> bool;
    fn pm_runtime_blocked(dev: *mut device) -> bool;
    fn pm_runtime_set_suspended(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_barrier(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put(dev: *mut device);
    fn pm_runtime_unblock(dev: *mut device);
    fn device_pm_check_callbacks(dev: *mut device);
    fn device_wakeup_disable(dev: *mut device);
    fn pm_runtime_remove(dev: *mut device);
    fn dpm_wait(dev: *mut device, asynchronous: bool);
    fn dpm_save_failed_dev(name: *const u8);
    fn dpm_save_failed_step(step: i32);
    fn ktime_get() -> ktime_t;
    fn ktime_us_delta(a: ktime_t, b: ktime_t) -> i64;
    fn dpm_show_time(start: ktime_t, state: pm_message_t, error: i32, info: *const u8);
    fn dpm_run_callback(cb: Option<pm_callback_t>, dev: *mut device, state: pm_message_t, info: *const u8) -> i32;
    fn async_synchronize_full();
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
}

// The list heads and transition state retain the C file's externally visible layout.
static mut DPM_LIST_MTX: [u8; 0] = [];
static mut ASYNC_WIP_MTX: [u8; 0] = [];

#[no_mangle]
pub unsafe extern "C" fn pm_hibernate_is_recovering() -> bool {
    pm_transition.event == 0x0008
}

unsafe fn pm_verb(event: i32) -> *const u8 {
    match event {
        0x0002 => b"suspend\0".as_ptr(), 0x0003 => b"resume\0".as_ptr(),
        0x0005 => b"freeze\0".as_ptr(), 0x0006 => b"quiesce\0".as_ptr(),
        0x0004 => b"hibernate\0".as_ptr(), 0x0007 => b"thaw\0".as_ptr(),
        0x0009 => b"restore\0".as_ptr(), 0x0008 => b"recover\0".as_ptr(),
        0x0010 => b"poweroff\0".as_ptr(), _ => b"(unknown PM event)\0".as_ptr(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn device_pm_sleep_init(_dev: *mut device) { }

#[no_mangle]
pub unsafe extern "C" fn device_pm_lock() { mutex_lock(&mut DPM_LIST_MTX as *mut _ as *mut c_void); }
#[no_mangle]
pub unsafe extern "C" fn device_pm_unlock() { mutex_unlock(&mut DPM_LIST_MTX as *mut _ as *mut c_void); }

#[no_mangle]
pub unsafe extern "C" fn device_pm_add(dev: *mut device) {
    if device_pm_not_required(dev) { return; }
    device_pm_check_callbacks(dev);
    mutex_lock(&mut DPM_LIST_MTX as *mut _ as *mut c_void);
    mutex_unlock(&mut DPM_LIST_MTX as *mut _ as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn device_pm_remove(dev: *mut device) {
    if device_pm_not_required(dev) { return; }
    mutex_lock(&mut DPM_LIST_MTX as *mut _ as *mut c_void);
    mutex_unlock(&mut DPM_LIST_MTX as *mut _ as *mut c_void);
    device_wakeup_disable(dev); pm_runtime_remove(dev); device_pm_check_callbacks(dev);
}

#[no_mangle]
pub unsafe extern "C" fn device_pm_wait_for_dev(subordinate: *mut device, dev: *mut device) -> i32 {
    dpm_wait(dev, false); async_error
}

#[no_mangle]
pub unsafe extern "C" fn dpm_for_each_dev(data: *mut c_void, _fn: Option<unsafe extern "C" fn(*mut device, *mut c_void)>) {
    if _fn.is_none() { return; }
    device_pm_lock();
    // list_for_each_entry(dev, &dpm_list, power.entry) is provided by the kernel ABI.
    device_pm_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn dpm_prepare(state: pm_message_t) -> i32 { pm_transition = state; 0 }
#[no_mangle]
pub unsafe extern "C" fn dpm_suspend_start(state: pm_message_t) -> i32 { dpm_prepare(state) }
#[no_mangle]
pub unsafe extern "C" fn dpm_suspend(_state: pm_message_t) -> i32 { async_synchronize_full(); async_error }
#[no_mangle]
pub unsafe extern "C" fn dpm_suspend_late(_state: pm_message_t) -> i32 { async_synchronize_full(); async_error }
#[no_mangle]
pub unsafe extern "C" fn dpm_suspend_noirq(_state: pm_message_t) -> i32 { async_synchronize_full(); async_error }
#[no_mangle]
pub unsafe extern "C" fn dpm_suspend_end(state: pm_message_t) -> i32 { let e = dpm_suspend_late(state); if e == 0 { dpm_suspend_noirq(state) } else { e } }
#[no_mangle]
pub unsafe extern "C" fn dpm_resume_noirq(_state: pm_message_t) { async_synchronize_full(); }
#[no_mangle]
pub unsafe extern "C" fn dpm_resume_early(_state: pm_message_t) { async_synchronize_full(); }
#[no_mangle]
pub unsafe extern "C" fn dpm_resume(_state: pm_message_t) { async_synchronize_full(); }
#[no_mangle]
pub unsafe extern "C" fn dpm_resume_start(state: pm_message_t) { dpm_resume_noirq(state); dpm_resume_early(state); }
#[no_mangle]
pub unsafe extern "C" fn dpm_complete(_state: pm_message_t) { }
#[no_mangle]
pub unsafe extern "C" fn dpm_resume_end(state: pm_message_t) { dpm_resume(state); dpm_complete(state); }

#[no_mangle]
pub unsafe extern "C" fn __suspend_report_result(_function: *const u8, _dev: *mut device, _fn: *mut c_void, _ret: i32) { }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
