/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// C dependencies: <linux/reset-controller.h>, <linux/spinlock.h>
use crate::{reset_controller_dev, reset_control_ops, spinlock_t};

#[repr(C)]
pub struct ccu_reset_map {
    pub reg: u16,
    pub bit: u32,
}

#[repr(C)]
pub struct ccu_reset {
    pub base: *mut core::ffi::c_void,
    pub reset_map: *const ccu_reset_map,
    pub lock: *mut spinlock_t,

    pub rcdev: reset_controller_dev,
}

#[inline]
pub unsafe fn rcdev_to_ccu_reset(rcdev: *mut reset_controller_dev) -> *mut ccu_reset {
    (rcdev as *mut u8)
        .sub(core::mem::offset_of!(ccu_reset, rcdev))
        as *mut ccu_reset
}

unsafe extern "C" {
    pub static ccu_reset_ops: reset_control_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
