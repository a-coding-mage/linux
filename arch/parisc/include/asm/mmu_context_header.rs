/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

/* on PA-RISC, we actually have enough contexts to justify an allocator
 * for them.  prumpf */

extern "C" {
    pub fn alloc_sid() -> ::core::ffi::c_ulong;
    pub fn free_sid(space_id: ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn init_new_context(
    tsk: *mut task_struct,
    mm: *mut mm_struct,
) -> ::core::ffi::c_int {
    let _ = tsk;
    BUG_ON(atomic_read(&(*mm).mm_users) != 1);

    (*mm).context.space_id = alloc_sid();
    0
}

#[inline]
pub unsafe fn destroy_context(mm: *mut mm_struct) {
    free_sid((*mm).context.space_id);
    (*mm).context.space_id = 0;
}

#[inline]
pub unsafe fn __space_to_prot(context: mm_context_t) -> ::core::ffi::c_ulong {
    // When SPACEID_SHIFT is zero, the C source shifts left; otherwise it shifts
    // right by SPACEID_SHIFT - 1.  Select the branch using the build-time value.
    if SPACEID_SHIFT == 0 {
        context.space_id << 1
    } else {
        context.space_id >> (SPACEID_SHIFT - 1)
    }
}

#[inline]
pub unsafe fn load_context(context: mm_context_t) {
    mtsp(context.space_id, SR_USER);
    mtctl(__space_to_prot(context), 8);
}

#[inline]
pub unsafe fn switch_mm_irqs_off(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    let _ = tsk;
    if prev != next {
        // CONFIG_TLB_PTLOCK: put the physical address of page_table_lock in
        // cr28 (tr4) for TLB faults.
        #[cfg(CONFIG_TLB_PTLOCK)]
        {
            let pgd_lock: *mut spinlock_t = &mut (*next).page_table_lock;
            mtctl(
                __pa(__ldcw_align(&mut (*pgd_lock).rlock.raw_lock)),
                28,
            );
        }
        mtctl(__pa((*next).pgd), 25);
        load_context((*next).context);
    }
}

#[inline]
pub unsafe fn switch_mm(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    let mut flags: ::core::ffi::c_ulong = 0;

    if prev == next {
        return;
    }

    local_irq_save(&mut flags);
    switch_mm_irqs_off(prev, next, tsk);
    local_irq_restore(flags);
}

#[inline]
pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    /*
     * Activate_mm is our one chance to allocate a space id
     * for a new mm created in the exec path. There's also
     * some lazy tlb stuff, which is currently dead code, but
     * we only allocate a space id if one hasn't been allocated
     * already, so we should be OK.
     */

    BUG_ON(next == &mut init_mm);

    if (*next).context.space_id == 0 {
        (*next).context.space_id = alloc_sid();
    }

    switch_mm(prev, next, current);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
