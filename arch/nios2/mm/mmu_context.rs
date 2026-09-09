/*
 * MMU context handling.
 *
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009 Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Dependencies supplied by the corresponding Linux/arch headers. */

/* The pids position and mask in context */
const PID_SHIFT: usize = 0;

#[inline]
fn pid_bits() -> usize {
    unsafe { cpuinfo.tlb_pid_num_bits as usize }
}

#[inline]
fn pid_mask() -> usize {
    (1usize << pid_bits()).wrapping_sub(1)
}

/* The versions position and mask in context */
const VERSION_BITS: usize = 32 - 0; // Build-time value: 32 - PID_BITS.

#[inline]
fn version_bits() -> usize {
    32 - pid_bits()
}

#[inline]
fn version_shift() -> usize {
    PID_SHIFT + pid_bits()
}

#[inline]
fn version_mask() -> usize {
    (1usize << version_bits()).wrapping_sub(1)
}

/* Return the version part of a context */
#[inline]
fn ctx_version(c: mm_context_t) -> mm_context_t {
    (c >> version_shift()) & version_mask()
}

/* Return the pid part of a context */
#[inline]
fn ctx_pid(c: mm_context_t) -> mm_context_t {
    (c >> PID_SHIFT) & pid_mask()
}

/* Value of the first context (version 1, pid 0) */
#[inline]
fn first_ctx() -> mm_context_t {
    ((1usize << version_shift()) | (0usize << PID_SHIFT)) as mm_context_t
}

static mut next_mmu_context: mm_context_t = 0;

/*
 * Initialize MMU context management stuff.
 */
pub unsafe fn mmu_context_init() {
    /* We need to set this here because the value depends on runtime data
     * from cpuinfo */
    next_mmu_context = first_ctx();
}

/*
 * Set new context (pid), keep way
 */
unsafe fn set_context(context: mm_context_t) {
    set_mmu_pid(ctx_pid(context));
}

unsafe fn get_new_context() -> mm_context_t {
    /* Return the next pid */
    next_mmu_context = next_mmu_context.wrapping_add((1usize << PID_SHIFT) as mm_context_t);

    /* If the pid field wraps around we increase the version and
     * flush the tlb */
    if ctx_pid(next_mmu_context) == 0 {
        /* Version is incremented since the pid increment above
         * overflows info version */
        flush_cache_all();
        flush_tlb_all();
    }

    /* If the version wraps we start over with the first generation, we do
     * not need to flush the tlb here since it's always done above */
    if ctx_version(next_mmu_context) == 0 {
        next_mmu_context = first_ctx();
    }

    next_mmu_context
}

pub unsafe fn switch_mm(
    _prev: *mut mm_struct,
    next: *mut mm_struct,
    _tsk: *mut task_struct,
) {
    let mut flags: c_ulong = 0;

    local_irq_save(&mut flags);

    /* If the process context we are swapping in has a different context
     * generation then we have it should get a new generation/pid */
    if ctx_version((*next).context) != ctx_version(next_mmu_context) {
        (*next).context = get_new_context();
    }

    /* Save the current pgd so the fast tlb handler can find it */
    pgd_current = (*next).pgd;

    /* Set the current context */
    set_context((*next).context);

    local_irq_restore(flags);
}

/*
 * After we have set current->mm to a new value, this activates
 * the context for the new mm so we see the new mappings.
 */
pub unsafe fn activate_mm(_prev: *mut mm_struct, next: *mut mm_struct) {
    (*next).context = get_new_context();
    set_context((*next).context);
    pgd_current = (*next).pgd;
}

pub unsafe fn get_pid_from_context(context: *mut mm_context_t) -> c_ulong {
    ctx_pid(*context) as c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
