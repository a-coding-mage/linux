// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SPU local store allocation routines
 *
 * Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
 */

// DEBUG is intentionally undefined, as in the original source.
// Dependencies corresponding to the Linux and SPU headers are supplied externally.

use core::ffi::c_void;

extern "C" {
    fn vzalloc(size: usize) -> *mut c_void;
    fn vmalloc_to_page(addr: *mut u8) -> *mut crate::page;
    fn SetPageReserved(page: *mut crate::page);
    fn ClearPageReserved(page: *mut crate::page);
    fn vfree(addr: *mut c_void);
}

pub unsafe fn spu_alloc_lscsa(csa: *mut crate::spu_state) -> i32 {
    let mut lscsa: *mut crate::spu_lscsa;
    let mut p: *mut u8;

    lscsa = vzalloc(core::mem::size_of::<crate::spu_lscsa>()) as *mut crate::spu_lscsa;
    if lscsa.is_null() {
        return -12;
    }
    (*csa).lscsa = lscsa;

    /* Set LS pages reserved to allow for user-space mapping. */
    p = (*lscsa).ls.as_mut_ptr();
    let end = p.add(crate::LS_SIZE);
    while p < end {
        SetPageReserved(vmalloc_to_page(p));
        p = p.add(crate::PAGE_SIZE);
    }

    0
}

pub unsafe fn spu_free_lscsa(csa: *mut crate::spu_state) {
    /* Clear reserved bit before vfree. */
    let mut p: *mut u8;

    if (*csa).lscsa.is_null() {
        return;
    }

    p = (*csa).lscsa.as_ref().unwrap().ls.as_ptr() as *mut u8;
    let end = p.add(crate::LS_SIZE);
    while p < end {
        ClearPageReserved(vmalloc_to_page(p));
        p = p.add(crate::PAGE_SIZE);
    }

    vfree((*csa).lscsa as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
