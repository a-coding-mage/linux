/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 * Derived from MIPS.
 *
 * C header translation. Required kernel/architecture symbols are supplied by
 * other translated headers.
 */

/* CONFIG_PGTABLE_LEVELS selects the corresponding folded page-table level. */
#[cfg(CONFIG_PGTABLE_LEVELS = "2")]
pub const PGDIR_SHIFT: usize = PAGE_SHIFT + (PAGE_SHIFT - PTRLOG);
#[cfg(CONFIG_PGTABLE_LEVELS = "3")]
pub const PMD_SHIFT: usize = PAGE_SHIFT + (PAGE_SHIFT - PTRLOG);
#[cfg(CONFIG_PGTABLE_LEVELS = "3")]
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
#[cfg(CONFIG_PGTABLE_LEVELS = "3")]
pub const PMD_MASK: usize = !(PMD_SIZE - 1);
#[cfg(CONFIG_PGTABLE_LEVELS = "3")]
pub const PGDIR_SHIFT: usize = PMD_SHIFT + (PAGE_SHIFT - PTRLOG);
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PMD_SHIFT: usize = PAGE_SHIFT + (PAGE_SHIFT - PTRLOG);
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PMD_MASK: usize = !(PMD_SIZE - 1);
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PUD_SHIFT: usize = PMD_SHIFT + (PAGE_SHIFT - PTRLOG);
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PUD_MASK: usize = !(PUD_SIZE - 1);
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PGDIR_SHIFT: usize = PUD_SHIFT + (PAGE_SHIFT - PTRLOG);

pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
#[cfg(CONFIG_32BIT)]
pub const VA_BITS: usize = 32;
#[cfg(not(CONFIG_32BIT))]
pub const VA_BITS: usize = PGDIR_SHIFT + (PAGE_SHIFT - PTRLOG);
pub const PTRS_PER_PGD: usize = PAGE_SIZE >> PTRLOG;
#[cfg(any(CONFIG_PGTABLE_LEVELS = "3", CONFIG_PGTABLE_LEVELS = "4"))]
pub const PTRS_PER_PUD: usize = PAGE_SIZE >> PTRLOG;
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PTRS_PER_PMD: usize = PAGE_SIZE >> PTRLOG;
pub const PTRS_PER_PTE: usize = PAGE_SIZE >> PTRLOG;
#[cfg(CONFIG_32BIT)]
pub const USER_PTRS_PER_PGD: usize = TASK_SIZE / PGDIR_SIZE;
#[cfg(not(CONFIG_32BIT))]
pub const USER_PTRS_PER_PGD: usize = if TASK_SIZE64 / PGDIR_SIZE != 0 { TASK_SIZE64 / PGDIR_SIZE } else { 1 };

#[repr(C)]
pub struct mm_struct;
#[repr(C)]
pub struct vm_area_struct;
#[repr(C)]
pub struct vm_fault;
#[repr(C)]
pub struct page;

#[cfg(CONFIG_32BIT)]
pub const VMALLOC_START: usize = vm_map_base + PCI_IOSIZE + 2 * PAGE_SIZE;
#[cfg(all(CONFIG_32BIT, CONFIG_HIGHMEM))]
pub const VMALLOC_END: usize = PKMAP_BASE - 2 * PAGE_SIZE;
#[cfg(all(CONFIG_32BIT, not(CONFIG_HIGHMEM)))]
pub const VMALLOC_END: usize = FIXADDR_START - 2 * PAGE_SIZE;
#[cfg(CONFIG_32BIT)]
pub const PKMAP_BASE: usize = PKMAP_END - PAGE_SIZE * LAST_PKMAP;
#[cfg(CONFIG_32BIT)]
pub const PKMAP_END: usize = FIXADDR_START & !( (LAST_PKMAP << PAGE_SHIFT) - 1);

