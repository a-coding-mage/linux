/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <asm/cacheflush.h>
// #include <asm/ptrace.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum regnames {
    GDB_R0,
    GDB_R1,
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

    GDB_PC,
    GDB_PR,
    GDB_SR,
    GDB_GBR,
    GDB_MACH,
    GDB_MACL,
    GDB_VBR,
}

pub const _GP_REGS: usize = 16;
pub const _EXTRA_REGS: usize = 7;
pub const GDB_SIZEOF_REG: usize = core::mem::size_of::<u32>();

pub const DBG_MAX_REG_NUM: usize = _GP_REGS + _EXTRA_REGS;
pub const NUMREGBYTES: usize = DBG_MAX_REG_NUM * GDB_SIZEOF_REG;

#[inline]
pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!("trapa #0x3c");
}

pub const BREAK_INSTR_SIZE: usize = 2;
pub const BUFMAX: usize = 2048;

// CONFIG_SMP selects the cache-flush safety value at build time.
#[cfg(CONFIG_SMP)]
pub const CACHE_FLUSH_IS_SAFE: i32 = 0;
#[cfg(not(CONFIG_SMP))]
pub const CACHE_FLUSH_IS_SAFE: i32 = 1;

// C macro with an empty replacement list: GDB_ADJUSTS_BREAK_OFFSET

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
