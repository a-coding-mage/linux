// SPDX-License-Identifier: GPL-2.0-only

/*
 * Functions provided for exec functionality which however are
 * specifically VMA-only logic.
 */

/*
 * Dependencies are supplied by vma_internal.h and vma.h in the C source.
 * Their corresponding Rust names are expected to be available to this file.
 */

/*
 * Relocate a VMA downwards by shift bytes. There cannot be any VMAs between
 * this VMA and its relocated range, which will now reside at [vma->vm_start -
 * shift, vma->vm_end - shift).
 *
 * This function is almost certainly NOT what you want for anything other than
 * early executable temporary stack relocation.
 */
pub unsafe fn relocate_vma_down(vma: *mut vm_area_struct, shift: c_ulong) -> c_int {
    /*
     * The process proceeds as follows:
     *
     * 1) Use shift to calculate the new vma endpoints.
     * 2) Extend vma to cover both the old and new ranges.  This ensures the
     *    arguments passed to subsequent functions are consistent.
     * 3) Move vma's page tables to the new range.
     * 4) Free up any cleared pgd range.
     * 5) Shrink the vma to cover only the new range.
     */
    let mm: *mut mm_struct = (*vma).vm_mm;
    let old_start: c_ulong = (*vma).vm_start;
    let old_end: c_ulong = (*vma).vm_end;
    let length: c_ulong = old_end.wrapping_sub(old_start);
    let new_start: c_ulong = old_start.wrapping_sub(shift);
    let new_end: c_ulong = old_end.wrapping_sub(shift);
    let mut vmi = VMA_ITERATOR!(vmi, mm, new_start);
    let mut vmg = VMG_STATE!(
        vmg,
        mm,
        &mut vmi,
        new_start,
        old_end,
        EMPTY_VMA_FLAGS,
        vma_start_pgoff(vma),
        vma_start_anon_pgoff(vma)
    );
    let mut next: *mut vm_area_struct;
    let mut tlb: mmu_gather;
    let mut pmc = PAGETABLE_MOVE!(pmc, vma, vma, old_start, new_start, length);

    BUG_ON!(new_start > new_end);

    /* ensure there are no vmas between where we want to go and where we are */
    if vma != vma_next(&mut vmi) {
        return -EFAULT;
    }

    vma_iter_prev_range(&mut vmi);
    /* cover the whole range: [new_start, old_end) */
    vmg.target = vma;
    if vma_expand(&mut vmg) != 0 {
        return -ENOMEM;
    }

    /*
     * move the page tables downwards, on failure we rely on
     * process cleanup to remove whatever mess we made.
     */
    pmc.for_stack = true;
    if length != move_page_tables(&mut pmc) {
        return -ENOMEM;
    }

    tlb_gather_mmu(&mut tlb, mm);
    next = vma_next(&mut vmi);
    if new_end > old_start {
        /* when the old and new regions overlap clear from new_end. */
        free_pgd_range(
            &mut tlb,
            new_end,
            old_end,
            new_end,
            if !next.is_null() { (*next).vm_start } else { USER_PGTABLES_CEILING },
        );
    } else {
        /*
         * otherwise, clean from old_start; this is done to not touch
         * the address space in [new_end, old_start) some architectures
         * have constraints on va-space that make this illegal (IA64) -
         * for the others its just a little faster.
         */
        free_pgd_range(
            &mut tlb,
            old_start,
            old_end,
            new_end,
            if !next.is_null() { (*next).vm_start } else { USER_PGTABLES_CEILING },
        );
    }
    tlb_finish_mmu(&mut tlb);

    vma_prev(&mut vmi);
    /* Shrink the vma to just the new range */
    vma_shrink(&mut vmi, vma, new_end)
}

/*
 * Establish the stack VMA in an execve'd process, located temporarily at the
 * maximum stack address provided by the architecture.
 *
 * We later relocate this downwards in relocate_vma_down().
 *
 * This function is almost certainly NOT what you want for anything other than
 * early executable initialisation.
 *
 * On success, returns 0 and sets *vmap to the stack VMA and *top_mem_p to the
 * maximum addressable location in the stack (that is capable of storing a
 * system word of data).
 */
pub unsafe fn create_init_stack_vma(
    mm: *mut mm_struct,
    vmap: *mut *mut vm_area_struct,
    top_mem_p: *mut c_ulong,
) -> c_int {
    let mut flags: vma_flags_t = VMA_STACK_INCOMPLETE_SETUP;
    let vma: *mut vm_area_struct;
    let mut err: c_int;

    /* VMA_STACK_FLAGS and VMA_STACK_INCOMPLETE_SETUP must not overlap. */
    VM_WARN_ON_ONCE!(vma_flags_test_any_mask(&flags, VMA_STACK_FLAGS));

    vma = vm_area_alloc(mm);
    if vma.is_null() {
        return -ENOMEM;
    }

    if mmap_write_lock_killable(mm) != 0 {
        err = -EINTR;
        *vmap = core::ptr::null_mut();
        vm_area_free(vma);
        return err;
    }

    /* Need to be called with mmap write lock held, to avoid race with ksmd. */
    err = ksm_execve(mm);
    if err != 0 {
        mmap_write_unlock(mm);
        *vmap = core::ptr::null_mut();
        vm_area_free(vma);
        return err;
    }

    vma_flags_set_mask(&mut flags, VMA_STACK_FLAGS);
    vma_set_anonymous(vma);

    /*
     * Place the stack at the largest stack address the architecture supports.
     * Later, we'll move this to an appropriate place. We don't use STACK_TOP
     * because that can depend on attributes which aren't configured yet.
     */
    (*vma).vm_end = STACK_TOP_MAX;
    (*vma).vm_start = (*vma).vm_end - PAGE_SIZE;
    if pgtable_supports_soft_dirty() {
        vma_flags_set(&mut flags, VMA_SOFTDIRTY_BIT);
    }
    (*vma).flags = flags;
    (*vma).vm_page_prot = vma_get_page_prot(vma);

    err = insert_vm_struct(mm, vma);
    if err != 0 {
        ksm_exit(mm);
        mmap_write_unlock(mm);
        *vmap = core::ptr::null_mut();
        vm_area_free(vma);
        return err;
    }

    (*mm).stack_vm = 1;
    (*mm).total_vm = 1;
    mmap_write_unlock(mm);
    *vmap = vma;
    *top_mem_p = (*vma).vm_end - core::mem::size_of::<*mut c_void>() as c_ulong;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
