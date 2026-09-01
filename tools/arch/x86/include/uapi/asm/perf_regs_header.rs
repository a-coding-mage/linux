/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const PERF_REG_X86_AX: i32 = 0;
pub const PERF_REG_X86_BX: i32 = 1;
pub const PERF_REG_X86_CX: i32 = 2;
pub const PERF_REG_X86_DX: i32 = 3;
pub const PERF_REG_X86_SI: i32 = 4;
pub const PERF_REG_X86_DI: i32 = 5;
pub const PERF_REG_X86_BP: i32 = 6;
pub const PERF_REG_X86_SP: i32 = 7;
pub const PERF_REG_X86_IP: i32 = 8;
pub const PERF_REG_X86_FLAGS: i32 = 9;
pub const PERF_REG_X86_CS: i32 = 10;
pub const PERF_REG_X86_SS: i32 = 11;
pub const PERF_REG_X86_DS: i32 = 12;
pub const PERF_REG_X86_ES: i32 = 13;
pub const PERF_REG_X86_FS: i32 = 14;
pub const PERF_REG_X86_GS: i32 = 15;
pub const PERF_REG_X86_R8: i32 = 16;
pub const PERF_REG_X86_R9: i32 = 17;
pub const PERF_REG_X86_R10: i32 = 18;
pub const PERF_REG_X86_R11: i32 = 19;
pub const PERF_REG_X86_R12: i32 = 20;
pub const PERF_REG_X86_R13: i32 = 21;
pub const PERF_REG_X86_R14: i32 = 22;
pub const PERF_REG_X86_R15: i32 = 23;
/* These are the limits for the GPRs. */
pub const PERF_REG_X86_32_MAX: i32 = PERF_REG_X86_GS + 1;
pub const PERF_REG_X86_64_MAX: i32 = PERF_REG_X86_R15 + 1;

/* These all need two bits set because they are 128bit */
pub const PERF_REG_X86_XMM0: i32 = 32;
pub const PERF_REG_X86_XMM1: i32 = 34;
pub const PERF_REG_X86_XMM2: i32 = 36;
pub const PERF_REG_X86_XMM3: i32 = 38;
pub const PERF_REG_X86_XMM4: i32 = 40;
pub const PERF_REG_X86_XMM5: i32 = 42;
pub const PERF_REG_X86_XMM6: i32 = 44;
pub const PERF_REG_X86_XMM7: i32 = 46;
pub const PERF_REG_X86_XMM8: i32 = 48;
pub const PERF_REG_X86_XMM9: i32 = 50;
pub const PERF_REG_X86_XMM10: i32 = 52;
pub const PERF_REG_X86_XMM11: i32 = 54;
pub const PERF_REG_X86_XMM12: i32 = 56;
pub const PERF_REG_X86_XMM13: i32 = 58;
pub const PERF_REG_X86_XMM14: i32 = 60;
pub const PERF_REG_X86_XMM15: i32 = 62;

/* These include both GPRs and XMMX registers */
pub const PERF_REG_X86_XMM_MAX: i32 = PERF_REG_X86_XMM15 + 2;

pub const PERF_REG_EXTENDED_MASK: u64 = !((1u64 << (PERF_REG_X86_XMM0 as u32)) - 1);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
