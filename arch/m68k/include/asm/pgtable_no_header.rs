/* SPDX-License-Identifier: GPL-2.0 */

// Translation of <asm-generic/pgtable-nopud.h> is supplied by another
// dependency; this header preserves the declarations and macros local to the
// original file.

/*
 * (C) Copyright 2000-2002, Greg Ungerer <gerg@snapgear.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Trivial page table functions.
 */
#[macro_export]
macro_rules! pgd_present {
    ($pgd:expr) => { 1 };
}

#[macro_export]
macro_rules! pgd_none {
    ($pgd:expr) => { 0 };
}

#[macro_export]
macro_rules! pgd_bad {
    ($pgd:expr) => { 0 };
}

#[macro_export]
macro_rules! pgd_clear {
    ($pgdp:expr) => {{
        let _ = &$pgdp;
    }};
}

#[macro_export]
macro_rules! pmd_offset {
    ($a:expr, $b:expr) => {{
        let _ = (&$a, &$b);
        core::ptr::null_mut::<core::ffi::c_void>()
    }};
}

#[macro_export]
macro_rules! PAGE_NONE {
    () => { __pgprot(0) };
}

#[macro_export]
macro_rules! PAGE_SHARED {
    () => { __pgprot(0) };
}

#[macro_export]
macro_rules! PAGE_COPY {
    () => { __pgprot(0) };
}

#[macro_export]
macro_rules! PAGE_READONLY {
    () => { __pgprot(0) };
}

#[macro_export]
macro_rules! PAGE_KERNEL {
    () => { __pgprot(0) };
}

// #define swapper_pg_dir ((pgd_t *) 0)
#[allow(non_upper_case_globals)]
pub const swapper_pg_dir: *mut pgd_t = core::ptr::null_mut();

/*
 * All 32bit addresses are effectively valid for vmalloc...
 * Sort of meaningless for non-VM targets.
 */
pub const VMALLOC_START: u32 = 0;
pub const VMALLOC_END: u32 = 0xffff_ffff;
pub const KMAP_START: u32 = 0;
pub const KMAP_END: u32 = 0xffff_ffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