#[cfg(CONFIG_64BIT)]
pub const MODULES_VADDR: usize = vm_map_base + PCI_IOSIZE + 2 * PAGE_SIZE;
#[cfg(CONFIG_64BIT)]
pub const MODULES_END: usize = MODULES_VADDR + SZ_2G;
#[cfg(all(CONFIG_64BIT, CONFIG_KFENCE))]
pub const KFENCE_AREA_SIZE: usize = ((CONFIG_KFENCE_NUM_OBJECTS + 1) * 2 + 2) * PAGE_SIZE;
#[cfg(all(CONFIG_64BIT, not(CONFIG_KFENCE)))]
pub const KFENCE_AREA_SIZE: usize = 0;
#[cfg(CONFIG_64BIT)]
pub const VMALLOC_START: usize = MODULES_END;
#[cfg(all(CONFIG_64BIT, not(CONFIG_KASAN)))]
pub const VMALLOC_END: usize = vm_map_base + core::cmp::min(PTRS_PER_PGD * PTRS_PER_PUD * PTRS_PER_PMD * PTRS_PER_PTE * PAGE_SIZE, 1usize << cpu_vabits) - PMD_SIZE - VMEMMAP_SIZE - KFENCE_AREA_SIZE;
#[cfg(all(CONFIG_64BIT, CONFIG_KASAN))]
pub const VMALLOC_END: usize = vm_map_base + core::cmp::min(PTRS_PER_PGD * PTRS_PER_PUD * PTRS_PER_PMD * PTRS_PER_PTE * PAGE_SIZE, (1usize << cpu_vabits) / 2) - PMD_SIZE - VMEMMAP_SIZE - KFENCE_AREA_SIZE;
#[cfg(CONFIG_64BIT)]
pub const VMEMMAP_ALIGN: usize = if PMD_SIZE > MAX_FOLIO_VMEMMAP_ALIGN { PMD_SIZE } else { MAX_FOLIO_VMEMMAP_ALIGN };
#[cfg(CONFIG_64BIT)]
pub const VMEMMAP_END: usize = (ALIGN(VMALLOC_END, VMEMMAP_ALIGN) as usize) + VMEMMAP_SIZE - 1;
#[cfg(CONFIG_64BIT)]
pub const KFENCE_AREA_START: usize = VMEMMAP_END + 1;
#[cfg(CONFIG_64BIT)]
pub const KFENCE_AREA_END: usize = KFENCE_AREA_START + KFENCE_AREA_SIZE - 1;

#[cfg(not(CONFIG_SPARSEMEM))]
pub const DIRECT_MAP_PHYSMEM_END: u64 = (1u64 << (cpu_pabits + 1)) - 1;
#[cfg(CONFIG_SPARSEMEM)]
pub const DIRECT_MAP_PHYSMEM_END: u64 = core::cmp::min((1u64 << (cpu_pabits + 1)) - 1, (1u64 << MAX_PHYSMEM_BITS) - 1);

#[inline] pub unsafe fn ptep_get(ptep: *const pte_t) -> pte_t { READ_ONCE(*ptep) }
#[inline] pub unsafe fn pmdp_get(pmdp: *const pmd_t) -> pmd_t { READ_ONCE(*pmdp) }

#[cfg(not(__PAGETABLE_PUD_FOLDED))]
#[repr(C)] pub struct pud_t { pub pud: usize }
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
#[inline] pub fn pud_val(x: pud_t) -> usize { x.pud }
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
#[inline] pub fn __pud(x: usize) -> pud_t { pud_t { pud: x } }

#[cfg(not(__PAGETABLE_PMD_FOLDED))]
#[repr(C)] pub struct pmd_t { pub pmd: usize }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
#[inline] pub fn pmd_val(x: pmd_t) -> usize { x.pmd }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
#[inline] pub fn __pmd(x: usize) -> pmd_t { pmd_t { pmd: x } }

extern "C" {
    pub static mut invalid_pte_table: [pte_t; PTRS_PER_PTE];
    #[cfg(not(__PAGETABLE_PUD_FOLDED))] pub static mut invalid_pud_table: [pud_t; PTRS_PER_PUD];
    #[cfg(not(__PAGETABLE_PMD_FOLDED))] pub static mut invalid_pmd_table: [pmd_t; PTRS_PER_PMD];
    pub static mut swapper_pg_dir: [pgd_t; 0];
    pub static mut invalid_pg_dir: [pgd_t; 0];
    pub fn set_pmd_at(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t, pmd: pmd_t);
    pub fn pgd_init(addr: *mut core::ffi::c_void);
    pub fn pud_init(addr: *mut core::ffi::c_void);
    pub fn pmd_init(addr: *mut core::ffi::c_void);
    pub fn kernel_pte_init(addr: *mut core::ffi::c_void);
    pub fn __update_tlb(vma: *mut vm_area_struct, address: usize, ptep: *mut pte_t);
}

#[inline] pub unsafe fn p4d_none(p4d: p4d_t) -> i32 { (p4d_val(p4d) == invalid_pud_table.as_ptr() as usize) as i32 }
#[inline] pub fn p4d_bad(p4d: p4d_t) -> i32 { (p4d_val(p4d) & !PAGE_MASK) as i32 }
#[inline] pub unsafe fn p4d_present(p4d: p4d_t) -> i32 { (p4d_val(p4d) != invalid_pud_table.as_ptr() as usize) as i32 }
#[inline] pub fn p4d_pgtable(p4d: p4d_t) -> *mut pud_t { p4d_val(p4d) as *mut pud_t }
#[inline] pub unsafe fn set_p4d(p4d: *mut p4d_t, v: p4d_t) { WRITE_ONCE(*p4d, v); }
#[inline] pub unsafe fn p4d_clear(p4d: *mut p4d_t) { set_p4d(p4d, __p4d(invalid_pud_table.as_ptr() as usize)); }

