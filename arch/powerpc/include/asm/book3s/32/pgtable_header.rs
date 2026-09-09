/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of asm/powerpc/book3s/32/pgtable.h. */

pub const _PAGE_PRESENT: usize = 0x001;
pub const _PAGE_HASHPTE: usize = 0x002;
pub const _PAGE_READ: usize = 0x004;
pub const _PAGE_GUARDED: usize = 0x008;
pub const _PAGE_COHERENT: usize = 0x010;
pub const _PAGE_NO_CACHE: usize = 0x020;
pub const _PAGE_WRITETHRU: usize = 0x040;
pub const _PAGE_DIRTY: usize = 0x080;
pub const _PAGE_ACCESSED: usize = 0x100;
pub const _PAGE_EXEC: usize = 0x200;
pub const _PAGE_WRITE: usize = 0x400;
pub const _PAGE_SPECIAL: usize = 0x800;

#[cfg(feature = "CONFIG_PTE_64BIT")]
pub const _PTE_NONE_MASK: u64 = 0xffffffff00000000u64 | _PAGE_HASHPTE as u64;
#[cfg(not(feature = "CONFIG_PTE_64BIT"))]
pub const _PTE_NONE_MASK: usize = _PAGE_HASHPTE;
pub const _PMD_PRESENT: usize = 0;
pub const _PAGE_SWP_EXCLUSIVE: usize = _PAGE_READ;
pub const _PAGE_HPTEFLAGS: usize = _PAGE_HASHPTE;

pub const PTE_RPN_SHIFT: usize = PAGE_SHIFT;
#[cfg(feature = "CONFIG_PTE_64BIT")]
pub const PTE_RPN_MASK: u64 = !((1u64 << PTE_RPN_SHIFT) - 1);
#[cfg(not(feature = "CONFIG_PTE_64BIT"))]
pub const PTE_RPN_MASK: usize = !((1usize << PTE_RPN_SHIFT) - 1);
#[cfg(feature = "CONFIG_PTE_64BIT")]
pub const MAX_POSSIBLE_PHYSMEM_BITS: usize = 36;
#[cfg(not(feature = "CONFIG_PTE_64BIT"))]
pub const MAX_POSSIBLE_PHYSMEM_BITS: usize = 32;

pub const _PAGE_CHG_MASK: usize = PTE_RPN_MASK as usize | _PAGE_HASHPTE | _PAGE_DIRTY | _PAGE_ACCESSED | _PAGE_SPECIAL;
pub const _PAGE_BASE_NC: usize = _PAGE_PRESENT | _PAGE_ACCESSED;
pub const _PAGE_BASE: usize = _PAGE_BASE_NC | _PAGE_COHERENT;
pub const PTE_INDEX_SIZE: usize = PTE_SHIFT;
pub const PMD_INDEX_SIZE: usize = 0;
pub const PUD_INDEX_SIZE: usize = 0;
pub const PGD_INDEX_SIZE: usize = 32 - PGDIR_SHIFT;
pub const PMD_CACHE_INDEX: usize = PMD_INDEX_SIZE;
pub const PUD_CACHE_INDEX: usize = PUD_INDEX_SIZE;
pub const PTRS_PER_PTE: usize = 1 << PTE_INDEX_SIZE;
pub const PTRS_PER_PGD: usize = 1 << PGD_INDEX_SIZE;
pub const PGDIR_SHIFT: usize = PAGE_SHIFT + PTE_INDEX_SIZE;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
pub const FIXADDR_SIZE: usize = 0;
pub const VMALLOC_OFFSET: usize = 0x1000000;
pub const _PAGE_CACHE_CTL: usize = _PAGE_COHERENT | _PAGE_GUARDED | _PAGE_NO_CACHE | _PAGE_WRITETHRU;

