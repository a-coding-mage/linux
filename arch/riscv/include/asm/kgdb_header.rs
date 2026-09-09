/* SPDX-License-Identifier: GPL-2.0-only */

/* Declarations are applicable to the kernel build (__KERNEL__). */

pub const GDB_SIZEOF_REG: usize = core::mem::size_of::<usize>();

pub const DBG_MAX_REG_NUM: usize = 36;
pub const NUMREGBYTES: usize = DBG_MAX_REG_NUM * GDB_SIZEOF_REG;
pub const CACHE_FLUSH_IS_SAFE: usize = 1;
pub const BUFMAX: usize = 2048;
// As per KGDB documentation, BUFMAX must be larger than NUMREGBYTES.

#[cfg(feature = "CONFIG_RISCV_ISA_C")]
pub const BREAK_INSTR_SIZE: usize = 2;
#[cfg(not(feature = "CONFIG_RISCV_ISA_C"))]
pub const BREAK_INSTR_SIZE: usize = 4;

pub unsafe extern "C" fn arch_kgdb_breakpoint();
pub static mut kgdb_compiled_break: usize;

pub const DBG_REG_ZERO: &str = "zero";
pub const DBG_REG_RA: &str = "ra";
pub const DBG_REG_SP: &str = "sp";
pub const DBG_REG_GP: &str = "gp";
pub const DBG_REG_TP: &str = "tp";
pub const DBG_REG_T0: &str = "t0";
pub const DBG_REG_T1: &str = "t1";
pub const DBG_REG_T2: &str = "t2";
pub const DBG_REG_FP: &str = "fp";
pub const DBG_REG_S1: &str = "s1";
pub const DBG_REG_A0: &str = "a0";
pub const DBG_REG_A1: &str = "a1";
pub const DBG_REG_A2: &str = "a2";
pub const DBG_REG_A3: &str = "a3";
pub const DBG_REG_A4: &str = "a4";
pub const DBG_REG_A5: &str = "a5";
pub const DBG_REG_A6: &str = "a6";
pub const DBG_REG_A7: &str = "a7";
pub const DBG_REG_S2: &str = "s2";
pub const DBG_REG_S3: &str = "s3";
pub const DBG_REG_S4: &str = "s4";
pub const DBG_REG_S5: &str = "s5";
pub const DBG_REG_S6: &str = "s6";
pub const DBG_REG_S7: &str = "s7";
pub const DBG_REG_S8: &str = "s8";
pub const DBG_REG_S9: &str = "s9";
pub const DBG_REG_S10: &str = "s10";
pub const DBG_REG_S11: &str = "s11";
pub const DBG_REG_T3: &str = "t3";
pub const DBG_REG_T4: &str = "t4";
pub const DBG_REG_T5: &str = "t5";
pub const DBG_REG_T6: &str = "t6";
pub const DBG_REG_EPC: &str = "pc";
pub const DBG_REG_STATUS: &str = "sstatus";
pub const DBG_REG_BADADDR: &str = "stval";
pub const DBG_REG_CAUSE: &str = "scause";

pub const DBG_REG_ZERO_OFF: usize = 0;
pub const DBG_REG_RA_OFF: usize = 1;
pub const DBG_REG_SP_OFF: usize = 2;
pub const DBG_REG_GP_OFF: usize = 3;
pub const DBG_REG_TP_OFF: usize = 4;
pub const DBG_REG_T0_OFF: usize = 5;
pub const DBG_REG_T1_OFF: usize = 6;
pub const DBG_REG_T2_OFF: usize = 7;
pub const DBG_REG_FP_OFF: usize = 8;
pub const DBG_REG_S1_OFF: usize = 9;
pub const DBG_REG_A0_OFF: usize = 10;
pub const DBG_REG_A1_OFF: usize = 11;
pub const DBG_REG_A2_OFF: usize = 12;
pub const DBG_REG_A3_OFF: usize = 13;
pub const DBG_REG_A4_OFF: usize = 14;
pub const DBG_REG_A5_OFF: usize = 15;
pub const DBG_REG_A6_OFF: usize = 16;
pub const DBG_REG_A7_OFF: usize = 17;
pub const DBG_REG_S2_OFF: usize = 18;
pub const DBG_REG_S3_OFF: usize = 19;
pub const DBG_REG_S4_OFF: usize = 20;
pub const DBG_REG_S5_OFF: usize = 21;
pub const DBG_REG_S6_OFF: usize = 22;
pub const DBG_REG_S7_OFF: usize = 23;
pub const DBG_REG_S8_OFF: usize = 24;
pub const DBG_REG_S9_OFF: usize = 25;
pub const DBG_REG_S10_OFF: usize = 26;
pub const DBG_REG_S11_OFF: usize = 27;
pub const DBG_REG_T3_OFF: usize = 28;
pub const DBG_REG_T4_OFF: usize = 29;
pub const DBG_REG_T5_OFF: usize = 30;
pub const DBG_REG_T6_OFF: usize = 31;
pub const DBG_REG_EPC_OFF: usize = 32;
pub const DBG_REG_STATUS_OFF: usize = 33;
pub const DBG_REG_BADADDR_OFF: usize = 34;
pub const DBG_REG_CAUSE_OFF: usize = 35;
/* NOTE: increase DBG_MAX_REG_NUM if you add more values here. */

pub static riscv_gdb_stub_feature: [core::ffi::c_char; 64];

pub use riscv_gdb_stub_feature as kgdb_arch_gdb_stub_feature;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
