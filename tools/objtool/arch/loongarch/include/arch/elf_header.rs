/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * See the following link for more info about ELF Relocation types:
 * https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html#_relocations
 */

/* Defined here when not supplied by the surrounding ELF headers in C. */
pub const R_LARCH_NONE: u32 = 0;
pub const R_LARCH_32: u32 = 1;
pub const R_LARCH_64: u32 = 2;
pub const R_LARCH_32_PCREL: u32 = 99;
pub const R_LARCH_64_PCREL: u32 = 109;

/* Defined here when not supplied by the surrounding ELF headers in C. */
pub const EM_LOONGARCH: u32 = 258;

pub const R_NONE: u32 = R_LARCH_NONE;
pub const R_ABS32: u32 = R_LARCH_32;
pub const R_ABS64: u32 = R_LARCH_64;
pub const R_DATA32: u32 = R_LARCH_32_PCREL;
pub const R_DATA64: u32 = R_LARCH_32_PCREL;
pub const R_TEXT32: u32 = R_LARCH_32_PCREL;
pub const R_TEXT64: u32 = R_LARCH_32_PCREL;
