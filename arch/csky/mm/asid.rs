// SPDX-License-Identifier: GPL-2.0
/*
 * Generic ASID allocator.
 *
 * Based on arch/arm/mm/context.c
 *
 * Copyright (C) 2002-2003 Deep Blue Solutions Ltd, all rights reserved.
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn reserved_asid(info: *mut asid_info, cpu: i32) -> *mut u64 {
    per_cpu_ptr((*info).reserved, cpu)
}

unsafe fn asid_mask(info: *const asid_info) -> u64 {
    !(genmask((*info).bits - 1, 0))
}

unsafe fn asid_first_version(info: *const asid_info) -> u64 {
    1u64 << (*info).bits
}

unsafe fn asid2idx(info: *const asid_info, asid: u64) -> usize {
    ((asid & !asid_mask(info)) >> (*info).ctxt_shift) as usize
}

unsafe fn idx2asid(info: *const asid_info, idx: usize) -> u64 {
    ((idx as u64) << (*info).ctxt_shift) & !asid_mask(info)
}

unsafe fn flush_context(info: *mut asid_info) {
    let mut i: i32;
    let mut asid: u64;

    bitmap_zero((*info).map, num_ctxt_asids(info));

    for_each_possible_cpu!(i) {
        asid = atomic64_xchg_relaxed(active_asid(info, i), 0);
        /*
         * If this CPU has already been through a
         * rollover, but hasn't run another task in the meantime, we must preserve its reserved
         * ASID, as this is the only trace we have of
         * the process it is still running.
         */
        if asid == 0 {
            asid = *reserved_asid(info, i);
        }
        set_bit(asid2idx(info, asid), (*info).map);
        *reserved_asid(info, i) = asid;
    }

    /* Queue a TLB invalidation for each CPU to perform on next context-switch */
    cpumask_setall(&mut (*info).flush_pending);
}

unsafe fn check_update_reserved_asid(info: *mut asid_info, asid: u64, newasid: u64) -> bool {
    let mut cpu: i32;
    let mut hit = false;

    /*
     * Iterate over the set of reserved ASIDs looking for a match.
     * If we find one, then we can update our mm to use newasid
     * (i.e. the same ASID in the current generation) but we can't
     * exit the loop early, since we need to ensure that all copies
     * of the old ASID are updated to reflect the mm. Failure to do
     * so could result in us missing the reserved ASID in a future
     * generation.
     */
    for_each_possible_cpu!(cpu) {
        if *reserved_asid(info, cpu) == asid {
            hit = true;
            *reserved_asid(info, cpu) = newasid;
        }
    }

    hit
}

unsafe fn new_context(info: *mut asid_info, pasid: *mut atomic64_t, mm: *mut mm_struct) -> u64 {
    static mut CUR_IDX: u32 = 1;
    let mut asid = atomic64_read(pasid);
    let mut generation = atomic64_read(&mut (*info).generation);

    if asid != 0 {
        let newasid = generation | (asid & !asid_mask(info));

        if check_update_reserved_asid(info, asid, newasid) {
            return newasid;
        }
        if !test_and_set_bit(asid2idx(info, asid), (*info).map) {
            return newasid;
        }
    }

    asid = find_next_zero_bit((*info).map, num_ctxt_asids(info), CUR_IDX as usize) as u64;
    if asid != num_ctxt_asids(info) as u64 {
        // set_asid
    } else {
        generation = atomic64_add_return_relaxed(asid_first_version(info), &mut (*info).generation);
        flush_context(info);
        asid = find_next_zero_bit((*info).map, num_ctxt_asids(info), 1) as u64;
    }

    set_bit(asid as usize, (*info).map);
    CUR_IDX = asid as u32;
    cpumask_clear(mm_cpumask(mm));
    idx2asid(info, asid as usize) | generation
}

/*
 * Generate a new ASID for the context.
 *
 * @pasid: Pointer to the current ASID batch allocated. It will be updated
 * with the new ASID batch.
 * @cpu: current CPU ID. Must have been acquired through get_cpu()
 */
pub unsafe fn asid_new_context(info: *mut asid_info, pasid: *mut atomic64_t,
                               cpu: u32, mm: *mut mm_struct) {
    let mut flags: c_ulong = 0;
    let mut asid: u64;

    raw_spin_lock_irqsave(&mut (*info).lock, &mut flags);
    asid = atomic64_read(pasid);
    if ((asid ^ atomic64_read(&mut (*info).generation)) >> (*info).bits) != 0 {
        asid = new_context(info, pasid, mm);
        atomic64_set(pasid, asid);
    }

    if cpumask_test_and_clear_cpu(cpu, &mut (*info).flush_pending) {
        ((*info).flush_cpu_ctxt_cb)();
    }

    atomic64_set(active_asid(info, cpu as i32), asid);
    cpumask_set_cpu(cpu, mm_cpumask(mm));
    raw_spin_unlock_irqrestore(&mut (*info).lock, flags);
}

/* Initialize the ASID allocator */
pub unsafe fn asid_allocator_init(info: *mut asid_info, bits: u32,
                                  asid_per_ctxt: u32,
                                  flush_cpu_ctxt_cb: Option<unsafe extern "C" fn()>) -> i32 {
    (*info).bits = bits;
    (*info).ctxt_shift = ilog2(asid_per_ctxt);
    (*info).flush_cpu_ctxt_cb = flush_cpu_ctxt_cb;
    warn_on(num_ctxt_asids(info) - 1 <= num_possible_cpus());
    atomic64_set(&mut (*info).generation, asid_first_version(info));
    (*info).map = bitmap_zalloc(num_ctxt_asids(info), GFP_KERNEL);
    if (*info).map.is_null() {
        return -ENOMEM;
    }
    raw_spin_lock_init(&mut (*info).lock);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
