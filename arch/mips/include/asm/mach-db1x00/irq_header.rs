/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 by Ralf Baechle
 */

// The C header undefines any prior NR_IRQS definition before defining it here.
// MIPS_CPU_IRQ_BASE defaults to zero when it is not supplied by the build.
pub const MIPS_CPU_IRQ_BASE: u32 = 0;

/* 8 (MIPS) + 128 (au1300) + 16 (cpld) */
pub const NR_IRQS: u32 = 152;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
