// SPDX-License-Identifier: GPL-2.0
/*
 * This code comes from arch/arm64/kernel/crash_dump.c
 * Created by: AKASHI Takahiro <takahiro.akashi@linaro.org>
 * Copyright (C) 2017 Linaro Limited
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

extern "C" {
    fn __pfn_to_phys(pfn: usize) -> usize;
    fn memremap(addr: usize, size: usize, flags: usize) -> *mut c_void;
    fn copy_to_iter(from: *const c_void, count: usize, iter: *mut iov_iter) -> usize;
    fn memunmap(addr: *mut c_void);
}

const PAGE_SIZE: usize = 4096;
const MEMREMAP_WB: usize = 1;
const ENOMEM: isize = 12;

pub unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: usize,
    mut csize: usize,
    offset: usize,
) -> isize {
    let vaddr: *mut c_void;

    if csize == 0 {
        return 0;
    }

    vaddr = memremap(__pfn_to_phys(pfn), PAGE_SIZE, MEMREMAP_WB);
    if vaddr.is_null() {
        return -ENOMEM;
    }

    csize = copy_to_iter(vaddr.add(offset), csize, iter);

    memunmap(vaddr);
    csize as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
