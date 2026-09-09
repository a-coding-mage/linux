/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2006 Freescale Semiconductor Inc.
 */

/*
 * Declaration for the various functions exported by the
 * mpc86xx_* files. Mostly for use by mpc86xx_setup().
 */

extern "C" {
    pub fn mpc86xx_smp_init();
    pub fn mpc86xx_init_irq();
    pub fn mpc86xx_time_init() -> core::ffi::c_long;
    pub fn mpc86xx_common_publish_devices() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
