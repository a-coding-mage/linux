/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * soc-acpi-intel-sdca-quirks.h - tables and support for SDCA quirks
 *
 * Copyright (c) 2024, Intel Corporation.
 *
 */

use core::ffi::c_void;

unsafe extern "C" {
    pub fn snd_soc_acpi_intel_sdca_is_device_rt712_vb(arg: *mut c_void) -> bool;
    pub fn snd_soc_acpi_intel_no_function_topology(arg: *mut c_void) -> bool;
    pub fn snd_soc_acpi_intel_rt712_vb_no_function_topology(arg: *mut c_void) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
