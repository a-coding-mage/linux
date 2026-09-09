/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Device physical location support
 *
 * Author: Won Chung <wonchung@google.com>
 */

// Dependency supplied by the surrounding device subsystem:
// use linux::device::{attribute_group, device};

#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn dev_add_physical_location(dev: *mut device) -> bool;
    pub static dev_attr_physical_location_group: attribute_group;
}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub fn dev_add_physical_location(_dev: *mut device) -> bool {
    false
}

#[cfg(not(CONFIG_ACPI))]
pub static dev_attr_physical_location_group: attribute_group = unsafe {
    core::mem::MaybeUninit::<attribute_group>::zeroed().assume_init()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
