/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/arm/mach-lpc32xx/common.h
 *
 * Author: Kevin Wells <kevin.wells@nxp.com>
 *
 * Copyright (C) 2009-2010 NXP Semiconductors
 */

/* Linux initialization annotation from the original declaration. */

/*
 * Other arch specific structures and functions
 */
unsafe extern "C" {
    pub fn lpc32xx_map_io();
    pub fn lpc32xx_check_uid();
    pub fn lpc32xx_pm_init();
    pub fn lpc32xx_serial_init();

    /*
     * Returns the LPC32xx unique 128-bit chip ID
     */
    pub fn lpc32xx_get_uid(devid: *mut u32);

    /*
     * Pointers used for sizing and copying suspend function data
     */
    pub fn lpc32xx_sys_suspend() -> i32;
    pub static mut lpc32xx_sys_suspend_sz: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
