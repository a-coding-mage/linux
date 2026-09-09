/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh3/freq.h
 *
 * Copyright (C) 2002, 2003 Paul Mundt
 */

// Original C header guard: __ASM_CPU_SH3_FREQ_H

// Original build-time condition: CONFIG_CPU_SUBTYPE_SH7712.
#[cfg(CONFIG_CPU_SUBTYPE_SH7712)]
pub const FRQCR: u32 = 0xA415FF80;

#[cfg(not(CONFIG_CPU_SUBTYPE_SH7712))]
pub const FRQCR: u32 = 0xFFFFFF80;

pub const MIN_DIVISOR_NR: u32 = 0;
pub const MAX_DIVISOR_NR: u32 = 4;

pub const FRQCR_CKOEN: u32 = 0x0100;
pub const FRQCR_PLLEN: u32 = 0x0080;
pub const FRQCR_PSTBY: u32 = 0x0040;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
