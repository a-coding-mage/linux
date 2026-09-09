/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: definitions from <asm/page.h> and <asm/xen/page.h> are
// supplied by other translation units.

/* The hypercall interface supports only 4KB page */
pub const XEN_PAGE_SHIFT: usize = 12;
pub const XEN_PAGE_SIZE: usize = 1usize << XEN_PAGE_SHIFT;
pub const XEN_PAGE_MASK: usize = !(XEN_PAGE_SIZE - 1);

#[macro_export]
macro_rules! xen_offset_in_page {
    ($p:expr) => {
        (($p as usize) & !XEN_PAGE_MASK)
    };
}

/*
 * We assume that PAGE_SIZE is a multiple of XEN_PAGE_SIZE
 * XXX: Add a BUILD_BUG_ON?
 */

#[macro_export]
macro_rules! xen_pfn_to_page {
    ($xen_pfn:expr) => {
        pfn_to_page(($xen_pfn as usize) >> (PAGE_SHIFT - XEN_PAGE_SHIFT))
    };
}

#[macro_export]
macro_rules! page_to_xen_pfn {
    ($page:expr) => {
        page_to_pfn($page) << (PAGE_SHIFT - XEN_PAGE_SHIFT)
    };
}

pub const XEN_PFN_PER_PAGE: usize = PAGE_SIZE / XEN_PAGE_SIZE;

#[macro_export]
macro_rules! XEN_PFN_DOWN {
    ($x:expr) => {
        ($x >> XEN_PAGE_SHIFT)
    };
}

#[macro_export]
macro_rules! XEN_PFN_UP {
    ($x:expr) => {
        (($x + XEN_PAGE_SIZE - 1) >> XEN_PAGE_SHIFT)
    };
}

/* Return the GFN associated to the first 4KB of the page */
pub unsafe fn xen_page_to_gfn(page: *mut crate::page) -> usize {
    pfn_to_gfn(page_to_xen_pfn!(page))
}

#[repr(C)]
pub struct xen_memory_region {
    pub start_pfn: usize,
    pub n_pfns: usize,
}

pub const XEN_EXTRA_MEM_MAX_REGIONS: usize = 128; /* == E820_MAX_ENTRIES_ZEROPAGE */

extern "C" {
    pub static mut xen_extra_mem: [xen_memory_region; XEN_EXTRA_MEM_MAX_REGIONS];
    pub static mut xen_released_pages: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
