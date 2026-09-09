/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2012 Tensilica Inc.
 */

// Dependency supplied by <asm/core.h> in the original header.

/// Memory barrier corresponding to the Xtensa `memw` instruction.
#[inline(always)]
pub unsafe fn __mb() {
    core::arch::asm!("memw", options(nostack));
}

/// Read memory barrier. `barrier` is supplied by asm-generic/barrier.h.
#[inline(always)]
pub unsafe fn __rmb() {
    barrier();
}

/// Write memory barrier.
#[inline(always)]
pub unsafe fn __wmb() {
    __mb();
}

// Under CONFIG_SMP, these macros are enabled in the original header.
#[cfg(CONFIG_SMP)]
#[inline(always)]
pub unsafe fn __smp_mb() {
    __mb();
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub unsafe fn __smp_rmb() {
    __rmb();
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub unsafe fn __smp_wmb() {
    __wmb();
}

// When XCHAL_HAVE_S32C1I is enabled, these atomic-operation barriers are
// provided by the original header.
#[cfg(XCHAL_HAVE_S32C1I)]
#[inline(always)]
pub unsafe fn __smp_mb__before_atomic() {
    barrier();
}

#[cfg(XCHAL_HAVE_S32C1I)]
#[inline(always)]
pub unsafe fn __smp_mb__after_atomic() {
    barrier();
}

// Declaration supplied by <asm-generic/barrier.h> in the original header.
unsafe extern "C" {
    pub fn barrier();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
