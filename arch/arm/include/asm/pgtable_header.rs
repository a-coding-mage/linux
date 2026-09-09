/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm/include/asm/pgtable.h. */

/* C includes and configuration guards are supplied by the surrounding build. */

/* CONFIG_MMU branch; the CONFIG_MMU-disabled branch includes asm/pgtable-nommu.h. */

pub const VMALLOC_OFFSET: usize = 8 * 1024 * 1024;
pub const VMALLOC_END: u32 = 0xff800000;
pub const LIBRARY_TEXT_START: usize = 0x0c000000;

#[cfg(not(assembly))]
extern "C" {
    pub fn __pte_error(file: *const core::ffi::c_char, line: i32, pte: pte_t);
    pub fn __pmd_error(file: *const core::ffi::c_char, line: i32, pmd: pmd_t);
    pub fn __pgd_error(file: *const core::ffi::c_char, line: i32, pgd: pgd_t);
    pub static mut high_memory: *mut core::ffi::c_void;
    pub static mut pgprot_user: pgprot_t;
    pub static mut pgprot_kernel: pgprot_t;
    pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD];
    pub fn mm_tlb_flush_pending(mm: *mut mm_struct) -> bool;
    pub fn __sync_icache_dcache(pteval: pte_t);
    pub fn set_ptes(mm: *mut mm_struct, addr: u64, ptep: *mut pte_t, pteval: pte_t, nr: u32);
}

#[cfg(not(assembly))]
pub const FIRST_USER_ADDRESS: usize = PAGE_SIZE * 2;

#[cfg(all(not(assembly), arm_lpae))]
pub const USER_PGTABLES_CEILING: usize = TASK_SIZE;

pub const _L_PTE_DEFAULT: u64 = L_PTE_PRESENT | L_PTE_YOUNG;

#[inline]
pub fn VMALLOC_START() -> usize {
    ((high_memory as usize + VMALLOC_OFFSET) & !(VMALLOC_OFFSET - 1))
}

#[inline] pub fn _MOD_PROT(p: pgprot_t, b: u64) -> pgprot_t { __pgprot(pgprot_val(p) | b) }
#[inline] pub fn PAGE_NONE() -> pgprot_t { _MOD_PROT(unsafe { pgprot_user }, L_PTE_XN | L_PTE_RDONLY | L_PTE_NONE) }
#[inline] pub fn PAGE_SHARED() -> pgprot_t { _MOD_PROT(unsafe { pgprot_user }, L_PTE_USER | L_PTE_XN) }
#[inline] pub fn PAGE_SHARED_EXEC() -> pgprot_t { _MOD_PROT(unsafe { pgprot_user }, L_PTE_USER) }
#[inline] pub fn PAGE_COPY() -> pgprot_t { _MOD_PROT(unsafe { pgprot_user }, L_PTE_USER | L_PTE_RDONLY | L_PTE_XN) }
#[inline] pub fn PAGE_COPY_EXEC() -> pgprot_t { _MOD_PROT(unsafe { pgprot_user }, L_PTE_USER | L_PTE_RDONLY) }
#[inline] pub fn PAGE_READONLY() -> pgprot_t { _MOD_PROT(unsafe { pgprot_user }, L_PTE_USER | L_PTE_RDONLY | L_PTE_XN) }
#[inline] pub fn PAGE_READONLY_EXEC() -> pgprot_t { _MOD_PROT(unsafe { pgprot_user }, L_PTE_USER | L_PTE_RDONLY) }
#[inline] pub fn PAGE_KERNEL() -> pgprot_t { _MOD_PROT(unsafe { pgprot_kernel }, L_PTE_XN) }
#[inline] pub fn PAGE_KERNEL_EXEC() -> pgprot_t { unsafe { pgprot_kernel } }

#[inline] pub fn __PAGE_NONE() -> pgprot_t { __pgprot(_L_PTE_DEFAULT | L_PTE_RDONLY | L_PTE_XN | L_PTE_NONE) }
#[inline] pub fn __PAGE_SHARED() -> pgprot_t { __pgprot(_L_PTE_DEFAULT | L_PTE_USER | L_PTE_XN) }
#[inline] pub fn __PAGE_SHARED_EXEC() -> pgprot_t { __pgprot(_L_PTE_DEFAULT | L_PTE_USER) }
#[inline] pub fn __PAGE_COPY() -> pgprot_t { __pgprot(_L_PTE_DEFAULT | L_PTE_USER | L_PTE_RDONLY | L_PTE_XN) }
#[inline] pub fn __PAGE_COPY_EXEC() -> pgprot_t { __pgprot(_L_PTE_DEFAULT | L_PTE_USER | L_PTE_RDONLY) }
#[inline] pub fn __PAGE_READONLY() -> pgprot_t { __pgprot(_L_PTE_DEFAULT | L_PTE_USER | L_PTE_RDONLY | L_PTE_XN) }
#[inline] pub fn __PAGE_READONLY_EXEC() -> pgprot_t { __pgprot(_L_PTE_DEFAULT | L_PTE_USER | L_PTE_RDONLY) }

