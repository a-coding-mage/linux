/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// C header guard and includes omitted; the declarations below depend on the
// corresponding Linux, MicroBlaze, and asm-generic definitions.

#[cfg(feature = "kernel")]
#[inline]
pub const fn ctx_to_vsid(ctx: u32, va: u32) -> u32 {
    ((ctx.wrapping_mul(897 * 16) + ((va >> 28).wrapping_mul(0x111))) & 0x00ff_ffff)
}

#[cfg(feature = "kernel")]
pub const NO_CONTEXT: u32 = 256;
#[cfg(feature = "kernel")]
pub const LAST_CONTEXT: u32 = 255;
#[cfg(feature = "kernel")]
pub const FIRST_CONTEXT: u32 = 1;

#[cfg(feature = "kernel")]
extern "C" {
    pub fn set_context(context: mm_context_t, pgd: *mut pgd_t);
    pub static mut context_map: [::core::ffi::c_ulong; 1];
    pub static mut next_mmu_context: mm_context_t;
    pub static mut nr_free_contexts: atomic_t;
    pub static mut context_mm: [*mut mm_struct; (LAST_CONTEXT + 1) as usize];
    pub fn steal_context();
    pub fn atomic_dec_if_positive(v: *mut atomic_t) -> i32;
    pub fn atomic_inc(v: *mut atomic_t);
    pub fn test_and_set_bit(nr: mm_context_t, addr: *mut ::core::ffi::c_ulong) -> bool;
    pub fn find_next_zero_bit(
        addr: *const ::core::ffi::c_ulong,
        size: mm_context_t,
        offset: mm_context_t,
    ) -> mm_context_t;
    pub fn clear_bit(nr: mm_context_t, addr: *mut ::core::ffi::c_ulong);
    pub fn mmu_context_init();
}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn get_mmu_context(mm: *mut mm_struct) {
    let mut ctx: mm_context_t;

    if (*mm).context != NO_CONTEXT as mm_context_t {
        return;
    }
    while atomic_dec_if_positive(&mut nr_free_contexts) < 0 {
        steal_context();
    }
    ctx = next_mmu_context;
    while test_and_set_bit(ctx, context_map.as_mut_ptr()) {
        ctx = find_next_zero_bit(context_map.as_ptr(), LAST_CONTEXT + 1, ctx);
        if ctx > LAST_CONTEXT as mm_context_t {
            ctx = 0;
        }
    }
    next_mmu_context = (ctx + 1) & LAST_CONTEXT as mm_context_t;
    (*mm).context = ctx;
    context_mm[ctx as usize] = mm;
}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 {
    (*mm).context = NO_CONTEXT as mm_context_t;
    0
}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn destroy_context(mm: *mut mm_struct) {
    if (*mm).context != NO_CONTEXT as mm_context_t {
        clear_bit((*mm).context, context_map.as_mut_ptr());
        (*mm).context = NO_CONTEXT as mm_context_t;
        atomic_inc(&mut nr_free_contexts);
    }
}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn switch_mm(
    _prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    (*tsk).thread.pgdir = (*next).pgd;
    get_mmu_context(next);
    set_context((*next).context, (*next).pgd);
}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn activate_mm(_active_mm: *mut mm_struct, mm: *mut mm_struct) {
    // `current` is supplied by the scheduler dependency, as in the C header.
    (*current).thread.pgdir = (*mm).pgd;
    get_mmu_context(mm);
    set_context((*mm).context, (*mm).pgd);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
