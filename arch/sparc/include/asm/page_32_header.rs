/* SPDX-License-Identifier: GPL-2.0 */
/*
 * page.h: Various defines and such for MMU operations on the Sparc for
 *         the Linux kernel.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

/* C dependencies: linux/const.h, vdso/page.h, and asm-generic memory helpers. */

#[inline]
pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    core::ptr::write_bytes(page as *mut u8, 0, PAGE_SIZE);
}

#[inline]
pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, PAGE_SIZE);
}

#[inline]
pub unsafe fn clear_user_page(
    addr: *mut core::ffi::c_void,
    vaddr: usize,
    page: *mut core::ffi::c_void,
) {
    clear_page(addr);
    sparc_flush_page_to_ram(page);
    let _ = vaddr;
}

#[inline]
pub unsafe fn copy_user_page(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    vaddr: usize,
    page: *mut core::ffi::c_void,
) {
    copy_page(to, from);
    sparc_flush_page_to_ram(page);
    let _ = vaddr;
}

/* The following structure holds the physical memory configuration of the
 * machine. This is filled in prom_meminit() and later used by mem_init() to
 * set up mem_map[]. The entry after the last valid one has num_bytes == 0.
 */
#[repr(C)]
pub struct sparc_phys_banks {
    pub base_addr: usize,
    pub num_bytes: usize,
}

pub const SPARC_PHYS_BANKS: usize = 32;

extern "C" {
    pub static mut sp_banks: [sparc_phys_banks; SPARC_PHYS_BANKS + 1];
    pub fn sparc_flush_page_to_ram(page: *mut core::ffi::c_void);
}

/* Passing structs on the Sparc slows us down tremendously. */

#[cfg(feature = "strict_mm_typechecks")]
mod strict_mm_typechecks {
    #[repr(transparent)]
    pub struct pte_t { pub pte: usize }
    #[repr(transparent)]
    pub struct iopte_t { pub iopte: usize }
    #[repr(transparent)]
    pub struct pmd_t { pub pmd: usize }
    #[repr(transparent)]
    pub struct pgd_t { pub pgd: usize }
    #[repr(transparent)]
    pub struct ctxd_t { pub ctxd: usize }
    #[repr(transparent)]
    pub struct pgprot_t { pub pgprot: usize }
    #[repr(transparent)]
    pub struct iopgprot_t { pub iopgprot: usize }
}

#[cfg(feature = "strict_mm_typechecks")]
pub use strict_mm_typechecks::*;

#[cfg(not(feature = "strict_mm_typechecks"))]
pub type pte_t = usize;
#[cfg(not(feature = "strict_mm_typechecks"))]
pub type iopte_t = usize;
#[cfg(not(feature = "strict_mm_typechecks"))]
pub type pmd_t = usize;
#[cfg(not(feature = "strict_mm_typechecks"))]
pub type pgd_t = usize;
#[cfg(not(feature = "strict_mm_typechecks"))]
pub type ctxd_t = usize;
#[cfg(not(feature = "strict_mm_typechecks"))]
pub type pgprot_t = usize;
#[cfg(not(feature = "strict_mm_typechecks"))]
pub type iopgprot_t = usize;

#[cfg(feature = "strict_mm_typechecks")]
pub type pgtable_t = *mut pte_t;
#[cfg(not(feature = "strict_mm_typechecks"))]
pub type pgtable_t = *mut pte_t;

pub const TASK_UNMAPPED_BASE: usize = 0x50000000;
pub const PAGE_OFFSET: usize = 0xf0000000;

extern "C" {
    pub static mut phys_base: usize;
    pub static mut pfn_base: usize;
}

#[inline]
pub unsafe fn __pa<T>(x: *const T) -> usize {
    x as usize - PAGE_OFFSET + phys_base
}

#[inline]
pub unsafe fn __va(x: usize) -> *mut core::ffi::c_void {
    (x - phys_base + PAGE_OFFSET) as *mut core::ffi::c_void
}

pub use __pa as virt_to_phys;
pub use __va as phys_to_virt;

pub const ARCH_PFN_OFFSET: usize = unsafe { pfn_base };

/* pfn_to_page, PAGE_SHIFT, and max_mapnr are supplied by dependencies. */
#[inline]
pub unsafe fn virt_to_page<T>(kaddr: *const T) -> *mut core::ffi::c_void {
    pfn_to_page(__pa(kaddr) >> PAGE_SHIFT)
}

#[inline]
pub unsafe fn virt_addr_valid<T>(kaddr: *const T) -> bool {
    (((kaddr as usize - PAGE_OFFSET) >> PAGE_SHIFT) < max_mapnr)
}

extern "C" {
    pub static PAGE_SIZE: usize;
    pub static PAGE_SHIFT: usize;
    pub static max_mapnr: usize;
    pub fn pfn_to_page(pfn: usize) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
