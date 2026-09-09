/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 * Copyright (C) 2025 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

use core::ffi::{c_int, c_uint, c_void};

// Forward declarations from the surrounding kernel code.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdca_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdca_entity {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdca_function_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn sdca_ump_get_owner_host(
        dev: *mut device,
        function_regmap: *mut regmap,
        function: *mut sdca_function_data,
        entity: *mut sdca_entity,
        control: *mut sdca_control,
    ) -> c_int;

    pub fn sdca_ump_set_owner_device(
        dev: *mut device,
        function_regmap: *mut regmap,
        function: *mut sdca_function_data,
        entity: *mut sdca_entity,
        control: *mut sdca_control,
    ) -> c_int;

    pub fn sdca_ump_read_message(
        dev: *mut device,
        device_regmap: *mut regmap,
        function_regmap: *mut regmap,
        function: *mut sdca_function_data,
        entity: *mut sdca_entity,
        offset_sel: c_uint,
        length_sel: c_uint,
        msg: *mut *mut c_void,
    ) -> c_int;

    pub fn sdca_ump_write_message(
        dev: *mut device,
        device_regmap: *mut regmap,
        function_regmap: *mut regmap,
        function: *mut sdca_function_data,
        entity: *mut sdca_entity,
        offset_sel: c_uint,
        msg_offset: c_uint,
        length_sel: c_uint,
        msg: *mut c_void,
        msg_len: c_int,
    ) -> c_int;

    pub fn sdca_ump_cancel_timeout(work: *mut delayed_work);

    pub fn sdca_ump_schedule_timeout(work: *mut delayed_work, timeout_us: c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