#[inline] pub unsafe fn pud_none(pud: pud_t) -> i32 { (pud_val(pud) == invalid_pmd_table.as_ptr() as usize) as i32 }
#[inline] pub fn pud_bad(pud: pud_t) -> i32 { (pud_val(pud) & !PAGE_MASK) as i32 }
#[inline] pub unsafe fn pud_present(pud: pud_t) -> i32 { (pud_val(pud) != invalid_pmd_table.as_ptr() as usize) as i32 }
#[inline] pub fn pud_pgtable(pud: pud_t) -> *mut pmd_t { pud_val(pud) as *mut pmd_t }
#[inline] pub unsafe fn set_pud(pud: *mut pud_t, v: pud_t) { WRITE_ONCE(*pud, v); }
#[inline] pub unsafe fn pud_clear(pud: *mut pud_t) { set_pud(pud, __pud(invalid_pmd_table.as_ptr() as usize)); }
#[inline] pub fn pud_phys(pud: pud_t) -> usize { PHYSADDR(pud_val(pud)) }
#[inline] pub fn pud_page(pud: pud_t) -> *mut page { pfn_to_page(pud_phys(pud) >> PAGE_SHIFT) }

#[inline] pub unsafe fn pmd_none(pmd: pmd_t) -> i32 { (pmd_val(pmd) == invalid_pte_table.as_ptr() as usize) as i32 }
#[inline] pub fn pmd_bad(pmd: pmd_t) -> i32 { (pmd_val(pmd) & !PAGE_MASK) as i32 }
#[inline] pub unsafe fn pmd_present(pmd: pmd_t) -> i32 { if unlikely(pmd_val(pmd) & _PAGE_HUGE != 0) { ((pmd_val(pmd) & (_PAGE_PRESENT | _PAGE_PROTNONE | _PAGE_PRESENT_INVALID)) != 0) as i32 } else { (pmd_val(pmd) != invalid_pte_table.as_ptr() as usize) as i32 } }
#[inline] pub unsafe fn set_pmd(pmd: *mut pmd_t, v: pmd_t) { WRITE_ONCE(*pmd, v); }
#[inline] pub unsafe fn pmd_clear(pmd: *mut pmd_t) { set_pmd(pmd, __pmd(invalid_pte_table.as_ptr() as usize)); }
#[inline] pub fn pmd_phys(pmd: pmd_t) -> usize { PHYSADDR(pmd_val(pmd)) }
#[inline] pub fn pmd_page(pmd: pmd_t) -> *mut page { pfn_to_page(pmd_phys(pmd) >> PAGE_SHIFT) }
#[inline] pub fn pmd_page_vaddr(pmd: pmd_t) -> usize { pmd_val(pmd) }
#[inline] pub fn pmd_pfn(pmd: pmd_t) -> usize { (pmd_val(pmd) & _PFN_MASK) >> PFN_PTE_SHIFT }

#[inline] pub fn pte_page(x: pte_t) -> *mut page { pfn_to_page(pte_pfn(x)) }
#[inline] pub fn pte_pfn(x: pte_t) -> usize { (x.pte & _PFN_MASK) >> PFN_PTE_SHIFT }
#[inline] pub fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t { __pte((pfn << PFN_PTE_SHIFT) | pgprot_val(prot)) }
#[inline] pub fn pfn_pmd(pfn: usize, prot: pgprot_t) -> pmd_t { __pmd((pfn << PFN_PTE_SHIFT) | pgprot_val(prot)) }

