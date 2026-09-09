/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 */

pub const GDB_SIZEOF_REG: usize = core::mem::size_of::<u64>();

/* gdb remote procotol expects the following register layout. */

/*
 * General purpose registers:
 *     r0-r31: 64 bit
 *     orig_a0: 64 bit
 *     pc : 64 bit
 *     csr_badvaddr: 64 bit
 */
pub const DBG_PT_REGS_BASE: usize = 0;
pub const DBG_PT_REGS_NUM: usize = 35;
pub const DBG_PT_REGS_END: usize = DBG_PT_REGS_BASE + DBG_PT_REGS_NUM - 1;

/*
 * Floating point registers:
 *     f0-f31: 64 bit
 */
pub const DBG_FPR_BASE: usize = DBG_PT_REGS_END + 1;
pub const DBG_FPR_NUM: usize = 32;
pub const DBG_FPR_END: usize = DBG_FPR_BASE + DBG_FPR_NUM - 1;

/*
 * Condition Flag registers:
 *     fcc0-fcc8: 8 bit
 */
pub const DBG_FCC_BASE: usize = DBG_FPR_END + 1;
pub const DBG_FCC_NUM: usize = 8;
pub const DBG_FCC_END: usize = DBG_FCC_BASE + DBG_FCC_NUM - 1;

/*
 * Floating-point Control and Status registers:
 *     fcsr: 32 bit
 */
pub const DBG_FCSR_NUM: usize = 1;
pub const DBG_FCSR: usize = DBG_FCC_END + 1;

pub const DBG_MAX_REG_NUM: usize = DBG_FCSR + 1;

/*
 * Size of I/O buffer for gdb packet.
 * considering to hold all register contents, size is set
 */
pub const BUFMAX: usize = 2048;

/*
 * Number of bytes required for gdb_regs buffer.
 * PT_REGS and FPR: 8 bytes; FCSR: 4 bytes; FCC: 1 bytes.
 * GDB fails to connect for size beyond this with error
 * "'g' packet reply is too long"
 */
pub const NUMREGBYTES: usize = (DBG_PT_REGS_NUM + DBG_FPR_NUM) * GDB_SIZEOF_REG
    + DBG_FCC_NUM * 1
    + DBG_FCSR_NUM * 4;

pub const BREAK_INSTR_SIZE: usize = 4;
pub const CACHE_FLUSH_IS_SAFE: usize = 0;

/* Register numbers of various important registers. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DbgLoongarchRegnum {
    DBG_LOONGARCH_ZERO = 0,
    DBG_LOONGARCH_RA,
    DBG_LOONGARCH_TP,
    DBG_LOONGARCH_SP,
    DBG_LOONGARCH_A0,
    DBG_LOONGARCH_FP = 22,
    DBG_LOONGARCH_S0,
    DBG_LOONGARCH_S1,
    DBG_LOONGARCH_S2,
    DBG_LOONGARCH_S3,
    DBG_LOONGARCH_S4,
    DBG_LOONGARCH_S5,
    DBG_LOONGARCH_S6,
    DBG_LOONGARCH_S7,
    DBG_LOONGARCH_S8,
    DBG_LOONGARCH_ORIG_A0,
    DBG_LOONGARCH_PC,
    DBG_LOONGARCH_BADV,
}

unsafe extern "C" {
    pub fn kgdb_breakinst();
    pub fn arch_kgdb_breakpoint();
}

#[cfg(feature = "CONFIG_KGDB")]
unsafe extern "C" {
    pub fn kgdb_breakpoint_handler(regs: *mut crate::pt_regs) -> bool;
}

#[cfg(not(feature = "CONFIG_KGDB"))]
#[inline]
pub unsafe fn kgdb_breakpoint_handler(_regs: *mut crate::pt_regs) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
