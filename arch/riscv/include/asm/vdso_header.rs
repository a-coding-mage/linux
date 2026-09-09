/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Limited
 * Copyright (C) 2014 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

/* Translated from the C header guard _ASM_RISCV_VDSO_H. */

/*
 * All systems with an MMU have a VDSO, but systems without an MMU don't
 * support shared libraries and therefore don't have one.
 */

pub const __VDSO_PAGES: usize = 4;

/* The following build-time conditions mirror CONFIG_MMU,
 * CONFIG_RISCV_USER_CFI, CONFIG_COMPAT, and __ASSEMBLER__. */

#[cfg(not(feature = "mmu"))]
unsafe extern "C" {
    pub static mut __vdso_alternatives_start_offset: usize;
    pub static mut __vdso_alternatives_end_offset: usize;
}

#[cfg(not(feature = "riscv_user_cfi"))]
unsafe extern "C" {
    pub static mut __vdso_alternatives_start_cfi_offset: usize;
    pub static mut __vdso_alternatives_end_cfi_offset: usize;
}

/*
 * C token concatenation in VDSO_SYMBOL(base, name) is represented here by
 * passing the selected offset explicitly.  The caller supplies the offset
 * declaration corresponding to `name`.
 */
#[cfg(feature = "riscv_user_cfi")]
#[macro_export]
macro_rules! VDSO_SYMBOL {
    ($base:expr, $offset:expr, $has_zimop:expr) => {{
        if $has_zimop {
            ($base as *mut u8).wrapping_add($offset as usize) as *mut core::ffi::c_void
        } else {
            ($base as *mut u8).wrapping_add($offset as usize) as *mut core::ffi::c_void
        }
    }};
}

#[cfg(not(feature = "riscv_user_cfi"))]
#[macro_export]
macro_rules! VDSO_SYMBOL {
    ($base:expr, $offset:expr) => {{
        ($base as *mut u8).wrapping_add($offset as usize) as *mut core::ffi::c_void
    }};
}

#[cfg(feature = "compat")]
#[macro_export]
macro_rules! COMPAT_VDSO_SYMBOL {
    ($base:expr, $offset:expr) => {{
        ($base as *mut u8).wrapping_add($offset as usize) as *mut core::ffi::c_void
    }};
}

#[cfg(not(feature = "compat"))]
unsafe extern "C" {
    pub static mut compat__vdso_alternatives_start_offset: usize;
    pub static mut compat__vdso_alternatives_end_offset: usize;
}

unsafe extern "C" {
    pub static mut vdso_start: [core::ffi::c_char; 0];
    pub static mut vdso_end: [core::ffi::c_char; 0];
    pub static mut vdso_cfi_start: [core::ffi::c_char; 0];
    pub static mut vdso_cfi_end: [core::ffi::c_char; 0];
    pub static mut compat_vdso_start: [core::ffi::c_char; 0];
    pub static mut compat_vdso_end: [core::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
