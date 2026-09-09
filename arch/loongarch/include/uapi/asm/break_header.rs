/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

pub const BRK_DEFAULT: i32 = 0; // Used as default
pub const BRK_BUG: i32 = 1; // Used by BUG()
pub const BRK_KDB: i32 = 2; // Used in KDB_ENTER()
pub const BRK_MATHEMU: i32 = 3; // Used by FPU emulator
pub const BRK_USERBP: i32 = 4; // User bp (used by debuggers)
pub const BRK_SSTEPBP: i32 = 5; // User bp (used by debuggers)
pub const BRK_OVERFLOW: i32 = 6; // Overflow check
pub const BRK_DIVZERO: i32 = 7; // Divide by zero check
pub const BRK_RANGE: i32 = 8; // Range error check
pub const BRK_MULOVFL: i32 = 9; // Multiply overflow
pub const BRK_KPROBE_BP: i32 = 10; // Kprobe break
pub const BRK_KPROBE_SSTEPBP: i32 = 11; // Kprobe single step break
pub const BRK_UPROBE_BP: i32 = 12; // See <asm/uprobes.h>
pub const BRK_UPROBE_XOLBP: i32 = 13; // See <asm/uprobes.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
