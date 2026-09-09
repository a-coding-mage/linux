/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependency: <uapi/asm/page.h>

/* CONFIG_ARC_HAS_PAE40 selects the 40-bit physical-memory layout. */
#[cfg(feature = "CONFIG_ARC_HAS_PAE40")]
pub const MAX_POSSIBLE_PHYSMEM_BITS: u32 = 40;
#[cfg(feature = "CONFIG_ARC_HAS_PAE40")]
pub const PAGE_MASK_PHYS: u64 = 0xff00000000u64 | PAGE_MASK as u64;

#[cfg(not(feature = "CONFIG_ARC_HAS_PAE40"))]
pub const MAX_POSSIBLE_PHYSMEM_BITS: u32 = 32;
#[cfg(not(feature = "CONFIG_ARC_HAS_PAE40"))]
pub const PAGE_MASK_PHYS: u64 = PAGE_MASK as u64;

#[repr(C)]
pub struct vm_area_struct;
#[repr(C)]
pub struct page;

pub unsafe fn clear_page(paddr: *mut core::ffi::c_void) {
    core::ptr::write_bytes(paddr.cast::<u8>(), 0, PAGE_SIZE as usize);
}

pub unsafe fn copy_user_page(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    _vaddr: usize,
    _pg: *mut page,
) {
    copy_page(to, from);
}

pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(from.cast::<u8>(), to.cast::<u8>(), PAGE_SIZE as usize);
}

pub const __HAVE_ARCH_COPY_USER_HIGHPAGE: bool = true;

unsafe extern "C" {
    pub fn copy_user_highpage(
        to: *mut page,
        from: *mut page,
        u_vaddr: usize,
        vma: *mut vm_area_struct,
    );
}

pub unsafe fn clear_user_page(to: *mut core::ffi::c_void, _u_vaddr: usize, _page: *mut page) {
    clear_page(to);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t {
    pub pgd: usize,
}

#[inline]
pub const fn pgd_val(x: pgd_t) -> usize { x.pgd }
#[inline]
pub const fn __pgd(x: usize) -> pgd_t { pgd_t { pgd: x } }

#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_3")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pud_t {
    pub pud: usize,
}
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_3")]
#[inline]
pub const fn pud_val(x: pud_t) -> usize { x.pud }
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_3")]
#[inline]
pub const fn __pud(x: usize) -> pud_t { pud_t { pud: x } }

#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_2")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_t {
    pub pmd: usize,
}
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_2")]
#[inline]
pub const fn pmd_val(x: pmd_t) -> usize { x.pmd }
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_2")]
#[inline]
pub const fn __pmd(x: usize) -> pmd_t { pmd_t { pmd: x } }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    #[cfg(feature = "CONFIG_ARC_HAS_PAE40")]
    pub pte: u64,
    #[cfg(not(feature = "CONFIG_ARC_HAS_PAE40"))]
    pub pte: usize,
}

#[inline]
pub const fn pte_val(x: pte_t) -> usize { x.pte as usize }
#[inline]
pub const fn __pte(x: usize) -> pte_t { pte_t { pte: x as _ } }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t {
    pub pgprot: usize,
}

#[inline]
pub const fn pgprot_val(x: pgprot_t) -> usize { x.pgprot }
#[inline]
pub const fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x } }
#[inline]
pub const fn pte_pgprot(x: pte_t) -> pgprot_t { __pgprot(pte_val(x)) }

pub type pgtable_t = *mut page;

#[cfg(feature = "CONFIG_HIGHMEM")]
pub static mut arch_pfn_offset: usize = 0;
#[cfg(feature = "CONFIG_HIGHMEM")]
pub const ARCH_PFN_OFFSET: usize = unsafe { arch_pfn_offset };

#[cfg(feature = "CONFIG_HIGHMEM")]
unsafe extern "C" {
    pub fn pfn_valid(pfn: usize) -> i32;
}

#[cfg(not(feature = "CONFIG_HIGHMEM"))]
pub const ARCH_PFN_OFFSET: usize = virt_to_pfn(CONFIG_LINUX_RAM_BASE as *const core::ffi::c_void);

#[inline]
pub const fn __pa(vaddr: usize) -> usize { vaddr }
#[inline]
pub const fn __va(paddr: usize) -> *mut core::ffi::c_void { paddr as *mut core::ffi::c_void }

#[inline]
pub fn virt_to_pfn(kaddr: *const core::ffi::c_void) -> usize {
    __pa(kaddr as usize) >> PAGE_SHIFT
}

pub unsafe extern "C" {
    pub fn pfn_to_page(pfn: usize) -> *mut page;
}

#[inline]
pub unsafe fn virt_to_page(kaddr: *const core::ffi::c_void) -> *mut page {
    pfn_to_page(virt_to_pfn(kaddr))
}

#[inline]
pub unsafe fn virt_addr_valid(kaddr: *const core::ffi::c_void) -> bool {
    pfn_valid(virt_to_pfn(kaddr)) != 0
}

/* Default Permissions for stack/heaps pages (Non Executable) */
pub const VMA_DATA_DEFAULT_FLAGS: usize = VMA_DATA_FLAGS_NON_EXEC;
pub const WANT_PAGE_VIRTUAL: usize = 1;

// Dependency: <asm-generic/memory_model.h> (page_to_pfn, pfn_to_page)
// Dependency: <asm-generic/getorder.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
