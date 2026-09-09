// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains common routines for dealing with free of page tables
 * Along with common page table handling code
 *
 * Derived from arch/powerpc/mm/tlb_64.c and arch/i386/mm/init.c.
 */

// C header dependencies are supplied by the surrounding kernel translation.

#[cfg(CONFIG_PPC64)]
const PGD_ALIGN: usize = core::mem::size_of::<pgd_t>() * MAX_PTRS_PER_PGD;
#[cfg(not(CONFIG_PPC64))]
const PGD_ALIGN: usize = PAGE_SIZE;

#[link_section = ".bss..page_aligned"]
#[repr(align(4096))]
pub static mut swapper_pg_dir: [pgd_t; MAX_PTRS_PER_PGD] = [unsafe { core::mem::zeroed() }; MAX_PTRS_PER_PGD];

#[inline]
unsafe fn is_exec_fault() -> i32 {
    (current.thread.regs != core::ptr::null_mut() && TRAP(current.thread.regs) == 0x400) as i32
}

#[inline]
unsafe fn pte_looks_normal(pte: pte_t, addr: c_ulong) -> i32 {
    if pte_present(pte) && !pte_special(pte) {
        if pte_ci(pte) { return 0; }
        if !is_kernel_addr(addr) { return 1; }
    }
    0
}

unsafe fn maybe_pte_to_folio(pte: pte_t) -> *mut folio {
    let pfn = pte_pfn(pte);
    if unlikely(!pfn_valid(pfn)) { return core::ptr::null_mut(); }
    let page = pfn_to_page(pfn);
    if PageReserved(page) { return core::ptr::null_mut(); }
    page_folio(page)
}

#[cfg(CONFIG_PPC_BOOK3S)]
unsafe fn set_pte_filter_hash(mut pte: pte_t, addr: c_ulong) -> pte_t {
    pte = __pte(pte_val(pte) & !_PAGE_HPTEFLAGS);
    if pte_looks_normal(pte, addr) != 0 && !(cpu_has_feature(CPU_FTR_COHERENT_ICACHE) || cpu_has_feature(CPU_FTR_NOEXECUTE)) {
        let folio = maybe_pte_to_folio(pte);
        if folio.is_null() { return pte; }
        if !test_bit(PG_dcache_clean, &(*folio).flags.f) {
            flush_dcache_icache_folio(folio);
            set_bit(PG_dcache_clean, &mut (*folio).flags.f);
        }
    }
    pte
}

#[cfg(not(CONFIG_PPC_BOOK3S))]
unsafe fn set_pte_filter_hash(pte: pte_t, _addr: c_ulong) -> pte_t { pte }

#[inline]
unsafe fn set_pte_filter(pte: pte_t, addr: c_ulong) -> pte_t {
    if radix_enabled() { return pte; }
    if mmu_has_feature(MMU_FTR_HPTE_TABLE) { return set_pte_filter_hash(pte, addr); }
    if !pte_exec(pte) || pte_looks_normal(pte, addr) == 0 { return pte; }
    let folio = maybe_pte_to_folio(pte);
    if folio.is_null() { return pte; }
    if test_bit(PG_dcache_clean, &(*folio).flags.f) { return pte; }
    if is_exec_fault() != 0 {
        flush_dcache_icache_folio(folio);
        set_bit(PG_dcache_clean, &mut (*folio).flags.f);
        return pte;
    }
    pte_exprotect(pte)
}

