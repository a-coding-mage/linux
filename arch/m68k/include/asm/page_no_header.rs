/* SPDX-License-Identifier: GPL-2.0 */
// Translated from m68k/include/asm/page_no.h.
// The original declarations are excluded for assembler builds.

use core::ffi::c_void;

extern "C" {
    pub static mut memory_start: c_ulong;
    pub static mut memory_end: c_ulong;

    // Supplied by the surrounding kernel environment.
    pub static mut mem_map: *mut Page;
    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn vma_alloc_folio(gfp: c_ulong, order: c_ulong, vma: *mut Vma, vaddr: c_ulong) -> *mut Folio;
}

// Build-time constants and types supplied by the surrounding kernel headers.
// Their names and use are preserved from the C header.
type c_ulong = usize;
type c_int = i32;

#[repr(C)]
pub struct Page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Vma {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Folio {
    _private: [u8; 0],
}

extern "C" {
    pub static PAGE_SIZE: usize;
    pub static PAGE_SHIFT: usize;
    pub static PAGE_OFFSET: c_ulong;
    pub static PAGE_OFFSET_RAW: c_ulong;
    pub static GFP_HIGHUSER_MOVABLE: c_ulong;
    pub static __GFP_ZERO: c_ulong;
}

#[inline]
pub unsafe fn clear_page(page: *mut c_void) {
    memset(page, 0, PAGE_SIZE);
}

#[inline]
pub unsafe fn copy_page(to: *mut c_void, from: *const c_void) {
    memcpy(to, from, PAGE_SIZE);
}

#[inline]
pub unsafe fn copy_user_page(
    to: *mut c_void,
    from: *const c_void,
    _vaddr: c_ulong,
    _pg: *mut Page,
) {
    copy_page(to, from);
}

#[inline]
pub unsafe fn vma_alloc_zeroed_movable_folio(vma: *mut Vma, vaddr: c_ulong) -> *mut Folio {
    vma_alloc_folio(GFP_HIGHUSER_MOVABLE | __GFP_ZERO, 0, vma, vaddr)
}

#[inline]
pub unsafe fn __pa(vaddr: *const c_void) -> c_ulong {
    vaddr as c_ulong
}

#[inline]
pub unsafe fn __va(paddr: c_ulong) -> *mut c_void {
    paddr as *mut c_void
}

#[inline]
pub unsafe fn virt_to_pfn(kaddr: *const c_void) -> c_ulong {
    __pa(kaddr) >> PAGE_SHIFT
}

#[inline]
pub unsafe fn pfn_to_virt(pfn: c_ulong) -> *mut c_void {
    __va(pfn << PAGE_SHIFT)
}

#[inline]
pub unsafe fn virt_to_page(addr: *const c_void) -> *mut Page {
    mem_map.add((((addr as c_ulong).wrapping_sub(PAGE_OFFSET)) >> PAGE_SHIFT) as usize)
}

#[inline]
pub unsafe fn page_to_virt(page: *const Page) -> *mut c_void {
    __va((((page.offset_from(mem_map) as c_ulong) << PAGE_SHIFT) + PAGE_OFFSET) as c_ulong)
}

#[inline]
pub unsafe fn virt_addr_valid(kaddr: *const c_void) -> bool {
    (kaddr as c_ulong >= PAGE_OFFSET) && (kaddr as c_ulong < memory_end)
}

// ARCH_PFN_OFFSET PHYS_PFN(PAGE_OFFSET_RAW)
pub const ARCH_PFN_OFFSET: c_ulong = phys_pfn(PAGE_OFFSET_RAW);

// External macro dependency preserved as a local declaration-level mapping.
#[inline]
const fn phys_pfn(value: c_ulong) -> c_ulong {
    value
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
