// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of arch/arm64/mm/hugetlbpage.c. */

#[cfg(feature = "CONFIG_CMA")]
pub unsafe fn arch_hugetlb_cma_order() -> ::core::ffi::c_uint {
    if pud_sect_supported() { PUD_SHIFT - PAGE_SHIFT } else { CONT_PMD_SHIFT - PAGE_SHIFT }
}

unsafe fn __hugetlb_valid_size(size: ::core::ffi::c_ulong) -> bool {
    match size {
        #[cfg(not(feature = "__PAGETABLE_PMD_FOLDED"))]
        PUD_SIZE => pud_sect_supported(),
        CONT_PMD_SIZE | PMD_SIZE | CONT_PTE_SIZE => true,
        _ => false,
    }
}

#[cfg(feature = "CONFIG_ARCH_ENABLE_HUGEPAGE_MIGRATION")]
pub unsafe fn arch_hugetlb_migration_supported(h: *mut hstate) -> bool {
    let pagesize = huge_page_size(h);
    if !__hugetlb_valid_size(pagesize) {
        pr_warn!("{}: unrecognized huge page size 0x{:x}\n", "arch_hugetlb_migration_supported", pagesize);
        return false;
    }
    true
}

unsafe fn find_num_contig(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, mut ptep: *mut pte_t, pgsize: *mut usize) -> ::core::ffi::c_int {
    let pgdp = pgd_offset(mm, addr);
    let p4dp = p4d_offset(pgdp, addr);
    let pudp = pud_offset(p4dp, addr);
    let pmdp = pmd_offset(pudp, addr);
    *pgsize = PAGE_SIZE as usize;
    if (pmdp as usize & !((::core::mem::size_of::<pmd_t>() * CONT_PMDS) - 1)) as *mut pte_t == ptep {
        *pgsize = PMD_SIZE as usize;
        return CONT_PMDS as ::core::ffi::c_int;
    }
    CONT_PTES as ::core::ffi::c_int
}

unsafe fn num_contig_ptes(size: ::core::ffi::c_ulong, pgsize: *mut usize) -> ::core::ffi::c_int {
    *pgsize = size as usize;
    match size {
        CONT_PMD_SIZE => { *pgsize = PMD_SIZE as usize; CONT_PMDS as ::core::ffi::c_int }
        CONT_PTE_SIZE => { *pgsize = PAGE_SIZE as usize; CONT_PTES as ::core::ffi::c_int }
        _ => { WARN_ON!(!__hugetlb_valid_size(size)); 1 }
    }
}

pub unsafe fn huge_ptep_get(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, mut ptep: *mut pte_t) -> pte_t {
    let mut orig_pte = __ptep_get(ptep);
    if !pte_present(orig_pte) || !pte_cont(orig_pte) { return orig_pte; }
    let mut pgsize = 0usize;
    let ncontig = find_num_contig(mm, addr, ptep, &mut pgsize);
    for _ in 0..ncontig {
        let pte = __ptep_get(ptep);
        if pte_dirty(pte) { orig_pte = pte_mkdirty(orig_pte); }
        if pte_young(pte) { orig_pte = pte_mkyoung(orig_pte); }
        ptep = ptep.add(1);
    }
    orig_pte
}

unsafe fn get_clear_contig(mm: *mut mm_struct, mut addr: ::core::ffi::c_ulong, mut ptep: *mut pte_t, pgsize: ::core::ffi::c_ulong, mut ncontig: ::core::ffi::c_ulong) -> pte_t {
    let mut pte = __ptep_get_and_clear_anysz(mm, addr, ptep, pgsize);
    let present = pte_present(pte);
    while { ncontig -= 1; ncontig != 0 } {
        ptep = ptep.add(1); addr += pgsize;
        let tmp_pte = __ptep_get_and_clear_anysz(mm, addr, ptep, pgsize);
        if present {
            if pte_dirty(tmp_pte) { pte = pte_mkdirty(pte); }
            if pte_young(tmp_pte) { pte = pte_mkyoung(pte); }
        }
    }
    pte
}

unsafe fn get_clear_contig_flush(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, ptep: *mut pte_t, pgsize: ::core::ffi::c_ulong, ncontig: ::core::ffi::c_ulong) -> pte_t {
    let orig_pte = get_clear_contig(mm, addr, ptep, pgsize, ncontig);
    let vma = TLB_FLUSH_VMA(mm, 0);
    __flush_hugetlb_tlb_range(&vma, addr, addr + pgsize * ncontig, pgsize, TLBF_NOWALKCACHE);
    orig_pte
}

unsafe fn clear_flush(mm: *mut mm_struct, mut addr: ::core::ffi::c_ulong, mut ptep: *mut pte_t, pgsize: ::core::ffi::c_ulong, ncontig: ::core::ffi::c_ulong) {
    let vma = TLB_FLUSH_VMA(mm, 0); let saddr = addr;
    for _ in 0..ncontig { __ptep_get_and_clear_anysz(mm, addr, ptep, pgsize); addr += pgsize; ptep = ptep.add(1); }
    if mm == &raw mut init_mm { flush_tlb_kernel_range(saddr, addr); }
    else { __flush_hugetlb_tlb_range(&vma, saddr, addr, pgsize, TLBF_NOWALKCACHE); }
}

