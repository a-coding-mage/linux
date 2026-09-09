/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm-generic/module.h.

/*
 * This file contains the s390 architecture specific module code.
 */

#[repr(C)]
pub struct mod_arch_syminfo {
    pub got_offset: ::core::ffi::c_ulong,
    pub plt_offset: ::core::ffi::c_ulong,
    pub got_initialized: ::core::ffi::c_int,
    pub plt_initialized: ::core::ffi::c_int,
}

#[repr(C)]
pub struct mod_arch_specific {
    /* Starting offset of got in the module core memory. */
    pub got_offset: ::core::ffi::c_ulong,
    /* Starting offset of plt in the module core memory. */
    pub plt_offset: ::core::ffi::c_ulong,
    /* Size of the got. */
    pub got_size: ::core::ffi::c_ulong,
    /* Size of the plt. */
    pub plt_size: ::core::ffi::c_ulong,
    /* Number of symbols in syminfo. */
    pub nsyms: ::core::ffi::c_int,
    /* Additional symbol information (got and plt offsets). */
    pub syminfo: *mut mod_arch_syminfo,
    /*
     * CONFIG_FUNCTION_TRACER conditional fields are represented with the
     * corresponding Rust configuration condition.
     */
    #[cfg(feature = "CONFIG_FUNCTION_TRACER")]
    /* Start of memory reserved for ftrace hotpatch trampolines. */
    pub trampolines_start: *mut ftrace_hotpatch_trampoline,
    #[cfg(feature = "CONFIG_FUNCTION_TRACER")]
    /* End of memory reserved for ftrace hotpatch trampolines. */
    pub trampolines_end: *mut ftrace_hotpatch_trampoline,
    #[cfg(feature = "CONFIG_FUNCTION_TRACER")]
    /* Next unused ftrace hotpatch trampoline slot. */
    pub next_trampoline: *mut ftrace_hotpatch_trampoline,
}

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

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
