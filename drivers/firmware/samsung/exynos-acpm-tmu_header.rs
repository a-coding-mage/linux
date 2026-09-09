/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2020 Samsung Electronics Co., Ltd.
 * Copyright 2020 Google LLC.
 * Copyright 2026 Linaro Ltd.
 */

// Dependency equivalent of <linux/types.h>.

#[repr(C)]
pub struct acpm_handle {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn acpm_tmu_init(handle: *mut acpm_handle, acpm_chan_id: u32) -> i32;

    pub fn acpm_tmu_read_temp(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
        temp: *mut i32,
    ) -> i32;

    pub fn acpm_tmu_set_threshold(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
        temperature: *const u8,
        tlen: usize,
    ) -> i32;

    pub fn acpm_tmu_set_interrupt_enable(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
        inten: u8,
    ) -> i32;

    pub fn acpm_tmu_tz_control(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
        enable: bool,
    ) -> i32;

    pub fn acpm_tmu_clear_tz_irq(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
    ) -> i32;

    pub fn acpm_tmu_suspend(handle: *mut acpm_handle, acpm_chan_id: u32) -> i32;

    pub fn acpm_tmu_resume(handle: *mut acpm_handle, acpm_chan_id: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
