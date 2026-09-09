// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2015 Nikolay Martynov <mar.kolya@gmail.com>
 * Copyright (C) 2015 John Crispin <john@phrozen.org>
 */

// Translated from the Linux kernel implementation. The original includes
// provide the external declarations and initialization annotations.

use core::ffi::c_void;

unsafe extern "C" {
    fn ralink_of_remap();
    fn of_clk_init(matches: *const c_void);
    fn timer_probe();
}

/// Equivalent of the kernel's `__init void plat_time_init(void)`.
pub unsafe extern "C" fn plat_time_init() {
    ralink_of_remap();

    of_clk_init(core::ptr::null());
    timer_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
