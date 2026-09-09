/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  syscore_ops.h - System core operations.
 *
 *  Copyright (C) 2011 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
 */

use core::ffi::c_void;

// Supplied by the translated linux/list.h dependency.
#[repr(C)]
pub struct list_head;

#[repr(C)]
pub struct syscore_ops {
    pub suspend: Option<unsafe extern "C" fn(data: *mut c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(data: *mut c_void)>,
    pub shutdown: Option<unsafe extern "C" fn(data: *mut c_void)>,
}

#[repr(C)]
pub struct syscore {
    pub node: list_head,
    pub ops: *const syscore_ops,
    pub data: *mut c_void,
}

unsafe extern "C" {
    pub fn register_syscore(syscore: *mut syscore);
    pub fn unregister_syscore(syscore: *mut syscore);

    // Preserved from the CONFIG_PM_SLEEP conditional declaration.
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub fn syscore_suspend() -> i32;

    // Preserved from the CONFIG_PM_SLEEP conditional declaration.
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub fn syscore_resume();

    pub fn syscore_shutdown();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
