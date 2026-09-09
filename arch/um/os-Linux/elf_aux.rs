// SPDX-License-Identifier: GPL-2.0
/*
 *  arch/um/kernel/elf_aux.c
 *
 *  Scan the ELF auxiliary vector provided by the host to extract
 *  information about vsyscall-page, etc.
 *
 *  Copyright (C) 2004 Fujitsu Siemens Computers GmbH
 *  Author: Bodo Stroesser (bodo.stroesser@fujitsu-siemens.com)
 */

// The following types and constants are supplied by the translated ELF and
// kernel headers.

#[cfg(target_pointer_width = "64")]
type elf_auxv_t = Elf64_auxv_t;
#[cfg(not(target_pointer_width = "64"))]
type elf_auxv_t = Elf32_auxv_t;

/* These are initialized very early in boot and never changed */
#[no_mangle]
pub static mut elf_aux_platform: *mut core::ffi::c_char = core::ptr::null_mut();
#[no_mangle]
pub static mut elf_aux_hwcap: core::ffi::c_long = 0;

// C __init annotation: this function is intended to run during early boot.
#[no_mangle]
pub unsafe extern "C" fn scan_elf_aux(mut envp: *mut *mut core::ffi::c_char) {
    let mut auxv: *mut elf_auxv_t;

    while !(*envp).is_null() {
        envp = envp.add(1);
    }

    auxv = envp as *mut elf_auxv_t;
    while (*auxv).a_type != AT_NULL {
        match (*auxv).a_type {
            AT_HWCAP => {
                elf_aux_hwcap = (*auxv).a_un.a_val as core::ffi::c_long;
            }
            AT_PLATFORM => {
                /* elf.h removed the pointer elements from
                 * a_un, so we have to use a_val, which is
                 * all that's left.
                 */
                elf_aux_platform = (*auxv).a_un.a_val as core::ffi::c_long
                    as *mut core::ffi::c_char;
            }
            _ => {}
        }
        auxv = auxv.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
