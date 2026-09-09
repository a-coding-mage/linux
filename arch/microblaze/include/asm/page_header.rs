/* SPDX-License-Identifier: GPL-2.0 */
/*
 * VM ops
 *
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 * Changes for MMU support:
 *    Copyright (C) 2007 Xilinx, Inc.  All rights reserved.
 */

/* External configuration and memory-model definitions are supplied elsewhere. */

#[cfg(feature = "kernel")]
pub const LOAD_OFFSET: usize = (CONFIG_KERNEL_START - CONFIG_KERNEL_BASE_ADDR) as usize;

#[cfg(feature = "kernel")]
pub const PTE_SHIFT: usize = PAGE_SHIFT - 2; /* 1024 ptes per page */

#[cfg(feature = "kernel")]
pub const PAGE_OFFSET: usize = CONFIG_KERNEL_START as usize;

#[cfg(feature = "kernel")]
pub type pte_basic_t = usize;

#[cfg(feature = "kernel")]
pub const PTE_FMT: &str = "%.8lx";

#[cfg(feature = "kernel")]
pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, PAGE_SIZE);
}

#[cfg(feature = "kernel")]
pub unsafe fn clear_page(pgaddr: *mut core::ffi::c_void) {
    core::ptr::write_bytes(pgaddr, 0, PAGE_SIZE);
}

#[cfg(feature = "kernel")]
pub unsafe fn copy_user_page(
    vto: *mut core::ffi::c_void,
    vfrom: *const core::ffi::c_void,
    _vaddr: usize,
    _topg: *mut page,
) {
    core::ptr::copy_nonoverlapping(vfrom as *const u8, vto as *mut u8, PAGE_SIZE);
}

#[cfg(feature = "kernel")]
pub enum page {}

#[cfg(feature = "kernel")]
pub type pgtable_t = *mut page;

#[cfg(feature = "kernel")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t { pub pte: usize }

#[cfg(feature = "kernel")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t { pub pgprot: usize }

#[cfg(feature = "kernel")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t { pub pgd: usize }

#[cfg(feature = "kernel")]
pub const fn pte_val(x: pte_t) -> usize { x.pte }

#[cfg(feature = "kernel")]
pub const fn pgprot_val(x: pgprot_t) -> usize { x.pgprot }

#[cfg(feature = "kernel")]
pub const fn pgd_val(x: pgd_t) -> usize { x.pgd }

#[cfg(feature = "kernel")]
pub const fn __pte(x: usize) -> pte_t { pte_t { pte: x } }

#[cfg(feature = "kernel")]
pub const fn __pgd(x: usize) -> pgd_t { pgd_t { pgd: x } }

#[cfg(feature = "kernel")]
pub const fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x } }

#[cfg(feature = "kernel")]
extern "C" {
    pub static mut max_low_pfn: usize;
    pub static mut min_low_pfn: usize;
    pub static mut max_pfn: usize;
    pub static mut memory_start: usize;
    pub static mut memory_size: usize;
    pub static mut lowmem_size: usize;
    pub static mut kernel_tlb: usize;
    pub fn page_is_ram(pfn: usize) -> core::ffi::c_int;
}

#[cfg(feature = "kernel")]
pub const fn phys_to_pfn(phys: usize) -> usize { phys >> PAGE_SHIFT }

#[cfg(feature = "kernel")]
pub const fn pfn_to_phys(pfn: usize) -> usize { pfn << PAGE_SHIFT }

#[cfg(feature = "kernel")]
pub unsafe fn virt_to_page(kaddr: usize) -> *mut page {
    pfn_to_page(__pa(kaddr) >> PAGE_SHIFT)
}

#[cfg(feature = "kernel")]
pub unsafe fn page_to_virt(p: *mut page) -> *mut core::ffi::c_void {
    __va(page_to_pfn(p) << PAGE_SHIFT)
}

#[cfg(feature = "kernel")]
pub const ARCH_PFN_OFFSET: usize = memory_start >> PAGE_SHIFT;

#[cfg(feature = "kernel")]
pub const fn __virt_to_phys(addr: usize) -> usize {
    addr + CONFIG_KERNEL_BASE_ADDR - CONFIG_KERNEL_START
}

#[cfg(feature = "kernel")]
pub const fn __phys_to_virt(addr: usize) -> usize {
    addr + CONFIG_KERNEL_START - CONFIG_KERNEL_BASE_ADDR
}

#[cfg(feature = "kernel")]
pub unsafe fn __pa(x: usize) -> usize { __virt_to_phys(x) }

#[cfg(feature = "kernel")]
pub unsafe fn __va(x: usize) -> *mut core::ffi::c_void {
    __phys_to_virt(x) as *mut core::ffi::c_void
}

#[cfg(feature = "kernel")]
pub unsafe fn virt_to_pfn(vaddr: *const core::ffi::c_void) -> usize {
    phys_to_pfn(__pa(vaddr as usize))
}

#[cfg(feature = "kernel")]
pub unsafe fn pfn_to_virt(pfn: usize) -> *const core::ffi::c_void {
    __va(pfn_to_phys(pfn)) as *const core::ffi::c_void
}

#[cfg(feature = "kernel")]
pub unsafe fn virt_addr_valid(vaddr: *const core::ffi::c_void) -> bool {
    pfn_valid(virt_to_pfn(vaddr))
}

#[cfg(feature = "kernel")]
pub const fn TOPHYS(addr: usize) -> usize { __virt_to_phys(addr) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
