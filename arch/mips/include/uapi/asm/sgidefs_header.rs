/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1999, 2001 Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 * Copyright (C) 2001 MIPS Technologies, Inc.
 */

/*
 * Definitions for the ISA levels
 *
 * With the introduction of MIPS32 / MIPS64 instruction sets definitions
 * MIPS ISAs are no longer subsets of each other.  Therefore comparisons
 * on these symbols except with == may result in unexpected results and
 * are forbidden!
 */
pub const _MIPS_ISA_MIPS1: i32 = 1;
pub const _MIPS_ISA_MIPS2: i32 = 2;
pub const _MIPS_ISA_MIPS3: i32 = 3;
pub const _MIPS_ISA_MIPS4: i32 = 4;
pub const _MIPS_ISA_MIPS5: i32 = 5;
pub const _MIPS_ISA_MIPS32: i32 = 6;
pub const _MIPS_ISA_MIPS64: i32 = 7;

/*
 * Subprogram calling convention
 */
pub const _MIPS_SIM_ABI32: i32 = 1;
pub const _MIPS_SIM_NABI32: i32 = 2;
pub const _MIPS_SIM_ABI64: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
