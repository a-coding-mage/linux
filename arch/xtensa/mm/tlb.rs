/*
 * arch/xtensa/mm/tlb.c
 *
 * Logic that manipulates the Xtensa MMU.  Derived from MIPS.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn __flush_itlb_all() {
    let mut w: i32;
    let mut i: i32;
    w = 0;
    while w < ITLB_ARF_WAYS {
        i = 0;
        while i < (1 << XCHAL_ITLB_ARF_ENTRIES_LOG2) {
            let e = w + (i << PAGE_SHIFT);
            invalidate_itlb_entry_no_isync(e);
            i += 1;
        }
        w += 1;
    }
    core::arch::asm!("isync");
}

unsafe fn __flush_dtlb_all() {
    let mut w: i32;
    let mut i: i32;
    w = 0;
    while w < DTLB_ARF_WAYS {
        i = 0;
        while i < (1 << XCHAL_DTLB_ARF_ENTRIES_LOG2) {
            let e = w + (i << PAGE_SHIFT);
            invalidate_dtlb_entry_no_isync(e);
            i += 1;
        }
        w += 1;
    }
    core::arch::asm!("isync");
}

pub unsafe fn local_flush_tlb_all() {
    __flush_itlb_all();
    __flush_dtlb_all();
}

pub unsafe fn local_flush_tlb_mm(mm: *mut mm_struct) {
    let cpu = smp_processor_id();
    if mm == (*current).active_mm {
        let mut flags: unsigned_long;
        local_irq_save(&mut flags);
        (*mm).context.asid[cpu as usize] = NO_CONTEXT;
        activate_context(mm, cpu);
        local_irq_restore(flags);
    } else {
        (*mm).context.asid[cpu as usize] = NO_CONTEXT;
        (*mm).context.cpu = -1;
    }
}

const _ITLB_ENTRIES: usize = ITLB_ARF_WAYS as usize << XCHAL_ITLB_ARF_ENTRIES_LOG2;
const _DTLB_ENTRIES: usize = DTLB_ARF_WAYS as usize << XCHAL_DTLB_ARF_ENTRIES_LOG2;
const _TLB_ENTRIES: usize = if _ITLB_ENTRIES > _DTLB_ENTRIES { _ITLB_ENTRIES } else { _DTLB_ENTRIES };

pub unsafe fn local_flush_tlb_range(vma: *mut vm_area_struct, mut start: unsigned_long, end: unsigned_long) {
    let cpu = smp_processor_id();
    let mm = (*vma).vm_mm;
    let mut flags: unsigned_long;
    if (*mm).context.asid[cpu as usize] == NO_CONTEXT { return; }
    pr_debug!("[tlbrange<%02lx,%08lx,%08lx>]\n", (*mm).context.asid[cpu as usize] as unsigned_long, start, end);
    local_irq_save(&mut flags);
    if end - start + (PAGE_SIZE - 1) <= ((_TLB_ENTRIES as unsigned_long) << PAGE_SHIFT) {
        let oldpid = get_rasid_register();
        set_rasid_register(ASID_INSERT((*mm).context.asid[cpu as usize]));
        start &= PAGE_MASK;
        if (*vma).vm_flags & VM_EXEC != 0 {
            while start < end { invalidate_itlb_mapping(start); invalidate_dtlb_mapping(start); start += PAGE_SIZE; }
        } else {
            while start < end { invalidate_dtlb_mapping(start); start += PAGE_SIZE; }
        }
        set_rasid_register(oldpid);
    } else { local_flush_tlb_mm(mm); }
    local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_page(vma: *mut vm_area_struct, page: unsigned_long) {
    let cpu = smp_processor_id();
    let mm = (*vma).vm_mm;
    let mut flags: unsigned_long;
    if (*mm).context.asid[cpu as usize] == NO_CONTEXT { return; }
    local_irq_save(&mut flags);
    let oldpid = get_rasid_register();
    set_rasid_register(ASID_INSERT((*mm).context.asid[cpu as usize]));
    if (*vma).vm_flags & VM_EXEC != 0 { invalidate_itlb_mapping(page); }
    invalidate_dtlb_mapping(page);
    set_rasid_register(oldpid);
    local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_kernel_range(mut start: unsigned_long, end: unsigned_long) {
    if end > start && start >= TASK_SIZE && end <= PAGE_OFFSET && end - start < ((_TLB_ENTRIES as unsigned_long) << PAGE_SHIFT) {
        start &= PAGE_MASK;
        while start < end { invalidate_itlb_mapping(start); invalidate_dtlb_mapping(start); start += PAGE_SIZE; }
    } else { local_flush_tlb_all(); }
}

pub unsafe fn update_mmu_tlb_range(vma: *mut vm_area_struct, address: unsigned_long, _ptep: *mut pte_t, nr: unsigned_int) {
    local_flush_tlb_range(vma, address, address + PAGE_SIZE * nr as unsigned_long);
}

#[cfg(CONFIG_DEBUG_TLB_SANITY)]
unsafe fn get_pte_for_vaddr(vaddr: unsigned) -> unsigned {
    let task = get_current();
    let mut mm = (*task).mm;
    if mm.is_null() { mm = (*task).active_mm; }
    let pgd = pgd_offset(mm, vaddr);
    if pgd_none_or_clear_bad(pgd) { return 0; }
    let p4d = p4d_offset(pgd, vaddr); if p4d_none_or_clear_bad(p4d) { return 0; }
    let pud = pud_offset(p4d, vaddr); if pud_none_or_clear_bad(pud) { return 0; }
    let pmd = pmd_offset(pud, vaddr); if pmd_none_or_clear_bad(pmd) { return 0; }
    let pte = pte_offset_map(pmd, vaddr); if pte.is_null() { return 0; }
    let pteval = pte_val(*pte); pte_unmap(pte); pteval
}

#[cfg(CONFIG_DEBUG_TLB_SANITY)]
const TLB_SUSPICIOUS: i32 = 1;
#[cfg(CONFIG_DEBUG_TLB_SANITY)]
const TLB_INSANE: i32 = 2;

#[cfg(CONFIG_DEBUG_TLB_SANITY)]
unsafe fn tlb_insane() { BUG_ON!(1); }
#[cfg(CONFIG_DEBUG_TLB_SANITY)]
unsafe fn tlb_suspicious() { WARN_ON!(1); }

#[cfg(CONFIG_DEBUG_TLB_SANITY)]
unsafe fn check_tlb_entry(w: unsigned, e: unsigned, dtlb: bool) -> i32 {
    let tlbidx = w | (e << PAGE_SHIFT);
    let r0 = if dtlb { read_dtlb_virtual(tlbidx) } else { read_itlb_virtual(tlbidx) };
    let r1 = if dtlb { read_dtlb_translation(tlbidx) } else { read_itlb_translation(tlbidx) };
    let vpn = (r0 & PAGE_MASK) | (e << PAGE_SHIFT);
    let pte = get_pte_for_vaddr(vpn);
    let mm_asid = (get_rasid_register() >> 8) & ASID_MASK;
    let tlb_asid = r0 & ASID_MASK;
    let kernel = tlb_asid == 1;
    let mut rc = 0;
    if tlb_asid > 0 && ((vpn < TASK_SIZE) == kernel) { pr_err!("%cTLB: way: %u, entry: %u, VPN %08x in %s PTE\n", if dtlb {'D'} else {'I'}, w, e, vpn, if kernel {"kernel"} else {"user"}); rc |= TLB_INSANE; }
    if tlb_asid == mm_asid && ((pte ^ r1) & PAGE_MASK) != 0 { if pte == 0 || !pte_present(__pte(pte)) { let p = pfn_to_page(r1 >> PAGE_SHIFT); let f = page_folio(p); pr_err!("folio refcount: %d, mapcount: %d\n", folio_ref_count(f), folio_mapcount(f)); if folio_ref_count(f) == 0 { rc |= TLB_INSANE; } else if folio_mapped(f) { rc |= TLB_SUSPICIOUS; } } else { rc |= TLB_INSANE; } }
    rc
}

#[cfg(CONFIG_DEBUG_TLB_SANITY)]
pub unsafe fn check_tlb_sanity() { let mut flags: unsigned_long; let mut bug = 0; local_irq_save(&mut flags); for w in 0..DTLB_ARF_WAYS { for e in 0..(1 << XCHAL_DTLB_ARF_ENTRIES_LOG2) { bug |= check_tlb_entry(w as unsigned, e as unsigned, true); } } for w in 0..ITLB_ARF_WAYS { for e in 0..(1 << XCHAL_ITLB_ARF_ENTRIES_LOG2) { bug |= check_tlb_entry(w as unsigned, e as unsigned, false); } } if bug & TLB_INSANE != 0 { tlb_insane(); } if bug & TLB_SUSPICIOUS != 0 { tlb_suspicious(); } local_irq_restore(flags); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
