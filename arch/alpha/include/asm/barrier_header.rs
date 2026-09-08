/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __BARRIER_H

/// Memory barrier corresponding to the Alpha `mb` instruction.
#[inline(always)]
pub unsafe fn mb() {
    core::arch::asm!("mb", options(nostack, preserves_flags));
}

/// Read memory barrier.  On Alpha this is the same instruction as `mb`.
#[inline(always)]
pub unsafe fn rmb() {
    core::arch::asm!("mb", options(nostack, preserves_flags));
}

/// Write memory barrier corresponding to the Alpha `wmb` instruction.
#[inline(always)]
pub unsafe fn wmb() {
    core::arch::asm!("wmb", options(nostack, preserves_flags));
}

// Equivalent of the C statement-expression macro.  `compiletime_assert_atomic_type`
// and `__READ_ONCE` are supplied by the surrounding translation unit.
#[macro_export]
macro_rules! __smp_load_acquire {
    ($p:expr) => {{
        compiletime_assert_atomic_type!(*$p);
        __READ_ONCE!(*$p)
    }};
}

// CONFIG_SMP selects the SMP memory-barrier assembly sequence in the original
// header.  The empty sequence is used when CONFIG_SMP is not enabled.
#[cfg(CONFIG_SMP)]
pub const __ASM_SMP_MB: &str = "\tmb\n";

#[cfg(not(CONFIG_SMP))]
pub const __ASM_SMP_MB: &str = "";

// <asm-generic/barrier.h> is an external dependency of the original header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
