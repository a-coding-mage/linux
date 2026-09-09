/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of sparc/include/asm/pgtable_64.h. */

// Includes and configuration checks from the C header are supplied externally.

pub const TLBTEMP_BASE: usize = 0x0000_0000_0600_0000;
pub const TSBMAP_8K_BASE: usize = 0x0000_0000_0800_0000;
pub const TSBMAP_4M_BASE: usize = 0x0000_0000_0840_0000;
pub const MODULES_VADDR: usize = 0x0000_0000_1000_0000;
pub const MODULES_LEN: usize = 0x0000_0000_e000_0000;
pub const MODULES_END: usize = 0x0000_0000_f000_0000;
pub const LOW_OBP_ADDRESS: usize = 0x0000_0000_f000_0000;
pub const HI_OBP_ADDRESS: usize = 0x0000_0001_0000_0000;
pub const VMALLOC_START: usize = 0x0000_0001_0000_0000;
pub const VMEMMAP_BASE: usize = VMALLOC_END;

pub const PMD_SHIFT: usize = PAGE_SHIFT + (PAGE_SHIFT - 3);
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);
pub const PMD_BITS: usize = PAGE_SHIFT - 3;
pub const PUD_SHIFT: usize = PMD_SHIFT + PMD_BITS;
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
pub const PUD_MASK: usize = !(PUD_SIZE - 1);
pub const PUD_BITS: usize = PAGE_SHIFT - 3;
pub const PGDIR_SHIFT: usize = PUD_SHIFT + PUD_BITS;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
pub const PGDIR_BITS: usize = PAGE_SHIFT - 3;

pub const PTRS_PER_PTE: usize = 1usize << (PAGE_SHIFT - 3);
pub const PTRS_PER_PMD: usize = 1usize << PMD_BITS;
pub const PTRS_PER_PUD: usize = 1usize << PUD_BITS;
pub const PTRS_PER_PGD: usize = 1usize << PGDIR_BITS;

pub const _PAGE_VALID: u64 = 0x8000_0000_0000_0000;
pub const _PAGE_R: u64 = 0x8000_0000_0000_0000;
pub const _PAGE_SPECIAL: u64 = 0x0200_0000_0000_0000;
pub const _PAGE_PMD_HUGE: u64 = 0x0100_0000_0000_0000;
pub const _PAGE_PUD_HUGE: u64 = _PAGE_PMD_HUGE;

pub const _PAGE_SZ4MB_4U: u64 = 0x6000_0000_0000_0000;
pub const _PAGE_SZ512K_4U: u64 = 0x4000_0000_0000_0000;
pub const _PAGE_SZ64K_4U: u64 = 0x2000_0000_0000_0000;
pub const _PAGE_SZ8K_4U: u64 = 0;
pub const _PAGE_NFO_4U: u64 = 0x1000_0000_0000_0000;
pub const _PAGE_IE_4U: u64 = 0x0800_0000_0000_0000;
pub const _PAGE_SOFT2_4U: u64 = 0x07fc_0000_0000_0000;
pub const _PAGE_SPECIAL_4U: u64 = 0x0200_0000_0000_0000;
pub const _PAGE_PMD_HUGE_4U: u64 = 0x0100_0000_0000_0000;
pub const _PAGE_RES1_4U: u64 = 0x0002_0000_0000_0000;
pub const _PAGE_SZ32MB_4U: u64 = 0x0001_0000_0000_0000;
pub const _PAGE_SZ256MB_4U: u64 = 0x2001_0000_0000_0000;
pub const _PAGE_SZALL_4U: u64 = 0x6001_0000_0000_0000;
pub const _PAGE_SN_4U: u64 = 0x0000_8000_0000_0000;
pub const _PAGE_RES2_4U: u64 = 0x0000_7800_0000_0000;
pub const _PAGE_PADDR_4U: u64 = 0x0000_07ff_ffff_e000;
pub const _PAGE_SOFT_4U: u64 = 0x1f80;
pub const _PAGE_EXEC_4U: u64 = 0x1000;
pub const _PAGE_MODIFIED_4U: u64 = 0x0800;
pub const _PAGE_ACCESSED_4U: u64 = 0x0400;
pub const _PAGE_READ_4U: u64 = 0x0200;
pub const _PAGE_WRITE_4U: u64 = 0x0100;
pub const _PAGE_PRESENT_4U: u64 = 0x0080;
pub const _PAGE_L_4U: u64 = 0x0040;
pub const _PAGE_CP_4U: u64 = 0x0020;
pub const _PAGE_CV_4U: u64 = 0x0010;
pub const _PAGE_E_4U: u64 = 0x0008;
pub const _PAGE_P_4U: u64 = 0x0004;
pub const _PAGE_W_4U: u64 = 0x0002;

