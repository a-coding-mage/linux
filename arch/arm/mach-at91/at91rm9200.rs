// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Setup code for AT91RM9200
 *
 *  Copyright (C) 2011 Atmel,
 *                2011 Nicolas Ferre <nicolas.ferre@atmel.com>
 *                2012 Joachim Eastwood <manabian@gmail.com>
 */

// Dependency supplied by asm/mach/arch.h.
// Dependency supplied by generic.h.

unsafe extern "C" {
    fn at91rm9200_pm_init();
}

static at91rm9200_dt_board_compat_0: &[u8] = b"atmel,at91rm9200\0";

static at91rm9200_dt_board_compat: [*const u8; 2] = [
    at91rm9200_dt_board_compat_0.as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(at91rm9200_dt, "Atmel AT91RM9200")
//     .init_late = at91rm9200_pm_init,
//     .dt_compat = at91rm9200_dt_board_compat,
// MACHINE_END
// The machine registration macro and its generated type are supplied by
// asm/mach/arch.h and are preserved here as the corresponding registration
// metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
