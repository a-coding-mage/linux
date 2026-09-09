// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

use core::ffi::c_char;

// Supplied by the kernel logging dependency.
unsafe extern "C" {
    fn pr_info(fmt: *const c_char, ...);
}

pub unsafe extern "C" fn machine_halt() {
    /* Halt the processor */
    core::arch::asm!("flag  1");
}

pub unsafe extern "C" fn machine_restart(_unused: *mut c_char) {
    /* Soft reset : jump to reset vector */
    pr_info(c"Put your restart handler here\n".as_ptr());
    machine_halt();
}

pub unsafe extern "C" fn machine_power_off() {
    /* FIXME ::  power off ??? */
    machine_halt();
}

pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;
// EXPORT_SYMBOL(pm_power_off);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
