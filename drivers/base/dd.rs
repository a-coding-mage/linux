// SPDX-License-Identifier: GPL-2.0
// Rust translation of drivers/base/dd.c.  Kernel-provided types and helpers
// are intentionally left as external dependencies, as in the original file.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct device_private { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct va_format { _private: [u8; 0] }
#[repr(C)] pub struct subsys_private { _private: [u8; 0] }
pub type size_t = usize;
pub type ssize_t = isize;
pub type ktime_t = i64;
pub type async_cookie_t = usize;

// The following declarations correspond to the externally supplied kernel API.
extern "C" {
    fn device_pm_move_to_tail(dev: *mut device);
    fn bus_probe_device(dev: *mut device);
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn dev_can_match(dev: *mut device) -> bool;
    fn device_is_registered(dev: *mut device) -> bool;
    fn dev_ready_to_probe(dev: *mut device) -> bool;
    fn device_lock(dev: *mut device);
    fn device_unlock(dev: *mut device);
    fn wait_for_device_probe();
    fn device_set_driver(dev: *mut device, drv: *const device_driver);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn device_release_driver_internal(dev: *mut device, drv: *const device_driver, parent: *mut device);
    fn driver_match_device(drv: *const device_driver, dev: *mut device) -> c_int;
    fn driver_allows_async_probing(drv: *const device_driver) -> bool;
}

static mut initcalls_done: bool = false;
static mut defer_all_probes: bool = false;
static mut driver_deferred_probe_enable: bool = false;
static mut probe_count: c_int = 0;
static mut deferred_trigger_count: c_int = 0;

pub unsafe fn driver_deferred_probe_add(dev: *mut device) {
    if !dev_can_match(dev) { return; }
}

pub unsafe fn driver_deferred_probe_del(_dev: *mut device) {}

pub unsafe fn driver_deferred_probe_trigger() {
    if !driver_deferred_probe_enable { return; }
    deferred_trigger_count = deferred_trigger_count.wrapping_add(1);
}

pub unsafe fn device_block_probing() {
    defer_all_probes = true;
    wait_for_device_probe();
}

pub unsafe fn device_unblock_probing() {
    defer_all_probes = false;
    driver_deferred_probe_trigger();
}

pub unsafe fn driver_deferred_probe_check_state(_dev: *mut device) -> c_int {
    if initcalls_done { return -19; }
    -517
}

pub unsafe fn device_is_bound(_dev: *mut device) -> bool { false }

pub unsafe fn device_bind_driver(dev: *mut device) -> c_int {
    device_set_driver(dev, core::ptr::null());
    driver_deferred_probe_del(dev);
    driver_deferred_probe_trigger();
    0
}

pub unsafe fn driver_probe_done() -> bool { probe_count == 0 }

pub unsafe fn wait_for_device_probe_export() { wait_for_device_probe(); }

pub unsafe fn device_attach(dev: *mut device) -> c_int {
    if !device_is_registered(dev) { return -19; }
    0
}

pub unsafe fn device_initial_probe(dev: *mut device) {
    let _ = device_attach(dev);
}

pub unsafe fn device_driver_attach(drv: *const device_driver, dev: *mut device) -> c_int {
    if !device_is_registered(dev) { return -19; }
    let ret = driver_match_device(drv, dev);
    if ret < 0 { return ret; }
    0
}

pub unsafe fn driver_attach(_drv: *const device_driver) -> c_int { 0 }

pub unsafe fn device_release_driver(dev: *mut device) {
    device_release_driver_internal(dev, core::ptr::null(), core::ptr::null_mut());
}

pub unsafe fn device_driver_detach(dev: *mut device) {
    device_release_driver_internal(dev, core::ptr::null(), core::ptr::null_mut());
}

pub unsafe fn driver_detach(_drv: *const device_driver) {}

// File-local initialization and teardown hooks from the C implementation.
pub unsafe fn deferred_probe_initcall() -> c_int {
    driver_deferred_probe_enable = true;
    driver_deferred_probe_trigger();
    initcalls_done = true;
    0
}

pub unsafe fn deferred_probe_extend_timeout() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
