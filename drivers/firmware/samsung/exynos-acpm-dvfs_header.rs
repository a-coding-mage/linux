/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2020 Samsung Electronics Co., Ltd.
 * Copyright 2020 Google LLC.
 * Copyright 2025 Linaro Ltd.
 */

// Dependency intent: Linux type definitions are represented using Rust FFI
// integer types in this translation.

use core::ffi::{c_int, c_uint, c_ulong};

#[repr(C)]
pub struct acpm_handle {
    _private: [u8; 0],
}

extern "C" {
    pub fn acpm_dvfs_set_rate(
        handle: *mut acpm_handle,
        acpm_chan_id: c_uint,
        id: c_uint,
        rate: c_ulong,
    ) -> c_int;

    pub fn acpm_dvfs_get_rate(
        handle: *mut acpm_handle,
        acpm_chan_id: c_uint,
        clk_id: c_uint,
    ) -> c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
