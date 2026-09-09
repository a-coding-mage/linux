/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 2003 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

/*
 * The following break codes are or were in use for specific purposes in
 * other MIPS operating systems.  Linux/MIPS doesn't use all of them.  The
 * unused ones are here as placeholders; we might encounter them in
 * non-Linux/MIPS object files or make use of them in the future.
 */
pub const BRK_USERBP: i32 = 0; /* User bp (used by debuggers) */
pub const BRK_SSTEPBP: i32 = 5; /* User bp (used by debuggers) */
pub const BRK_OVERFLOW: i32 = 6; /* Overflow check */
pub const BRK_DIVZERO: i32 = 7; /* Divide by zero check */
pub const BRK_RANGE: i32 = 8; /* Range error check */
pub const BRK_BUG: i32 = 12; /* Used by BUG() */
pub const BRK_UPROBE: i32 = 13; /* See <asm/uprobes.h> */
pub const BRK_UPROBE_XOL: i32 = 14; /* See <asm/uprobes.h> */
pub const BRK_MEMU: i32 = 514; /* Used by FPU emulator */
pub const BRK_KPROBE_BP: i32 = 515; /* Kprobe break */
pub const BRK_KPROBE_SSTEPBP: i32 = 516; /* Kprobe single step software implementation */
pub const BRK_MULOVF: i32 = 1023; /* Multiply overflow */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
