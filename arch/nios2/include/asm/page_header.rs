/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * MMU support based on asm/page.h from mips.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* PAGE_OFFSET -- the first address of the first page of memory. */
pub const PAGE_OFFSET: usize = CONFIG_NIOS2_MEM_BASE + CONFIG_NIOS2_KERNEL_REGION_BASE;

/* This gives the physical RAM offset. */
pub const PHYS_OFFSET: usize = CONFIG_NIOS2_MEM_BASE;

/* Always defined; used by early memory initialization for all memory models. */
pub const ARCH_PFN_OFFSET: usize = (PHYS_OFFSET + PAGE_SIZE - 1) >> PAGE_SHIFT;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    core::ptr::write_bytes(page as *mut u8, 0, PAGE_SIZE);
}

pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, PAGE_SIZE);
}

pub type pgtable_t = *mut page;

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

extern "C" {
    pub static mut memory_start: usize;
    pub static mut memory_end: usize;
    pub static mut memory_size: usize;
    pub static mut mem_map: *mut page;

    pub fn clear_user_page(addr: *mut core::ffi::c_void, vaddr: usize, page: *mut page);
    pub fn copy_user_page(
        vto: *mut core::ffi::c_void,
        vfrom: *mut core::ffi::c_void,
        vaddr: usize,
        to: *mut page,
    );
}

#[inline]
pub const fn __pa(x: usize) -> usize {
    x - PAGE_OFFSET + PHYS_OFFSET
}

#[inline]
pub const fn __va(x: usize) -> *mut core::ffi::c_void {
    (x + PAGE_OFFSET - PHYS_OFFSET) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn page_to_virt(p: *mut page) -> *mut core::ffi::c_void {
    (((p as usize - mem_map as usize) >> core::mem::size_of::<page>()) << PAGE_SHIFT) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn pfn_to_kaddr(pfn: usize) -> *mut core::ffi::c_void {
    __va(pfn << PAGE_SHIFT)
}

/* virt_to_page and virt_addr_valid depend on the generic memory-model APIs. */
pub const VMA_DATA_DEFAULT_FLAGS: usize = VMA_DATA_FLAGS_NON_EXEC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
