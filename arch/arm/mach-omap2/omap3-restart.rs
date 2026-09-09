// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap3-restart.c - Code common to all OMAP3xxx machines.
 *
 * Copyright (C) 2009, 2012 Texas Instruments
 * Copyright (C) 2010 Nokia Corporation
 * Tony Lindgren <tony@atomide.com>
 * Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

// Dependencies supplied by the corresponding kernel headers and source files.
pub type u8 = core::ffi::c_uchar;
pub type reboot_mode = i32;

unsafe extern "C" {
    pub fn omap3_ctrl_write_boot_mode(boot_mode: u8);
    pub fn omap_prm_reset_system();
}

/* Global address base setup code */

/**
 * omap3xxx_restart - trigger a software restart of the SoC
 * @mode: the "reboot mode", see arch/arm/kernel/{setup,process}.c
 * @cmd: passed from the userspace program rebooting the system (if provided)
 *
 * Resets the SoC.  For @cmd, see the 'reboot' syscall in
 * kernel/sys.c.  No return value.
 */
pub unsafe fn omap3xxx_restart(mode: reboot_mode, cmd: *const core::ffi::c_char) {
    let _ = mode;
    omap3_ctrl_write_boot_mode(if !cmd.is_null() {
        *(cmd as *const u8)
    } else {
        0
    });
    omap_prm_reset_system();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
