/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2020 Samsung Electronics Co., Ltd.
 * Copyright 2020 Google LLC.
 * Copyright 2024 Linaro Ltd.
 */

// Dependency intent: `u8` corresponds to Linux `u8`, and `u32` corresponds
// to the C `unsigned int` used by this interface.

#[repr(C)]
pub struct acpm_handle {
    _private: [u8; 0],
}

extern "C" {
    pub fn acpm_pmic_read_reg(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        buf: *mut u8,
    ) -> i32;

    pub fn acpm_pmic_bulk_read(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        count: u8,
        buf: *mut u8,
    ) -> i32;

    pub fn acpm_pmic_write_reg(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        value: u8,
    ) -> i32;

    pub fn acpm_pmic_bulk_write(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        count: u8,
        buf: *const u8,
    ) -> i32;

    pub fn acpm_pmic_update_reg(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        value: u8,
        mask: u8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
