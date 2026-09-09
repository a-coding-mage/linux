// SPDX-License-Identifier: GPL-2.0-only
/*
 * Device Tree board file for NXP LPC18xx/43xx
 *
 * Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
 */

// Dependency supplied by the architecture-specific machine framework:
// #include <asm/mach/arch.h>

/// Device-tree compatibility strings for the LPC18xx/43xx boards.
#[allow(non_upper_case_globals)]
static lpc18xx_43xx_compat: [*const core::ffi::c_char; 4] = [
    b"nxp,lpc1850\0".as_ptr() as *const core::ffi::c_char,
    b"nxp,lpc4350\0".as_ptr() as *const core::ffi::c_char,
    b"nxp,lpc4370\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(LPC18XXDT, "NXP LPC18xx/43xx (Device Tree)")
//     .dt_compat = lpc18xx_43xx_compat,
// MACHINE_END
//
// The declarations above are emitted by the architecture machine-description
// framework in the original C source; the framework-provided Rust equivalent
// consumes `lpc18xx_43xx_compat` and registers the machine named LPC18XXDT.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
