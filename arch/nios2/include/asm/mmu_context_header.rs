/*
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 1996, 1997, 1998, 1999 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 *
 * based on MIPS asm/mmu_context.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependency: linux/mm_types.h
// Dependency: asm-generic/mm_hooks.h

extern "C" {
    pub fn mmu_context_init();
    pub fn get_pid_from_context(ctx: *mut mm_context_t) -> ::core::ffi::c_ulong;

    /*
     * For the fast tlb miss handlers, we keep a pointer to the current pgd.
     * processor.
     */
    pub static mut pgd_current: *mut pgd_t;
}

/*
 * Initialize the context related info for a new mm_struct instance.
 *
 * Set all new contexts to 0, that way the generation will never match
 * the currently running generation when this context is switched in.
 *
 * The C macro `init_new_context init_new_context` aliases the declaration to
 * itself for feature detection.
 */
pub unsafe fn init_new_context(
    _tsk: *mut task_struct,
    mm: *mut mm_struct,
) -> ::core::ffi::c_int {
    (*mm).context = 0;
    0
}

extern "C" {
    pub fn switch_mm(
        prev: *mut mm_struct,
        next: *mut mm_struct,
        tsk: *mut task_struct,
    );
}

/*
 * After we have set current->mm to a new value, this activates
 * the context for the new mm so we see the new mappings.
 *
 * The C macro `activate_mm activate_mm` aliases the declaration to itself
 * for feature detection.
 */
extern "C" {
    pub fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct);
}

// Declarations from asm-generic/mmu_context.h are supplied by that dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
