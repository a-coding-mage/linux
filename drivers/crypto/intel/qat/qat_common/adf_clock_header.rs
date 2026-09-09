/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Dependency intent: `u32` and `u64` correspond to Linux `u32` and `u64`.

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn adf_dev_measure_clock(
        accel_dev: *mut adf_accel_dev,
        frequency: *mut u32,
        min: u32,
        max: u32,
    ) -> i32;

    pub fn adf_clock_get_current_time() -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
