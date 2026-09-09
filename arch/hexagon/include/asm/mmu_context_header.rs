/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MM context support for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependency intent from the original header:
// linux/mm_types.h, asm/setup.h, asm/page.h, asm/pgalloc.h,
// asm/mem-layout.h, asm-generic/mm_hooks.h, asm-generic/mmu_context.h

/*
 * VM port hides all TLB management, so "lazy TLB" isn't very
 * meaningful.  Even for ports to architectures with visble TLBs,
 * this is almost invariably a null function.
 *
 * mm->context is set up by pgd_alloc, so no init_new_context required.
 */

/* Switch active mm context. */
#[inline]
pub unsafe fn switch_mm(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    _tsk: *mut task_struct,
) {
    let mut l1: libc::c_int;

    /*
     * For virtual machine, we have to update system map if it's been
     * touched.
     */
    if (*next).context.generation < (*prev).context.generation {
        l1 = MIN_KERNEL_SEG;
        while l1 <= max_kernel_seg {
            (*next).pgd[l1 as usize] = init_mm.pgd[l1 as usize];
            l1 += 1;
        }

        (*next).context.generation = (*prev).context.generation;
    }

    __vmnewmap((*next).context.ptbase as *mut libc::c_void);
}

/* Activate new memory map for task. */
#[inline]
pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    let mut flags: libc::c_ulong = 0;

    local_irq_save(&mut flags);
    switch_mm(prev, next, current_thread_info().task);
    local_irq_restore(flags);
}

// The original `#define activate_mm activate_mm` preserves the arch hook name.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