pub const _PAGE_NFO_4V: u64 = 0x4000_0000_0000_0000;
pub const _PAGE_SOFT2_4V: u64 = 0x3f00_0000_0000_0000;
pub const _PAGE_MODIFIED_4V: u64 = 0x2000_0000_0000_0000;
pub const _PAGE_ACCESSED_4V: u64 = 0x1000_0000_0000_0000;
pub const _PAGE_READ_4V: u64 = 0x0800_0000_0000_0000;
pub const _PAGE_WRITE_4V: u64 = 0x0400_0000_0000_0000;
pub const _PAGE_SPECIAL_4V: u64 = 0x0200_0000_0000_0000;
pub const _PAGE_PMD_HUGE_4V: u64 = 0x0100_0000_0000_0000;
pub const _PAGE_PADDR_4V: u64 = 0x00ff_ffff_ffff_e000;
pub const _PAGE_IE_4V: u64 = 0x1000;
pub const _PAGE_E_4V: u64 = 0x0800;
pub const _PAGE_CP_4V: u64 = 0x0400;
pub const _PAGE_CV_4V: u64 = 0x0200;
pub const _PAGE_MCD_4V: u64 = 0x0200;
pub const _PAGE_P_4V: u64 = 0x0100;
pub const _PAGE_EXEC_4V: u64 = 0x0080;
pub const _PAGE_W_4V: u64 = 0x0040;
pub const _PAGE_SOFT_4V: u64 = 0x0030;
pub const _PAGE_PRESENT_4V: u64 = 0x0010;
pub const _PAGE_RESV_4V: u64 = 0x0008;
pub const _PAGE_SZ16GB_4V: u64 = 7;
pub const _PAGE_SZ2GB_4V: u64 = 6;
pub const _PAGE_SZ256MB_4V: u64 = 5;
pub const _PAGE_SZ32MB_4V: u64 = 4;
pub const _PAGE_SZ4MB_4V: u64 = 3;
pub const _PAGE_SZ512K_4V: u64 = 2;
pub const _PAGE_SZ64K_4V: u64 = 1;
pub const _PAGE_SZ8K_4V: u64 = 0;
pub const _PAGE_SZALL_4V: u64 = 7;
pub const _PAGE_SZBITS_4U: u64 = _PAGE_SZ8K_4U;
pub const _PAGE_SZBITS_4V: u64 = _PAGE_SZ8K_4V;
pub const _PAGE_SZHUGE_4U: u64 = _PAGE_SZ4MB_4U;
pub const _PAGE_SZHUGE_4V: u64 = _PAGE_SZ4MB_4V;
pub const _PAGE_SWP_EXCLUSIVE: u64 = 0x0000_0000_0010_0000;

extern "C" {
    pub static mut VMALLOC_END: usize;
    pub fn kern_addr_valid(addr: usize) -> bool;
    pub fn mk_pte_io(a: usize, b: pgprot_t, c: i32, d: usize) -> pte_t;
    pub fn pte_sz_bits(size: usize) -> usize;
    pub static mut PAGE_KERNEL: pgprot_t;
    pub static mut PAGE_KERNEL_LOCKED: pgprot_t;
    pub static mut PAGE_COPY: pgprot_t;
    pub static mut PAGE_SHARED: pgprot_t;
    pub static mut _PAGE_IE: usize;
    pub static mut _PAGE_E: usize;
    pub static mut _PAGE_CACHE: usize;
    pub static mut pg_iobits: usize;
    pub static mut _PAGE_ALL_SZ_BITS: usize;
}

