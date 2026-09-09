/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Generic hooks for NOMMU architectures, which do not need to do
 * anything special here.
 *
 * Dependency: asm-generic/mm_hooks.h
 */

/// Generic NOMMU address-space switch hook.
#[inline]
pub unsafe fn switch_mm(
    _prev: *mut mm_struct,
    _next: *mut mm_struct,
    _tsk: *mut task_struct,
) {
}

/* Dependency: asm-generic/mmu_context.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
