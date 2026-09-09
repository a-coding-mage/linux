/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2013, The Linux Foundation. All rights reserved.
 */

// Translated from the C header. The Linux reset-controller definitions are
// supplied by the corresponding external Rust dependency.

#[repr(C)]
pub struct QcomResetMap {
    pub reg: ::core::ffi::c_uint,
    pub bit: u8,
    pub udelay: u16,
    pub bitmask: u32,
}

pub struct Regmap;

#[repr(C)]
pub struct QcomResetController {
    pub reset_map: *const QcomResetMap,
    pub regmap: *mut Regmap,
    pub rcdev: ResetControllerDev,
}

// External type supplied by <linux/reset-controller.h>.
#[repr(C)]
pub struct ResetControllerDev {
    _private: [u8; 0],
}

// External symbol supplied by the reset-controller implementation.
extern "C" {
    pub static qcom_reset_ops: ResetControlOps;
}

// External type supplied by <linux/reset-controller.h>.
#[repr(C)]
pub struct ResetControlOps {
    _private: [u8; 0],
}

// Equivalent of:
// container_of(r, struct qcom_reset_controller, rcdev)
#[macro_export]
macro_rules! to_qcom_reset_controller {
    ($r:expr) => {
        container_of!($r, QcomResetController, rcdev);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
