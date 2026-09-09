// SPDX-License-Identifier: GPL-2.0
// Translated from Linux kernel C source; symbols from included kernel headers
// remain external dependencies.

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn huge_ptep_get(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t) -> pte_t {
    let mut pte_num: c_ulong;
    let orig_pte = ptep_get(ptep);
    if !pte_present(orig_pte) || !pte_napot(orig_pte) { return orig_pte; }
    pte_num = napot_pte_num(napot_cont_order(orig_pte));
    let mut result = orig_pte;
    for _ in 0..pte_num {
        let pte = ptep_get(ptep);
        if pte_dirty(pte) { result = pte_mkdirty(result); }
        if pte_young(pte) { result = pte_mkyoung(result); }
        ptep = ptep.add(1);
    }
    result
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn huge_pte_alloc(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: c_ulong, sz: c_ulong) -> *mut pte_t {
    let mut pte: *mut pte_t = core::ptr::null_mut();
    let pgd = pgd_offset(mm, addr);
    let p4d = p4d_alloc(mm, pgd, addr); if p4d.is_null() { return core::ptr::null_mut(); }
    let pud = pud_alloc(mm, p4d, addr); if pud.is_null() { return core::ptr::null_mut(); }
    if sz == PUD_SIZE { pte = pud as *mut pte_t; }
    else if sz == PMD_SIZE {
        if want_pmd_share(vma, addr) && pud_none(pudp_get(pud)) { pte = huge_pmd_share(mm, vma, addr, pud); }
        else { pte = pmd_alloc(mm, pud, addr) as *mut pte_t; }
    } else {
        let pmd = pmd_alloc(mm, pud, addr); if pmd.is_null() { return core::ptr::null_mut(); }
        for_each_napot_order!(order) {
            if napot_cont_size(order) == sz { pte = pte_alloc_huge(mm, pmd, addr & napot_cont_mask(order)); break; }
        }
    }
    if !pte.is_null() {
        let pteval = ptep_get_lockless(pte);
        WARN_ON_ONCE!(pte_present(pteval) && !pte_huge(pteval));
    }
    pte
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn huge_pte_offset(mm: *mut mm_struct, addr: c_ulong, sz: c_ulong) -> *mut pte_t {
    let pgd = pgd_offset(mm, addr); if !pgd_present(pgdp_get(pgd)) { return core::ptr::null_mut(); }
    let p4d = p4d_offset(pgd, addr); if !p4d_present(p4dp_get(p4d)) { return core::ptr::null_mut(); }
    let pud = pud_offset(p4d, addr);
    if sz == PUD_SIZE { return pud as *mut pte_t; }
    if !pud_present(pudp_get(pud)) { return core::ptr::null_mut(); }
    let pmd = pmd_offset(pud, addr);
    if sz == PMD_SIZE { return pmd as *mut pte_t; }
    if !pmd_present(pmdp_get(pmd)) { return core::ptr::null_mut(); }
    let mut pte: *mut pte_t = core::ptr::null_mut();
    for_each_napot_order!(order) {
        if napot_cont_size(order) == sz { pte = pte_offset_huge(pmd, addr & napot_cont_mask(order)); break; }
    }
    pte
}

pub unsafe fn hugetlb_mask_last_page(h: *mut hstate) -> c_ulong {
    match huge_page_size(h) {
        #[cfg(not(__PAGETABLE_PMD_FOLDED))] PUD_SIZE => P4D_SIZE - PUD_SIZE,
        PMD_SIZE => PUD_SIZE - PMD_SIZE,
        x if x == napot_cont_size(NAPOT_CONT64KB_ORDER) => PMD_SIZE - napot_cont_size(NAPOT_CONT64KB_ORDER),
        _ => 0,
    }
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
unsafe fn get_clear_contig(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t, mut ncontig: c_ulong) -> pte_t {
    let mut pte = ptep_get_and_clear(mm, addr, ptep); let present = pte_present(pte);
    while { ncontig -= 1; ncontig != 0 } {
        ptep = ptep.add(1); addr += PAGE_SIZE; let tmp = ptep_get_and_clear(mm, addr, ptep);
        if present { if pte_dirty(tmp) { pte = pte_mkdirty(pte); } if pte_young(tmp) { pte = pte_mkyoung(pte); } }
    }
    pte
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
unsafe fn get_clear_contig_flush(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, n: c_ulong) -> pte_t {
    let pte = get_clear_contig(mm, addr, ptep, n); let vma = TLB_FLUSH_VMA!(mm, 0);
    if !pte_none(pte) { flush_tlb_range(&vma, addr, addr + PAGE_SIZE * n); } pte
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn arch_make_huge_pte(mut entry: pte_t, shift: c_uint, flags: vm_flags_t) -> pte_t {
    let mut matched = false;
    for_each_napot_order!(order) { if shift == napot_cont_shift(order) { entry = pte_mknapot(entry, order); matched = true; break; } }
    if !matched { entry = pte_mkhuge(entry); } entry
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
unsafe fn clear_flush(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t, pgsize: c_ulong, ncontig: c_ulong) {
    let vma = TLB_FLUSH_VMA!(mm, 0); let saddr = addr;
    for _ in 0..ncontig { ptep_get_and_clear(mm, addr, ptep); addr += pgsize; ptep = ptep.add(1); }
    flush_tlb_range(&vma, saddr, addr);
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
unsafe fn num_contig_ptes_from_size(sz: c_ulong, pgsize: *mut usize) -> c_int {
    let shift = if sz >= PGDIR_SIZE { PGDIR_SHIFT } else if sz >= P4D_SIZE { P4D_SHIFT } else if sz >= PUD_SIZE { PUD_SHIFT } else if sz >= PMD_SIZE { PMD_SHIFT } else { PAGE_SHIFT };
    *pgsize = (1u64 << shift) as usize; (sz >> shift) as c_int
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn set_huge_pte_at(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t, pte: pte_t, sz: c_ulong) {
    let mut pgsize = 0usize; let n = num_contig_ptes_from_size(sz, &mut pgsize) as usize;
    if !pte_present(pte) { for _ in 0..n { set_ptes(mm, addr, ptep, pte, 1); addr += pgsize as c_ulong; ptep = ptep.add(1); } return; }
    if !pte_napot(pte) { set_ptes(mm, addr, ptep, pte, 1); return; }
    clear_flush(mm, addr, ptep, pgsize as c_ulong, n as c_ulong);
    for _ in 0..n { set_pte_at(mm, addr, ptep, pte); addr += pgsize as c_ulong; ptep = ptep.add(1); }
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn huge_ptep_set_access_flags(vma: *mut vm_area_struct, mut addr: c_ulong, mut ptep: *mut pte_t, mut pte: pte_t, dirty: c_int) -> bool {
    let mm = (*vma).vm_mm; if !pte_napot(pte) { return ptep_set_access_flags(vma, addr, ptep, pte, dirty); }
    let order = napot_cont_order(pte); let n = napot_pte_num(order); ptep = huge_pte_offset(mm, addr, napot_cont_size(order));
    let orig = get_clear_contig_flush(mm, addr, ptep, n); if pte_dirty(orig) { pte = pte_mkdirty(pte); } if pte_young(orig) { pte = pte_mkyoung(pte); }
    for _ in 0..n { set_pte_at(mm, addr, ptep, pte); addr += PAGE_SIZE; ptep = ptep.add(1); } true
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn huge_ptep_get_and_clear(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, sz: c_ulong) -> pte_t {
    let pte = ptep_get(ptep); if !pte_napot(pte) { return ptep_get_and_clear(mm, addr, ptep); }
    let mut pgsize = 0usize; let n = num_contig_ptes_from_size(sz, &mut pgsize); get_clear_contig(mm, addr, ptep, n as c_ulong)
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn huge_ptep_set_wrprotect(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t) {
    let pte = ptep_get(ptep); if !pte_napot(pte) { ptep_set_wrprotect(mm, addr, ptep); return; }
    let order = napot_cont_order(pte); let n = napot_pte_num(order); ptep = huge_pte_offset(mm, addr, napot_cont_size(order));
    let pte = pte_wrprotect(get_clear_contig_flush(mm, addr, ptep, n));
    for _ in 0..n { set_pte_at(mm, addr, ptep, pte); addr += PAGE_SIZE; ptep = ptep.add(1); }
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn huge_ptep_clear_flush(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t {
    let pte = ptep_get(ptep); if !pte_napot(pte) { return ptep_clear_flush(vma, addr, ptep); }
    get_clear_contig_flush((*vma).vm_mm, addr, ptep, napot_pte_num(napot_cont_order(pte)))
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub unsafe fn huge_pte_clear(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t, sz: c_ulong) {
    let pte = ptep_get(ptep); if !pte_napot(pte) { pte_clear(mm, addr, ptep); return; }
    let mut pgsize = 0usize; let n = num_contig_ptes_from_size(sz, &mut pgsize) as usize;
    for _ in 0..n { pte_clear(mm, addr, ptep); addr += pgsize as c_ulong; ptep = ptep.add(1); }
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
unsafe fn is_napot_size(size: c_ulong) -> bool {
    if !has_svnapot() { return false; } for_each_napot_order!(order) { if size == napot_cont_size(order) { return true; } } false
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
unsafe fn napot_hugetlbpages_init() -> c_int { if has_svnapot() { for_each_napot_order!(order) { hugetlb_add_hstate(order); } } 0 }
#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
arch_initcall!(napot_hugetlbpages_init);

#[cfg(not(CONFIG_RISCV_ISA_SVNAPOT))]
unsafe fn is_napot_size(size: c_ulong) -> bool { false }

unsafe fn __hugetlb_valid_size(size: c_ulong) -> bool {
    size == HPAGE_SIZE || (IS_ENABLED!(CONFIG_64BIT) && size == PUD_SIZE) || is_napot_size(size)
}
pub unsafe fn arch_hugetlb_valid_size(size: c_ulong) -> bool { __hugetlb_valid_size(size) }

#[cfg(CONFIG_ARCH_ENABLE_HUGEPAGE_MIGRATION)]
pub unsafe fn arch_hugetlb_migration_supported(h: *mut hstate) -> bool { __hugetlb_valid_size(huge_page_size(h)) }

#[cfg(CONFIG_CONTIG_ALLOC)]
unsafe fn gigantic_pages_init() -> c_int { if IS_ENABLED!(CONFIG_64BIT) { hugetlb_add_hstate(PUD_SHIFT - PAGE_SHIFT); } 0 }
#[cfg(CONFIG_CONTIG_ALLOC)]
arch_initcall!(gigantic_pages_init);

pub unsafe fn arch_hugetlb_cma_order() -> c_uint { if IS_ENABLED!(CONFIG_64BIT) { PUD_SHIFT - PAGE_SHIFT } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