pub const fn pmd_none(pmd: pmd_t) -> bool { pmd_val(pmd) == 0 }
pub const fn pmd_bad(pmd: pmd_t) -> usize { pmd_val(pmd) & !_PMD_PRESENT_MASK }
pub const fn pmd_present(pmd: pmd_t) -> usize { pmd_val(pmd) & _PMD_PRESENT_MASK }

pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { *pmdp = __pmd(0); }

extern "C" {
    pub fn map_kernel_page(va: usize, pa: phys_addr_t, prot: pgprot_t) -> i32;
    pub fn unmap_kernel_page(va: usize);
    pub fn flush_hash_pages(context: u32, va: usize, pmdval: usize, count: i32) -> i32;
    pub fn add_hash_page(context: u32, va: usize, pmdval: usize);
}

pub unsafe fn flush_hash_entry(mm: *mut mm_struct, ptep: *mut pte_t, addr: usize) {
    if mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        let ptephys = __pa(ptep as usize) & PAGE_MASK;
        flush_hash_pages((*mm).context.id, addr, ptephys, 1);
    }
}

pub unsafe fn pte_update(mm: *mut mm_struct, addr: usize, p: *mut pte_t,
                         clr: usize, set: usize, _huge: i32) -> pte_basic_t {
    if mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        /* The source uses lwarx/stwcx. atomic PowerPC assembly here. */
        let old = pte_val(*p);
        let newval = (old & !clr) | set;
        *p = __pte(newval);
        old
    } else {
        let old = pte_val(*p);
        *p = __pte((old & !(clr as pte_basic_t)) | set);
        old
    }
}

pub unsafe fn __ptep_test_and_clear_young(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t) -> bool {
    let old = pte_update(mm, addr, ptep, _PAGE_ACCESSED, 0, 0);
    if old & _PAGE_HASHPTE as pte_basic_t != 0 { flush_hash_entry(mm, ptep, addr); }
    old & _PAGE_ACCESSED as pte_basic_t != 0
}
pub unsafe fn ptep_get_and_clear(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t) -> pte_t {
    let old = __pte(pte_update(mm, addr, ptep, !_PAGE_HASHPTE, 0, 0));
    page_table_check_pte_clear(mm, addr, old);
    old
}
pub unsafe fn ptep_set_wrprotect(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t) {
    pte_update(mm, addr, ptep, _PAGE_WRITE, 0, 0);
}
pub unsafe fn __ptep_set_access_flags(vma: *mut vm_area_struct, ptep: *mut pte_t, entry: pte_t, address: usize, _psize: i32) {
    let set = pte_val(entry) & (_PAGE_DIRTY | _PAGE_ACCESSED | _PAGE_RW | _PAGE_EXEC);
    pte_update((*vma).vm_mm, address, ptep, 0, set, 0);
    flush_tlb_page(vma, address);
}

pub fn pte_same(a: pte_t, b: pte_t) -> bool { ((pte_val(a) ^ pte_val(b)) & !_PAGE_HASHPTE) == 0 }
pub fn pmd_pfn(pmd: pmd_t) -> usize { pmd_val(pmd) >> PAGE_SHIFT }
pub fn pmd_page(pmd: pmd_t) -> *mut page { pfn_to_page(pmd_pfn(pmd)) }

pub fn swp_type(entry: swp_entry_t) -> usize { entry.val & 0x1f }
pub fn swp_offset(entry: swp_entry_t) -> usize { entry.val >> 5 }
pub fn swp_entry(ty: usize, offset: usize) -> swp_entry_t { swp_entry_t { val: (ty & 0x1f) | (offset << 5) } }
pub fn pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) >> 3 } }
pub fn swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val << 3) }

