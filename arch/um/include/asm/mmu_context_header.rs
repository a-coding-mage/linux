/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/sched.h, linux/mm_types.h, linux/mmap_lock.h,
// asm/mm_hooks.h, asm/mmu.h, and asm-generic/mmu_context.h.

/// Corresponds to the empty C `static inline switch_mm` definition.
#[inline]
pub unsafe fn switch_mm(
    _prev: *mut mm_struct,
    _next: *mut mm_struct,
    _tsk: *mut task_struct,
) {
}

// C macro aliases: `#define init_new_context init_new_context` and
// `#define destroy_context destroy_context`.

extern "C" {
    pub fn init_new_context(task: *mut task_struct, mm: *mut mm_struct) -> ::core::ffi::c_int;
    pub fn destroy_context(mm: *mut mm_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
