/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2012 Regents of the University of California */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/mmzone.h, linux/sizes.h, asm/pgtable-bits.h, asm/page.h,
// asm/tlbflush.h, linux/mm_types.h, asm/compat.h, asm/cpufeature.h,
// asm/pgtable-{32,64}.h, linux/page_table_check.h.

#[repr(C)]
pub struct PtAllocOps {
    pub get_pte_virt: Option<unsafe extern "C" fn(pa: phys_addr_t) -> *mut pte_t>,
    pub alloc_pte: Option<unsafe extern "C" fn(va: usize) -> phys_addr_t>,
    // Present unless __PAGETABLE_PMD_FOLDED is configured.
    pub get_pmd_virt: Option<unsafe extern "C" fn(pa: phys_addr_t) -> *mut pmd_t>,
    pub alloc_pmd: Option<unsafe extern "C" fn(va: usize) -> phys_addr_t>,
    pub get_pud_virt: Option<unsafe extern "C" fn(pa: phys_addr_t) -> *mut pud_t>,
    pub alloc_pud: Option<unsafe extern "C" fn(va: usize) -> phys_addr_t>,
    pub get_p4d_virt: Option<unsafe extern "C" fn(pa: phys_addr_t) -> *mut p4d_t>,
    pub alloc_p4d: Option<unsafe extern "C" fn(va: usize) -> phys_addr_t>,
}

extern "C" {
    pub static mut pt_ops: PtAllocOps;
    pub static mut swapper_pg_dir: pgd_t;
    pub static mut trampoline_pg_dir: pgd_t;
    pub static mut early_pg_dir: pgd_t;
    pub static mut _start: u8;
    pub static mut _dtb_early_va: *mut core::ffi::c_void;
    pub static mut _dtb_early_pa: usize;
    pub static mut satp_mode: u64;
    pub fn pte_mkwrite(pte: pte_t, vma: *mut vm_area_struct) -> pte_t;
    pub fn pmd_mkwrite(pmd: pmd_t, vma: *mut vm_area_struct) -> pmd_t;
    pub fn pmdp_collapse_flush(vma: *mut vm_area_struct, address: usize, pmdp: *mut pmd_t) -> pmd_t;
    pub fn pudp_invalidate(vma: *mut vm_area_struct, address: usize, pudp: *mut pud_t) -> pud_t;
    pub fn flush_icache_pte(mm: *mut mm_struct, pte: pte_t);
    pub fn ptep_set_access_flags(vma: *mut vm_area_struct, address: usize, ptep: *mut pte_t, entry: pte_t, dirty: i32) -> i32;
    pub fn ptep_test_and_clear_young(vma: *mut vm_area_struct, address: usize, ptep: *mut pte_t) -> bool;
    pub fn paging_init();
    pub fn misc_mem_init();
}

#[inline]
pub unsafe fn __page_val_to_pfn(val: usize) -> usize { (val & _PAGE_PFN_MASK) >> _PAGE_PFN_SHIFT }

pub const VA_BITS_SV32: usize = 32;
pub const VA_BITS_SV39: usize = 39;
pub const VA_BITS_SV48: usize = 48;
pub const VA_BITS_SV57: usize = 57;
pub const PFN_PTE_SHIFT: usize = _PAGE_PFN_SHIFT;
pub const __SWP_TYPE_SHIFT: usize = 7;
pub const __SWP_TYPE_BITS: usize = 5;
pub const __SWP_TYPE_MASK: usize = (1usize << __SWP_TYPE_BITS) - 1;
pub const __SWP_OFFSET_SHIFT: usize = __SWP_TYPE_BITS + __SWP_TYPE_SHIFT;

#[inline] pub unsafe fn pmd_present(pmd: pmd_t) -> i32 { (pmd_val(pmd) & (_PAGE_PRESENT | _PAGE_PROT_NONE | _PAGE_LEAF)) as i32 }
#[inline] pub unsafe fn pmd_none(pmd: pmd_t) -> i32 { (pmd_val(pmd) == 0) as i32 }
#[inline] pub unsafe fn pmd_bad(pmd: pmd_t) -> i32 { (!pmd_present(pmd) != 0 || pmd_val(pmd) & _PAGE_LEAF != 0) as i32 }
#[inline] pub unsafe fn pmd_leaf(pmd: pmd_t) -> bool { pmd_present(pmd) != 0 && pmd_val(pmd) & _PAGE_LEAF != 0 }
#[inline] pub unsafe fn set_pmd(p: *mut pmd_t, v: pmd_t) { core::ptr::write_volatile(p, v) }
#[inline] pub unsafe fn pmd_clear(p: *mut pmd_t) { set_pmd(p, __pmd(0)); }
#[inline] pub unsafe fn pfn_pgd(pfn: usize, prot: pgprot_t) -> pgd_t { let mut v = pgprot_val(prot); ALT_THEAD_PMA!(v); __pgd((pfn << _PAGE_PFN_SHIFT) | v) }
#[inline] pub unsafe fn _pgd_pfn(pgd: pgd_t) -> usize { __page_val_to_pfn(pgd_val(pgd)) }
#[inline] pub unsafe fn pmd_page(pmd: pmd_t) -> *mut page { pfn_to_page(__page_val_to_pfn(pmd_val(pmd))) }
#[inline] pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> usize { pfn_to_virt(__page_val_to_pfn(pmd_val(pmd))) as usize }
#[inline] pub unsafe fn pmd_pte(pmd: pmd_t) -> pte_t { __pte(pmd_val(pmd)) }
#[inline] pub unsafe fn pud_pte(pud: pud_t) -> pte_t { __pte(pud_val(pud)) }

