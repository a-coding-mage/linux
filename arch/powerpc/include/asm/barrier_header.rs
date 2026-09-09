/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
 */

/*
 * Memory barrier.
 * The sync instruction guarantees that all memory accesses initiated by this
 * processor have been performed.  eieio provides ordering for cacheable
 * stores and for loads and stores to non-cacheable memory.
 */

#[inline(always)]
pub unsafe fn __mb() {
    core::arch::asm!("sync", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __rmb() {
    core::arch::asm!("sync", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __wmb() {
    core::arch::asm!("sync", options(nostack, preserves_flags));
}

/* The sub-architecture selects SMPWMB: LWSYNC, mbar, or eieio.
 * The original build-time CONFIG_PPC64, CONFIG_PPC_E500MC, and
 * CONFIG_BOOKE conditions are preserved here as conditional intent.
 */

#[inline(always)]
pub unsafe fn __lwsync() {
    core::arch::asm!("lwsync", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __dma_rmb() {
    __lwsync();
}

#[inline(always)]
pub unsafe fn __dma_wmb() {
    // SMPWMB is LWSYNC, mbar, or eieio according to the selected sub-architecture.
    core::arch::asm!("lwsync", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __smp_lwsync() {
    __lwsync();
}

#[inline(always)]
pub unsafe fn __smp_mb() {
    __mb();
}

#[inline(always)]
pub unsafe fn __smp_rmb() {
    __lwsync();
}

#[inline(always)]
pub unsafe fn __smp_wmb() {
    // SMPWMB is LWSYNC, mbar, or eieio according to the selected sub-architecture.
    core::arch::asm!("lwsync", options(nostack, preserves_flags));
}

/* Prevent following instructions from being started until x is known. */
#[macro_export]
macro_rules! data_barrier {
    ($x:expr) => {{
        core::arch::asm!("twi 0,{0},0; isync", in(reg) $x, options(nostack));
    }};
}

#[macro_export]
macro_rules! __smp_store_release {
    ($p:expr, $v:expr) => {{
        compiletime_assert_atomic_type!(*$p);
        $crate::__smp_lwsync();
        WRITE_ONCE!(*$p, $v);
    }};
}

#[macro_export]
macro_rules! __smp_load_acquire {
    ($p:expr) => {{
        let ___p1 = READ_ONCE!(*$p);
        compiletime_assert_atomic_type!(*$p);
        $crate::__smp_lwsync();
        ___p1
    }};
}

/* CONFIG_PPC_BOOK3S_64 selects nop; CONFIG_PPC_E500 selects nop; nop. */

/* CONFIG_PPC_BARRIER_NOSPEC selects the following instruction barrier. */
#[cfg(feature = "CONFIG_PPC_BARRIER_NOSPEC")]
#[inline(always)]
pub unsafe fn barrier_nospec() {
    // NOSPEC_BARRIER_FIXUP_SECTION; NOSPEC_BARRIER_SLOT
    core::arch::asm!("nop", options(nostack, preserves_flags));
}

#[cfg(not(feature = "CONFIG_PPC_BARRIER_NOSPEC"))]
#[inline(always)]
pub fn barrier_nospec() {}

#[inline(always)]
pub unsafe fn pmem_wmb() {
    // PPC_PHWSYNC is supplied by the architecture-specific dependencies.
    core::arch::asm!("phwsync", options(nostack, preserves_flags));
}

/* asm-generic/barrier.h supplies the generic barrier declarations. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
