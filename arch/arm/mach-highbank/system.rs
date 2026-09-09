// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011 Calxeda, Inc.
 */

// Dependencies supplied by the corresponding kernel headers and source files:
// linux/io.h, asm/proc-fns.h, linux/reboot.h, core.h, and sysregs.h.

use core::ffi::c_char;

// C enum reboot_mode; its concrete definition is supplied by the kernel
// headers.  REBOOT_HARD is the third enumerator in Linux's reboot_mode enum.
pub type RebootMode = i32;
pub const REBOOT_HARD: RebootMode = 2;
extern "C" {
    fn highbank_set_pwr_hard_reset();
    fn highbank_set_pwr_soft_reset();
    fn cpu_do_idle();
}

pub unsafe fn highbank_restart(mode: RebootMode, _cmd: *const c_char) {
    if mode == REBOOT_HARD {
        highbank_set_pwr_hard_reset();
    } else {
        highbank_set_pwr_soft_reset();
    }

    loop {
        cpu_do_idle();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
