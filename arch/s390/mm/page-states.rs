// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2008
 *
 * Guest page hinting for unused pages.
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Translated from the Linux kernel headers:
// <linux/mm.h>, <asm/page-states.h>, <asm/sections.h>, and <asm/page.h>.

use core::ffi::c_int;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    // __bootdata_preserved(cmma_flag)
    pub static mut cmma_flag: c_int;

    fn page_to_virt(page: *mut page) -> *mut core::ffi::c_void;
    fn __set_page_unused(address: *mut core::ffi::c_void, size: usize);
    fn __set_page_stable_dat(address: *mut core::ffi::c_void, size: usize);
    fn __set_page_stable_nodat(address: *mut core::ffi::c_void, size: usize);
}

// EXPORT_SYMBOL(cmma_flag);

#[no_mangle]
pub unsafe extern "C" fn arch_free_page(page: *mut page, order: c_int) {
    if cmma_flag == 0 {
        return;
    }
    __set_page_unused(page_to_virt(page), 1usize << (order as usize));
}

#[no_mangle]
pub unsafe extern "C" fn arch_alloc_page(page: *mut page, order: c_int) {
    if cmma_flag == 0 {
        return;
    }
    if cmma_flag < 2 {
        __set_page_stable_dat(page_to_virt(page), 1usize << (order as usize));
    } else {
        __set_page_stable_nodat(page_to_virt(page), 1usize << (order as usize));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