#[inline] pub fn __pgprot_modify(prot: pgprot_t, mask: u64, bits: u64) -> pgprot_t { __pgprot((pgprot_val(prot) & !mask) | bits) }
#[inline] pub fn pgprot_noncached(prot: pgprot_t) -> pgprot_t { __pgprot_modify(prot, L_PTE_MT_MASK, L_PTE_MT_UNCACHED) }
#[inline] pub fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t { __pgprot_modify(prot, L_PTE_MT_MASK, L_PTE_MT_BUFFERABLE) }
#[inline] pub fn pgprot_stronglyordered(prot: pgprot_t) -> pgprot_t { __pgprot_modify(prot, L_PTE_MT_MASK, L_PTE_MT_UNCACHED) }
#[inline] pub fn pgprot_device(prot: pgprot_t) -> pgprot_t { __pgprot_modify(prot, L_PTE_MT_MASK, L_PTE_MT_DEV_SHARED | L_PTE_SHARED | L_PTE_DIRTY | L_PTE_XN) }

#[cfg(arm_dma_mem_bufferable)]
#[inline] pub fn pgprot_dmacoherent(prot: pgprot_t) -> pgprot_t { __pgprot_modify(prot, L_PTE_MT_MASK, L_PTE_MT_BUFFERABLE | L_PTE_XN) }
#[cfg(not(arm_dma_mem_bufferable))]
#[inline] pub fn pgprot_dmacoherent(prot: pgprot_t) -> pgprot_t { __pgprot_modify(prot, L_PTE_MT_MASK, L_PTE_MT_UNCACHED | L_PTE_XN) }

#[inline] pub fn pgdp_get(pgdp: *const pgd_t) -> pgd_t { unsafe { core::ptr::read_volatile(pgdp) } }
#[inline] pub fn pud_page(pud: pud_t) -> pmd_t { pmd_page(__pmd(pud_val(pud))) }
#[inline] pub fn pud_write(pud: pud_t) -> bool { pmd_write(__pmd(pud_val(pud))) }
#[inline] pub fn pmd_none(pmd: pmd_t) -> bool { pmd_val(pmd) == 0 }
#[inline] pub fn pmd_page_vaddr(pmd: pmd_t) -> *mut core::ffi::c_void { __va(pmd_val(pmd) & PHYS_MASK & (PAGE_MASK as i32 as u64)) }
#[inline] pub fn pmd_page(pmd: pmd_t) -> *mut page { pfn_to_page(__phys_to_pfn(pmd_val(pmd) & PHYS_MASK)) }
#[inline] pub fn pte_pfn(pte: pte_t) -> u64 { (pte_val(pte) & PHYS_MASK) >> PAGE_SHIFT }
#[inline] pub fn pfn_pte(pfn: u64, prot: pgprot_t) -> pte_t { __pte(__pfn_to_phys(pfn) | pgprot_val(prot)) }
#[inline] pub fn pte_page(pte: pte_t) -> *mut page { pfn_to_page(pte_pfn(pte)) }
#[inline] pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: u64, ptep: *mut pte_t) { set_pte_ext(ptep, __pte(0), 0) }
#[inline] pub fn pte_isset(pte: pte_t, val: u64) -> u64 { if (val as u32 as u64) == val { pte_val(pte) & val } else { (pte_val(pte) & val != 0) as u64 } }
#[inline] pub fn pte_isclear(pte: pte_t, val: u64) -> bool { pte_val(pte) & val == 0 }
#[inline] pub fn pte_none(pte: pte_t) -> bool { pte_val(pte) == 0 }
#[inline] pub fn pte_present(pte: pte_t) -> bool { pte_isset(pte, L_PTE_PRESENT) != 0 }
#[inline] pub fn pte_valid(pte: pte_t) -> bool { pte_isset(pte, L_PTE_VALID) != 0 }
#[inline] pub fn pte_accessible(mm: *mut mm_struct, pte: pte_t) -> bool { if mm_tlb_flush_pending(mm) { pte_present(pte) } else { pte_valid(pte) } }
#[inline] pub fn pte_write(pte: pte_t) -> bool { pte_isclear(pte, L_PTE_RDONLY) }
#[inline] pub fn pte_dirty(pte: pte_t) -> bool { pte_isset(pte, L_PTE_DIRTY) != 0 }
#[inline] pub fn pte_young(pte: pte_t) -> bool { pte_isset(pte, L_PTE_YOUNG) != 0 }
#[inline] pub fn pte_exec(pte: pte_t) -> bool { pte_isclear(pte, L_PTE_XN) }
#[inline] pub fn pte_valid_user(pte: pte_t) -> bool { pte_valid(pte) && pte_isset(pte, L_PTE_USER) != 0 && pte_young(pte) }

