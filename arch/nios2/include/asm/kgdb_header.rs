/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Altera Corporation
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 *
 * Based on the code posted by Kazuyasu on the Altera Forum at:
 * http://www.alteraforum.com/forum/showpost.php?p=77003&postcount=20
 */

pub const CACHE_FLUSH_IS_SAFE: i32 = 1;
pub const BUFMAX: usize = 2048;

#[repr(i32)]
pub enum regnames {
    GDB_R0 = 0,
    GDB_AT,
    GDB_R2,
    GDB_R3,
    GDB_R4,
    GDB_R5,
    GDB_R6,
    GDB_R7,
    GDB_R8,
    GDB_R9,
    GDB_R10,
    GDB_R11,
    GDB_R12,
    GDB_R13,
    GDB_R14,
    GDB_R15,
    GDB_R16,
    GDB_R17,
    GDB_R18,
    GDB_R19,
    GDB_R20,
    GDB_R21,
    GDB_R22,
    GDB_R23,
    GDB_ET,
    GDB_BT,
    GDB_GP,
    GDB_SP,
    GDB_FP,
    GDB_EA,
    GDB_BA,
    GDB_RA,
    GDB_PC,
    GDB_STATUS,
    GDB_ESTATUS,
    GDB_BSTATUS,
    GDB_IENABLE,
    GDB_IPENDING,
    GDB_CPUID,
    GDB_CTL6,
    GDB_EXCEPTION,
    GDB_PTEADDR,
    GDB_TLBACC,
    GDB_TLBMISC,
    GDB_ECCINJ,
    GDB_BADADDR,
    GDB_CONFIG,
    GDB_MPUBASE,
    GDB_MPUACC,
    /* do not change the last entry or anything below! */
    GDB_NUMREGBYTES,
}

pub const GDB_SIZEOF_REG: usize = core::mem::size_of::<u32>();
pub const DBG_MAX_REG_NUM: usize = 49;
/* C's sizeof(GDB_SIZEOF_REG) is sizeof the macro's resulting size_t value. */
pub const NUMREGBYTES: usize =
    DBG_MAX_REG_NUM * core::mem::size_of::<usize>();

pub const BREAK_INSTR_SIZE: usize = 4;

pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!("trap 30", options(nostack, nomem, preserves_flags));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
