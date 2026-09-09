/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * acpi_lpat.h - LPAT table processing functions
 *
 * Copyright (C) 2015 Intel Corporation. All rights reserved.
 */

#[repr(C)]
pub struct acpi_lpat {
    pub temp: core::ffi::c_int,
    pub raw: core::ffi::c_int,
}

#[repr(C)]
pub struct acpi_lpat_conversion_table {
    pub lpat: *mut acpi_lpat,
    pub lpat_count: core::ffi::c_int,
}

/* CONFIG_ACPI selects the externally provided ACPI implementations. */
#[cfg(CONFIG_ACPI)]
unsafe extern "C" {
    pub fn acpi_lpat_raw_to_temp(
        lpat_table: *mut acpi_lpat_conversion_table,
        raw: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn acpi_lpat_temp_to_raw(
        lpat_table: *mut acpi_lpat_conversion_table,
        temp: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn acpi_lpat_get_conversion_table(
        handle: acpi_handle,
    ) -> *mut acpi_lpat_conversion_table;
    pub fn acpi_lpat_free_conversion_table(
        lpat_table: *mut acpi_lpat_conversion_table,
    );
}

/* Declaration supplied by the ACPI dependency. */
#[cfg(CONFIG_ACPI)]
pub type acpi_handle = *mut core::ffi::c_void;

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_lpat_raw_to_temp(
    _lpat_table: *mut acpi_lpat_conversion_table,
    _raw: core::ffi::c_int,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_lpat_temp_to_raw(
    _lpat_table: *mut acpi_lpat_conversion_table,
    _temp: core::ffi::c_int,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_lpat_get_conversion_table(
    _handle: acpi_handle,
) -> *mut acpi_lpat_conversion_table {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_lpat_free_conversion_table(
    _lpat_table: *mut acpi_lpat_conversion_table,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
