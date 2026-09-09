// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Setup code for AT91SAM9
 *
 *  Copyright (C) 2011 Atmel,
 *                2011 Nicolas Ferre <nicolas.ferre@atmel.com>
 */

// Dependencies supplied by the surrounding architecture code:
// asm/mach/arch.h, asm/system_misc.h, and "generic.h".

use core::ffi::c_char;

unsafe extern "C" {
    fn at91sam9_pm_init();
}

static at91_dt_board_compat: [*const c_char; 2] = [
    b"atmel,at91sam9\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Equivalent machine descriptor emitted by:
// DT_MACHINE_START(at91sam_dt, "Atmel AT91SAM9")
//     /* Maintainer: Atmel */
//     .init_late = at91sam9_pm_init,
//     .dt_compat = at91_dt_board_compat,
// MACHINE_END
//
// The DT_MACHINE_START/MACHINE_END definitions are provided externally by
// asm/mach/arch.h and therefore cannot be materialized here without defining
// that dependency's architecture-specific descriptor type.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