pub fn pte_swp_exclusive(pte: pte_t) -> bool { pte_val(pte) & _PAGE_SWP_EXCLUSIVE != 0 }
pub fn pte_swp_mkexclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_SWP_EXCLUSIVE) }
pub fn pte_swp_clear_exclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_SWP_EXCLUSIVE) }
pub fn pte_read(pte: pte_t) -> bool { pte_val(pte) & _PAGE_READ != 0 }
pub fn pte_write(pte: pte_t) -> bool { pte_val(pte) & _PAGE_WRITE != 0 }
pub fn pte_dirty(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_DIRTY != 0) as i32 }
pub fn pte_young(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_ACCESSED != 0) as i32 }
pub fn pte_special(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_SPECIAL != 0) as i32 }
pub fn pte_none(pte: pte_t) -> i32 { ((pte_val(pte) & !(_PTE_NONE_MASK as usize)) == 0) as i32 }
pub fn pte_exec(pte: pte_t) -> bool { pte_val(pte) & _PAGE_EXEC != 0 }
pub fn pte_present(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_PRESENT != 0) as i32 }
pub fn pte_hw_valid(pte: pte_t) -> bool { pte_present(pte) != 0 }
pub fn pte_hashpte(pte: pte_t) -> bool { pte_val(pte) & _PAGE_HASHPTE != 0 }
pub fn pte_ci(pte: pte_t) -> bool { pte_val(pte) & _PAGE_NO_CACHE != 0 }

pub fn pte_access_permitted(pte: pte_t, write: bool) -> bool {
    if pte_present(pte) == 0 || !pte_read(pte) { return false; }
    if write && !pte_write(pte) { return false; }
    true
}
pub unsafe fn pte_user_accessible_page(_mm: *mut mm_struct, addr: usize, pte: pte_t) -> bool {
    pte_present(pte) != 0 && !is_kernel_addr(addr)
}
pub fn pfn_pte(pfn: usize, pgprot: pgprot_t) -> pte_t { __pte(((pfn as pte_basic_t) << PTE_RPN_SHIFT) | pgprot_val(pgprot)) }

pub fn pte_wrprotect(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_WRITE) }
pub fn pte_exprotect(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_EXEC) }
pub fn pte_mkclean(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_DIRTY) }
pub fn pte_mkold(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_ACCESSED) }
pub fn pte_mkexec(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_EXEC) }
pub fn pte_mkpte(pte: pte_t) -> pte_t { pte }
pub fn pte_mkwrite_novma(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_RW) }
pub fn pte_mkdirty(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_DIRTY) }
pub fn pte_mkyoung(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_ACCESSED) }
pub fn pte_mkspecial(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_SPECIAL) }
pub fn pte_mkhuge(pte: pte_t) -> pte_t { pte }
pub fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t { __pte((pte_val(pte) & _PAGE_CHG_MASK) | pgprot_val(newprot)) }

pub unsafe fn __set_pte_at(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pte: pte_t, percpu: i32) {
    if percpu != 0 {
        *ptep = __pte((pte_val(*ptep) & _PAGE_HASHPTE) | (pte_val(pte) & !_PAGE_HASHPTE));
    } else if cfg!(feature = "CONFIG_PTE_64BIT") {
        if pte_val(*ptep) & _PAGE_HASHPTE != 0 { flush_hash_entry(mm, ptep, addr); }
        *ptep = pte;
    } else {
        pte_update(mm, addr, ptep, !_PAGE_HASHPTE, pte_val(pte), 0);
    }
}

pub fn pgprot_noncached(prot: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(prot) & !_PAGE_CACHE_CTL) | _PAGE_NO_CACHE | _PAGE_GUARDED) }
pub fn pgprot_noncached_wc(prot: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(prot) & !_PAGE_CACHE_CTL) | _PAGE_NO_CACHE) }
pub fn pgprot_cached(prot: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(prot) & !_PAGE_CACHE_CTL) | _PAGE_COHERENT) }
pub fn pgprot_cached_wthru(prot: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(prot) & !_PAGE_CACHE_CTL) | _PAGE_COHERENT | _PAGE_WRITETHRU) }
pub fn pgprot_cached_noncoherent(prot: pgprot_t) -> pgprot_t { __pgprot(pgprot_val(prot) & !_PAGE_CACHE_CTL) }
pub fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t { pgprot_noncached_wc(prot) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
