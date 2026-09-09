// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap4-restart.c - Common to OMAP4 and OMAP5
 */

// Translated dependencies:
// <linux/types.h>, <linux/reboot.h>, "common.h", and "prm.h"

use core::ffi::{c_char, c_int};

// enum reboot_mode is supplied by the translated reboot interface.
pub type reboot_mode = c_int;

unsafe extern "C" {
    fn omap_prm_reset_system();
}

/**
 * omap44xx_restart - trigger a software restart of the SoC
 * @mode: the "reboot mode", see arch/arm/kernel/{setup,process}.c
 * @cmd: passed from the userspace program rebooting the system (if provided)
 *
 * Resets the SoC.  For @cmd, see the 'reboot' syscall in
 * kernel/sys.c.  No return value.
 */
pub unsafe fn omap44xx_restart(mode: reboot_mode, cmd: *const c_char) {
    // XXX Should save 'cmd' into scratchpad for use after reboot
    let _ = mode;
    let _ = cmd;
    unsafe {
        omap_prm_reset_system();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