pub unsafe fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t {
    BUILD_BUG_ON(_PAGE_SZBITS_4U != 0 || _PAGE_SZBITS_4V != 0);
    __pte((pfn << PAGE_SHIFT) | pgprot_val(prot))
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pfn_pmd(page_nr: usize, pgprot: pgprot_t) -> pmd_t {
    __pmd(pte_val(pfn_pte(page_nr, pgprot)))
}

pub unsafe fn pte_pfn(pte: pte_t) -> usize {
    // The original uses patchable SUN4U/SUN4V shifts; retain the effective operation.
    pte_val(pte) >> (21 + PAGE_SHIFT)
}
pub unsafe fn pte_page(x: pte_t) -> *mut page { pfn_to_page(pte_pfn(x)) }

pub unsafe fn pte_modify(pte: pte_t, prot: pgprot_t) -> pte_t {
    let mask = _PAGE_PADDR_4U | _PAGE_MODIFIED_4U | _PAGE_ACCESSED_4U |
        _PAGE_CP_4U | _PAGE_CV_4U | _PAGE_E_4U | _PAGE_SPECIAL |
        _PAGE_PMD_HUGE | _PAGE_SZALL_4U;
    __pte((pte_val(pte) & mask) | (pgprot_val(prot) & !mask))
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_modify(pmd: pmd_t, newprot: pgprot_t) -> pmd_t {
    __pmd(pte_val(pte_modify(__pte(pmd_val(pmd)), newprot)))
}

pub unsafe fn pgprot_noncached(prot: pgprot_t) -> pgprot_t {
    let mut val = pgprot_val(prot);
    val = (val & !(_PAGE_CP_4U | _PAGE_CV_4U)) | _PAGE_E_4U;
    __pgprot(val)
}

pub unsafe fn pte_dirty(pte: pte_t) -> usize { pte_val(pte) & _PAGE_MODIFIED_4U }
pub unsafe fn pte_write(pte: pte_t) -> usize { pte_val(pte) & _PAGE_WRITE_4U }

#[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))]
pub unsafe fn __pte_default_huge_mask() -> usize { _PAGE_SZHUGE_4U as usize }
#[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))]
pub unsafe fn pte_mkhuge(pte: pte_t) -> pte_t { __pte(pte_val(pte) | __pte_default_huge_mask()) }
#[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))]
pub unsafe fn is_default_hugetlb_pte(pte: pte_t) -> bool { (pte_val(pte) & __pte_default_huge_mask()) == __pte_default_huge_mask() }
#[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))]
pub unsafe fn is_hugetlb_pmd(pmd: pmd_t) -> bool { pmd_val(pmd) & _PAGE_PMD_HUGE as usize != 0 }
#[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))]
pub unsafe fn is_hugetlb_pud(pud: pud_t) -> bool { pud_val(pud) & _PAGE_PUD_HUGE as usize != 0 }
#[cfg(not(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE)))]
pub fn is_hugetlb_pte(_: pte_t) -> bool { false }

pub unsafe fn __pte_mkhwwrite(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_W_4U as usize) }
pub unsafe fn pte_mkdirty(pte: pte_t) -> pte_t {
    let p = __pte(pte_val(pte) | _PAGE_MODIFIED_4U as usize);
    if pte_write(p) != 0 { __pte_mkhwwrite(p) } else { p }
}
pub unsafe fn pte_mkclean(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !(_PAGE_MODIFIED_4U | _PAGE_W_4U) as usize) }
pub unsafe fn pte_mkwrite_novma(pte: pte_t) -> pte_t {
    let p = __pte(pte_val(pte) | _PAGE_WRITE_4U as usize);
    if pte_dirty(p) != 0 { __pte_mkhwwrite(p) } else { p }
}
pub unsafe fn pte_wrprotect(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !(_PAGE_WRITE_4U | _PAGE_W_4U) as usize) }
pub unsafe fn pte_mkold(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !(_PAGE_ACCESSED_4U | _PAGE_R) as usize) }
pub unsafe fn pte_mkyoung(pte: pte_t) -> pte_t { __pte(pte_val(pte) | (_PAGE_ACCESSED_4U | _PAGE_R) as usize) }
pub unsafe fn pte_mkspecial(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_SPECIAL as usize; pte }
pub unsafe fn pte_mkmcd(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_MCD_4V as usize; pte }
pub unsafe fn pte_mknotmcd(mut pte: pte_t) -> pte_t { pte_val(pte) &= !(_PAGE_MCD_4V as usize); pte }
pub unsafe fn pte_young(pte: pte_t) -> usize { pte_val(pte) & _PAGE_ACCESSED_4U as usize }
pub unsafe fn pte_exec(pte: pte_t) -> usize { pte_val(pte) & _PAGE_EXEC_4U as usize }
pub unsafe fn pte_present(pte: pte_t) -> usize { pte_val(pte) & _PAGE_PRESENT_4U as usize }
pub unsafe fn pte_accessible(_: *mut mm_struct, a: pte_t) -> usize { pte_val(a) & _PAGE_VALID as usize }
pub unsafe fn pte_special(pte: pte_t) -> usize { pte_val(pte) & _PAGE_SPECIAL as usize }

