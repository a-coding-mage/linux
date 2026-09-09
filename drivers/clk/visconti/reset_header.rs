/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Toshiba ARM SoC reset controller driver
 *
 * Copyright (c) 2021 TOSHIBA CORPORATION
 *
 * Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
 */

// Dependency supplied externally by the Linux reset-controller interface.

#[repr(C)]
pub struct visconti_reset_data {
    pub rson_offset: u32,
    pub rsoff_offset: u32,
    pub rs_idx: u8,
}

#[repr(C)]
pub struct visconti_reset {
    pub rcdev: reset_controller_dev,
    pub regmap: *mut regmap,
    pub resets: *const visconti_reset_data,
    pub lock: *mut spinlock_t,
}

extern "C" {
    pub static visconti_reset_ops: reset_control_ops;

    pub fn visconti_register_reset_controller(
        dev: *mut device,
        regmap: *mut regmap,
        resets: *const visconti_reset_data,
        num_resets: c_uint,
        reset_ops: *const reset_control_ops,
        lock: *mut spinlock_t,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
