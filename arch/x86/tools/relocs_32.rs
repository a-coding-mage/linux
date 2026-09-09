// SPDX-License-Identifier: GPL-2.0

// Dependency provided by relocs.h.

pub const ELF_BITS: u32 = 32;

pub const ELF_MACHINE: _ = EM_386;
pub const ELF_MACHINE_NAME: &str = "i386";
pub const SHT_REL_TYPE: _ = SHT_REL;

// C macro: Elf_Rel is ElfW(Rel).  The concrete ElfW expansion is supplied by
// the surrounding ELF definitions.
pub type Elf_Rel = ElfW<Rel>;

pub const ELF_CLASS: _ = ELFCLASS32;

macro_rules! ELF_R_SYM {
    ($val:expr) => {
        ELF32_R_SYM($val)
    };
}

macro_rules! ELF_R_TYPE {
    ($val:expr) => {
        ELF32_R_TYPE($val)
    };
}

macro_rules! ELF_ST_TYPE {
    ($o:expr) => {
        ELF32_ST_TYPE($o)
    };
}

macro_rules! ELF_ST_BIND {
    ($o:expr) => {
        ELF32_ST_BIND($o)
    };
}

macro_rules! ELF_ST_VISIBILITY {
    ($o:expr) => {
        ELF32_ST_VISIBILITY($o)
    };
}

// The remainder of this translation is supplied by the shared implementation
// included by the C source: #include "relocs.c"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
