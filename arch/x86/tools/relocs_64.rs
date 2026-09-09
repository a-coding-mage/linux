// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by the surrounding relocs implementation:
// #include "relocs.h"

pub const ELF_BITS: usize = 64;

pub const ELF_MACHINE: _ = EM_X86_64;
pub const ELF_MACHINE_NAME: &str = "x86_64";
pub const SHT_REL_TYPE: _ = SHT_RELA;
pub type Elf_Rel = Elf64_Rela;

pub const ELF_CLASS: _ = ELFCLASS64;

#[inline]
pub const unsafe fn ELF_R_SYM(val: _) -> _ {
    ELF64_R_SYM(val)
}

#[inline]
pub const unsafe fn ELF_R_TYPE(val: _) -> _ {
    ELF64_R_TYPE(val)
}

#[inline]
pub const unsafe fn ELF_ST_TYPE(o: _) -> _ {
    ELF64_ST_TYPE(o)
}

#[inline]
pub const unsafe fn ELF_ST_BIND(o: _) -> _ {
    ELF64_ST_BIND(o)
}

#[inline]
pub const unsafe fn ELF_ST_VISIBILITY(o: _) -> _ {
    ELF64_ST_VISIBILITY(o)
}

// The C source includes the shared implementation here:
// #include "relocs.c"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
