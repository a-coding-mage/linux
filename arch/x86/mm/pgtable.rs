// SPDX-License-Identifier: GPL-2.0

#[cfg(CONFIG_DYNAMIC_PHYSICAL_MASK)]
pub static mut physical_mask: phys_addr_t = (1u64 << __PHYSICAL_MASK_SHIFT) - 1;

pub unsafe fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t {
    __pte_alloc_one(mm, GFP_PGTABLE_USER)
}

pub unsafe fn ___pte_free_tlb(tlb: *mut mmu_gather, pte: *mut page) {
    paravirt_release_pte(page_to_pfn(pte));
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
}

#[cfg(CONFIG_PGTABLE_LEVELS > 2)]
pub unsafe fn ___pmd_free_tlb(tlb: *mut mmu_gather, pmd: *mut pmd_t) {
    paravirt_release_pmd(__pa(pmd) >> PAGE_SHIFT);
    // NOTE! For PAE, changes to top page-directory-pointer-table entries need a full cr3 reload.
    #[cfg(CONFIG_X86_PAE)]
    {
        (*tlb).need_flush_all = 1;
    }
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(pmd));
}

#[cfg(CONFIG_PGTABLE_LEVELS > 3)]
pub unsafe fn ___pud_free_tlb(tlb: *mut mmu_gather, pud: *mut pud_t) {
    paravirt_release_pud(__pa(pud) >> PAGE_SHIFT);
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(pud));
}

#[cfg(CONFIG_PGTABLE_LEVELS > 4)]
pub unsafe fn ___p4d_free_tlb(tlb: *mut mmu_gather, p4d: *mut p4d_t) {
    paravirt_release_p4d(__pa(p4d) >> PAGE_SHIFT);
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(p4d));
}

unsafe fn pgd_list_add(pgd: *mut pgd_t) {
    let ptdesc = virt_to_ptdesc(pgd);
    list_add(&mut (*ptdesc).pt_list, &mut pgd_list);
}

unsafe fn pgd_list_del(pgd: *mut pgd_t) {
    let ptdesc = virt_to_ptdesc(pgd);
    list_del(&mut (*ptdesc).pt_list);
}

unsafe fn pgd_set_mm(pgd: *mut pgd_t, mm: *mut mm_struct) {
    (*virt_to_ptdesc(pgd)).pt_mm = mm;
}

pub unsafe fn pgd_page_get_mm(pt: *mut ptdesc) -> *mut mm_struct { (*pt).pt_mm }

unsafe fn pgd_ctor(mm: *mut mm_struct, pgd: *mut pgd_t) {
    // PAE preallocates all its PMDs. No cloning needed.
    if !IS_ENABLED(CONFIG_X86_PAE) {
        clone_pgd_range(pgd.add(KERNEL_PGD_BOUNDARY), swapper_pg_dir.add(KERNEL_PGD_BOUNDARY), KERNEL_PGD_PTRS);
    }
    // List used to sync kernel mapping updates
    pgd_set_mm(pgd, mm);
    pgd_list_add(pgd);
}

unsafe fn pgd_dtor(pgd: *mut pgd_t) {
    spin_lock(&mut pgd_lock);
    pgd_list_del(pgd);
    spin_unlock(&mut pgd_lock);
}

#[cfg(CONFIG_X86_PAE)]
pub const PREALLOCATED_PMDS: usize = PTRS_PER_PGD;
#[cfg(not(CONFIG_X86_PAE))]
pub const PREALLOCATED_PMDS: usize = 0;

#[cfg(CONFIG_X86_PAE)]
pub const PREALLOCATED_USER_PMDS: usize = if boot_cpu_has(X86_FEATURE_PTI) { KERNEL_PGD_PTRS } else { 0 };
#[cfg(not(CONFIG_X86_PAE))]
pub const PREALLOCATED_USER_PMDS: usize = 0;
#[cfg(CONFIG_X86_PAE)]
pub const MAX_PREALLOCATED_USER_PMDS: usize = KERNEL_PGD_PTRS;
#[cfg(not(CONFIG_X86_PAE))]
pub const MAX_PREALLOCATED_USER_PMDS: usize = 0;

#[cfg(CONFIG_X86_PAE)]
pub unsafe fn pud_populate(mm: *mut mm_struct, pudp: *mut pud_t, pmd: *mut pmd_t) {
    paravirt_alloc_pmd(mm, __pa(pmd) >> PAGE_SHIFT);
    set_pud(pudp, __pud(__pa(pmd) | _PAGE_PRESENT));
    flush_tlb_mm(mm);
}