unsafe fn set_access_flags_filter(mut pte: pte_t, vma: *mut vm_area_struct, dirty: i32) -> pte_t {
    if IS_ENABLED(CONFIG_PPC_BOOK3S_64) || mmu_has_feature(MMU_FTR_HPTE_TABLE) { return pte; }
    if dirty != 0 || pte_exec(pte) || is_exec_fault() == 0 { return pte; }
    #[cfg(CONFIG_DEBUG_VM)]
    if WARN_ON((*vma).vm_flags & VM_EXEC == 0) { return pte; }
    let folio = maybe_pte_to_folio(pte);
    if folio.is_null() { return pte_mkexec(pte); }
    if !test_bit(PG_dcache_clean, &(*folio).flags.f) {
        flush_dcache_icache_folio(folio);
        set_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
    pte_mkexec(pte)
}

pub unsafe fn set_ptes(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t, mut pte: pte_t, mut nr: c_uint) {
    pte = set_pte_filter(pte, addr);
    page_table_check_ptes_set(mm, addr, ptep, pte, nr);
    loop {
        VM_WARN_ON(pte_hw_valid(*ptep) && !pte_protnone(*ptep));
        __set_pte_at(mm, addr, ptep, pte, 0);
        nr -= 1;
        if nr == 0 { break; }
        ptep = ptep.add(1);
        addr += PAGE_SIZE;
        pte = pte_next_pfn(pte);
    }
}

pub unsafe fn set_pte_at_unchecked(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, mut pte: pte_t) {
    VM_WARN_ON(pte_hw_valid(*ptep) && !pte_protnone(*ptep));
    pte = set_pte_filter(pte, addr);
    __set_pte_at(mm, addr, ptep, pte, 0);
}

pub unsafe fn unmap_kernel_page(va: c_ulong) {
    let pmdp = pmd_off_k(va);
    let ptep = pte_offset_kernel(pmdp, va);
    pte_clear(&mut init_mm, va, ptep);
    flush_tlb_kernel_range(va, va + PAGE_SIZE);
}

pub unsafe fn ptep_set_access_flags(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t, entry: pte_t, dirty: i32) -> i32 {
    let entry = set_access_flags_filter(entry, vma, dirty);
    let changed = (!pte_same(*ptep, entry)) as i32;
    if changed != 0 {
        assert_pte_locked((*vma).vm_mm, address);
        __ptep_set_access_flags(vma, ptep, entry, address, mmu_virtual_psize);
    }
    changed
}

#[cfg(CONFIG_HUGETLB_PAGE)]
pub unsafe fn huge_ptep_set_access_flags(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t, pte: pte_t, dirty: i32) -> i32 {
    #[cfg(HUGETLB_NEED_PRELOAD)] { ptep_set_access_flags(vma, addr, ptep, pte, dirty); return 1; }
    #[cfg(not(HUGETLB_NEED_PRELOAD))]
    {
        let pte = set_access_flags_filter(pte, vma, dirty);
        let changed = (!pte_same(*ptep, pte)) as i32;
        if changed != 0 {
            #[cfg(CONFIG_PPC_BOOK3S_64)]
            let psize = { let h = hstate_vma(vma); #[cfg(CONFIG_DEBUG_VM)] assert_spin_locked(huge_pte_lockptr(h, (*vma).vm_mm, ptep)); hstate_get_psize(h) };
            #[cfg(not(CONFIG_PPC_BOOK3S_64))]
            let psize = MMU_PAGE_COUNT;
            __ptep_set_access_flags(vma, ptep, pte, addr, psize);
        }
        changed
    }
}

#[cfg(CONFIG_HUGETLB_PAGE)]
pub unsafe fn set_huge_pte_at(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t, mut pte: pte_t, sz: c_ulong) {
    pte = set_pte_filter(pte, addr);
    VM_WARN_ON(pte_hw_valid(*ptep) && !pte_protnone(*ptep));
    let pdsize = if sz < PMD_SIZE { PAGE_SIZE } else if sz < PUD_SIZE { PMD_SIZE } else if sz < P4D_SIZE { PUD_SIZE } else if sz < PGDIR_SIZE { P4D_SIZE } else { PGDIR_SIZE };
    for _ in 0..(sz / pdsize) {
        __set_pte_at(mm, addr, ptep, pte, 0);
        ptep = ptep.add(1); addr += pdsize;
        pte = __pte(pte_val(pte) + ((pdsize as u64 / PAGE_SIZE as u64) << PFN_PTE_SHIFT));
    }
}

#[cfg(CONFIG_DEBUG_VM)]
pub unsafe fn assert_pte_locked(mm: *mut mm_struct, addr: c_ulong) {
    if mm == &mut init_mm { return; }
    let pgd = (*mm).pgd.add(pgd_index(addr)); BUG_ON(pgd_none(*pgd));
    let p4d = p4d_offset(pgd, addr); BUG_ON(p4d_none(*p4d));
    let pud = pud_offset(p4d, addr); BUG_ON(pud_none(*pud));
    let pmd = pmd_offset(pud, addr); if pmd_none(*pmd) { return; }
    let mut ptl: *mut spinlock_t = core::ptr::null_mut();
    let pte = pte_offset_map_ro_nolock(mm, pmd, addr, &mut ptl);
    BUG_ON(pte.is_null()); assert_spin_locked(ptl); pte_unmap(pte);
}

pub unsafe fn vmalloc_to_phys(va: *mut core::ffi::c_void) -> c_ulong {
    let pfn = vmalloc_to_pfn(va); BUG_ON(!pfn); __pa(pfn_to_kaddr(pfn)) + offset_in_page(va)
}

pub unsafe fn __find_linux_pte(pgdir: *mut pgd_t, ea: c_ulong, is_thp: *mut bool, hpage_shift: *mut c_uint) -> *mut pte_t {
    if !hpage_shift.is_null() { *hpage_shift = 0; } if !is_thp.is_null() { *is_thp = false; }
    let pgdp = pgdir.add(pgd_index(ea));
    #[cfg(CONFIG_PPC64)] { let p4dp = p4d_offset(pgdp, ea); let p4d = READ_ONCE(*p4dp); if p4d_none(p4d) { return core::ptr::null_mut(); } if p4d_leaf(p4d) { if !hpage_shift.is_null() {*hpage_shift=P4D_SHIFT;} return p4dp as *mut pte_t; } let pudp=pud_offset(&p4d,ea); let pud=READ_ONCE(*pudp); if pud_none(pud){return core::ptr::null_mut();} if pud_leaf(pud){if !hpage_shift.is_null(){*hpage_shift=PUD_SHIFT;} return pudp as *mut pte_t;} let pmdp=pmd_offset(&pud,ea); let pmd=READ_ONCE(*pmdp); if pmd_none(pmd){return core::ptr::null_mut();} if pmd_trans_huge(pmd){if !is_thp.is_null(){*is_thp=true;} if !hpage_shift.is_null(){*hpage_shift=PMD_SHIFT;} return pmdp as *mut pte_t;} if pmd_leaf(pmd){if !hpage_shift.is_null(){*hpage_shift=PMD_SHIFT;} return pmdp as *mut pte_t;} return pte_offset_kernel(&pmd,ea); }
    #[cfg(not(CONFIG_PPC64))] { let pmdp=pmd_offset(pud_offset(p4d_offset(pgdp,ea),ea),ea); let pmd=READ_ONCE(*pmdp); if pmd_none(pmd){return core::ptr::null_mut();} if pmd_trans_huge(pmd)||pmd_leaf(pmd){if !is_thp.is_null(){*is_thp=pmd_trans_huge(pmd);} if !hpage_shift.is_null(){*hpage_shift=PMD_SHIFT;} return pmdp as *mut pte_t;} pte_offset_kernel(&pmd,ea) }
}

pub static protection_map: [pgprot_t; 16] = [PAGE_NONE, PAGE_READONLY, PAGE_COPY, PAGE_COPY, PAGE_EXECONLY_X, PAGE_READONLY_X, PAGE_COPY_X, PAGE_COPY_X, PAGE_NONE, PAGE_READONLY, PAGE_SHARED, PAGE_SHARED, PAGE_EXECONLY_X, PAGE_READONLY_X, PAGE_SHARED_X, PAGE_SHARED_X];

#[cfg(not(CONFIG_PPC_BOOK3S_64))]
DECLARE_VM_GET_PAGE_PROT!();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
