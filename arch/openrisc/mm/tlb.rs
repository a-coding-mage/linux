// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC tlb.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Julius Baxter <julius.baxter@orsoc.se>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// Dependencies supplied by the surrounding kernel port.

const NO_CONTEXT: i32 = -1;

#[inline]
unsafe fn num_dtlb_sets() -> usize {
    1usize << ((mfspr(SPR_IMMUCFGR) & SPR_IMMUCFGR_NTS) >> SPR_DMMUCFGR_NTS_OFF)
}

#[inline]
unsafe fn num_itlb_sets() -> usize {
    1usize << ((mfspr(SPR_IMMUCFGR) & SPR_IMMUCFGR_NTS) >> SPR_DMMUCFGR_NTS_OFF)
}

#[inline]
unsafe fn dtlb_offset(addr: usize) -> usize {
    (addr >> PAGE_SHIFT) & (num_dtlb_sets() - 1)
}

#[inline]
unsafe fn itlb_offset(addr: usize) -> usize {
    (addr >> PAGE_SHIFT) & (num_itlb_sets() - 1)
}

/*
 * Invalidate all TLB entries.
 *
 * This comes down to setting the 'valid' bit for all xTLBMR registers to 0.
 * Easiest way to accomplish this is to just zero out the xTLBMR register
 * completely.
 */
pub unsafe fn local_flush_tlb_all() {
    let num_tlb_sets = num_itlb_sets();

    /* Determine number of sets for IMMU. */
    /* FIXME: Assumption is I & D nsets equal. */
    for i in 0..num_tlb_sets {
        mtspr_off(SPR_DTLBMR_BASE(0), i, 0);
        mtspr_off(SPR_ITLBMR_BASE(0), i, 0);
    }
}

#[inline]
unsafe fn have_dtlbeir() -> bool {
    (mfspr(SPR_DMMUCFGR) & SPR_DMMUCFGR_TEIRI) != 0
}

#[inline]
unsafe fn have_itlbeir() -> bool {
    (mfspr(SPR_IMMUCFGR) & SPR_IMMUCFGR_TEIRI) != 0
}

/* Invalidate a single page using the available xTLBEIR or TLBMR register. */
#[inline]
unsafe fn flush_dtlb_page_eir(addr: usize) { mtspr(SPR_DTLBEIR, addr); }

#[inline]
unsafe fn flush_dtlb_page_no_eir(addr: usize) {
    mtspr_off(SPR_DTLBMR_BASE(0), dtlb_offset(addr), 0);
}

#[inline]
unsafe fn flush_itlb_page_eir(addr: usize) { mtspr(SPR_ITLBEIR, addr); }

#[inline]
unsafe fn flush_itlb_page_no_eir(addr: usize) {
    mtspr_off(SPR_ITLBMR_BASE(0), itlb_offset(addr), 0);
}

pub unsafe fn local_flush_tlb_page(vma: *mut vm_area_struct, addr: usize) {
    let _ = vma;
    if have_dtlbeir() { flush_dtlb_page_eir(addr); } else { flush_dtlb_page_no_eir(addr); }
    if have_itlbeir() { flush_itlb_page_eir(addr); } else { flush_itlb_page_no_eir(addr); }
}

pub unsafe fn local_flush_tlb_range(
    vma: *mut vm_area_struct,
    mut start: usize,
    end: usize,
) {
    let _ = vma;
    let dtlbeir = have_dtlbeir();
    let itlbeir = have_itlbeir();

    while start < end {
        if dtlbeir { flush_dtlb_page_eir(start); } else { flush_dtlb_page_no_eir(start); }
        if itlbeir { flush_itlb_page_eir(start); } else { flush_itlb_page_no_eir(start); }
        start = start.wrapping_add(PAGE_SIZE);
    }
}

/* Invalidate the selected mm context only. */
pub unsafe fn local_flush_tlb_mm(mm: *mut mm_struct) {
    let _ = mm;
    /* Was seeing bugs with the mm struct passed to us. Scrapped most of
       this function. */
    /* Several architectures do this */
    local_flush_tlb_all();
}

/* called in schedule() just before actually doing the switch_to */
pub unsafe fn switch_mm(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    next_tsk: *mut task_struct,
) {
    let _ = next_tsk;
    if unlikely(prev == next) { return; }

    let cpu = smp_processor_id();
    cpumask_clear_cpu(cpu, mm_cpumask(prev));
    cpumask_set_cpu(cpu, mm_cpumask(next));

    /* remember the pgd for the fault handlers */
    current_pgd[cpu] = (*next).pgd;

    /* We don't have context support implemented, so flush all
     * entries belonging to previous map
     */
    local_flush_tlb_mm(prev);
}

/* Initialize the context related info for a new mm_struct instance. */
pub unsafe fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> i32 {
    let _ = tsk;
    (*mm).context = NO_CONTEXT;
    0
}

/* called by __exit_mm to destroy the used MMU context if any before
 * destroying the mm itself. this is only called when the last user of the mm
 * drops it.
 */
pub unsafe fn destroy_context(mm: *mut mm_struct) {
    flush_tlb_mm(mm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