unsafe fn free_pmds(mm: *mut mm_struct, pmds: *mut *mut pmd_t, count: i32) {
    for i in 0..count {
        let pmd = *pmds.add(i as usize);
        if !pmd.is_null() {
            let ptdesc = virt_to_ptdesc(pmd);
            pagetable_dtor(ptdesc);
            pagetable_free(ptdesc);
            mm_dec_nr_pmds(mm);
        }
    }
}

unsafe fn preallocate_pmds(mm: *mut mm_struct, pmds: *mut *mut pmd_t, count: i32) -> i32 {
    let mut failed = false;
    let mut gfp = GFP_PGTABLE_USER;
    if mm == &mut init_mm { gfp &= !__GFP_ACCOUNT; }
    gfp &= !__GFP_HIGHMEM;
    for i in 0..count {
        let mut pmd: *mut pmd_t = core::ptr::null_mut();
        let mut ptdesc = pagetable_alloc(gfp, 0);
        if ptdesc.is_null() { failed = true; }
        if !ptdesc.is_null() && !pagetable_pmd_ctor(mm, ptdesc) {
            pagetable_free(ptdesc); ptdesc = core::ptr::null_mut(); failed = true;
        }
        if !ptdesc.is_null() { mm_inc_nr_pmds(mm); pmd = ptdesc_address(ptdesc); }
        *pmds.add(i as usize) = pmd;
    }
    if failed { free_pmds(mm, pmds, count); return -ENOMEM; }
    0
}

unsafe fn mop_up_one_pmd(mm: *mut mm_struct, pgdp: *mut pgd_t) {
    let pgd = *pgdp;
    if pgd_val(pgd) != 0 {
        let pmd = pgd_page_vaddr(pgd) as *mut pmd_t;
        pgd_clear(pgdp);
        paravirt_release_pmd(pgd_val(pgd) >> PAGE_SHIFT);
        pmd_free(mm, pmd);
        mm_dec_nr_pmds(mm);
    }
}

unsafe fn pgd_mop_up_pmds(mm: *mut mm_struct, pgdp: *mut pgd_t) {
    for i in 0..PREALLOCATED_PMDS { mop_up_one_pmd(mm, pgdp.add(i)); }
    #[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
    {
        if !boot_cpu_has(X86_FEATURE_PTI) { return; }
        let user_pgdp = kernel_to_user_pgdp(pgdp);
        for i in 0..PREALLOCATED_USER_PMDS { mop_up_one_pmd(mm, user_pgdp.add(i + KERNEL_PGD_BOUNDARY)); }
    }
}

unsafe fn pgd_prepopulate_pmd(mm: *mut mm_struct, pgd: *mut pgd_t, pmds: *mut *mut pmd_t) {
    let mut pud = pud_offset(p4d_offset(pgd, 0), 0);
    for i in 0..PREALLOCATED_PMDS {
        let pmd = *pmds.add(i);
        if i >= KERNEL_PGD_BOUNDARY {
            memcpy(pmd, pgd_page_vaddr(*swapper_pg_dir.add(i)) as *mut pmd_t, core::mem::size_of::<pmd_t>() * PTRS_PER_PMD);
        }
        pud_populate(mm, pud, pmd); pud = pud.add(1);
    }
}

#[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
unsafe fn pgd_prepopulate_user_pmd(mm: *mut mm_struct, k_pgd: *mut pgd_t, pmds: *mut *mut pmd_t) {
    let mut s_pgd = kernel_to_user_pgdp(swapper_pg_dir).add(KERNEL_PGD_BOUNDARY);
    let mut u_pud = pud_offset(p4d_offset(kernel_to_user_pgdp(k_pgd), 0), 0).add(KERNEL_PGD_BOUNDARY);
    for i in 0..PREALLOCATED_USER_PMDS {
        let pmd = *pmds.add(i);
        memcpy(pmd, pgd_page_vaddr(*s_pgd) as *mut pmd_t, core::mem::size_of::<pmd_t>() * PTRS_PER_PMD);
        pud_populate(mm, u_pud, pmd); u_pud = u_pud.add(1); s_pgd = s_pgd.add(1);
    }
}
#[cfg(not(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION))]
unsafe fn pgd_prepopulate_user_pmd(_mm: *mut mm_struct, _k_pgd: *mut pgd_t, _pmds: *mut *mut pmd_t) {}

unsafe fn _pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t { __pgd_alloc(mm, pgd_allocation_order()) }
unsafe fn _pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) { __pgd_free(mm, pgd); }

pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let mut pgd = _pgd_alloc(mm);
    if pgd.is_null() { return core::ptr::null_mut(); }
    (*mm).pgd = pgd;
    let mut pmds: [*mut pmd_t; PREALLOCATED_PMDS] = [core::ptr::null_mut(); PREALLOCATED_PMDS];
    let mut u_pmds: [*mut pmd_t; MAX_PREALLOCATED_USER_PMDS] = [core::ptr::null_mut(); MAX_PREALLOCATED_USER_PMDS];
    if PREALLOCATED_PMDS != 0 && preallocate_pmds(mm, pmds.as_mut_ptr(), PREALLOCATED_PMDS as i32) != 0 { _pgd_free(mm, pgd); return core::ptr::null_mut(); }
    if PREALLOCATED_USER_PMDS != 0 && preallocate_pmds(mm, u_pmds.as_mut_ptr(), PREALLOCATED_USER_PMDS as i32) != 0 { free_pmds(mm, pmds.as_mut_ptr(), PREALLOCATED_PMDS as i32); _pgd_free(mm, pgd); return core::ptr::null_mut(); }
    if paravirt_pgd_alloc(mm) != 0 { free_pmds(mm, u_pmds.as_mut_ptr(), PREALLOCATED_USER_PMDS as i32); free_pmds(mm, pmds.as_mut_ptr(), PREALLOCATED_PMDS as i32); _pgd_free(mm, pgd); return core::ptr::null_mut(); }
    spin_lock(&mut pgd_lock);
    pgd_ctor(mm, pgd);
    if PREALLOCATED_PMDS != 0 { pgd_prepopulate_pmd(mm, pgd, pmds.as_mut_ptr()); }
    if PREALLOCATED_USER_PMDS != 0 { pgd_prepopulate_user_pmd(mm, pgd, u_pmds.as_mut_ptr()); }
    spin_unlock(&mut pgd_lock);
    pgd
}

pub unsafe fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) { pgd_mop_up_pmds(mm, pgd); pgd_dtor(pgd); paravirt_pgd_free(mm, pgd); _pgd_free(mm, pgd); }

pub unsafe fn ptep_set_access_flags(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t, entry: pte_t, dirty: i32) -> i32 {
    let changed = (!pte_same(*ptep, entry)) as i32;
    if changed != 0 && dirty != 0 { set_pte(ptep, entry); }
    changed
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmdp_set_access_flags(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t, entry: pmd_t, dirty: i32) -> i32 {
    let changed = (!pmd_same(*pmdp, entry)) as i32;
    VM_BUG_ON(address & !HPAGE_PMD_MASK);
    if changed != 0 && dirty != 0 { set_pmd(pmdp, entry); }
    changed
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pudp_set_access_flags(vma: *mut vm_area_struct, address: c_ulong, pudp: *mut pud_t, entry: pud_t, dirty: i32) -> i32 {
    let changed = (!pud_same(*pudp, entry)) as i32;
    VM_BUG_ON(address & !HPAGE_PUD_MASK);
    if changed != 0 && dirty != 0 { set_pud(pudp, entry); }
    changed
}

pub unsafe fn ptep_test_and_clear_young(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t) -> bool {
    if pte_young(*ptep) { test_and_clear_bit(_PAGE_BIT_ACCESSED, &mut (*ptep).pte as *mut _ as *mut c_ulong) } else { false }
}

#[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_ARCH_HAS_NONLEAF_PMD_YOUNG))]
pub unsafe fn pmdp_test_and_clear_young(vma: *mut vm_area_struct, addr: c_ulong, pmdp: *mut pmd_t) -> bool {
    if pmd_young(*pmdp) { test_and_clear_bit(_PAGE_BIT_ACCESSED, pmdp as *mut c_ulong) } else { false }
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pudp_test_and_clear_young(vma: *mut vm_area_struct, addr: c_ulong, pudp: *mut pud_t) -> bool {
    if pud_young(*pudp) { test_and_clear_bit(_PAGE_BIT_ACCESSED, pudp as *mut c_ulong) } else { false }
}

pub unsafe fn ptep_clear_flush_young(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t) -> bool { ptep_test_and_clear_young(vma, address, ptep) }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmdp_clear_flush_young(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t) -> bool {
    VM_BUG_ON(address & !HPAGE_PMD_MASK);
    let young = pmdp_test_and_clear_young(vma, address, pmdp);
    if young { flush_tlb_range(vma, address, address + HPAGE_PMD_SIZE); }
    young
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pmdp_invalidate_ad(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t) -> pmd_t {
    VM_WARN_ON_ONCE(!pmd_present(*pmdp));
    pmdp_establish(vma, address, pmdp, pmd_mkinvalid(*pmdp))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
