/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * hed.h - ACPI Hardware Error Device
 *
 * Copyright (C) 2009, Intel Corp.
 *	Author: Huang Ying <ying.huang@intel.com>
 */

// Dependency corresponding to <linux/notifier.h>.

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

extern "C" {
    pub fn register_acpi_hed_notifier(nb: *mut notifier_block) -> ::std::os::raw::c_int;
    pub fn unregister_acpi_hed_notifier(nb: *mut notifier_block);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
