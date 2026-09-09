/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/page-nommu.h
 *
 *  Copyright (C) 2004 Hyok S. Choi
 */

// The original header guard is omitted in Rust; module inclusion provides the guard.

use core::ffi::c_ulong;

extern "C" {
    pub fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
}

macro_rules! clear_page {
    ($page:expr) => {{
        unsafe {
            memset(
                ($page) as *mut core::ffi::c_void,
                0,
                PAGE_SIZE,
            )
        }
    }};
}

macro_rules! copy_page {
    ($to:expr, $from:expr) => {{
        unsafe {
            memcpy(
                ($to) as *mut core::ffi::c_void,
                ($from) as *const core::ffi::c_void,
                PAGE_SIZE,
            )
        }
    }};
}

macro_rules! copy_user_page {
    ($to:expr, $from:expr, $vaddr:expr, $pg:expr) => {{
        copy_page!($to, $from)
    }};
}

/*
 * These are used to make use of C type-checking..
 */
pub type pte_t = c_ulong;
pub type pmd_t = c_ulong;
pub type pgd_t = [c_ulong; 2];
pub type pgprot_t = c_ulong;

#[inline]
pub fn pte_val(x: pte_t) -> c_ulong {
    x
}

#[inline]
pub fn pmd_val(x: pmd_t) -> c_ulong {
    x
}

#[inline]
pub fn pgd_val(x: &pgd_t) -> c_ulong {
    x[0]
}

#[inline]
pub fn pgprot_val(x: pgprot_t) -> c_ulong {
    x
}

#[inline]
pub fn __pte(x: c_ulong) -> pte_t {
    x
}

#[inline]
pub fn __pmd(x: c_ulong) -> pmd_t {
    x
}

#[inline]
pub fn __pgprot(x: c_ulong) -> pgprot_t {
    x
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
