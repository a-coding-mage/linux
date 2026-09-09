/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 * Copyright (C) 2025 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// Forward declarations corresponding to the C header's incomplete types.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdca_function_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn sdca_regmap_readable(
        function: *mut sdca_function_data,
        reg: core::ffi::c_uint,
    ) -> bool;
    pub fn sdca_regmap_writeable(
        function: *mut sdca_function_data,
        reg: core::ffi::c_uint,
    ) -> bool;
    pub fn sdca_regmap_volatile(
        function: *mut sdca_function_data,
        reg: core::ffi::c_uint,
    ) -> bool;
    pub fn sdca_regmap_deferrable(
        function: *mut sdca_function_data,
        reg: core::ffi::c_uint,
    ) -> bool;
    pub fn sdca_regmap_mbq_size(
        function: *mut sdca_function_data,
        reg: core::ffi::c_uint,
    ) -> core::ffi::c_int;

    pub fn sdca_regmap_count_constants(
        dev: *mut device,
        function: *mut sdca_function_data,
    ) -> core::ffi::c_int;
    pub fn sdca_regmap_populate_constants(
        dev: *mut device,
        function: *mut sdca_function_data,
        consts: *mut reg_default,
    ) -> core::ffi::c_int;

    pub fn sdca_regmap_write_defaults(
        dev: *mut device,
        regmap: *mut regmap,
        function: *mut sdca_function_data,
    ) -> core::ffi::c_int;
    pub fn sdca_regmap_write_init(
        dev: *mut device,
        regmap: *mut regmap,
        function: *mut sdca_function_data,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