pub unsafe fn set_huge_pte_at(mm: *mut mm_struct, mut addr: ::core::ffi::c_ulong, mut ptep: *mut pte_t, pte: pte_t, sz: ::core::ffi::c_ulong) {
    let mut pgsize = 0usize; let ncontig = num_contig_ptes(sz, &mut pgsize);
    if !pte_present(pte) { for _ in 0..ncontig { __set_ptes_anysz(mm, addr, ptep, pte, 1, pgsize as _); addr += pgsize as _; ptep = ptep.add(1); } return; }
    if pte_cont(pte) && pte_valid(__ptep_get(ptep)) { clear_flush(mm, addr, ptep, pgsize as _, ncontig as _); }
    __set_ptes_anysz(mm, addr, ptep, pte, ncontig, pgsize as _);
}

pub unsafe fn huge_pte_alloc(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong, sz: ::core::ffi::c_ulong) -> *mut pte_t {
    let pgdp = pgd_offset(mm, addr); let p4dp = p4d_alloc(mm, pgdp, addr); if p4dp.is_null() { return core::ptr::null_mut(); }
    let pudp = pud_alloc(mm, p4dp, addr); if pudp.is_null() { return core::ptr::null_mut(); }
    if sz == PUD_SIZE { pudp as *mut pte_t }
    else if sz == CONT_PTE_SIZE { let pmdp = pmd_alloc(mm, pudp, addr); if pmdp.is_null() { return core::ptr::null_mut(); } WARN_ON!(addr & (sz - 1) != 0); pte_alloc_huge(mm, pmdp, addr) }
    else if sz == PMD_SIZE { if want_pmd_share(vma, addr) && pud_none(READ_ONCE!(*pudp)) { huge_pmd_share(mm, vma, addr, pudp) } else { pmd_alloc(mm, pudp, addr) as *mut pte_t } }
    else if sz == CONT_PMD_SIZE { let pmdp = pmd_alloc(mm, pudp, addr); WARN_ON!(addr & (sz - 1) != 0); pmdp as *mut pte_t }
    else { core::ptr::null_mut() }
}

pub unsafe fn huge_pte_offset(mm: *mut mm_struct, mut addr: ::core::ffi::c_ulong, sz: ::core::ffi::c_ulong) -> *mut pte_t {
    let pgdp = pgd_offset(mm, addr); if !pgd_present(READ_ONCE!(*pgdp)) { return core::ptr::null_mut(); }
    let p4dp = p4d_offset(pgdp, addr); if !p4d_present(READ_ONCE!(*p4dp)) { return core::ptr::null_mut(); }
    let pudp = pud_offset(p4dp, addr); let pud = READ_ONCE!(*pudp);
    if sz != PUD_SIZE && pud_none(pud) { return core::ptr::null_mut(); }
    if pud_leaf(pud) || !pud_present(pud) { return pudp as *mut pte_t; }
    if sz == CONT_PMD_SIZE { addr &= CONT_PMD_MASK; }
    let pmdp = pmd_offset(pudp, addr); let pmd = READ_ONCE!(*pmdp);
    if sz != PMD_SIZE && sz != CONT_PMD_SIZE && pmd_none(pmd) { return core::ptr::null_mut(); }
    if pmd_leaf(pmd) || !pmd_present(pmd) { return pmdp as *mut pte_t; }
    if sz == CONT_PTE_SIZE { return pte_offset_huge(pmdp, addr & CONT_PTE_MASK); }
    core::ptr::null_mut()
}

pub unsafe fn hugetlb_mask_last_page(h: *mut hstate) -> ::core::ffi::c_ulong { match huge_page_size(h) {
    #[cfg(not(feature = "__PAGETABLE_PMD_FOLDED"))] PUD_SIZE if pud_sect_supported() => PGDIR_SIZE - PUD_SIZE,
    CONT_PMD_SIZE => PUD_SIZE - CONT_PMD_SIZE, PMD_SIZE => PUD_SIZE - PMD_SIZE, CONT_PTE_SIZE => PMD_SIZE - CONT_PTE_SIZE, _ => 0,
} }

