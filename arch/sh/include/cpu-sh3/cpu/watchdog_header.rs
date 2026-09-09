/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh3/watchdog.h
 *
 * Copyright (C) 2002, 2003 Paul Mundt
 */

// Header guard: __ASM_CPU_SH3_WATCHDOG_H

/* Register definitions */
pub const WTCNT: usize = 0xffffff84;
pub const WTCSR: usize = 0xffffff86;

/* Bit definitions */
pub const WTCSR_TME: u8 = 0x80;
pub const WTCSR_WT: u8 = 0x40;
pub const WTCSR_RSTS: u8 = 0x20;
pub const WTCSR_WOVF: u8 = 0x10;
pub const WTCSR_IOVF: u8 = 0x08;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
