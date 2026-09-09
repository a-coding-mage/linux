/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

/*
 * This file is partially exported to userspace.  This allows us to keep
 * the ELF bits in one place which should assist in keeping the kernel and
 * userspace in sync.
 */

/*
 * ELF register definitions..
 */

/* `struct user_regs_struct` is supplied by asm/ptrace.h. */

/* The OR1K relocation types... not all relevant for module loader */
pub const R_OR1K_NONE: u32 = 0;
pub const R_OR1K_32: u32 = 1;
pub const R_OR1K_16: u32 = 2;
pub const R_OR1K_8: u32 = 3;
pub const R_OR1K_LO_16_IN_INSN: u32 = 4;
pub const R_OR1K_HI_16_IN_INSN: u32 = 5;
pub const R_OR1K_INSN_REL_26: u32 = 6;
pub const R_OR1K_GNU_VTENTRY: u32 = 7;
pub const R_OR1K_GNU_VTINHERIT: u32 = 8;
pub const R_OR1K_32_PCREL: u32 = 9;
pub const R_OR1K_16_PCREL: u32 = 10;
pub const R_OR1K_8_PCREL: u32 = 11;
pub const R_OR1K_GOTPC_HI16: u32 = 12;
pub const R_OR1K_GOTPC_LO16: u32 = 13;
pub const R_OR1K_GOT16: u32 = 14;
pub const R_OR1K_PLT26: u32 = 15;
pub const R_OR1K_GOTOFF_HI16: u32 = 16;
pub const R_OR1K_GOTOFF_LO16: u32 = 17;
pub const R_OR1K_COPY: u32 = 18;
pub const R_OR1K_GLOB_DAT: u32 = 19;
pub const R_OR1K_JMP_SLOT: u32 = 20;
pub const R_OR1K_RELATIVE: u32 = 21;
pub const R_OR1K_TLS_GD_HI16: u32 = 22;
pub const R_OR1K_TLS_GD_LO16: u32 = 23;
pub const R_OR1K_TLS_LDM_HI16: u32 = 24;
pub const R_OR1K_TLS_LDM_LO16: u32 = 25;
pub const R_OR1K_TLS_LDO_HI16: u32 = 26;
pub const R_OR1K_TLS_LDO_LO16: u32 = 27;
pub const R_OR1K_TLS_IE_HI16: u32 = 28;
pub const R_OR1K_TLS_IE_LO16: u32 = 29;
pub const R_OR1K_TLS_LE_HI16: u32 = 30;
pub const R_OR1K_TLS_LE_LO16: u32 = 31;
pub const R_OR1K_TLS_TPOFF: u32 = 32;
pub const R_OR1K_TLS_DTPOFF: u32 = 33;
pub const R_OR1K_TLS_DTPMOD: u32 = 34;
pub const R_OR1K_AHI16: u32 = 35;
pub const R_OR1K_GOTOFF_AHI16: u32 = 36;
pub const R_OR1K_TLS_IE_AHI16: u32 = 37;
pub const R_OR1K_TLS_LE_AHI16: u32 = 38;
pub const R_OR1K_SLO16: u32 = 39;
pub const R_OR1K_GOTOFF_SLO16: u32 = 40;
pub const R_OR1K_TLS_LE_SLO16: u32 = 41;
pub const R_OR1K_PCREL_PG21: u32 = 42;
pub const R_OR1K_GOT_PG21: u32 = 43;
pub const R_OR1K_TLS_GD_PG21: u32 = 44;
pub const R_OR1K_TLS_LDM_PG21: u32 = 45;
pub const R_OR1K_TLS_IE_PG21: u32 = 46;
pub const R_OR1K_LO13: u32 = 47;
pub const R_OR1K_GOT_LO13: u32 = 48;
pub const R_OR1K_TLS_GD_LO13: u32 = 49;
pub const R_OR1K_TLS_LDM_LO13: u32 = 50;
pub const R_OR1K_TLS_IE_LO13: u32 = 51;
pub const R_OR1K_SLO13: u32 = 52;
pub const R_OR1K_PLTA26: u32 = 53;
pub const R_OR1K_GOT_AHI16: u32 = 54;

/* Old relocation names */
pub const R_OR32_NONE: u32 = R_OR1K_NONE;
pub const R_OR32_32: u32 = R_OR1K_32;
pub const R_OR32_16: u32 = R_OR1K_16;
pub const R_OR32_8: u32 = R_OR1K_8;
pub const R_OR32_CONST: u32 = R_OR1K_LO_16_IN_INSN;
pub const R_OR32_CONSTH: u32 = R_OR1K_HI_16_IN_INSN;
pub const R_OR32_JUMPTARG: u32 = R_OR1K_INSN_REL_26;
pub const R_OR32_VTENTRY: u32 = R_OR1K_GNU_VTENTRY;
pub const R_OR32_VTINHERIT: u32 = R_OR1K_GNU_VTINHERIT;

pub type elf_greg_t = ::core::ffi::c_ulong;

/*
 * Note that NGREG is defined to ELF_NGREG in include/linux/elfcore.h, and is
 * thus exposed to user-space.
 */
pub const ELF_NGREG: usize =
    ::core::mem::size_of::<user_regs_struct>() / ::core::mem::size_of::<elf_greg_t>();
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];

pub type elf_fpregset_t = __or1k_fpu_state;

/* EM_OPENRISC is defined in linux/elf-em.h */
pub const EM_OR32: u32 = 0x8472;

/*
 * These are used to set parameters in the core dumps.
 */
pub const ELF_ARCH: u32 = EM_OR32;
pub const ELF_CLASS: u32 = ELFCLASS32;
pub const ELF_DATA: u32 = ELFDATA2MSB;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
