// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016, Intel Corporation.
 */
// C dependencies: "test/nfit_test.h", <linux/mm.h>,
// "../../../drivers/dax/dax-private.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;

pub type phys_addr_t = u64;
pub type pgoff_t = u64;

#[repr(C)]
pub struct range {
    pub start: phys_addr_t,
    pub end: phys_addr_t,
}

#[repr(C)]
pub struct dev_dax_range {
    pub pgoff: pgoff_t,
    pub range: range,
}

#[repr(C)]
pub struct dev_dax_region {
    pub align: u64,
}

#[repr(C)]
pub struct dev_dax {
    pub nr_range: i32,
    pub ranges: *mut dev_dax_range,
    pub region: *mut dev_dax_region,
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

unsafe extern "C" {
    static PAGE_SIZE: u64;
    static PAGE_SHIFT: u32;

    fn get_nfit_res(addr: phys_addr_t) -> *mut c_void;
    fn vmalloc_to_page(addr: *mut c_void) -> *mut page;
    fn page_to_pfn(page: *mut page) -> u64;
}

#[inline]
unsafe fn PFN_PHYS(pfn: u64) -> phys_addr_t {
    pfn << unsafe { PAGE_SHIFT }
}

#[inline]
unsafe fn PHYS_PFN(phys: phys_addr_t) -> u64 {
    phys >> unsafe { PAGE_SHIFT }
}

#[inline]
unsafe fn range_len(range: *mut range) -> u64 {
    unsafe {
        (*range)
            .end
            .wrapping_sub((*range).start)
            .wrapping_add(1)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dax_pgoff_to_phys(
    dev_dax: *mut dev_dax,
    pgoff: pgoff_t,
    size: u64,
) -> phys_addr_t {
    let mut i: i32;

    i = 0;
    while i < unsafe { (*dev_dax).nr_range } {
        let dax_range: *mut dev_dax_range =
            unsafe { (*dev_dax).ranges.offset(i as isize) };
        let range: *mut range = unsafe { &mut (*dax_range).range };
        let pgoff_end: u64;
        let addr: phys_addr_t;

        pgoff_end = unsafe {
            (*dax_range)
                .pgoff
                .wrapping_add(PHYS_PFN(range_len(range)))
                .wrapping_sub(1)
        };
        if pgoff < unsafe { (*dax_range).pgoff } || pgoff > pgoff_end {
            i += 1;
            continue;
        }
        addr = unsafe {
            PFN_PHYS(pgoff.wrapping_sub((*dax_range).pgoff)).wrapping_add((*range).start)
        };
        if addr.wrapping_add(size).wrapping_sub(1) <= unsafe { (*range).end } {
            if unsafe { !get_nfit_res(addr).is_null() } {
                let page: *mut page;

                if unsafe { (*(*dev_dax).region).align > PAGE_SIZE } {
                    return phys_addr_t::MAX;
                }

                page = unsafe { vmalloc_to_page(addr as *mut c_void) };
                return unsafe { PFN_PHYS(page_to_pfn(page)) };
            }
            return addr;
        }
        break;
    }
    phys_addr_t::MAX
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
