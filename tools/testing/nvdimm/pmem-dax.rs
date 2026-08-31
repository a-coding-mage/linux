// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014-2016, Intel Corporation.
 */

// C dependencies from:
// #include "test/nfit_test.h"
// #include <linux/blkdev.h>
// #include <linux/dax.h>
// #include <pmem.h>
// #include <nd.h>

use core::ffi::{c_char, c_long, c_ulong, c_void};

pub type pgoff_t = c_ulong;
pub type resource_size_t = c_ulong;
pub type dax_access_mode = core::ffi::c_uint;

pub const EIO: c_long = 5;

#[repr(C)]
pub struct badblocks {
    pub count: c_ulong,
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmem_device {
    pub bb: badblocks,
    pub data_offset: resource_size_t,
    pub phys_addr: resource_size_t,
    pub virt_addr: *mut c_void,
    pub size: resource_size_t,
    pub pfn_pad: resource_size_t,
}

unsafe extern "C" {
    fn is_bad_pmem(bb: *mut badblocks, sector: resource_size_t, len: resource_size_t) -> bool;
    fn get_nfit_res(addr: resource_size_t) -> *mut c_void;
    fn vmalloc_to_page(addr: *mut c_void) -> *mut page;
    fn page_to_pfn(page: *mut page) -> c_ulong;
    fn pr_debug_ratelimited(fmt: *const c_char, ...);
}

#[inline]
fn unlikely(value: bool) -> bool {
    value
}

#[inline]
fn PFN_PHYS(pfn: c_ulong) -> resource_size_t {
    pfn << 12
}

#[inline]
fn PHYS_PFN(phys: resource_size_t) -> c_ulong {
    phys >> 12
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pmem_direct_access(
    pmem: *mut pmem_device,
    pgoff: pgoff_t,
    nr_pages: c_long,
    _mode: dax_access_mode,
    kaddr: *mut *mut c_void,
    pfn: *mut c_ulong,
) -> c_long {
    let offset: resource_size_t = PFN_PHYS(pgoff).wrapping_add((*pmem).data_offset);

    if unlikely(is_bad_pmem(
        &mut (*pmem).bb,
        PFN_PHYS(pgoff) / 512,
        PFN_PHYS(nr_pages as c_ulong),
    )) {
        return -EIO;
    }

    /*
     * Limit dax to a single page at a time given vmalloc()-backed
     * in the nfit_test case.
     */
    if !get_nfit_res((*pmem).phys_addr.wrapping_add(offset)).is_null() {
        let page: *mut page;

        if !kaddr.is_null() {
            *kaddr = ((*pmem).virt_addr as *mut u8).add(offset as usize) as *mut c_void;
        }
        page = vmalloc_to_page(((*pmem).virt_addr as *mut u8).add(offset as usize) as *mut c_void);
        if !pfn.is_null() {
            *pfn = page_to_pfn(page);
        }
        pr_debug_ratelimited(
            c"%s: pmem: %p pgoff: %#lx pfn: %#lx\n".as_ptr(),
            c"__pmem_direct_access".as_ptr(),
            pmem,
            pgoff,
            page_to_pfn(page),
        );

        return 1;
    }

    if !kaddr.is_null() {
        *kaddr = ((*pmem).virt_addr as *mut u8).add(offset as usize) as *mut c_void;
    }
    if !pfn.is_null() {
        *pfn = PHYS_PFN((*pmem).phys_addr.wrapping_add(offset));
    }

    /*
     * If badblocks are present, limit known good range to the
     * requested range.
     */
    if unlikely((*pmem).bb.count != 0) {
        return nr_pages;
    }
    PHYS_PFN(
        (*pmem)
            .size
            .wrapping_sub((*pmem).pfn_pad)
            .wrapping_sub(offset),
    ) as c_long
}
