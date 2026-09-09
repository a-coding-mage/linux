/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

// Translated from the RISC-V MMU context header.
// C header dependencies are supplied by other translation units.

extern "C" {
    pub fn switch_mm(
        prev: *mut mm_struct,
        next: *mut mm_struct,
        task: *mut task_struct,
    );

    pub fn shstk_release(tsk: *mut task_struct);
}

// Opaque types supplied by the translated kernel headers.
pub enum mm_struct {}
pub enum task_struct {}

// #ifdef CONFIG_RISCV_ISA_SUPM
#[inline]
pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    // next->context.pmlen = 0;
    (*next).context.pmlen = 0;
    switch_mm(prev, next, core::ptr::null_mut());
}
// #endif

// #ifdef CONFIG_MMU
// atomic_long_set(&mm->context.id, 0);
// #endif
#[inline]
pub unsafe fn init_new_context(
    _tsk: *mut task_struct,
    mm: *mut mm_struct,
) -> i32 {
    atomic_long_set(&mut (*mm).context.id, 0);
    // IS_ENABLED(CONFIG_RISCV_ISA_SUPM)
    clear_bit(MM_CONTEXT_LOCK_PMLEN, &mut (*mm).context.flags);
    0
}

extern "C" {
    pub static use_asid_allocator: static_key_false;
    pub fn atomic_long_set(value: *mut atomic_long_t, i: i64);
    pub fn clear_bit(nr: usize, addr: *mut usize);
}

// Opaque synchronization types supplied by the translated kernel headers.
pub enum static_key_false {}
pub enum atomic_long_t {}

pub const MM_CONTEXT_LOCK_PMLEN: usize = 0; // Supplied by the MM context dependency.

// #ifdef CONFIG_RISCV_ISA_SUPM
#[inline]
pub unsafe fn mm_untag_mask(mm: *mut mm_struct) -> usize {
    usize::MAX >> (*mm).context.pmlen
}
// #endif

#[inline]
pub unsafe fn deactivate_mm(_tsk: *mut task_struct, mm: *mut mm_struct) {
    // The C implementation calls shstk_release(tsk); preserve that side effect.
    shstk_release(_tsk);
}

// #include <asm-generic/mmu_context.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
