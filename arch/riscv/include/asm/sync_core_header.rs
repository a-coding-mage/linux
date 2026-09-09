/* SPDX-License-Identifier: GPL-2.0 */

/*
 * RISC-V implements return to user-space through an xRET instruction,
 * which is not core serializing.
 */
#[inline]
pub unsafe fn sync_core_before_usermode() {
    core::arch::asm!("fence.i");
}

/*
 * Ensure the next switch_mm() on every CPU issues a core serializing
 * instruction for the given @mm.
 *
 * CONFIG_SMP is a build-time configuration condition from the C header.
 */
#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn prepare_sync_core_cmd(mm: *mut mm_struct) {
    cpumask_setall(&mut (*mm).context.icache_stale_mask);
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn prepare_sync_core_cmd(_mm: *mut mm_struct) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
