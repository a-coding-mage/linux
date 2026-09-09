// SPDX-License-Identifier: GPL-2.0-only
/*
 * 32-bit compatibility support for ELF format executables and core dumps.
 *
 * Copyright (C) 2007 Red Hat, Inc.  All rights reserved.
 *
 * Red Hat Author: Roland McGrath.
 *
 * This file is used in a 64-bit kernel that wants to support 32-bit ELF.
 * asm/elf.h is responsible for defining the compat_* and COMPAT_* macros
 * used below, with definitions appropriate for 32-bit ABI compatibility.
 *
 * We use macros to rename the ABI types and machine-dependent
 * functions used in binfmt_elf.c to compat versions.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/elfcore-compat.h, linux/time.h

pub const ELF_COMPAT: i32 = 1;

/*
 * Rename the basic ELF layout types to refer to the 32-bit class of files.
 * The following aliases preserve the C preprocessor mappings and refer to
 * definitions supplied by the surrounding kernel translation unit.
 */
pub type ELF_CLASS = ELFCLASS32;
pub type elfhdr = elf32_hdr;
pub type elf_phdr = elf32_phdr;
pub type elf_shdr = elf32_shdr;
pub type elf_note = elf32_note;
pub type elf_addr_t = Elf32_Addr;
pub type ELF_GNU_PROPERTY_ALIGN = ELF32_GNU_PROPERTY_ALIGN;

/*
 * Some data types as stored in coredump.
 */
pub type user_long_t = compat_long_t;
pub type user_siginfo_t = compat_siginfo_t;
// C macro alias: copy_siginfo_to_external -> copy_siginfo_to_external32
pub use copy_siginfo_to_external32 as copy_siginfo_to_external;

/*
 * The machine-dependent core note format types are defined in elfcore-compat.h,
 * which requires asm/elf.h to define compat_elf_gregset_t et al.
 */
pub type elf_prstatus = compat_elf_prstatus;
pub type elf_prstatus_common = compat_elf_prstatus_common;
pub type elf_prpsinfo = compat_elf_prpsinfo;

// C macro alias: ns_to_kernel_old_timeval -> ns_to_old_timeval32
pub use ns_to_old_timeval32 as ns_to_kernel_old_timeval;

/*
 * To use this file, asm/elf.h must define compat_elf_check_arch.
 * The other following macros can be defined if the compat versions differ
 * from the native ones, or omitted when they match.
 *
 * Conditional COMPAT_* mappings from the C source are retained as comments;
 * their availability is supplied by the target kernel configuration.
 */

// #ifdef COMPAT_ELF_PLATFORM
// pub use COMPAT_ELF_PLATFORM as ELF_PLATFORM;
// #endif
// #ifdef COMPAT_ELF_HWCAP
// pub use COMPAT_ELF_HWCAP as ELF_HWCAP;
// #endif
// #ifdef COMPAT_ELF_HWCAP2
// pub use COMPAT_ELF_HWCAP2 as ELF_HWCAP2;
// #endif
// #ifdef COMPAT_ELF_HWCAP3
// pub use COMPAT_ELF_HWCAP3 as ELF_HWCAP3;
// #endif
// #ifdef COMPAT_ELF_HWCAP4
// pub use COMPAT_ELF_HWCAP4 as ELF_HWCAP4;
// #endif
// #ifdef COMPAT_ARCH_DLINFO
// pub use COMPAT_ARCH_DLINFO as ARCH_DLINFO;
// #endif
// #ifdef COMPAT_ELF_ET_DYN_BASE
// pub use COMPAT_ELF_ET_DYN_BASE as ELF_ET_DYN_BASE;
// #endif
// #ifdef COMPAT_ELF_PLAT_INIT
// pub use COMPAT_ELF_PLAT_INIT as ELF_PLAT_INIT;
// #endif
// #ifdef COMPAT_SET_PERSONALITY
// pub use COMPAT_SET_PERSONALITY as SET_PERSONALITY;
// #endif

// Equivalent of the C function-like macro:
// COMPAT_START_THREAD(ex, regs, new_ip, new_sp) ->
//     compat_start_thread(regs, new_ip, new_sp)
#[inline(always)]
pub unsafe fn COMPAT_START_THREAD(
    _ex: *mut core::ffi::c_void,
    regs: *mut core::ffi::c_void,
    new_ip: usize,
    new_sp: usize,
) {
    compat_start_thread(regs, new_ip, new_sp);
}

// #ifdef COMPAT_START_THREAD
// pub use COMPAT_START_THREAD as START_THREAD;
// #endif

// Equivalent of the C function-like macro:
// COMPAT_ARCH_SETUP_ADDITIONAL_PAGES(bprm, ex, interpreter) ->
//     compat_arch_setup_additional_pages(bprm, interpreter)
#[inline(always)]
pub unsafe fn COMPAT_ARCH_SETUP_ADDITIONAL_PAGES(
    bprm: *mut core::ffi::c_void,
    _ex: *mut core::ffi::c_void,
    interpreter: *mut core::ffi::c_void,
) {
    compat_arch_setup_additional_pages(bprm, interpreter);
}

// #ifdef COMPAT_ARCH_SETUP_ADDITIONAL_PAGES
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: i32 = 1;
// pub use COMPAT_ARCH_SETUP_ADDITIONAL_PAGES as ARCH_SETUP_ADDITIONAL_PAGES;
// #endif

// #ifdef compat_elf_read_implies_exec
// pub use compat_elf_read_implies_exec as elf_read_implies_exec;
// #endif

/*
 * Rename a few of the symbols that binfmt_elf.c will define.
 * These are all local so the names don't really matter, but it
 * might make some debugging less confusing not to duplicate them.
 */
// elf_format -> compat_elf_format
// init_elf_binfmt -> init_compat_elf_binfmt
// exit_elf_binfmt -> exit_compat_elf_binfmt
// binfmt_elf_test_cases -> compat_binfmt_elf_test_cases
// binfmt_elf_test_suite -> compat_binfmt_elf_test_suite

/*
 * We share all the actual code with the native (64-bit) version.
 * The included binfmt_elf.c implementation is supplied by the surrounding
 * translation unit and is intentionally not duplicated here.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
