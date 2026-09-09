/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2007 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Prototypes for MPC512x shared code
 */

use core::ffi::c_char;

/* __init and __noreturn are kernel build attributes in the C source. */
unsafe extern "C" {
    pub fn mpc512x_init_IRQ();
    pub fn mpc512x_init_early();
    pub fn mpc512x_init();
    pub fn mpc512x_setup_arch();
    pub fn mpc5121_clk_init() -> i32;
    pub fn mpc512x_select_psc_compat() -> *const c_char;
    pub fn mpc512x_restart(cmd: *mut c_char) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
