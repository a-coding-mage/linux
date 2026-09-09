/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Many architectures just need a simple module
 * loader without arch specific data.
 */
/* CONFIG_HAVE_MOD_ARCH_SPECIFIC is a build-time condition from the C header. */
#[cfg(not(feature = "CONFIG_HAVE_MOD_ARCH_SPECIFIC"))]
#[repr(C)]
pub struct mod_arch_specific {}

/* CONFIG_64BIT is represented here by the target pointer width. */
#[cfg(target_pointer_width = "64")]
pub type Elf_Shdr = Elf64_Shdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Phdr = Elf64_Phdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Sym = Elf64_Sym;
#[cfg(target_pointer_width = "64")]
pub type Elf_Dyn = Elf64_Dyn;
#[cfg(target_pointer_width = "64")]
pub type Elf_Ehdr = Elf64_Ehdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Addr = Elf64_Addr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Rel = Elf64_Rel;
#[cfg(target_pointer_width = "64")]
pub type Elf_Rela = Elf64_Rela;

#[cfg(target_pointer_width = "64")]
macro_rules! ELF_R_TYPE {
    ($x:expr) => {
        ELF64_R_TYPE!($x)
    };
}
#[cfg(target_pointer_width = "64")]
macro_rules! ELF_R_SYM {
    ($x:expr) => {
        ELF64_R_SYM!($x)
    };
}

#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Shdr = Elf32_Shdr;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Phdr = Elf32_Phdr;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Sym = Elf32_Sym;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Dyn = Elf32_Dyn;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Ehdr = Elf32_Ehdr;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Addr = Elf32_Addr;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Rel = Elf32_Rel;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Rela = Elf32_Rela;

#[cfg(not(target_pointer_width = "64"))]
macro_rules! ELF_R_TYPE {
    ($x:expr) => {
        ELF32_R_TYPE!($x)
    };
}
#[cfg(not(target_pointer_width = "64"))]
macro_rules! ELF_R_SYM {
    ($x:expr) => {
        ELF32_R_SYM!($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