pub unsafe fn pmd_leaf(pmd: pmd_t) -> bool { pmd_val(pmd) & _PAGE_PMD_HUGE as usize != 0 }
pub unsafe fn pmd_pfn(pmd: pmd_t) -> usize { pte_pfn(__pte(pmd_val(pmd))) }
pub unsafe fn pmd_write(pmd: pmd_t) -> usize { pte_write(__pte(pmd_val(pmd))) }
pub unsafe fn pud_write(pud: pud_t) -> usize { pte_write(__pte(pud_val(pud))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_dirty(pmd: pmd_t) -> usize { pte_dirty(__pte(pmd_val(pmd))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_young(pmd: pmd_t) -> usize { pte_young(__pte(pmd_val(pmd))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pte_young(pte: pte_t) -> usize { pte_val(pte) & _PAGE_ACCESSED_4U as usize }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_trans_huge(pmd: pmd_t) -> usize { pte_val(__pte(pmd_val(pmd))) & _PAGE_PMD_HUGE as usize }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_mkold(pmd: pmd_t) -> pmd_t { __pmd(pte_val(pte_mkold(__pte(pmd_val(pmd))))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_wrprotect(pmd: pmd_t) -> pmd_t { __pmd(pte_val(pte_wrprotect(__pte(pmd_val(pmd))))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_mkdirty(pmd: pmd_t) -> pmd_t { __pmd(pte_val(pte_mkdirty(__pte(pmd_val(pmd))))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_mkclean(pmd: pmd_t) -> pmd_t { __pmd(pte_val(pte_mkclean(__pte(pmd_val(pmd))))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_mkyoung(pmd: pmd_t) -> pmd_t { __pmd(pte_val(pte_mkyoung(__pte(pmd_val(pmd))))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_mkwrite_novma(pmd: pmd_t) -> pmd_t { __pmd(pte_val(pte_mkwrite_novma(__pte(pmd_val(pmd))))) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmd_pgprot(entry: pmd_t) -> pgprot_t { __pgprot(pmd_val(entry)) }

pub unsafe fn pmd_present(pmd: pmd_t) -> i32 { (pmd_val(pmd) != 0) as i32 }
pub unsafe fn pmd_none(pmd: pmd_t) -> bool { pmd_val(pmd) == 0 }
pub unsafe fn pmd_bad(pmd: pmd_t) -> usize { pmd_val(pmd) & !PAGE_MASK }
pub unsafe fn pud_none(pud: pud_t) -> bool { pud_val(pud) == 0 }
pub unsafe fn pud_bad(pud: pud_t) -> usize { pud_val(pud) & !PAGE_MASK }
pub unsafe fn p4d_none(p4d: p4d_t) -> bool { p4d_val(p4d) == 0 }
pub unsafe fn p4d_bad(p4d: p4d_t) -> usize { p4d_val(p4d) & !PAGE_MASK }

extern "C" {
    pub static mut init_mm: mm_struct;
    pub fn tlb_batch_add(mm: *mut mm_struct, vaddr: usize, ptep: *mut pte_t, orig: pte_t, fullmm: i32, hugepage_shift: u32);
    pub fn paging_init();
    pub fn find_ecache_flush_span(size: usize) -> usize;
    pub fn mmu_info(seq: *mut seq_file);
    pub fn update_mmu_cache_range(fault: *mut vm_fault, vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t, nr: u32);
    pub fn page_in_phys_avail(paddr: usize) -> i32;
    pub fn adi_restore_tags(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: usize, pte: pte_t);
    pub fn adi_save_tags(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: usize, oldpte: pte_t) -> i32;
    pub fn get_fb_unmapped_area(filp: *mut file, a: usize, b: usize, c: usize, d: usize) -> usize;
    pub fn sun4v_register_fault_status();
    pub fn sun4v_ktsb_register();
    pub fn cheetah_ecache_flush_init();
    pub fn sun4v_patch_tlb_handlers();
    pub static mut cmdline_memory_size: usize;
    pub fn do_sparc64_fault(regs: *mut pt_regs);
}

pub unsafe fn pmd_set(_: *mut mm_struct, pmdp: *mut pmd_t, ptep: *mut pte_t) { pmd_val(*pmdp) = __pa(ptep as usize); }
pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> usize { __va(pte_pfn(__pte(pmd_val(pmd))) << PAGE_SHIFT) }
pub unsafe fn pud_pgtable(pud: pud_t) -> *mut pmd_t { __va((pte_pfn(__pte(pud_val(pud))) << PAGE_SHIFT) as usize) as *mut pmd_t }
pub unsafe fn pud_leaf(pud: pud_t) -> bool { pud_val(pud) & _PAGE_PMD_HUGE as usize != 0 }
pub unsafe fn pud_pfn(pud: pud_t) -> usize { pte_pfn(__pte(pud_val(pud))) }
pub unsafe fn pte_none(pte: pte_t) -> bool { pte_val(pte) == 0 }
pub unsafe fn pte_swp_exclusive(pte: pte_t) -> bool { pte_val(pte) & _PAGE_SWP_EXCLUSIVE as usize != 0 }
pub unsafe fn pte_swp_mkexclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_SWP_EXCLUSIVE as usize) }
pub unsafe fn pte_swp_clear_exclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !(_PAGE_SWP_EXCLUSIVE as usize)) }
pub unsafe fn io_remap_pfn_range_pfn(pfn: usize, size: usize) -> usize { let _ = size; ((GET_PFN(pfn) << PAGE_SHIFT) | ((GET_IOSPACE(pfn) as usize) << 32)) >> PAGE_SHIFT }
pub unsafe fn __untagged_addr(start: usize) -> usize { if adi_capable() { let n = adi_nbits() as usize; ((start << n) as isize >> n) as usize } else { start } }
pub unsafe fn pte_access_permitted(pte: pte_t, write: bool) -> bool {
    let mut prot = if tlb_type == hypervisor { _PAGE_PRESENT_4V | _PAGE_P_4V } else { _PAGE_PRESENT_4U | _PAGE_P_4U };
    if write { prot |= if tlb_type == hypervisor { _PAGE_WRITE_4V } else { _PAGE_WRITE_4U }; }
    (pte_val(pte) as u64 & (prot | _PAGE_SPECIAL)) == prot
}

pub unsafe fn arch_do_swap_page(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: usize, pte: pte_t, oldpte: pte_t) { if !pte_none(oldpte) && adi_state.enabled && pte_val(pte) & _PAGE_MCD_4V as usize != 0 { adi_restore_tags(mm, vma, addr, pte); } }
pub unsafe fn arch_unmap_one(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: usize, oldpte: pte_t) -> i32 { if adi_state.enabled && pte_val(oldpte) & _PAGE_MCD_4V as usize != 0 { adi_save_tags(mm, vma, addr, oldpte) } else { 0 } }
pub unsafe fn GET_IOSPACE(pfn: usize) -> usize { pfn >> (BITS_PER_LONG - 4) }
pub unsafe fn GET_PFN(pfn: usize) -> usize { pfn & 0x0fff_ffff_ffff_ffff }

// External kernel types, constructors, accessors, globals, and configuration symbols are supplied by dependent headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