pub unsafe fn arch_make_huge_pte(mut entry: pte_t, shift: ::core::ffi::c_uint, _flags: vm_flags_t) -> pte_t {
    match 1usize << shift { #[cfg(not(feature = "__PAGETABLE_PMD_FOLDED"))] PUD_SIZE if pud_sect_supported() => pud_pte(pud_mkhuge(pte_pud(entry))), CONT_PMD_SIZE => pmd_pte(pmd_mkhuge(pmd_mkcont(pte_pmd(entry)))), PMD_SIZE => pmd_pte(pmd_mkhuge(pte_pmd(entry))), CONT_PTE_SIZE => pte_mkcont(entry), _ => { pr_warn!("unrecognized huge page size 0x{:x}\n", 1usize << shift); entry } }
}

pub unsafe fn huge_pte_clear(mm: *mut mm_struct, mut addr: ::core::ffi::c_ulong, mut ptep: *mut pte_t, sz: ::core::ffi::c_ulong) { let mut pgsize=0usize; let n=num_contig_ptes(sz,&mut pgsize); for _ in 0..n { __pte_clear(mm,addr,ptep); addr+=pgsize as _; ptep=ptep.add(1); } }
pub unsafe fn huge_ptep_get_and_clear(mm:*mut mm_struct,addr:::core::ffi::c_ulong,ptep:*mut pte_t,sz:::core::ffi::c_ulong)->pte_t { let mut s=0usize; let n=num_contig_ptes(sz,&mut s); get_clear_contig(mm,addr,ptep,s as _,n as _) }

unsafe fn __cont_access_flags_changed(ptep:*mut pte_t,pte:pte_t,ncontig:::core::ffi::c_int)->::core::ffi::c_int { if pte_write(pte)!=pte_write(__ptep_get(ptep)){return 1;} for i in 0..ncontig { let o=__ptep_get(ptep.add(i as usize)); if pte_dirty(pte)!=pte_dirty(o)||pte_young(pte)!=pte_young(o){return 1;} } 0 }
pub unsafe fn huge_ptep_set_access_flags(vma:*mut vm_area_struct,addr:::core::ffi::c_ulong,ptep:*mut pte_t,pte:pte_t,dirty:::core::ffi::c_int)->::core::ffi::c_int { VM_WARN_ON!(!pte_present(pte)); let mut s=0usize; let mm=(*vma).vm_mm; let n=num_contig_ptes(huge_page_size(hstate_vma(vma)),&mut s); if !pte_cont(pte){return __ptep_set_access_flags_anysz(vma,addr,ptep,pte,dirty,s as _);} if __cont_access_flags_changed(ptep,pte,n)!=0 { let o=get_clear_contig_flush(mm,addr,ptep,s as _,n as _); VM_WARN_ON!(!pte_present(o)); let mut p=pte; if pte_dirty(o){p=pte_mkdirty(p);} if pte_young(o){p=pte_mkyoung(p);} __set_ptes_anysz(mm,addr,ptep,p,n,s as _); return 1;} 0 }
pub unsafe fn huge_ptep_set_wrprotect(mm:*mut mm_struct,mut addr:::core::ffi::c_ulong,ptep:*mut pte_t) { let mut p=__ptep_get(ptep); VM_WARN_ON!(!pte_present(p)); if !pte_cont(p){__ptep_set_wrprotect(mm,addr,ptep);return;} let mut s=0usize; let n=find_num_contig(mm,addr,ptep,&mut s); p=pte_wrprotect(get_clear_contig_flush(mm,addr,ptep,s as _,n as _)); __set_ptes_anysz(mm,addr,ptep,p,n,s as _); }
pub unsafe fn huge_ptep_clear_flush(vma:*mut vm_area_struct,addr:::core::ffi::c_ulong,ptep:*mut pte_t)->pte_t { let mut s=0usize; let n=num_contig_ptes(huge_page_size(hstate_vma(vma)),&mut s); get_clear_contig_flush((*vma).vm_mm,addr,ptep,s as _,n as _) }

pub unsafe fn arch_hugetlb_valid_size(size:::core::ffi::c_ulong)->bool { __hugetlb_valid_size(size) }
pub unsafe fn huge_ptep_modify_prot_start(vma:*mut vm_area_struct,addr:::core::ffi::c_ulong,ptep:*mut pte_t)->pte_t { let psize=huge_page_size(hstate_vma(vma)); if alternative_has_cap_unlikely(ARM64_WORKAROUND_2645198) && pte_user_exec(__ptep_get(ptep)){return huge_ptep_clear_flush(vma,addr,ptep);} huge_ptep_get_and_clear((*vma).vm_mm,addr,ptep,psize) }
pub unsafe fn huge_ptep_modify_prot_commit(vma:*mut vm_area_struct,addr:::core::ffi::c_ulong,ptep:*mut pte_t,_old_pte:pte_t,pte:pte_t) { set_huge_pte_at((*vma).vm_mm,addr,ptep,pte,huge_page_size(hstate_vma(vma))); }

#[allow(non_snake_case)]
unsafe fn hugetlbpage_init() -> ::core::ffi::c_int {
    BUILD_BUG_ON!(HUGE_MAX_HSTATE < 4);
    if pud_sect_supported() { hugetlb_add_hstate(PUD_SHIFT - PAGE_SHIFT); }
    hugetlb_add_hstate(CONT_PMD_SHIFT - PAGE_SHIFT);
    hugetlb_add_hstate(PMD_SHIFT - PAGE_SHIFT);
    hugetlb_add_hstate(CONT_PTE_SHIFT - PAGE_SHIFT);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
