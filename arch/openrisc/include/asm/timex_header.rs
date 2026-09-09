/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// C header guard: __ASM_OPENRISC_TIMEX_H

// #define get_cycles get_cycles

// Dependencies supplied by the corresponding architectural headers:
// #include <asm-generic/timex.h>
// #include <asm/spr.h>
// #include <asm/spr_defs.h>

#[inline]
pub unsafe fn get_cycles() -> cycles_t {
    mfspr(SPR_TTCR)
}

// #define get_cycles get_cycles

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
