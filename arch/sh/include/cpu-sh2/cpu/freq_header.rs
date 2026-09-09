/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh2/freq.h
 *
 * Copyright (C) 2006  Yoshinori Sato
 */

// Equivalent of the C preprocessor condition:
// #if defined(CONFIG_CPU_SUBTYPE_SH7619)
#[cfg(CONFIG_CPU_SUBTYPE_SH7619)]
pub const FREQCR: usize = 0xf815_ff80;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
