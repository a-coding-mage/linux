// SPDX-License-Identifier: GPL-2.0+
/*
 * Secure VM platform
 *
 * Copyright 2018 IBM Corporation
 * Author: Anshuman Khandual <khandual@linux.vnet.ibm.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    fn is_secure_guest() -> bool;
    fn cc_platform_has(attr: u32) -> bool;
    fn uv_unshare_page(pfn: usize, numpages: i32);
    fn uv_share_page(pfn: usize, numpages: i32);
    fn swiotlb_update_mem_attributes();
    fn pfn_to_page(pfn: usize) -> *mut page;
    fn __pa(addr: *mut core::ffi::c_void) -> usize;
    fn WARN_ON(condition: bool) -> bool;
}

extern "C" {
    static mut ppc_swiotlb_enable: i32;
    static mut ppc_swiotlb_flags: u32;
}

extern "C" {
    static CC_ATTR_MEM_ENCRYPT: u32;
    static SWIOTLB_ANY: u32;
    static PAGE_SIZE: usize;
    static DISPATCH_LOG_BYTES: usize;
    static CONFIG_NR_CPUS: usize;
    static EINVAL: i32;
}

// machine_early_initcall(pseries, init_svm);
unsafe fn init_svm() -> i32 {
    if !is_secure_guest() {
        return 0;
    }

    /* Don't release the SWIOTLB buffer. */
    ppc_swiotlb_enable = 1;

    /*
     * Since the guest memory is inaccessible to the host, devices always
     * need to use the SWIOTLB buffer for DMA even if dma_capable() says
     * otherwise.
     */
    ppc_swiotlb_flags |= SWIOTLB_ANY;

    /* Share the SWIOTLB buffer with the host. */
    swiotlb_update_mem_attributes();

    0
}

pub unsafe fn set_memory_encrypted(addr: usize, numpages: i32) -> i32 {
    if !cc_platform_has(CC_ATTR_MEM_ENCRYPT) {
        return 0;
    }

    if addr % PAGE_SIZE != 0 {
        return -EINVAL;
    }

    uv_unshare_page(__pa(addr as *mut core::ffi::c_void) / PAGE_SIZE, numpages);

    0
}

pub unsafe fn set_memory_decrypted(addr: usize, numpages: i32) -> i32 {
    if !cc_platform_has(CC_ATTR_MEM_ENCRYPT) {
        return 0;
    }

    if addr % PAGE_SIZE != 0 {
        return -EINVAL;
    }

    uv_share_page(__pa(addr as *mut core::ffi::c_void) / PAGE_SIZE, numpages);

    0
}

/* There's one dispatch log per CPU. */
const NR_DTL_PAGE: usize = DISPATCH_LOG_BYTES * CONFIG_NR_CPUS / PAGE_SIZE;

static mut dtl_page_store: [*mut page; NR_DTL_PAGE] = [core::ptr::null_mut(); NR_DTL_PAGE];
static mut dtl_nr_pages: isize = 0;

unsafe fn is_dtl_page_shared(page: *mut page) -> bool {
    let mut i: isize = 0;

    while i < dtl_nr_pages {
        if dtl_page_store[i as usize] == page {
            return true;
        }
        i += 1;
    }

    false
}

pub unsafe fn dtl_cache_ctor(addr: *mut core::ffi::c_void) {
    let pfn: usize = __pa(addr) / PAGE_SIZE;
    let page: *mut page = pfn_to_page(pfn);

    if !is_dtl_page_shared(page) {
        dtl_page_store[dtl_nr_pages as usize] = page;
        dtl_nr_pages += 1;
        WARN_ON(dtl_nr_pages >= NR_DTL_PAGE as isize);
        uv_share_page(pfn, 1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
