/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * pm_clock.h - Definitions and headers related to device clocks.
 *
 * Copyright (C) 2011 Rafael J. Wysocki <rjw@sisk.pl>, Renesas Electronics Corp.
 */

use core::ffi::{c_char, c_int, c_void};

// Types supplied by linux/device.h and linux/notifier.h.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dev_pm_domain {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pm_clk_notifier_block {
    pub nb: notifier_block,
    pub pm_domain: *mut dev_pm_domain,
    pub con_ids: *mut *mut c_char,
}

#[cfg(CONFIG_PM)]
unsafe extern "C" {
    pub fn pm_clk_runtime_suspend(dev: *mut device) -> c_int;
    pub fn pm_clk_runtime_resume(dev: *mut device) -> c_int;
}

// CONFIG_PM: USE_PM_CLK_RUNTIME_OPS expands to runtime suspend/resume
// operation initializers; it has no standalone Rust item equivalent.

#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_no_clocks(dev: *mut device) -> bool {
    // Requires the field layout and list_empty() supplied by linux/device.h.
    // SAFETY: caller provides a valid device pointer, as in the C inline helper.
    if dev.is_null() {
        return false;
    }
    todo!("access device.power.subsys_data.clock_list")
}

#[cfg(CONFIG_PM_CLK)]
unsafe extern "C" {
    pub fn pm_clk_init(dev: *mut device);
    pub fn pm_clk_create(dev: *mut device) -> c_int;
    pub fn pm_clk_destroy(dev: *mut device);
    pub fn pm_clk_add(dev: *mut device, con_id: *const c_char) -> c_int;
    pub fn pm_clk_add_clk(dev: *mut device, clk: *mut clk) -> c_int;
    pub fn of_pm_clk_add_clks(dev: *mut device) -> c_int;
    pub fn pm_clk_remove_clk(dev: *mut device, clk: *mut clk);
    pub fn pm_clk_suspend(dev: *mut device) -> c_int;
    pub fn pm_clk_resume(dev: *mut device) -> c_int;
    pub fn devm_pm_clk_create(dev: *mut device) -> c_int;
}

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn pm_clk_no_clocks(_dev: *mut device) -> bool {
    true
}

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn pm_clk_init(_dev: *mut device) {}

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn pm_clk_create(_dev: *mut device) -> c_int {
    -EINVAL
}

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn pm_clk_destroy(_dev: *mut device) {}

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn pm_clk_add(_dev: *mut device, _con_id: *const c_char) -> c_int {
    -EINVAL
}

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn pm_clk_add_clk(_dev: *mut device, _clk: *mut clk) -> c_int {
    -EINVAL
}

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn of_pm_clk_add_clks(_dev: *mut device) -> c_int {
    -EINVAL
}

// #define pm_clk_suspend NULL
// #define pm_clk_resume NULL

#[cfg(not(CONFIG_PM_CLK))]
pub static mut pm_clk_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int> = None;
#[cfg(not(CONFIG_PM_CLK))]
pub static mut pm_clk_resume: Option<unsafe extern "C" fn(*mut device) -> c_int> = None;

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn pm_clk_remove_clk(_dev: *mut device, _clk: *mut clk) {}

#[cfg(not(CONFIG_PM_CLK))]
pub unsafe fn devm_pm_clk_create(_dev: *mut device) -> c_int {
    -EINVAL
}

#[cfg(CONFIG_HAVE_CLK)]
unsafe extern "C" {
    pub fn pm_clk_add_notifier(bus: *const bus_type, clknb: *mut pm_clk_notifier_block);
}

#[cfg(not(CONFIG_HAVE_CLK))]
pub unsafe fn pm_clk_add_notifier(_bus: *const bus_type, _clknb: *mut pm_clk_notifier_block) {}

// EINVAL is supplied by the kernel's errno definitions.
unsafe extern "C" {
    static EINVAL: c_int;
}

// Unused here; retained as a dependency marker for C declarations using void.
type _CVoid = c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
