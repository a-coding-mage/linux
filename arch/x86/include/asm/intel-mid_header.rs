/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel MID specific setup code
 *
 * (C) Copyright 2009, 2021 Intel Corporation
 */

// Dependency: <linux/pci.h>

extern "C" {
    pub fn intel_mid_pci_init() -> ::core::ffi::c_int;
    pub fn intel_mid_pci_set_power_state(
        pdev: *mut pci_dev,
        state: pci_power_t,
    ) -> ::core::ffi::c_int;
    pub fn intel_mid_pci_get_power_state(pdev: *mut pci_dev) -> pci_power_t;

    pub fn intel_mid_pwr_power_off();

    pub fn intel_mid_pwr_get_lss_id(pdev: *mut pci_dev) -> ::core::ffi::c_int;
}

pub const INTEL_MID_PWR_LSS_OFFSET: ::core::ffi::c_uint = 4;
pub const INTEL_MID_PWR_LSS_TYPE: ::core::ffi::c_uint = 1 << 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
