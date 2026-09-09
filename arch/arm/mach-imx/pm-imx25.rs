// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2016 NXP Semiconductors
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/suspend.h, linux/io.h, and "common.h".

use core::ffi::c_int;

// C: typedef for suspend_state_t is supplied by the suspend subsystem.
pub type SuspendStateT = u32;

// C: PM_SUSPEND_MEM and EINVAL are supplied by the suspend/kernel headers.
pub const PM_SUSPEND_MEM: SuspendStateT = 3;
pub const EINVAL: c_int = 22;

#[repr(C)]
pub struct PlatformSuspendOps {
    pub enter: Option<unsafe extern "C" fn(state: SuspendStateT) -> c_int>,
    pub valid: Option<unsafe extern "C" fn(state: SuspendStateT) -> bool>,
}

extern "C" {
    fn cpu_do_idle();
    fn suspend_valid_only_mem(state: SuspendStateT) -> bool;
    fn suspend_set_ops(ops: *const PlatformSuspendOps);
}

unsafe fn imx25_suspend_enter(state: SuspendStateT) -> c_int {
    // !IS_ENABLED(CONFIG_PM): preserve the build-time kernel condition.
    if !cfg!(feature = "CONFIG_PM") {
        return 0;
    }

    match state {
        PM_SUSPEND_MEM => {
            unsafe { cpu_do_idle(); }
        }
        _ => return -EINVAL,
    }

    0
}

static IMX25_SUSPEND_OPS: PlatformSuspendOps = PlatformSuspendOps {
    enter: Some(imx25_suspend_enter),
    valid: Some(suspend_valid_only_mem),
};

pub unsafe extern "C" fn imx25_pm_init() {
    unsafe { suspend_set_ops(&IMX25_SUSPEND_OPS); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
