/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies corresponding to <linux/device.h> and <linux/power_supply.h>
use core::ffi::{c_char, c_int};

pub const ACPI_BATTERY_CLASS: &[u8] = b"battery\0";

pub const ACPI_BATTERY_NOTIFY_STATUS: c_int = 0x80;
pub const ACPI_BATTERY_NOTIFY_INFO: c_int = 0x81;
pub const ACPI_BATTERY_NOTIFY_THRESHOLD: c_int = 0x82;

#[repr(C)]
pub struct acpi_battery_hook {
    pub name: *const c_char,
    pub add_battery: Option<
        unsafe extern "C" fn(
            battery: *mut crate::power_supply,
            hook: *mut acpi_battery_hook,
        ) -> c_int,
    >,
    pub remove_battery: Option<
        unsafe extern "C" fn(
            battery: *mut crate::power_supply,
            hook: *mut acpi_battery_hook,
        ) -> c_int,
    >,
    pub list: crate::list_head,
}

extern "C" {
    pub fn battery_hook_register(hook: *mut acpi_battery_hook);
    pub fn battery_hook_unregister(hook: *mut acpi_battery_hook);
    pub fn devm_battery_hook_register(
        dev: *mut crate::device,
        hook: *mut acpi_battery_hook,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
