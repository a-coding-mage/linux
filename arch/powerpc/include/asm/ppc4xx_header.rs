/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * PPC4xx Prototypes and definitions
 *
 * Copyright 2008 DENX Software Engineering, Stefan Roese <sr@denx.de>
 */

// C declaration: extern void __noreturn ppc4xx_reset_system(char *cmd);
// `__noreturn` is represented by the never return type `!`.
extern "C" {
    pub fn ppc4xx_reset_system(cmd: *mut core::ffi::c_char) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
