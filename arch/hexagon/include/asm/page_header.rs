/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Page management definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

/* C header dependency: <linux/const.h> */

/* Build-time page-size selection is preserved from the original header. */
#[cfg(CONFIG_PAGE_SIZE_4KB)]
pub const HEXAGON_L1_PTE_SIZE: usize = __HVM_PDE_S_4KB;
#[cfg(CONFIG_PAGE_SIZE_16KB)]
pub const HEXAGON_L1_PTE_SIZE: usize = __HVM_PDE_S_16KB;
#[cfg(CONFIG_PAGE_SIZE_64KB)]
pub const HEXAGON_L1_PTE_SIZE: usize = __HVM_PDE_S_64KB;
#[cfg(CONFIG_PAGE_SIZE_256KB)]
pub const HEXAGON_L1_PTE_SIZE: usize = __HVM_PDE_S_256KB;
#[cfg(CONFIG_PAGE_SIZE_1MB)]
pub const HEXAGON_L1_PTE_SIZE: usize = __HVM_PDE_S_1MB;

/* C header dependency: <vdso/page.h> */

#[cfg(CONFIG_HUGETLB_PAGE)]
pub const HPAGE_SHIFT: usize = 22;
#[cfg(CONFIG_HUGETLB_PAGE)]
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
#[cfg(CONFIG_HUGETLB_PAGE)]
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
#[cfg(CONFIG_HUGETLB_PAGE)]
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;
#[cfg(CONFIG_HUGETLB_PAGE)]
pub const HVM_HUGEPAGE_SIZE: usize = 0x5;

/* C header dependency: <linux/pfn.h> */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    pub pte: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t {
    pub pgd: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t {
    pub pgprot: usize,
}

pub type pgtable_t = *mut page;

pub struct page;

#[inline]
pub const fn pte_val(x: pte_t) -> usize { x.pte }
#[inline]
pub const fn pgd_val(x: pgd_t) -> usize { x.pgd }
#[inline]
pub const fn pgprot_val(x: pgprot_t) -> usize { x.pgprot }
#[inline]
pub const fn __pte(x: usize) -> pte_t { pte_t { pte: x } }
#[inline]
pub const fn __pgd(x: usize) -> pgd_t { pgd_t { pgd: x } }
#[inline]
pub const fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x } }

/* C header dependency: <asm/mem-layout.h> */
#[inline]
pub fn __pa<T>(x: *const T) -> usize {
    x as usize - PAGE_OFFSET + PHYS_OFFSET
}

#[inline]
pub fn __va<T>(x: usize) -> *mut T {
    (x - PHYS_OFFSET + PAGE_OFFSET) as *mut T
}

#[inline]
pub fn virt_to_page<T>(kaddr: *const T) -> *mut page {
    pfn_to_page(PFN_DOWN(__pa(kaddr)))
}

pub const VMA_DATA_DEFAULT_FLAGS: usize = VMA_DATA_FLAGS_NON_EXEC;

#[inline]
pub fn virt_addr_valid<T>(kaddr: *const T) -> bool {
    pfn_valid(__pa(kaddr) >> PAGE_SHIFT)
}

#[inline]
pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    /* Hexagon inline assembly retained verbatim in the source-level translation. */
    core::arch::asm!(
        "loop0(1f,{1});",
        "1: {{ dczeroa({0});",
        "{0} = add({0},#32); }}:endloop0",
        inout(reg) page => page,
        in(reg) PAGE_SIZE / 32,
        options(nostack)
    );
}

#[inline]
pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void) {
    memcpy(to, from, PAGE_SIZE);
}

#[inline]
pub unsafe fn copy_user_page(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    _vaddr: usize,
    _pg: *mut page,
) {
    copy_page(to, from);
}

#[inline]
pub fn virt_to_pfn<T>(kaddr: *const T) -> usize {
    __pa(kaddr) >> PAGE_SHIFT
}

#[inline]
pub fn page_to_virt(page: *mut page) -> *mut core::ffi::c_void {
    __va(page_to_phys(page))
}

/* C header dependencies: <asm/mem-layout.h>, <asm-generic/memory_model.h>,
 * and <asm-generic/getorder.h>. */

/* External declarations supplied by other translated dependencies. */
extern "C" {
    static PAGE_OFFSET: usize;
    static PHYS_OFFSET: usize;
    static PAGE_SIZE: usize;
    static PAGE_SHIFT: usize;
    static VMA_DATA_FLAGS_NON_EXEC: usize;
    fn PFN_DOWN(x: usize) -> usize;
    fn pfn_to_page(x: usize) -> *mut page;
    fn pfn_valid(x: usize) -> bool;
    fn memcpy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize);
    fn page_to_phys(page: *mut page) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