#[inline] pub unsafe fn has_svnapot() -> bool { riscv_has_extension_likely(RISCV_ISA_EXT_SVNAPOT) }
#[inline] pub unsafe fn pte_napot(pte: pte_t) -> usize { pte_val(pte) & _PAGE_NAPOT }
#[inline] pub unsafe fn pte_mknapot(pte: pte_t, order: u32) -> pte_t { let pos = order - 1 + _PAGE_PFN_SHIFT; let bit = 1usize << pos; let mask = !(genmask(pos, _PAGE_PFN_SHIFT)); __pte((pte_val(pte) & mask) | bit | _PAGE_NAPOT) }
#[inline] pub unsafe fn pte_pfn(pte: pte_t) -> usize { let mut r = __page_val_to_pfn(pte_val(pte)); if has_svnapot() && pte_napot(pte) != 0 { r &= r.wrapping_sub(1); } r }
#[inline] pub unsafe fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t { let mut v = pgprot_val(prot); ALT_THEAD_PMA!(v); __pte((pfn << _PAGE_PFN_SHIFT) | v) }
#[inline] pub unsafe fn pte_pgprot(pte: pte_t) -> pgprot_t { let pfn = pte_pfn(pte); __pgprot(pte_val(pfn_pte(pfn, __pgprot(0))) ^ pte_val(pte)) }
#[inline] pub unsafe fn pte_present(pte: pte_t) -> i32 { (pte_val(pte) & (_PAGE_PRESENT | _PAGE_PROT_NONE)) as i32 }
#[inline] pub unsafe fn pte_none(pte: pte_t) -> i32 { (pte_val(pte) == 0) as i32 }
#[inline] pub unsafe fn pte_write(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_WRITE) as i32 }
#[inline] pub unsafe fn pte_exec(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_EXEC) as i32 }
#[inline] pub unsafe fn pte_user(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_USER) as i32 }
#[inline] pub unsafe fn pte_huge(pte: pte_t) -> i32 { (pte_present(pte) != 0 && pte_val(pte) & _PAGE_LEAF != 0) as i32 }
#[inline] pub unsafe fn pte_dirty(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_DIRTY) as i32 }
#[inline] pub unsafe fn pte_young(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_ACCESSED) as i32 }
#[inline] pub unsafe fn pte_special(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_SPECIAL) as i32 }
#[inline] pub unsafe fn pte_wrprotect(pte: pte_t) -> pte_t { __pte((pte_val(pte) & !_PAGE_WRITE) | _PAGE_READ) }
#[inline] pub unsafe fn pte_mkwrite_novma(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_WRITE) }
#[inline] pub unsafe fn pte_mkwrite_shstk(pte: pte_t) -> pte_t { __pte((pte_val(pte) & !_PAGE_LEAF) | _PAGE_WRITE) }
#[inline] pub unsafe fn pte_mkdirty(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_DIRTY | _PAGE_SOFT_DIRTY) }
#[inline] pub unsafe fn pte_mkclean(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_DIRTY) }
#[inline] pub unsafe fn pte_mkyoung(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_ACCESSED) }
#[inline] pub unsafe fn pte_mkold(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_ACCESSED) }
#[inline] pub unsafe fn pte_mkspecial(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_SPECIAL) }
#[inline] pub unsafe fn pte_mkhuge(pte: pte_t) -> pte_t { pte }

