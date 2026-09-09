// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/kernel/time.c
 *
 * Copyright (C) 1991, 1992, 1995  Linus Torvalds
 * Modifications for ARM (C) 1994-2001 Russell King
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency declarations supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn in_lock_functions(pc: c_ulong) -> bool;
    fn arch_stack_walk(
        callback: unsafe extern "C" fn(arg: *mut c_void, pc: c_ulong) -> bool,
        cookie: *mut c_void,
        task: *mut task_struct,
        regs: *mut pt_regs,
    );
    fn of_clk_init(matches: *const c_void);
    fn timer_probe();
    fn tick_setup_hrtimer_broadcast();
    fn arch_timer_get_rate() -> u32;
    fn panic(message: *const u8) -> !;
    fn pv_time_init();

    static mut current: *mut task_struct;
    static mut lpj_fine: c_ulong;
}

type c_ulong = usize;

// Supplied by the kernel build configuration.
const HZ: c_ulong = 0;

unsafe extern "C" fn profile_pc_cb(arg: *mut c_void, pc: c_ulong) -> bool {
    let prof_pc = arg as *mut c_ulong;

    if in_lock_functions(pc) {
        return true;
    }
    *prof_pc = pc;
    false
}

pub unsafe extern "C" fn profile_pc(regs: *mut pt_regs) -> c_ulong {
    let mut prof_pc: c_ulong = 0;

    arch_stack_walk(
        profile_pc_cb,
        &mut prof_pc as *mut c_ulong as *mut c_void,
        current,
        regs,
    );

    prof_pc
}

// EXPORT_SYMBOL(profile_pc);

// __init
pub unsafe extern "C" fn time_init() {
    let arch_timer_rate: u32;

    of_clk_init(core::ptr::null());
    timer_probe();

    tick_setup_hrtimer_broadcast();

    arch_timer_rate = arch_timer_get_rate();
    if arch_timer_rate == 0 {
        panic(b"Unable to initialise architected timer.\n\0".as_ptr());
    }

    /* Calibrate the delay loop directly */
    lpj_fine = (arch_timer_rate as c_ulong) / HZ;

    pv_time_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
