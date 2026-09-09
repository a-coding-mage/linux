// SPDX-License-Identifier: GPL-2.0
/*
 * Standalone xHCI debug capability driver
 *
 * Copyright (C) 2016 Intel Corporation
 *
 * Author: Lu Baolu <baolu.lu@linux.intel.com>
 */

// C header guard: __LINUX_XHCI_DBGP_H

// CONFIG_EARLY_PRINTK_USB_XDBC selects the externally provided early xDBC
// declarations. The fallback definitions are retained below for the
// configuration in which it is disabled.
#[cfg(CONFIG_EARLY_PRINTK_USB_XDBC)]
extern "C" {
    pub fn early_xdbc_parse_parameter(s: *mut core::ffi::c_char, keep_early: core::ffi::c_int) -> core::ffi::c_int;
    pub fn early_xdbc_setup_hardware() -> core::ffi::c_int;
    pub fn early_xdbc_register_console();
}

#[cfg(not(CONFIG_EARLY_PRINTK_USB_XDBC))]
#[inline]
pub unsafe fn early_xdbc_setup_hardware() -> core::ffi::c_int {
    // -ENODEV
    -19
}

#[cfg(not(CONFIG_EARLY_PRINTK_USB_XDBC))]
#[inline]
pub unsafe fn early_xdbc_register_console() {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
