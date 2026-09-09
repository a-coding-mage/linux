/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/mach/irq.h
 *
 *  Copyright (C) 1995-2000 Russell King.
 */

// Dependency intent: declarations from <linux/irq.h> are supplied externally.

use core::ffi::c_int;

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

/*
 * This is internal.  Do not use it.
 */
unsafe extern "C" {
    pub fn init_FIQ(arg: c_int);
    pub fn show_fiq_list(file: *mut seq_file, arg: c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
