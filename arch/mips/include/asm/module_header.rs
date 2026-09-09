/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers are referenced here
// but intentionally not implemented in this translation.

#[repr(C)]
pub struct mod_arch_specific {
    /* Data Bus Error exception tables */
    pub dbe_list: list_head,
    pub dbe_start: *const exception_table_entry,
    pub dbe_end: *const exception_table_entry,
    pub r_mips_hi16_list: *mut mips_hi16,
}

pub type Elf64_Byte = u8; /* Type for a 8-bit quantity.  */

#[repr(C)]
pub struct Elf64_Mips_Rel {
    pub r_offset: Elf64_Addr, /* Address of relocation.  */
    pub r_sym: Elf64_Word,    /* Symbol index.  */
    pub r_ssym: Elf64_Byte,   /* Special symbol.  */
    pub r_type3: Elf64_Byte,  /* Third relocation.  */
    pub r_type2: Elf64_Byte,  /* Second relocation.  */
    pub r_type: Elf64_Byte,   /* First relocation.  */
}

#[repr(C)]
pub struct Elf64_Mips_Rela {
    pub r_offset: Elf64_Addr, /* Address of relocation.  */
    pub r_sym: Elf64_Word,    /* Symbol index.  */
    pub r_ssym: Elf64_Byte,   /* Special symbol.  */
    pub r_type3: Elf64_Byte,  /* Third relocation.  */
    pub r_type2: Elf64_Byte,  /* Second relocation.  */
    pub r_type: Elf64_Byte,   /* First relocation.  */
    pub r_addend: Elf64_Sxword, /* Addend.  */
}

// The following aliases and macros are selected by the build-time
// CONFIG_32BIT/CONFIG_64BIT condition in the original header.
#[cfg(CONFIG_32BIT)]
pub type Elf_Shdr = Elf32_Shdr;
#[cfg(CONFIG_32BIT)]
pub type Elf_Sym = Elf32_Sym;
#[cfg(CONFIG_32BIT)]
pub type Elf_Ehdr = Elf32_Ehdr;
#[cfg(CONFIG_32BIT)]
pub type Elf_Addr = Elf32_Addr;
#[cfg(CONFIG_32BIT)]
pub type Elf_Rel = Elf32_Rel;
#[cfg(CONFIG_32BIT)]
pub type Elf_Rela = Elf32_Rela;
#[cfg(CONFIG_32BIT)]
pub type Elf_Mips_Rel = Elf32_Rel;
#[cfg(CONFIG_32BIT)]
pub type Elf_Mips_Rela = Elf32_Rela;

#[cfg(CONFIG_64BIT)]
pub type Elf_Shdr = Elf64_Shdr;
#[cfg(CONFIG_64BIT)]
pub type Elf_Sym = Elf64_Sym;
#[cfg(CONFIG_64BIT)]
pub type Elf_Ehdr = Elf64_Ehdr;
#[cfg(CONFIG_64BIT)]
pub type Elf_Addr = Elf64_Addr;
#[cfg(CONFIG_64BIT)]
pub type Elf_Rel = Elf64_Rel;
#[cfg(CONFIG_64BIT)]
pub type Elf_Rela = Elf64_Rela;
#[cfg(CONFIG_64BIT)]
pub type Elf_Mips_Rel = Elf64_Mips_Rel;
#[cfg(CONFIG_64BIT)]
pub type Elf_Mips_Rela = Elf64_Mips_Rela;

#[cfg(CONFIG_MODULES)]
extern "C" {
    /* Given an address, look for it in the exception tables. */
    pub fn search_module_dbetables(addr: c_ulong) -> *const exception_table_entry;
}

#[cfg(not(CONFIG_MODULES))]
/* Given an address, look for it in the exception tables. */
pub unsafe fn search_module_dbetables(_addr: c_ulong) -> *const exception_table_entry {
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
