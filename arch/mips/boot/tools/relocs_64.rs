// SPDX-License-Identifier: GPL-2.0
// Dependency: declarations and definitions supplied by relocs.h.

pub const ELF_BITS: i32 = 64;

// ELF_MACHINE             EM_MIPS
// ELF_MACHINE_NAME        "MIPS64"
// SHT_REL_TYPE            SHT_RELA
// Elf_Rel                 Elf64_Rela
pub const ELF_MACHINE_NAME: &str = "MIPS64";

pub type Elf64_Byte = u8;

#[repr(C)]
pub struct Elf64_Mips_RelaFields {
    pub r_sym: Elf64_Word, // Symbol index.
    pub r_ssym: Elf64_Byte, // Special symbol.
    pub r_type3: Elf64_Byte, // Third relocation.
    pub r_type2: Elf64_Byte, // Second relocation.
    pub r_type: Elf64_Byte, // First relocation.
}

#[repr(C)]
pub union Elf64_Mips_Rela {
    pub fields: Elf64_Mips_RelaFields,
    pub unused: Elf64_Xword,
}

// ELF_CLASS               ELFCLASS64
pub const ELF_CLASS: i32 = ELFCLASS64;

#[inline]
pub unsafe fn ELF_R_SYM(val: *mut Elf64_Mips_Rela) -> *mut Elf64_Word {
    unsafe { &mut (*val).fields.r_sym }
}

#[inline]
pub unsafe fn ELF_R_TYPE(val: *mut Elf64_Mips_Rela) -> *mut Elf64_Byte {
    unsafe { &mut (*val).fields.r_type }
}

#[inline]
pub fn ELF_ST_TYPE(o: Elf64_Xword) -> Elf64_Xword {
    ELF64_ST_TYPE(o)
}

#[inline]
pub fn ELF_ST_BIND(o: Elf64_Xword) -> Elf64_Xword {
    ELF64_ST_BIND(o)
}

#[inline]
pub fn ELF_ST_VISIBILITY(o: Elf64_Xword) -> Elf64_Xword {
    ELF64_ST_VISIBILITY(o)
}

// The implementation is supplied by relocs.c in the original translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
