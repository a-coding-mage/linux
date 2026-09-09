/*
 * Prototypes, etc. for the Freescale MPC8xx embedded cpu chips
 * May need to be cleaned as the port goes on ...
 *
 * Copyright (C) 2008 Jochen Friedrich <jochen@scram.de>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

extern "C" {
    pub fn mpc8xx_restart(cmd: *mut ::core::ffi::c_char) -> !;
    pub fn mpc8xx_calibrate_decr();
    pub fn mpc8xx_set_rtc_time(tm: *mut rtc_time) -> ::core::ffi::c_int;
    pub fn mpc8xx_get_rtc_time(tm: *mut rtc_time);
    pub fn mpc8xx_get_irq() -> ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
