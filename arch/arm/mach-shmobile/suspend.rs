// SPDX-License-Identifier: GPL-2.0
/*
 * Suspend-to-RAM support code for SH-Mobile ARM
 *
 *  Copyright (C) 2011 Magnus Damm
 */

// Dependencies supplied by the kernel and other translation units:
// linux/pm.h, linux/suspend.h, linux/module.h, linux/err.h, linux/cpu.h,
// asm/io.h, asm/system_misc.h, and common.h.

#[allow(non_camel_case_types)]
pub type suspend_state_t = i32;

#[repr(C)]
pub struct platform_suspend_ops {
    pub begin: Option<unsafe extern "C" fn(suspend_state_t) -> i32>,
    pub end: Option<unsafe extern "C" fn()>,
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> i32>,
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
}

extern "C" {
    fn cpu_do_idle();
    fn cpu_idle_poll_ctrl(enable: bool);
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
}

unsafe extern "C" fn shmobile_suspend_default_enter(
    _suspend_state: suspend_state_t,
) -> i32 {
    cpu_do_idle();
    0
}

unsafe extern "C" fn shmobile_suspend_begin(_state: suspend_state_t) -> i32 {
    cpu_idle_poll_ctrl(true);
    0
}

unsafe extern "C" fn shmobile_suspend_end() {
    cpu_idle_poll_ctrl(false);
}

pub static mut shmobile_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    begin: Some(shmobile_suspend_begin),
    end: Some(shmobile_suspend_end),
    enter: Some(shmobile_suspend_default_enter),
    valid: Some(suspend_valid_only_mem),
};

pub unsafe extern "C" fn shmobile_suspend_init() -> i32 {
    suspend_set_ops(&shmobile_suspend_ops);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