pub const __SWP_TYPE_BITS: usize = if IS_ENABLED(CONFIG_32BIT) { 5 } else { 7 };
pub const __SWP_TYPE_MASK: usize = (1usize << __SWP_TYPE_BITS) - 1;
pub const __SWP_TYPE_SHIFT: usize = if IS_ENABLED(CONFIG_32BIT) { 8 } else { 16 };
pub const __SWP_OFFSET_SHIFT: usize = __SWP_TYPE_BITS + __SWP_TYPE_SHIFT + 1;
#[inline] pub fn mk_swap_pte(ty: usize, offset: usize) -> pte_t { __pte(((ty & __SWP_TYPE_MASK) << __SWP_TYPE_SHIFT) | (offset << __SWP_OFFSET_SHIFT)) }
#[inline] pub fn __swp_type(x: swp_entry_t) -> usize { (x.val >> __SWP_TYPE_SHIFT) & __SWP_TYPE_MASK }
#[inline] pub fn __swp_offset(x: swp_entry_t) -> usize { x.val >> __SWP_OFFSET_SHIFT }
#[inline] pub fn __swp_entry(ty: usize, offset: usize) -> swp_entry_t { swp_entry_t { val: pte_val(mk_swap_pte(ty, offset)) } }
#[inline] pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }
#[inline] pub fn __swp_entry_to_pmd(x: swp_entry_t) -> pmd_t { __pmd(x.val | _PAGE_HUGE) }
#[inline] pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline] pub fn __pmd_to_swp_entry(pmd: pmd_t) -> swp_entry_t { swp_entry_t { val: pmd_val(pmd) } }
#[inline] pub fn pte_swp_exclusive(pte: pte_t) -> bool { pte_val(pte) & _PAGE_SWP_EXCLUSIVE != 0 }
#[inline] pub fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_SWP_EXCLUSIVE; pte }
#[inline] pub fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) &= !_PAGE_SWP_EXCLUSIVE; pte }
#[inline] pub fn pte_none(pte: pte_t) -> bool { pte_val(pte) & !_PAGE_GLOBAL == 0 }
#[inline] pub fn pte_present(pte: pte_t) -> bool { pte_val(pte) & (_PAGE_PRESENT | _PAGE_PROTNONE) != 0 }
#[inline] pub fn pte_no_exec(pte: pte_t) -> bool { pte_val(pte) & _PAGE_NO_EXEC != 0 }
#[inline] pub unsafe fn set_pte(ptep: *mut pte_t, pteval: pte_t) { WRITE_ONCE(*ptep, pteval); #[cfg(CONFIG_SMP)] if pte_val(pteval) & _PAGE_GLOBAL != 0 { DBAR(0b11000); } }
#[inline] pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t) { let mut pte = ptep_get(ptep); pte_val_mut(&mut pte) &= _PAGE_GLOBAL; set_pte(ptep, pte); }

#[inline] pub fn pte_write(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_WRITE) as i32 }
#[inline] pub fn pte_young(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_ACCESSED) as i32 }
#[inline] pub fn pte_dirty(pte: pte_t) -> i32 { (pte_val(pte) & (_PAGE_DIRTY | _PAGE_MODIFIED)) as i32 }
#[inline] pub fn pte_mkold(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) &= !_PAGE_ACCESSED; pte }
#[inline] pub fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_ACCESSED; pte }
#[inline] pub fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) &= !(_PAGE_DIRTY | _PAGE_MODIFIED); pte }
#[inline] pub fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_MODIFIED; if pte_val(pte) & _PAGE_WRITE != 0 { pte_val_mut(&mut pte) |= _PAGE_DIRTY; } pte }
#[inline] pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_WRITE; if pte_val(pte) & _PAGE_MODIFIED != 0 { pte_val_mut(&mut pte) |= _PAGE_DIRTY; } pte }
#[inline] pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { if pte_val(pte) & _PAGE_DIRTY != 0 { pte_val_mut(&mut pte) |= _PAGE_MODIFIED; } pte_val_mut(&mut pte) &= !(_PAGE_WRITE | _PAGE_DIRTY); pte }
#[inline] pub fn pte_huge(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_HUGE) as i32 }
#[inline] pub fn pte_mkhuge(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_HUGE; pte }

#[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
#[inline] pub fn pte_special(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_SPECIAL) as i32 }
#[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
#[inline] pub fn pte_mkspecial(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_SPECIAL; pte }

#[inline] pub fn pte_accessible(mm: *mut mm_struct, a: pte_t) -> bool { if pte_val(a) & _PAGE_PRESENT != 0 { return true; } if pte_val(a) & _PAGE_PROTNONE != 0 && atomic_read(unsafe { &(*mm).tlb_flush_pending }) != 0 { return true; } false }
#[inline] pub fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t { if pte_val(pte) & _PAGE_DIRTY != 0 { pte_val_mut(&mut pte) |= _PAGE_MODIFIED; } __pte((pte_val(pte) & _PAGE_CHG_MASK) | (pgprot_val(newprot) & !_PAGE_CHG_MASK)) }

