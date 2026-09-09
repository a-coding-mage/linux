/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// PAGE_SHIFT, PTRLOG, PHYS_OFFSET, PAGE_OFFSET, _PFN_MASK, VMA_DATA_FLAGS_TSK_EXEC,
// and the address, PFN, page, and memory-model helpers.

pub const HPAGE_SHIFT: usize = PAGE_SHIFT + PAGE_SHIFT - PTRLOG;
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;

/*
 * It's normally defined only for FLATMEM config but it's
 * used in our early mem init code for all memory models.
 * So always define it.
 */
pub const ARCH_PFN_OFFSET: usize = PFN_UP(PHYS_OFFSET);

pub unsafe extern "C" {
    pub fn clear_page(page: *mut core::ffi::c_void);
    pub fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
    pub static mut shm_align_mask: usize;
    pub fn __virt_addr_valid(kaddr: *mut core::ffi::c_void) -> i32;
}

#[inline]
pub unsafe fn copy_user_page(
    to: *mut core::ffi::c_void,
    from: *mut core::ffi::c_void,
    _vaddr: usize,
    _pg: *mut Page,
) {
    unsafe { copy_page(to, from) }
}

#[repr(C)]
pub struct Page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VmAreaStruct {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PteT {
    pub pte: usize,
}

#[inline]
pub const fn pte_val(x: PteT) -> usize { x.pte }

#[inline]
pub const fn __pte(x: usize) -> PteT { PteT { pte: x } }

pub type PgtableT = *mut Page;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PgdT {
    pub pgd: usize,
}

#[inline]
pub const fn pgd_val(x: PgdT) -> usize { x.pgd }

#[inline]
pub const fn __pgd(x: usize) -> PgdT { PgdT { pgd: x } }

/* Manipulate page protection bits. */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PgprotT {
    pub pgprot: usize,
}

#[inline]
pub const fn pgprot_val(x: PgprotT) -> usize { x.pgprot }

#[inline]
pub const fn __pgprot(x: usize) -> PgprotT { PgprotT { pgprot: x } }

#[inline]
pub const fn pte_pgprot(x: PteT) -> PgprotT { __pgprot(pte_val(x) & !_PFN_MASK) }

#[inline]
pub fn ptep_buddy(x: *mut PteT) -> *mut PteT {
    ((x as usize) ^ core::mem::size_of::<PteT>()) as *mut PteT
}

/* __pa/__va should be used only during mem init. */
#[inline]
pub fn __pa<T>(x: T) -> usize { PHYSADDR(x) }

#[inline]
pub fn __va(x: usize) -> *mut core::ffi::c_void {
    (x + PAGE_OFFSET - PHYS_OFFSET) as *mut core::ffi::c_void
}

#[inline]
pub fn pfn_to_kaddr(pfn: usize) -> *mut core::ffi::c_void { __va(pfn << PAGE_SHIFT) }

#[inline]
pub fn sym_to_pfn<T>(x: *const T) -> usize { __phys_to_pfn(__pa_symbol(x)) }

pub unsafe extern "C" {
    pub fn dmw_virt_to_page(kaddr: usize) -> *mut Page;
    pub fn tlb_virt_to_page(kaddr: usize) -> *mut Page;
}

#[inline]
pub fn pfn_to_phys(pfn: usize) -> usize { __pfn_to_phys(pfn) }

#[inline]
pub fn phys_to_pfn(paddr: usize) -> usize { __phys_to_pfn(paddr) }

// CONFIG_KFENCE selects the alternate page_to_virt/virt_to_page definitions.
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline]
pub fn page_to_virt(page: *mut Page) -> *mut core::ffi::c_void {
    __va(page_to_phys(page))
}

#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline]
pub fn virt_to_page(kaddr: usize) -> *mut Page { phys_to_page(__pa(kaddr)) }

#[cfg(feature = "CONFIG_KFENCE")]
pub const WANT_PAGE_VIRTUAL: bool = true;

#[cfg(feature = "CONFIG_KFENCE")]
#[inline]
pub unsafe fn page_to_virt(page: *mut Page) -> *mut core::ffi::c_void {
    if __kfence_pool.is_null() { __va(page_to_phys(page)) } else { page_address(page) }
}

#[cfg(feature = "CONFIG_KFENCE")]
#[inline]
pub unsafe fn virt_to_page(kaddr: usize) -> *mut Page {
    if kaddr < vm_map_base { dmw_virt_to_page(kaddr) } else { tlb_virt_to_page(kaddr) }
}

#[inline]
pub unsafe fn pfn_to_virt(pfn: usize) -> *mut core::ffi::c_void {
    page_to_virt(pfn_to_page(pfn))
}

#[inline]
pub unsafe fn virt_to_pfn(kaddr: usize) -> usize { page_to_pfn(virt_to_page(kaddr)) }

#[inline]
pub unsafe fn virt_addr_valid(kaddr: *mut core::ffi::c_void) -> i32 {
    __virt_addr_valid(kaddr)
}

pub const VMA_DATA_DEFAULT_FLAGS: usize = VMA_DATA_FLAGS_TSK_EXEC;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
