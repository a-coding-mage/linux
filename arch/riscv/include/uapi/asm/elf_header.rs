/*
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * Copyright (C) 2012 Regents of the University of California
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

// Dependency supplied by the corresponding ptrace header.

/* ELF register definitions */
pub type elf_greg_t = ::core::ffi::c_ulong;
pub type elf_gregset_t = user_regs_struct;
pub const ELF_NGREG: usize = ::core::mem::size_of::<elf_gregset_t>() / ::core::mem::size_of::<elf_greg_t>();

/* We don't support f without d, or q.  */
pub type elf_fpreg_t = __u64;
pub type elf_fpregset_t = __riscv_fp_state;
pub const ELF_NFPREG: usize = ::core::mem::size_of::<__riscv_d_ext_state>() / ::core::mem::size_of::<elf_fpreg_t>();

// The C header selects these definitions according to __riscv_xlen.
#[cfg(target_pointer_width = "64")]
macro_rules! ELF_RISCV_R_SYM {
    ($r_info:expr) => { ELF64_R_SYM!($r_info) };
}
#[cfg(target_pointer_width = "64")]
macro_rules! ELF_RISCV_R_TYPE {
    ($r_info:expr) => { ELF64_R_TYPE!($r_info) };
}
#[cfg(not(target_pointer_width = "64"))]
macro_rules! ELF_RISCV_R_SYM {
    ($r_info:expr) => { ELF32_R_SYM!($r_info) };
}
#[cfg(not(target_pointer_width = "64"))]
macro_rules! ELF_RISCV_R_TYPE {
    ($r_info:expr) => { ELF32_R_TYPE!($r_info) };
}

/*
 * RISC-V relocation types
 */

/* Relocation types used by the dynamic linker */
pub const R_RISCV_NONE: u32 = 0;
pub const R_RISCV_32: u32 = 1;
pub const R_RISCV_64: u32 = 2;
pub const R_RISCV_RELATIVE: u32 = 3;
pub const R_RISCV_COPY: u32 = 4;
pub const R_RISCV_JUMP_SLOT: u32 = 5;
pub const R_RISCV_TLS_DTPMOD32: u32 = 6;
pub const R_RISCV_TLS_DTPMOD64: u32 = 7;
pub const R_RISCV_TLS_DTPREL32: u32 = 8;
pub const R_RISCV_TLS_DTPREL64: u32 = 9;
pub const R_RISCV_TLS_TPREL32: u32 = 10;
pub const R_RISCV_TLS_TPREL64: u32 = 11;
pub const R_RISCV_IRELATIVE: u32 = 58;

/* Relocation types not used by the dynamic linker */
pub const R_RISCV_BRANCH: u32 = 16;
pub const R_RISCV_JAL: u32 = 17;
pub const R_RISCV_CALL: u32 = 18;
pub const R_RISCV_CALL_PLT: u32 = 19;
pub const R_RISCV_GOT_HI20: u32 = 20;
pub const R_RISCV_TLS_GOT_HI20: u32 = 21;
pub const R_RISCV_TLS_GD_HI20: u32 = 22;
pub const R_RISCV_PCREL_HI20: u32 = 23;
pub const R_RISCV_PCREL_LO12_I: u32 = 24;
pub const R_RISCV_PCREL_LO12_S: u32 = 25;
pub const R_RISCV_HI20: u32 = 26;
pub const R_RISCV_LO12_I: u32 = 27;
pub const R_RISCV_LO12_S: u32 = 28;
pub const R_RISCV_TPREL_HI20: u32 = 29;
pub const R_RISCV_TPREL_LO12_I: u32 = 30;
pub const R_RISCV_TPREL_LO12_S: u32 = 31;
pub const R_RISCV_TPREL_ADD: u32 = 32;
pub const R_RISCV_ADD8: u32 = 33;
pub const R_RISCV_ADD16: u32 = 34;
pub const R_RISCV_ADD32: u32 = 35;
pub const R_RISCV_ADD64: u32 = 36;
pub const R_RISCV_SUB8: u32 = 37;
pub const R_RISCV_SUB16: u32 = 38;
pub const R_RISCV_SUB32: u32 = 39;
pub const R_RISCV_SUB64: u32 = 40;
pub const R_RISCV_GNU_VTINHERIT: u32 = 41;
pub const R_RISCV_GNU_VTENTRY: u32 = 42;
pub const R_RISCV_ALIGN: u32 = 43;
pub const R_RISCV_RVC_BRANCH: u32 = 44;
pub const R_RISCV_RVC_JUMP: u32 = 45;
pub const R_RISCV_GPREL_I: u32 = 47;
pub const R_RISCV_GPREL_S: u32 = 48;
pub const R_RISCV_TPREL_I: u32 = 49;
pub const R_RISCV_TPREL_S: u32 = 50;
pub const R_RISCV_RELAX: u32 = 51;
pub const R_RISCV_SUB6: u32 = 52;
pub const R_RISCV_SET6: u32 = 53;
pub const R_RISCV_SET8: u32 = 54;
pub const R_RISCV_SET16: u32 = 55;
pub const R_RISCV_SET32: u32 = 56;
pub const R_RISCV_32_PCREL: u32 = 57;
pub const R_RISCV_PLT32: u32 = 59;
pub const R_RISCV_SET_ULEB128: u32 = 60;
pub const R_RISCV_SUB_ULEB128: u32 = 61;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
