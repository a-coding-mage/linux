// SPDX-License-Identifier: GPL-2.0-or-later
/*  Kernel module help for powerpc.
    Copyright (C) 2001, 2003 Rusty Russell IBM Corporation.
    Copyright (C) 2008 Freescale Semiconductor, Inc.

    C header dependencies are supplied by the surrounding kernel translation.
*/

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn module_finalize_ftrace(me: *mut module, sechdrs: *const Elf_Shdr) -> c_int;
    fn do_feature_fixups(features: u64, start: *mut c_void, end: *mut c_void);
    fn do_lwsync_fixups(features: u64, start: *mut c_void, end: *mut c_void);
    fn do_barrier_nospec_fixups(enabled: c_int, start: *mut c_void, end: *mut c_void);

    static cur_cpu_spec: *mut cpu_spec;
    static powerpc_firmware_features: u64;
    static barrier_nospec_enabled: c_int;
}

#[repr(C)]
pub struct Elf_Ehdr {
    pub e_shstrndx: u16,
    pub e_shnum: u16,
}

#[repr(C)]
pub struct Elf_Shdr {
    pub sh_name: c_uint,
    pub sh_offset: u64,
    pub sh_addr: u64,
    pub sh_size: u64,
}

#[repr(C)]
pub struct module {
    pub arch: module_arch,
}

#[repr(C)]
pub struct module_arch {
    pub start_opd: u64,
    pub end_opd: u64,
}

#[repr(C)]
pub struct cpu_spec {
    pub cpu_features: u64,
    pub mmu_features: u64,
}

unsafe fn find_section(
    hdr: *const Elf_Ehdr,
    sechdrs: *const Elf_Shdr,
    name: *const c_char,
) -> *const Elf_Shdr {
    let secstrings = (hdr as *const u8).add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset as usize)
        as *const c_char;
    let mut i: c_uint = 1;
    while i < (*hdr).e_shnum as c_uint {
        if strcmp(secstrings.add((*sechdrs.add(i as usize)).sh_name as usize), name) == 0 {
            return sechdrs.add(i as usize);
        }
        i += 1;
    }
    core::ptr::null()
}

pub unsafe fn module_finalize(
    hdr: *const Elf_Ehdr,
    sechdrs: *const Elf_Shdr,
    me: *mut module,
) -> c_int {
    let rc = module_finalize_ftrace(me, sechdrs);
    if rc != 0 {
        return rc;
    }

    /* Apply feature fixups */
    let mut sect = find_section(hdr, sechdrs, b"__ftr_fixup\0".as_ptr() as *const c_char);
    if !sect.is_null() {
        do_feature_fixups(
            (*cur_cpu_spec).cpu_features,
            (*sect).sh_addr as *mut c_void,
            ((*sect).sh_addr + (*sect).sh_size) as *mut c_void,
        );
    }

    sect = find_section(hdr, sechdrs, b"__mmu_ftr_fixup\0".as_ptr() as *const c_char);
    if !sect.is_null() {
        do_feature_fixups(
            (*cur_cpu_spec).mmu_features,
            (*sect).sh_addr as *mut c_void,
            ((*sect).sh_addr + (*sect).sh_size) as *mut c_void,
        );
    }

    // CONFIG_PPC64
    sect = find_section(hdr, sechdrs, b"__fw_ftr_fixup\0".as_ptr() as *const c_char);
    if !sect.is_null() {
        do_feature_fixups(
            powerpc_firmware_features,
            (*sect).sh_addr as *mut c_void,
            ((*sect).sh_addr + (*sect).sh_size) as *mut c_void,
        );
    }

    // CONFIG_PPC64_ELF_ABI_V1
    sect = find_section(hdr, sechdrs, b".opd\0".as_ptr() as *const c_char);
    if !sect.is_null() {
        (*me).arch.start_opd = (*sect).sh_addr;
        (*me).arch.end_opd = (*sect).sh_addr + (*sect).sh_size;
    }

    // CONFIG_PPC_BARRIER_NOSPEC
    sect = find_section(hdr, sechdrs, b"__spec_barrier_fixup\0".as_ptr() as *const c_char);
    if !sect.is_null() {
        do_barrier_nospec_fixups(
            barrier_nospec_enabled,
            (*sect).sh_addr as *mut c_void,
            ((*sect).sh_addr + (*sect).sh_size) as *mut c_void,
        );
    }

    sect = find_section(hdr, sechdrs, b"__lwsync_fixup\0".as_ptr() as *const c_char);
    if !sect.is_null() {
        do_lwsync_fixups(
            (*cur_cpu_spec).cpu_features,
            (*sect).sh_addr as *mut c_void,
            ((*sect).sh_addr + (*sect).sh_size) as *mut c_void,
        );
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
