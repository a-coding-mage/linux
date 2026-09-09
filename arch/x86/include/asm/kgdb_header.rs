/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2001-2004 Amit S. Kale
 * Copyright (C) 2008 Wind River Systems, Inc.
 *
 * Dependency: asm/ptrace.h supplies struct pt_regs.
 */

/*
 * BUFMAX defines the maximum number of characters in inbound/outbound
 * buffers at least NUMREGBYTES*2 are needed for register packets
 * Longer buffer is needed to list all threads
 */
pub const BUFMAX: i32 = 1024;

/*
 * Note that this register image is in a different order than the register
 * image that Linux produces at interrupt time.
 *
 * Linux's register image is defined by struct pt_regs in ptrace.h.
 * Just why GDB uses a different order is a historical mystery.
 */
#[cfg(CONFIG_X86_32)]
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum regnames {
    GDB_AX = 0,
    GDB_CX,
    GDB_DX,
    GDB_BX,
    GDB_SP,
    GDB_BP,
    GDB_SI,
    GDB_DI,
    GDB_PC,
    GDB_PS,
    GDB_CS,
    GDB_SS,
    GDB_DS,
    GDB_ES,
    GDB_FS,
    GDB_GS,
}

#[cfg(CONFIG_X86_32)]
pub const GDB_ORIG_AX: i32 = 41;
#[cfg(CONFIG_X86_32)]
pub const DBG_MAX_REG_NUM: i32 = 16;
#[cfg(CONFIG_X86_32)]
pub const NUMREGBYTES: i32 = ((GDB_GS as i32 + 1) * 4);

#[cfg(not(CONFIG_X86_32))]
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum regnames {
    GDB_AX = 0,
    GDB_BX,
    GDB_CX,
    GDB_DX,
    GDB_SI,
    GDB_DI,
    GDB_BP,
    GDB_SP,
    GDB_R8,
    GDB_R9,
    GDB_R10,
    GDB_R11,
    GDB_R12,
    GDB_R13,
    GDB_R14,
    GDB_R15,
    GDB_PC,
    GDB_PS,
    GDB_CS,
    GDB_SS,
    GDB_DS,
    GDB_ES,
    GDB_FS,
    GDB_GS,
}

#[cfg(not(CONFIG_X86_32))]
pub const GDB_ORIG_AX: i32 = 57;
#[cfg(not(CONFIG_X86_32))]
pub const DBG_MAX_REG_NUM: i32 = 24;
/* 17 64 bit regs and 5 32 bit regs */
#[cfg(not(CONFIG_X86_32))]
pub const NUMREGBYTES: i32 = (17 * 8) + (5 * 4);

#[inline]
pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!("int $3");
}

pub const BREAK_INSTR_SIZE: i32 = 1;
pub const CACHE_FLUSH_IS_SAFE: i32 = 1;
/* GDB_ADJUSTS_BREAK_OFFSET */

extern "C" {
    pub fn kgdb_ll_trap(
        cmd: i32,
        str_: *const core::ffi::c_char,
        regs: *mut pt_regs,
        err: core::ffi::c_long,
        trap: i32,
        sig: i32,
    ) -> i32;
}

/* Supplied by asm/ptrace.h. */
#[allow(non_camel_case_types)]
pub type pt_regs = crate::pt_regs;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
