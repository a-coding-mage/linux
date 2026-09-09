// SPDX-License-Identifier: GPL-2.0-only
/*
 * Routines for doing kexec-based kdump
 *
 * Copyright (C) 2017 Linaro Limited
 * Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_void};

pub type ssize_t = isize;
pub type size_t = usize;
pub type u64 = core::ffi::c_ulonglong;
pub type phys_addr_t = usize;

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

const PAGE_SIZE: usize = 4096;
const MEMREMAP_WB: u32 = 1;

extern "C" {
    fn __pfn_to_phys(pfn: usize) -> phys_addr_t;
    fn memremap(offset: phys_addr_t, size: usize, flags: u32) -> *mut c_void;
    fn copy_to_iter(from: *const c_void, bytes: usize, iter: *mut iov_iter) -> usize;
    fn memunmap(addr: *mut c_void);
    fn phys_to_virt(address: phys_addr_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
}

const ENOMEM: isize = 12;

pub unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: usize,
    mut csize: usize,
    offset: usize,
) -> ssize_t {
    let vaddr: *mut c_void;

    if csize == 0 {
        return 0;
    }

    vaddr = memremap(__pfn_to_phys(pfn), PAGE_SIZE, MEMREMAP_WB);
    if vaddr.is_null() {
        return -ENOMEM;
    }

    csize = copy_to_iter((vaddr as *mut u8).add(offset) as *const c_void, csize, iter);

    memunmap(vaddr);

    csize as ssize_t
}

/**
 * elfcorehdr_read - read from ELF core header
 * @buf: buffer where the data is placed
 * @count: number of bytes to read
 * @ppos: address in the memory
 *
 * This function reads @count bytes from elf core header which exists
 * on crash dump kernel's memory.
 */
pub unsafe fn elfcorehdr_read(buf: *mut c_char, count: usize, ppos: *mut u64) -> ssize_t {
    memcpy(
        buf as *mut c_void,
        phys_to_virt(*ppos as phys_addr_t) as *const c_void,
        count,
    );
    *ppos += count as u64;

    count as ssize_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