#[inline] pub unsafe fn update_mmu_cache_range(_vmf: *mut vm_fault, vma: *mut vm_area_struct, mut address: usize, mut ptep: *mut pte_t, mut nr: u32) { loop { __update_tlb(vma, address, ptep); nr -= 1; if nr == 0 { break; } address += PAGE_SIZE; ptep = ptep.add(1); } }
#[inline] pub unsafe fn update_mmu_cache_pmd(vma: *mut vm_area_struct, address: usize, pmdp: *mut pmd_t) { __update_tlb(vma, address, pmdp as *mut pte_t); }
#[inline] pub fn pmd_leaf(pmd: pmd_t) -> bool { pmd_val(pmd) & _PAGE_HUGE != 0 }
#[inline] pub fn pud_leaf(pud: pud_t) -> bool { pud_val(pud) & _PAGE_HUGE != 0 }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_trans_huge(pmd: pmd_t) -> bool { pmd_val(pmd) & _PAGE_HUGE != 0 && unsafe { pmd_present(pmd) != 0 } }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_mkhuge(mut pmd: pmd_t) -> pmd_t { let global = pmd_val(pmd) & !_PAGE_GLOBAL; let shifted = (pmd_val(pmd) & _PAGE_GLOBAL) << (_PAGE_HGLOBAL_SHIFT - _PAGE_GLOBAL_SHIFT); pmd_val_mut(&mut pmd) = global | shifted; pmd_val_mut(&mut pmd) |= _PAGE_HUGE; pmd }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_write(pmd: pmd_t) -> i32 { ((pmd_val(pmd) & _PAGE_WRITE) != 0) as i32 }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_mkwrite_novma(mut pmd: pmd_t) -> pmd_t { pmd_val_mut(&mut pmd) |= _PAGE_WRITE; if pmd_val(pmd) & _PAGE_MODIFIED != 0 { pmd_val_mut(&mut pmd) |= _PAGE_DIRTY; } pmd }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_wrprotect(mut pmd: pmd_t) -> pmd_t { if pmd_val(pmd) & _PAGE_DIRTY != 0 { pmd_val_mut(&mut pmd) |= _PAGE_MODIFIED; } pmd_val_mut(&mut pmd) &= !(_PAGE_WRITE | _PAGE_DIRTY); pmd }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_dirty(pmd: pmd_t) -> i32 { ((pmd_val(pmd) & (_PAGE_DIRTY | _PAGE_MODIFIED)) != 0) as i32 }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_mkclean(mut pmd: pmd_t) -> pmd_t { pmd_val_mut(&mut pmd) &= !(_PAGE_DIRTY | _PAGE_MODIFIED); pmd }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_mkdirty(mut pmd: pmd_t) -> pmd_t { pmd_val_mut(&mut pmd) |= _PAGE_MODIFIED; if pmd_val(pmd) & _PAGE_WRITE != 0 { pmd_val_mut(&mut pmd) |= _PAGE_DIRTY; } pmd }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_young(pmd: pmd_t) -> i32 { ((pmd_val(pmd) & _PAGE_ACCESSED) != 0) as i32 }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_mkold(mut pmd: pmd_t) -> pmd_t { pmd_val_mut(&mut pmd) &= !_PAGE_ACCESSED; pmd }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_mkyoung(mut pmd: pmd_t) -> pmd_t { pmd_val_mut(&mut pmd) |= _PAGE_ACCESSED; pmd }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_modify(mut pmd: pmd_t, newprot: pgprot_t) -> pmd_t { if pmd_val(pmd) & _PAGE_DIRTY != 0 { pmd_val_mut(&mut pmd) |= _PAGE_MODIFIED; } __pmd((pmd_val(pmd) & _HPAGE_CHG_MASK) | (pgprot_val(newprot) & !_HPAGE_CHG_MASK)) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn pmd_mkinvalid(mut pmd: pmd_t) -> pmd_t { pmd_val_mut(&mut pmd) |= _PAGE_PRESENT_INVALID; pmd_val_mut(&mut pmd) &= !(_PAGE_PRESENT | _PAGE_VALID | _PAGE_DIRTY | _PAGE_PROTNONE); pmd }
#[cfg(CONFIG_ARCH_HAS_PTE_PROTNONE)]
#[inline] pub fn pte_protnone(pte: pte_t) -> i64 { (pte_val(pte) & _PAGE_PROTNONE) as i64 }
#[cfg(CONFIG_ARCH_HAS_PTE_PROTNONE)]
#[inline] pub fn pmd_protnone(pmd: pmd_t) -> i64 { (pmd_val(pmd) & _PAGE_PROTNONE) as i64 }

/* CONFIG_TRANSPARENT_HUGEPAGE and CONFIG_ARCH_HAS_PTE_* additions retain the
 * source header's conditional intent; dependent generic implementations are
 * intentionally supplied by other translated units. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
