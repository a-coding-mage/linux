/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency: <asm/brk-imm.h>

/* A64 instructions are always 32 bits. */
pub const AARCH64_INSN_SIZE: u32 = 4;

/*
 * BRK instruction encoding
 * The #imm16 value should be placed at bits[20:5] within BRK ins
 */
pub const AARCH64_BREAK_MON: u32 = 0xd4200000;

/*
 * BRK instruction for provoking a fault on purpose
 * Unlike kgdb, #imm16 value with unallocated handler is used for faulting.
 */
pub const AARCH64_BREAK_FAULT: u32 = AARCH64_BREAK_MON | (FAULT_BRK_IMM << 5);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