#[inline] pub fn pte_access_permitted(pte: pte_t, write: bool) -> bool { let mask = if write { L_PTE_PRESENT | L_PTE_USER | L_PTE_RDONLY } else { L_PTE_PRESENT | L_PTE_USER }; (pte_val(pte) & mask) == (L_PTE_PRESENT | L_PTE_USER) }

#[cfg(not(arm_arch_lt_6))]
#[inline] pub fn __sync_icache_dcache_inline(_pteval: pte_t) {}
pub const PFN_PTE_SHIFT: u64 = PAGE_SHIFT;

#[inline] pub fn clear_pte_bit(mut pte: pte_t, prot: pgprot_t) -> pte_t { pte = __pte(pte_val(pte) & !pgprot_val(prot)); pte }
#[inline] pub fn set_pte_bit(mut pte: pte_t, prot: pgprot_t) -> pte_t { pte = __pte(pte_val(pte) | pgprot_val(prot)); pte }
#[inline] pub fn pte_wrprotect(pte: pte_t) -> pte_t { set_pte_bit(pte, __pgprot(L_PTE_RDONLY)) }
#[inline] pub fn pte_mkwrite_novma(pte: pte_t) -> pte_t { clear_pte_bit(pte, __pgprot(L_PTE_RDONLY)) }
#[inline] pub fn pte_mkclean(pte: pte_t) -> pte_t { clear_pte_bit(pte, __pgprot(L_PTE_DIRTY)) }
#[inline] pub fn pte_mkdirty(pte: pte_t) -> pte_t { set_pte_bit(pte, __pgprot(L_PTE_DIRTY)) }
#[inline] pub fn pte_mkold(pte: pte_t) -> pte_t { clear_pte_bit(pte, __pgprot(L_PTE_YOUNG)) }
#[inline] pub fn pte_mkyoung(pte: pte_t) -> pte_t { set_pte_bit(pte, __pgprot(L_PTE_YOUNG)) }
#[inline] pub fn pte_mkexec(pte: pte_t) -> pte_t { clear_pte_bit(pte, __pgprot(L_PTE_XN)) }
#[inline] pub fn pte_mknexec(pte: pte_t) -> pte_t { set_pte_bit(pte, __pgprot(L_PTE_XN)) }
#[inline] pub fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t { let mask = L_PTE_XN | L_PTE_RDONLY | L_PTE_USER | L_PTE_NONE | L_PTE_VALID; pte = __pte((pte_val(pte) & !mask) | (pgprot_val(newprot) & mask)); pte }

pub const __SWP_TYPE_SHIFT: u64 = 2;
pub const __SWP_TYPE_BITS: u64 = 5;
pub const __SWP_TYPE_MASK: u64 = (1 << __SWP_TYPE_BITS) - 1;
pub const __SWP_OFFSET_SHIFT: u64 = __SWP_TYPE_BITS + __SWP_TYPE_SHIFT + 1;
#[inline] pub fn __swp_type(x: swp_entry_t) -> u64 { (x.val >> __SWP_TYPE_SHIFT) & __SWP_TYPE_MASK }
#[inline] pub fn __swp_offset(x: swp_entry_t) -> u64 { x.val >> __SWP_OFFSET_SHIFT }
#[inline] pub fn __swp_entry(typ: u64, offset: u64) -> swp_entry_t { swp_entry_t { val: ((typ & __SWP_TYPE_MASK) << __SWP_TYPE_SHIFT) | (offset << __SWP_OFFSET_SHIFT) } }
#[inline] pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline] pub fn __swp_entry_to_pte(swp: swp_entry_t) -> pte_t { __pte(swp.val) }
#[inline] pub fn pte_swp_exclusive(pte: pte_t) -> bool { pte_isset(pte, L_PTE_SWP_EXCLUSIVE) != 0 }
#[inline] pub fn pte_swp_mkexclusive(pte: pte_t) -> pte_t { set_pte_bit(pte, __pgprot(L_PTE_SWP_EXCLUSIVE)) }
#[inline] pub fn pte_swp_clear_exclusive(pte: pte_t) -> pte_t { clear_pte_bit(pte, __pgprot(L_PTE_SWP_EXCLUSIVE)) }

/* MAX_SWAPFILES_CHECK() is BUILD_BUG_ON(MAX_SWAPFILES_SHIFT > __SWP_TYPE_BITS). */
/* HAVE_ARCH_UNMAPPED_AREA and HAVE_ARCH_UNMAPPED_AREA_TOPDOWN are defined. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
