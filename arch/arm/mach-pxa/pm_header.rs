/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2005 Richard Purdie
 */

/* Dependency supplied by the Linux suspend subsystem. */

#[repr(C)]
pub struct pxa_cpu_pm_fns {
    pub save_count: core::ffi::c_int,
    pub save: Option<unsafe extern "C" fn(*mut core::ffi::c_ulong)>,
    pub restore: Option<unsafe extern "C" fn(*mut core::ffi::c_ulong)>,
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> core::ffi::c_int>,
    pub enter: Option<unsafe extern "C" fn(suspend_state_t)>,
    pub prepare: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub finish: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    pub static mut pxa_cpu_pm_fns: *mut pxa_cpu_pm_fns;

    /* sleep.S */
    pub fn pxa25x_finish_suspend(arg: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn pxa27x_finish_suspend(arg: core::ffi::c_ulong) -> core::ffi::c_int;

    pub fn pxa_pm_enter(state: suspend_state_t) -> core::ffi::c_int;
    pub fn pxa_pm_prepare() -> core::ffi::c_int;
    pub fn pxa_pm_finish();

    pub static pm_enter_standby_start: [core::ffi::c_char; 0];
    pub static pm_enter_standby_end: [core::ffi::c_char; 0];
    pub fn pxa3xx_finish_suspend(arg: core::ffi::c_ulong) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
