// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/symsrc.h.
// C dependencies: <stdbool.h>, <stddef.h>, "dso.h", <elf.h>;
// when HAVE_LIBELF_SUPPORT is enabled: <libelf.h>, <gelf.h>.

use core::ffi::c_char;
use std::os::raw::c_int;

use crate::dso::{dso, dso_binary_type};

#[cfg(HAVE_LIBELF_SUPPORT)]
use crate::libelf::{Elf, Elf_Scn, GElf_Ehdr, GElf_Shdr};

#[repr(C)]
pub struct symsrc {
    pub name: *mut c_char,
    pub fd: c_int,
    pub r#type: dso_binary_type,

    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub elf: *mut Elf,
    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub ehdr: GElf_Ehdr,

    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub opdsec: *mut Elf_Scn,
    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub opdidx: usize,
    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub opdshdr: GElf_Shdr,

    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub symtab: *mut Elf_Scn,
    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub symtab_idx: usize,
    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub symshdr: GElf_Shdr,

    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub dynsym: *mut Elf_Scn,
    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub dynsym_idx: usize,
    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub dynshdr: GElf_Shdr,

    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub adjust_symbols: bool,
    #[cfg(HAVE_LIBELF_SUPPORT)]
    pub is_64_bit: bool,
}

unsafe extern "C" {
    pub fn symsrc__init(
        ss: *mut symsrc,
        dso: *mut dso,
        name: *const c_char,
        r#type: dso_binary_type,
    ) -> c_int;
    pub fn symsrc__destroy(ss: *mut symsrc);

    pub fn symsrc__has_symtab(ss: *mut symsrc) -> bool;
    pub fn symsrc__possibly_runtime(ss: *mut symsrc) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
