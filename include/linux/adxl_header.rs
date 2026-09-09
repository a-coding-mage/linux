/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Address translation interface via ACPI DSM.
 * Copyright (C) 2018 Intel Corporation
 */

use core::ffi::c_char;

unsafe extern "C" {
    pub fn adxl_get_component_names() -> *const *const c_char;
    pub fn adxl_decode(addr: u64, component_values: *mut u64) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
