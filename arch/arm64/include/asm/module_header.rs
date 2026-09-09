/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

/* Dependency supplied by asm-generic/module.h. */

#[repr(C)]
pub struct mod_plt_sec {
    pub plt_shndx: ::core::ffi::c_int,
    pub plt_num_entries: ::core::ffi::c_int,
    pub plt_max_entries: ::core::ffi::c_int,
}

#[repr(C)]
pub struct mod_arch_specific {
    pub core: mod_plt_sec,
    pub init: mod_plt_sec,

    /* for CONFIG_DYNAMIC_FTRACE */
    pub ftrace_trampolines: *mut plt_entry,
    pub init_ftrace_trampolines: *mut plt_entry,
}

extern "C" {
    pub fn module_emit_plt_entry(
        modu: *mut module,
        sechdrs: *mut Elf64_Shdr,
        loc: *mut ::core::ffi::c_void,
        rela: *const Elf64_Rela,
        sym: *mut Elf64_Sym,
    ) -> u64;

    pub fn module_emit_veneer_for_adrp(
        modu: *mut module,
        sechdrs: *mut Elf64_Shdr,
        loc: *mut ::core::ffi::c_void,
        val: u64,
    ) -> u64;
}

#[repr(C)]
pub struct plt_entry {
    /*
     * A program that conforms to the AArch64 Procedure Call Standard
     * (AAPCS64) must assume that a veneer that alters IP0 (x16) and/or
     * IP1 (x17) may be inserted at any branch instruction that is
     * exposed to a relocation that supports long branches. Since that
     * is exactly what we are dealing with here, we are free to use x16
     * as a scratch register in the PLT veneers.
     */
    pub adrp: u32, /* adrp x16, .... */
    pub add: u32,  /* add  x16, x16, #0x.... */
    pub br: u32,   /* br   x16 */
}

/* Build-time architecture capability supplied by another header. */
extern "C" {
    pub fn cpus_have_final_cap(cap: ::core::ffi::c_int) -> bool;
}

#[inline]
pub unsafe fn is_forbidden_offset_for_adrp(place: *mut ::core::ffi::c_void) -> bool {
    cpus_have_final_cap(ARM64_WORKAROUND_843419)
        && ((place as u64) & 0xfff) >= 0xff8
}

extern "C" {
    pub fn get_plt_entry(dst: u64, pc: *mut ::core::ffi::c_void) -> plt_entry;
}

#[inline]
pub unsafe fn find_section(
    hdr: *const Elf_Ehdr,
    sechdrs: *const Elf_Shdr,
    name: *const ::core::ffi::c_char,
) -> *const Elf_Shdr {
    let secstrs = (hdr as *const u8)
        .add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset as usize)
        as *const ::core::ffi::c_char;

    let mut s = sechdrs;
    let se = sechdrs.add((*hdr).e_shnum as usize);
    while s < se {
        if strcmp(name, secstrs.add((*s).sh_name as usize)) == 0 {
            return s;
        }
        s = s.add(1);
    }

    ::core::ptr::null()
}

extern "C" {
    fn strcmp(
        lhs: *const ::core::ffi::c_char,
        rhs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
