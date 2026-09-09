/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

/* Dependency: linux/ptrace.h */

/* Relocation types */
pub const R_NIOS2_NONE: u32 = 0;
pub const R_NIOS2_S16: u32 = 1;
pub const R_NIOS2_U16: u32 = 2;
pub const R_NIOS2_PCREL16: u32 = 3;
pub const R_NIOS2_CALL26: u32 = 4;
pub const R_NIOS2_IMM5: u32 = 5;
pub const R_NIOS2_CACHE_OPX: u32 = 6;
pub const R_NIOS2_IMM6: u32 = 7;
pub const R_NIOS2_IMM8: u32 = 8;
pub const R_NIOS2_HI16: u32 = 9;
pub const R_NIOS2_LO16: u32 = 10;
pub const R_NIOS2_HIADJ16: u32 = 11;
pub const R_NIOS2_BFD_RELOC_32: u32 = 12;
pub const R_NIOS2_BFD_RELOC_16: u32 = 13;
pub const R_NIOS2_BFD_RELOC_8: u32 = 14;
pub const R_NIOS2_GPREL: u32 = 15;
pub const R_NIOS2_GNU_VTINHERIT: u32 = 16;
pub const R_NIOS2_GNU_VTENTRY: u32 = 17;
pub const R_NIOS2_UJMP: u32 = 18;
pub const R_NIOS2_CJMP: u32 = 19;
pub const R_NIOS2_CALLR: u32 = 20;
pub const R_NIOS2_ALIGN: u32 = 21;
/* Keep this the last entry. */
pub const R_NIOS2_NUM: u32 = 22;

pub type elf_greg_t = ::core::ffi::c_ulong;

pub const ELF_NGREG: usize = 49;
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];

pub type elf_fpregset_t = ::core::ffi::c_ulong;

/*
 * These are used to set parameters in the core dumps.
 */
pub const ELF_CLASS: u32 = ELFCLASS32;
pub const ELF_DATA: u32 = ELFDATA2LSB;
pub const ELF_ARCH: u32 = EM_ALTERA_NIOS2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
