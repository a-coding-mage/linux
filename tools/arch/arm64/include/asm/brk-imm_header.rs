/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

/*
 * #imm16 values used for BRK instruction generation
 * 0x004: for installing kprobes
 * 0x005: for installing uprobes
 * 0x006: for kprobe software single-step
 * 0x007: for kretprobe return
 * Allowed values for kgdb are 0x400 - 0x7ff
 * 0x100: for triggering a fault on purpose (reserved)
 * 0x400: for dynamic BRK instruction
 * 0x401: for compile time BRK instruction
 * 0x800: kernel-mode BUG() and WARN() traps
 * 0x9xx: tag-based KASAN trap (allowed values 0x900 - 0x9ff)
 * 0x55xx: Undefined Behavior Sanitizer traps ('U' << 8)
 * 0x8xxx: Control-Flow Integrity traps
 */
pub const KPROBES_BRK_IMM: u32 = 0x004;
pub const UPROBES_BRK_IMM: u32 = 0x005;
pub const KPROBES_BRK_SS_IMM: u32 = 0x006;
pub const KRETPROBES_BRK_IMM: u32 = 0x007;
pub const FAULT_BRK_IMM: u32 = 0x100;
pub const KGDB_DYN_DBG_BRK_IMM: u32 = 0x400;
pub const KGDB_COMPILED_DBG_BRK_IMM: u32 = 0x401;
pub const BUG_BRK_IMM: u32 = 0x800;
pub const KASAN_BRK_IMM: u32 = 0x900;
pub const KASAN_BRK_MASK: u32 = 0x0ff;
pub const UBSAN_BRK_IMM: u32 = 0x5500;
pub const UBSAN_BRK_MASK: u32 = 0x00ff;

pub const CFI_BRK_IMM_TARGET: u32 = 0x001f;
pub const CFI_BRK_IMM_TYPE: u32 = 0x03e0;
pub const CFI_BRK_IMM_BASE: u32 = 0x8000;
pub const CFI_BRK_IMM_MASK: u32 = CFI_BRK_IMM_TARGET | CFI_BRK_IMM_TYPE;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