#[inline] pub unsafe fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t { let mut v = pgprot_val(newprot); ALT_THEAD_PMA!(v); __pte((pte_val(pte) & _PAGE_CHG_MASK) | v) }
#[inline] pub unsafe fn pte_pmd(pte: pte_t) -> pmd_t { __pmd(pte_val(pte)) }
#[inline] pub unsafe fn pte_pud(pte: pte_t) -> pud_t { __pud(pte_val(pte)) }
#[inline] pub unsafe fn pmd_mkhuge(pmd: pmd_t) -> pmd_t { pmd }
#[inline] pub unsafe fn pmd_mkinvalid(pmd: pmd_t) -> pmd_t { __pmd(pmd_val(pmd) & !(_PAGE_PRESENT | _PAGE_PROT_NONE)) }
#[inline] pub unsafe fn pmd_pfn(pmd: pmd_t) -> usize { ((__page_val_to_pfn(pmd_val(pmd)) << PAGE_SHIFT) & PMD_MASK) >> PAGE_SHIFT }
#[inline] pub unsafe fn pud_pfn(pud: pud_t) -> usize { ((__page_val_to_pfn(pud_val(pud)) << PAGE_SHIFT) & PUD_MASK) >> PAGE_SHIFT }
#[inline] pub unsafe fn pmd_pgprot(pmd: pmd_t) -> pgprot_t { pte_pgprot(pmd_pte(pmd)) }
#[inline] pub unsafe fn pud_pgprot(pud: pud_t) -> pgprot_t { pte_pgprot(pud_pte(pud)) }
#[inline] pub unsafe fn pmd_modify(pmd: pmd_t, prot: pgprot_t) -> pmd_t { pte_pmd(pte_modify(pmd_pte(pmd), prot)) }
#[inline] pub unsafe fn pmd_write(pmd: pmd_t) -> i32 { pte_write(pmd_pte(pmd)) }
#[inline] pub unsafe fn pud_write(pud: pud_t) -> i32 { pte_write(pud_pte(pud)) }
#[inline] pub unsafe fn pmd_dirty(pmd: pmd_t) -> i32 { pte_dirty(pmd_pte(pmd)) }
#[inline] pub unsafe fn pmd_young(pmd: pmd_t) -> i32 { pte_young(pmd_pte(pmd)) }
#[inline] pub unsafe fn pmd_user(pmd: pmd_t) -> i32 { pte_user(pmd_pte(pmd)) }
#[inline] pub unsafe fn pmd_mkold(pmd: pmd_t) -> pmd_t { pte_pmd(pte_mkold(pmd_pte(pmd))) }
#[inline] pub unsafe fn pmd_mkyoung(pmd: pmd_t) -> pmd_t { pte_pmd(pte_mkyoung(pmd_pte(pmd))) }
#[inline] pub unsafe fn pmd_mkwrite_novma(pmd: pmd_t) -> pmd_t { pte_pmd(pte_mkwrite_novma(pmd_pte(pmd))) }
#[inline] pub unsafe fn pmd_mkwrite_shstk(pmd: pmd_t) -> pmd_t { __pmd((pmd_val(pmd) & !_PAGE_LEAF) | _PAGE_WRITE) }
#[inline] pub unsafe fn pmd_wrprotect(pmd: pmd_t) -> pmd_t { pte_pmd(pte_wrprotect(pmd_pte(pmd))) }
#[inline] pub unsafe fn pmd_mkclean(pmd: pmd_t) -> pmd_t { pte_pmd(pte_mkclean(pmd_pte(pmd))) }
#[inline] pub unsafe fn pmd_mkdirty(pmd: pmd_t) -> pmd_t { pte_pmd(pte_mkdirty(pmd_pte(pmd))) }
#[inline] pub unsafe fn pud_wrprotect(pud: pud_t) -> pud_t { pte_pud(pte_wrprotect(pud_pte(pud))) }
#[inline] pub unsafe fn pud_trans_huge(pud: pud_t) -> i32 { pud_leaf(pud) as i32 }
#[inline] pub unsafe fn pud_dirty(pud: pud_t) -> i32 { pte_dirty(pud_pte(pud)) }
#[inline] pub unsafe fn pud_young(pud: pud_t) -> i32 { pte_young(pud_pte(pud)) }
#[inline] pub unsafe fn pud_mkyoung(pud: pud_t) -> pud_t { pte_pud(pte_mkyoung(pud_pte(pud))) }
#[inline] pub unsafe fn pud_mkold(pud: pud_t) -> pud_t { pte_pud(pte_mkold(pud_pte(pud))) }
#[inline] pub unsafe fn pud_mkdirty(pud: pud_t) -> pud_t { pte_pud(pte_mkdirty(pud_pte(pud))) }
#[inline] pub unsafe fn pud_mkclean(pud: pud_t) -> pud_t { pte_pud(pte_mkclean(pud_pte(pud))) }
#[inline] pub unsafe fn pud_mkwrite(pud: pud_t) -> pud_t { pte_pud(pte_mkwrite_novma(pud_pte(pud))) }
#[inline] pub unsafe fn pud_mkhuge(pud: pud_t) -> pud_t { pud }
#[inline] pub unsafe fn pud_mkinvalid(pud: pud_t) -> pud_t { __pud(pud_val(pud) & !(_PAGE_PRESENT | _PAGE_PROT_NONE)) }

#[inline] pub unsafe fn pte_swp_exclusive(pte: pte_t) -> bool { pte_val(pte) & _PAGE_SWP_EXCLUSIVE != 0 }
#[inline] pub unsafe fn pte_swp_mkexclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_SWP_EXCLUSIVE) }
#[inline] pub unsafe fn pte_swp_clear_exclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_SWP_EXCLUSIVE) }

// The remaining declarations are intentionally external: their definitions and
// architecture-specific constants are supplied by the included kernel headers.
extern "C" {
    pub fn set_pte(ptep: *mut pte_t, pteval: pte_t);
    pub fn set_ptes(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pteval: pte_t, nr: u32);
    pub fn pte_clear(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t);
    pub fn update_mmu_cache_range(vmf: *mut vm_fault, vma: *mut vm_area_struct, address: usize, ptep: *mut pte_t, nr: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
