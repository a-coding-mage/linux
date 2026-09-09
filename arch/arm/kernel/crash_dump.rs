// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/kernel/crash_dump.c
 *
 * Copyright (C) 2010 Nokia Corporation.
 * Author: Mika Westerberg
 *
 * This code is taken from arch/x86/kernel/crash_dump_64.c
 *   Created by: Hariprasad Nellitheertha (hari@in.ibm.com)
 *   Copyright (C) IBM Corporation, 2004. All rights reserved
 */

// Declarations supplied by the corresponding Linux kernel headers.
use core::ffi::c_void;

type ssize_t = isize;
type size_t = usize;
type c_ulong = usize;

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

extern "C" {
    fn __pfn_to_phys(pfn: c_ulong) -> usize;
    fn ioremap(phys_addr: usize, size: size_t) -> *mut c_void;
    fn copy_to_iter(from: *const c_void, bytes: size_t, iter: *mut iov_iter) -> size_t;
    fn iounmap(addr: *mut c_void);
}

// PAGE_SIZE and ENOMEM are build-time constants supplied by the kernel headers.
extern "C" {
    static PAGE_SIZE: size_t;
    static ENOMEM: i32;
}

pub unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: c_ulong,
    mut csize: size_t,
    offset: c_ulong,
) -> ssize_t {
    let vaddr: *mut c_void;

    if csize == 0 {
        return 0;
    }

    vaddr = ioremap(__pfn_to_phys(pfn), PAGE_SIZE);
    if vaddr.is_null() {
        return -(ENOMEM as ssize_t);
    }

    csize = copy_to_iter(vaddr.add(offset), csize, iter);

    iounmap(vaddr);
    csize as ssize_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
