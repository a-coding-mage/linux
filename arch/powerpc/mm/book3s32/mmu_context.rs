// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for handling the MMU on those
 * PowerPC implementations where the MMU substantially follows the
 * architecture specification.  This includes the 6xx, 7xx, 7xxx,
 * and 8260 implementations but excludes the 8xx and 4xx.
 *  -- paulus
 *
 * Derived from arch/ppc/mm/init.c:
 *   Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 * Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 * and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *   Copyright (C) 1996 Paul Mackerras
 *
 * Derived from "arch/i386/mm/init.c"
 *   Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[no_mangle]
pub static mut abatron_pteptrs: [*mut core::ffi::c_void; 2] = [core::ptr::null_mut(); 2];

/*
 * On 32-bit PowerPC 6xx/7xx/7xxx CPUs, we use a set of 16 VSIDs
 * (virtual segment identifiers) for each context.  Although the
 * hardware supports 24-bit VSIDs, and thus >1 million contexts,
 * we only use 32,768 of them.  That is ample, since there can be
 * at most around 30,000 tasks in the system anyway, and it means
 * that we can use a bitmap to indicate which contexts are in use.
 * Using a bitmap means that we entirely avoid all of the problems
 * that we used to have when the context number overflowed,
 * particularly on SMP systems.
 *  -- paulus.
 */
pub const NO_CONTEXT: ::core::ffi::c_ulong = !0;
pub const LAST_CONTEXT: usize = 32767;
pub const FIRST_CONTEXT: usize = 1;

static mut next_mmu_context: ::core::ffi::c_ulong = 0;
static mut context_map: [::core::ffi::c_ulong; LAST_CONTEXT / (8 * core::mem::size_of::<::core::ffi::c_ulong>()) + 1] =
    [0; LAST_CONTEXT / (8 * core::mem::size_of::<::core::ffi::c_ulong>()) + 1];

pub unsafe fn __init_new_context() -> ::core::ffi::c_ulong {
    let mut ctx = next_mmu_context;

    while test_and_set_bit(ctx, context_map.as_mut_ptr()) != 0 {
        ctx = find_next_zero_bit(context_map.as_ptr(), (LAST_CONTEXT + 1) as _, ctx);
        if ctx > LAST_CONTEXT as _ {
            ctx = 0;
        }
    }
    next_mmu_context = (ctx + 1) & LAST_CONTEXT as _;

    ctx
}

/* Set up the context for a new address space. */
pub unsafe fn init_new_context(t: *mut task_struct, mm: *mut mm_struct) -> i32 {
    (*mm).context.id = __init_new_context();
    (*mm).context.sr0 = CTX_TO_VSID((*mm).context.id, 0);

    if IS_ENABLED(CONFIG_PPC_KUEP) {
        (*mm).context.sr0 |= SR_NX;
    }
    if !kuap_is_disabled() {
        (*mm).context.sr0 |= SR_KS;
    }

    0
}

/* Free a context ID. Make sure to call this with preempt disabled! */
pub unsafe fn __destroy_context(ctx: ::core::ffi::c_ulong) {
    clear_bit(ctx, context_map.as_mut_ptr());
}

/* We're finished using the context for an address space. */
pub unsafe fn destroy_context(mm: *mut mm_struct) {
    preempt_disable();
    if (*mm).context.id != NO_CONTEXT {
        __destroy_context((*mm).context.id);
        (*mm).context.id = NO_CONTEXT;
    }
    preempt_enable();
}

/* Initialize the context management stuff. */
pub unsafe fn mmu_context_init() {
    /* Reserve context 0 for kernel use */
    context_map[0] = (1 << FIRST_CONTEXT) - 1;
    next_mmu_context = FIRST_CONTEXT as _;
}

pub unsafe fn switch_mmu_context(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    let id = (*next).context.id as isize;

    if id < 0 {
        panic!("mm_struct {:p} has no context ID", next);
    }

    isync();

    update_user_segments((*next).context.sr0);

    if IS_ENABLED(CONFIG_BDI_SWITCH) {
        abatron_pteptrs[1] = (*next).pgd as *mut core::ffi::c_void;
    }

    if !mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        mtspr(SPRN_SDR1, rol32(__pa((*next).pgd), 4) & 0xffff01ff);
    }

    mb(); /* sync */
    isync();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
