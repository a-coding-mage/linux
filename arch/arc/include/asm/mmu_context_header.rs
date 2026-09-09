/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * vineetg: May 2011
 *  -Refactored get_new_mmu_context( ) to only handle live-mm.
 *   retiring-mm handled in other hooks
 *
 * Vineetg: March 25th, 2008: Bug #92690
 *  -Major rewrite of Core ASID allocation routine get_new_mmu_context
 *
 * Amit Bhor, Sameer Dhavale: Codito Technologies 2004
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const MM_CTXT_ASID_MASK: u32 = 0x000000ff; /* MMU PID reg :8 bit PID */
pub const MM_CTXT_CYCLE_MASK: u32 = !MM_CTXT_ASID_MASK;

pub const MM_CTXT_FIRST_CYCLE: u32 = MM_CTXT_ASID_MASK + 1;
pub const MM_CTXT_NO_ASID: usize = 0;

#[inline]
pub unsafe fn asid_mm(mm: *mut mm_struct, cpu: usize) -> *mut u32 {
    (*mm).context.asid.as_mut_ptr().add(cpu)
}

#[inline]
pub unsafe fn hw_pid(mm: *mut mm_struct, cpu: usize) -> u32 {
    (*asid_mm(mm, cpu)) & MM_CTXT_ASID_MASK
}

extern "C" {
    static mut asid_cache: per_cpu<unsigned int>;
}

#[inline]
pub unsafe fn asid_cpu(cpu: usize) -> *mut u32 {
    per_cpu_ptr(&raw mut asid_cache, cpu)
}

/*
 * Get a new ASID if task doesn't have a valid one (unalloc or from prev cycle)
 * Also set the MMU PID register to existing/updated ASID
 */
#[inline]
pub unsafe fn get_new_mmu_context(mm: *mut mm_struct) {
    let cpu: u32 = smp_processor_id();
    let mut flags: unsigned long = 0;

    local_irq_save(&mut flags);

    /* Move to new ASID if it was not from current alloc-cycle/generation. */
    if ((*asid_mm(mm, cpu as usize) ^ *asid_cpu(cpu as usize)) & MM_CTXT_CYCLE_MASK) == 0 {
        mmu_setup_asid(mm, hw_pid(mm, cpu as usize));
        local_irq_restore(flags);
        return;
    }

    /* move to new ASID and handle rollover */
    *asid_cpu(cpu as usize) = (*asid_cpu(cpu as usize)).wrapping_add(1);
    if (*asid_cpu(cpu as usize) & MM_CTXT_ASID_MASK) == 0 {
        local_flush_tlb_all();

        /* If the container itself wrapped around, use a non-zero generation. */
        if *asid_cpu(cpu as usize) == 0 {
            *asid_cpu(cpu as usize) = MM_CTXT_FIRST_CYCLE;
        }
    }

    /* Assign new ASID to tsk */
    *asid_mm(mm, cpu as usize) = *asid_cpu(cpu as usize);

    mmu_setup_asid(mm, hw_pid(mm, cpu as usize));

    local_irq_restore(flags);
}

/* Initialize the context related info for a new mm_struct instance. */
#[inline]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 {
    let mut i: i32 = 0;

    for_each_possible_cpu!(i) {
        *asid_mm(mm, i as usize) = MM_CTXT_NO_ASID as u32;
    }

    0
}

#[inline]
pub unsafe fn destroy_context(mm: *mut mm_struct) {
    let mut flags: unsigned long = 0;

    /* Needed to elide CONFIG_DEBUG_PREEMPT warning */
    local_irq_save(&mut flags);
    *asid_mm(mm, smp_processor_id() as usize) = MM_CTXT_NO_ASID as u32;
    local_irq_restore(flags);
}

/* Prepare the MMU for task: setup PID reg with allocated ASID. */
#[inline]
pub unsafe fn switch_mm(
    _prev: *mut mm_struct,
    next: *mut mm_struct,
    _tsk: *mut task_struct,
) {
    let cpu: i32 = smp_processor_id();

    /* mm_cpumask is aggregating and is intentionally not cleared here. */
    cpumask_set_cpu(cpu, mm_cpumask(next));

    mmu_setup_pgd(next, (*next).pgd);

    get_new_mmu_context(next);
}

/* activate_mm defaults in asm-generic to switch_mm. */
/* deactivate_mm bookkeeping and generic MMU context support are external. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
