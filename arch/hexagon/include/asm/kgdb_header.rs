/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/hexagon/include/asm/kgdb.h - Hexagon KGDB Support
 *
 * Copyright (c) 2011, The Linux Foundation. All rights reserved.
 */

pub const BREAK_INSTR_SIZE: usize = 4;
pub const CACHE_FLUSH_IS_SAFE: usize = 1;
pub const BUFMAX: usize = (NUMREGBYTES * 2) + 512;

pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!("trap0(#0xDB)");
}

/* Registers:
 * 32 gpr + sa0/1 + lc0/1 + m0/1 + gp + ugp + pred + pc = 42 total.
 * vm regs = psp+elr+est+badva = 4
 * syscall+restart = 2 more
 * also add cs0/1 = 2
 * so 48 = 42 + 4 + 2 + 2
 */
pub const DBG_USER_REGS: usize = 42;
pub const DBG_MAX_REG_NUM: usize = DBG_USER_REGS + 8;
pub const NUMREGBYTES: usize = DBG_MAX_REG_NUM * 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
