/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

/* C dependency: <linux/math.h> supplies round_up(). */

/* genelf.c */
unsafe extern "C" {
    pub fn jit_write_elf(
        fd: c_int,
        code_addr: u64,
        sym: *const c_char,
        code: *const c_void,
        csize: c_int,
        debug: *mut c_void,
        nr_debug_entries: c_int,
        unwinding: *mut c_void,
        unwinding_header_size: u64,
        unwinding_size: u64,
    ) -> c_int;
}

/* Original C condition: #ifdef HAVE_LIBDW_SUPPORT */
#[cfg(HAVE_LIBDW_SUPPORT)]
unsafe extern "C" {
    /* genelf_debug.c */
    pub fn jit_add_debug_info(
        e: *mut Elf,
        code_addr: u64,
        debug: *mut c_void,
        nr_debug_entries: c_int,
    ) -> c_int;
}

#[cfg(target_arch = "arm")]
pub const GEN_ELF_ARCH: u32 = EM_ARM;
#[cfg(target_arch = "arm")]
pub const GEN_ELF_CLASS: u32 = ELFCLASS32;

#[cfg(target_arch = "aarch64")]
pub const GEN_ELF_ARCH: u32 = EM_AARCH64;
#[cfg(target_arch = "aarch64")]
pub const GEN_ELF_CLASS: u32 = ELFCLASS64;

#[cfg(target_arch = "x86_64")]
pub const GEN_ELF_ARCH: u32 = EM_X86_64;
#[cfg(target_arch = "x86_64")]
pub const GEN_ELF_CLASS: u32 = ELFCLASS64;

#[cfg(target_arch = "x86")]
pub const GEN_ELF_ARCH: u32 = EM_386;
#[cfg(target_arch = "x86")]
pub const GEN_ELF_CLASS: u32 = ELFCLASS32;

#[cfg(target_arch = "powerpc64")]
pub const GEN_ELF_ARCH: u32 = EM_PPC64;
#[cfg(target_arch = "powerpc64")]
pub const GEN_ELF_CLASS: u32 = ELFCLASS64;

#[cfg(target_arch = "powerpc")]
pub const GEN_ELF_ARCH: u32 = EM_PPC;
#[cfg(target_arch = "powerpc")]
pub const GEN_ELF_CLASS: u32 = ELFCLASS32;

#[cfg(all(target_arch = "sparc64"))]
pub const GEN_ELF_ARCH: u32 = EM_SPARCV9;
#[cfg(all(target_arch = "sparc64"))]
pub const GEN_ELF_CLASS: u32 = ELFCLASS64;

#[cfg(all(target_arch = "sparc", not(target_arch = "sparc64")))]
pub const GEN_ELF_ARCH: u32 = EM_SPARC;
#[cfg(all(target_arch = "sparc", not(target_arch = "sparc64")))]
pub const GEN_ELF_CLASS: u32 = ELFCLASS32;

#[cfg(target_arch = "s390x")]
pub const GEN_ELF_ARCH: u32 = EM_S390;
#[cfg(target_arch = "s390x")]
pub const GEN_ELF_CLASS: u32 = ELFCLASS64;

#[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
pub const GEN_ELF_ARCH: u32 = EM_RISCV;
#[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
pub const GEN_ELF_CLASS: u32 = ELFCLASS64;

#[cfg(all(target_arch = "riscv32", target_pointer_width = "32"))]
pub const GEN_ELF_ARCH: u32 = EM_RISCV;
#[cfg(all(target_arch = "riscv32", target_pointer_width = "32"))]
pub const GEN_ELF_CLASS: u32 = ELFCLASS32;

#[cfg(target_arch = "loongarch64")]
pub const GEN_ELF_ARCH: u32 = EM_LOONGARCH;
#[cfg(target_arch = "loongarch64")]
pub const GEN_ELF_CLASS: u32 = ELFCLASS64;

/* Original C fallback: #error "unsupported architecture" */

#[cfg(target_endian = "big")]
pub const GEN_ELF_ENDIAN: u32 = ELFDATA2MSB;
#[cfg(not(target_endian = "big"))]
pub const GEN_ELF_ENDIAN: u32 = ELFDATA2LSB;

#[cfg(target_pointer_width = "64")]
pub use elf64_getshdr as elf_getshdr;
#[cfg(target_pointer_width = "64")]
pub use elf64_newehdr as elf_newehdr;
#[cfg(target_pointer_width = "64")]
pub use elf64_newphdr as elf_newphdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Ehdr = Elf64_Ehdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Phdr = Elf64_Phdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Shdr = Elf64_Shdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Sym = Elf64_Sym;
#[cfg(target_pointer_width = "64")]
pub use ELF64_ST_BIND as ELF_ST_BIND;
#[cfg(target_pointer_width = "64")]
pub use ELF64_ST_TYPE as ELF_ST_TYPE;
#[cfg(target_pointer_width = "64")]
pub use ELF64_ST_VISIBILITY as ELF_ST_VIS;

#[cfg(not(target_pointer_width = "64"))]
pub use elf32_getshdr as elf_getshdr;
#[cfg(not(target_pointer_width = "64"))]
pub use elf32_newehdr as elf_newehdr;
#[cfg(not(target_pointer_width = "64"))]
pub use elf32_newphdr as elf_newphdr;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Ehdr = Elf32_Ehdr;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Phdr = Elf32_Phdr;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Shdr = Elf32_Shdr;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Sym = Elf32_Sym;
#[cfg(not(target_pointer_width = "64"))]
pub use ELF32_ST_BIND as ELF_ST_BIND;
#[cfg(not(target_pointer_width = "64"))]
pub use ELF32_ST_TYPE as ELF_ST_TYPE;
#[cfg(not(target_pointer_width = "64"))]
pub use ELF32_ST_VISIBILITY as ELF_ST_VIS;

/* The .text section is directly after the ELF header */
pub const GEN_ELF_TEXT_OFFSET: usize =
    round_up(core::mem::size_of::<Elf_Ehdr>() + core::mem::size_of::<Elf_Phdr>(), 16);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
