/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from powerpc/include/asm/pgtable.h. */

/* Dependencies supplied by the surrounding kernel translation unit. */

pub const PAGE_KERNEL_EXEC: _ = PAGE_KERNEL_X;
pub const PAGE_AGP: _ = PAGE_KERNEL_NC;
pub const HAVE_PAGE_AGP: bool = true;

pub const PFN_PTE_SHIFT: _ = PTE_RPN_SHIFT;

extern "C" {
    pub fn set_ptes(mm: *mut mm_struct, addr: ::core::ffi::c_ulong,
                    ptep: *mut pte_t, pte: pte_t, nr: ::core::ffi::c_uint);
    pub fn set_pte_at_unchecked(mm: *mut mm_struct, addr: ::core::ffi::c_ulong,
                                ptep: *mut pte_t, pte: pte_t);
    pub static mut swapper_pg_dir: [pgd_t; 0];
    pub fn paging_init();
    pub fn poking_init();
    pub static mut ioremap_bot: ::core::ffi::c_ulong;
    pub static protection_map: [pgprot_t; 16];
    pub fn vmalloc_to_phys(vmalloc_addr: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    pub fn pgtable_cache_add(shift: ::core::ffi::c_uint);
    pub fn early_alloc_pgtable(size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub fn early_pte_alloc_kernel(pmdp: *mut pmd_t, va: ::core::ffi::c_ulong) -> *mut pte_t;
    pub fn ptep_set_access_flags(vma: *mut vm_area_struct, address: ::core::ffi::c_ulong,
                                 ptep: *mut pte_t, entry: pte_t, dirty: ::core::ffi::c_int)
                                 -> ::core::ffi::c_int;
    pub fn __phys_mem_access_prot(pfn: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong,
                                  vma_prot: pgprot_t) -> pgprot_t;
    pub fn __update_mmu_cache(vma: *mut vm_area_struct, address: ::core::ffi::c_ulong,
                              ptep: *mut pte_t);
    pub fn mark_initmem_nx();
}

pub const __HAVE_ARCH_PTEP_SET_ACCESS_FLAGS: bool = true;
pub const __HAVE_PHYS_MEM_ACCESS_PROT: bool = true;

#[inline]
pub unsafe fn pte_page(x: pte_t) -> *mut page {
    pfn_to_page(pte_pfn(x))
}

#[inline]
pub unsafe fn pte_pfn(pte: pte_t) -> ::core::ffi::c_ulong {
    (pte_val(pte) & PTE_RPN_MASK) >> PTE_RPN_SHIFT
}

#[inline]
pub unsafe fn pte_pgprot(pte: pte_t) -> pgprot_t {
    __pgprot(pte_val(pte) & !PTE_RPN_MASK)
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn pmd_pgprot(pmd: pmd_t) -> pgprot_t { pte_pgprot(pmd_pte(pmd)) }

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn pud_pgprot(pud: pud_t) -> pgprot_t { pte_pgprot(pud_pte(pud)) }

#[inline]
pub unsafe fn pgprot_nx(prot: pgprot_t) -> pgprot_t {
    pte_pgprot(pte_exprotect(__pte(pgprot_val(prot))))
}

#[inline]
pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> *const ::core::ffi::c_void {
    __va(pmd_val(pmd) & !PMD_MASKED_BITS)
}

#[inline]
pub unsafe fn update_mmu_cache_range(vmf: *mut vm_fault, vma: *mut vm_area_struct,
                                     address: ::core::ffi::c_ulong, ptep: *mut pte_t,
                                     nr: ::core::ffi::c_uint) {
    let _ = (vmf, nr);
    if (mmu_has_feature(MMU_FTR_HPTE_TABLE) && !radix_enabled()) ||
       (IS_ENABLED(CONFIG_PPC_E500) && IS_ENABLED(CONFIG_HUGETLB_PAGE)) {
        __update_mmu_cache(vma, address, ptep);
    }
}

#[inline]
pub unsafe fn update_mmu_cache(vma: *mut vm_area_struct, address: ::core::ffi::c_ulong,
                               ptep: *mut pte_t) {
    update_mmu_cache_range(core::ptr::null_mut(), vma, address, ptep, 1);
}

#[inline]
pub unsafe fn phys_mem_access_prot(_file: *mut file, pfn: ::core::ffi::c_ulong,
                                   size: ::core::ffi::c_ulong, vma_prot: pgprot_t) -> pgprot_t {
    __phys_mem_access_prot(pfn, size, vma_prot)
}

#[inline]
pub unsafe fn pte_frag_get(ctx: *mut mm_context_t) -> *mut ::core::ffi::c_void {
    (*ctx).pte_frag
}

#[inline]
pub unsafe fn pte_frag_set(ctx: *mut mm_context_t, p: *mut ::core::ffi::c_void) {
    (*ctx).pte_frag = p;
}

pub const PTE_FRAG_NR: usize = 1;
pub const PTE_FRAG_SIZE_SHIFT: _ = PAGE_SHIFT;
pub const PTE_FRAG_SIZE: _ = 1usize << PTE_FRAG_SIZE_SHIFT;

#[inline]
pub unsafe fn pmd_pgtable(pmd: pmd_t) -> pgtable_t {
    pmd_page_vaddr(pmd) as pgtable_t
}

#[cfg(target_pointer_width = "64")]
extern "C" {
    pub fn vmemmap_populated(vmemmap_addr: ::core::ffi::c_ulong,
                              vmemmap_map_size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn altmap_cross_boundary(altmap: *mut vmem_altmap, start: ::core::ffi::c_ulong,
                                 page_size: ::core::ffi::c_ulong) -> bool;
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn arch_supports_memmap_on_memory(vmemmap_size: ::core::ffi::c_ulong) -> bool {
    if !radix_enabled() { return false; }
    if IS_ENABLED(CONFIG_PPC_4K_PAGES) { return IS_ALIGNED(vmemmap_size, PMD_SIZE); }
    true
}

#[inline]
pub unsafe fn pmd_user_accessible_page(_mm: *mut mm_struct, _addr: ::core::ffi::c_ulong,
                                       _pmd: pmd_t) -> bool { false }

#[inline]
pub unsafe fn pud_user_accessible_page(_mm: *mut mm_struct, _addr: ::core::ffi::c_ulong,
                                       _pud: pud_t) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
