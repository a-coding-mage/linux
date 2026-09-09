// SPDX-License-Identifier: GPL-2.0
//
// This file is the 32-bit MIPS instantiation of the shared relocation
// implementation.  The declarations below correspond to the C preprocessor
// definitions used while including relocs.c.

pub const ELF_BITS: u32 = 32;

pub const ELF_MACHINE: u32 = EM_MIPS;
pub const ELF_MACHINE_NAME: &str = "MIPS";
pub const SHT_REL_TYPE: u32 = SHT_REL;

// C: typedef ElfW(Rel) Elf_Rel;
pub type Elf_Rel = ElfW(Rel);

pub const ELF_CLASS: u32 = ELFCLASS32;

#[inline]
pub const fn ELF_R_SYM(val: u32) -> u32 {
    ELF32_R_SYM(val)
}

#[inline]
pub const fn ELF_R_TYPE(val: u32) -> u32 {
    ELF32_R_TYPE(val)
}

#[inline]
pub const fn ELF_ST_TYPE(o: u32) -> u32 {
    ELF32_ST_TYPE(o)
}

#[inline]
pub const fn ELF_ST_BIND(o: u32) -> u32 {
    ELF32_ST_BIND(o)
}

#[inline]
pub const fn ELF_ST_VISIBILITY(o: u32) -> u32 {
    ELF32_ST_VISIBILITY(o)
}

// The C source includes the shared implementation here:
//
//     #include "relocs.c"
//
// Its declarations and definitions are supplied by the corresponding Rust
// translation of that shared source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
