// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2010 Freescale Semiconductor, Inc.
 */

// Translated dependencies from linux/kernel.h, linux/suspend.h, linux/io.h,
// and pm.h are expected to be supplied by the surrounding kernel bindings.

pub type suspend_state_t = u32;

pub const PM_SUSPEND_MEM: suspend_state_t = 3;
pub const EINVAL: i32 = 22;

#[repr(C)]
pub struct platform_suspend_ops {
    pub enter: Option<unsafe extern "C" fn(state: suspend_state_t) -> i32>,
    pub valid: Option<unsafe extern "C" fn(state: suspend_state_t) -> bool>,
}

unsafe extern "C" {
    fn cpu_do_idle();
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
}

unsafe fn mxs_suspend_enter(state: suspend_state_t) -> i32 {
    match state {
        PM_SUSPEND_MEM => {
            unsafe {
                cpu_do_idle();
            }
        }
        _ => return -EINVAL,
    }
    0
}

static mxs_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    enter: Some(mxs_suspend_enter),
    valid: Some(suspend_valid_only_mem),
};

// __init
pub unsafe fn mxs_pm_init() {
    unsafe {
        suspend_set_ops(&mxs_suspend_ops);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
