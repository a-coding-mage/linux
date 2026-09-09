/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Translated from adf_timer.h.
// The following types are supplied by the corresponding kernel dependencies.

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

pub type ktime_t = i64;

#[repr(C)]
pub struct adf_timer {
    pub accel_dev: *mut adf_accel_dev,
    pub work_ctx: delayed_work,
    pub initial_ktime: ktime_t,
}

unsafe extern "C" {
    pub fn adf_timer_start(accel_dev: *mut adf_accel_dev) -> ::core::ffi::c_int;
    pub fn adf_timer_stop(accel_dev: *mut adf_accel_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
