/* SPDX-License-Identifier: GPL-2.0-or-later */

// Original header contents apply only when building the kernel.
// The following dependency is selected by CONFIG_PPC_QUEUED_SPINLOCKS:
// - asm/qspinlock.h and asm/qrwlock.h when enabled
// - asm/simple_spinlock.h otherwise

/* See include/linux/spinlock.h */
// Equivalent to: #define smp_mb__after_spinlock() smp_mb()
extern "C" {
    pub fn smp_mb();
}

#[inline(always)]
pub unsafe fn smp_mb__after_spinlock() {
    smp_mb();
}

// Equivalent to:
// #ifndef CONFIG_PPC_QUEUED_SPINLOCKS
// static inline void pv_spinlocks_init(void) { }
// #endif
#[cfg(not(feature = "CONFIG_PPC_QUEUED_SPINLOCKS"))]
#[inline(always)]
pub unsafe fn pv_spinlocks_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
