// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for handling the MMU.
 *
 *    Copyright (C) 2007 Xilinx, Inc.  All rights reserved.
 *
 *  Derived from arch/ppc/mm/4xx_mmu.c:
 *  -- paulus
 *
 *  Derived from arch/ppc/mm/init.c:
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *  Amiga/APUS changes by Jesper Skov (jskov@cygnus.co.uk).
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/mm.h, linux/init.h, asm/tlbflush.h, asm/mmu_context.h

pub static mut next_mmu_context: mm_context_t = 0;
pub static mut context_map: [c_ulong; LAST_CONTEXT / BITS_PER_LONG + 1] =
    [0; LAST_CONTEXT / BITS_PER_LONG + 1];
pub static mut nr_free_contexts: atomic_t = atomic_t { counter: 0 };
pub static mut context_mm: [*mut mm_struct; LAST_CONTEXT + 1] =
    [core::ptr::null_mut(); LAST_CONTEXT + 1];

/*
 * Initialize the context management stuff.
 */
#[no_mangle]
pub unsafe extern "C" fn mmu_context_init() {
    /*
     * The use of context zero is reserved for the kernel.
     * This code assumes FIRST_CONTEXT < 32.
     */
    context_map[0] = (1 << FIRST_CONTEXT) - 1;
    next_mmu_context = FIRST_CONTEXT;
    atomic_set(
        &mut nr_free_contexts,
        LAST_CONTEXT - FIRST_CONTEXT + 1,
    );
}

/*
 * Steal a context from a task that has one at the moment.
 *
 * This isn't an LRU system, it just frees up each context in
 * turn (sort-of pseudo-random replacement :).  This would be the
 * place to implement an LRU scheme if anyone were motivated to do it.
 */
#[no_mangle]
pub unsafe extern "C" fn steal_context() {
    let mut mm: *mut mm_struct;

    /* free up context `next_mmu_context' */
    /* if we shouldn't free context 0, don't... */
    if next_mmu_context < FIRST_CONTEXT {
        next_mmu_context = FIRST_CONTEXT;
    }
    mm = context_mm[next_mmu_context as usize];
    flush_tlb_mm(mm);
    destroy_context(mm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
