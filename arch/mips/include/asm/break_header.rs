/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 2003 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

// The original header rejects direct inclusion of <uapi/asm/break.h> when
// __UAPI_ASM_BREAK_H is defined. The uapi header is an external dependency.

/*
 * Break codes used internally to the kernel.
 */
pub const BRK_KDB: i32 = 513; // Used in KDB_ENTER()
pub const BRK_MEMU: i32 = 514; // Used by FPU emulator
pub const BRK_KPROBE_BP: i32 = 515; // Kprobe break
pub const BRK_KPROBE_SSTEPBP: i32 = 516; // Kprobe single step software implementation
pub const BRK_MULOVF: i32 = 1023; // Multiply overflow

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
