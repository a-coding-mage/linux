/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2006 by Ralf Baechle (ralf@linux-mips.org)
 */

// C dependencies: <asm/addrspace.h>, <asm/sync.h>, and
// <asm-generic/barrier.h> supply CKSEG1, CKSEG1ADDR, __SYNC, and barrier APIs.

#[inline(always)]
pub unsafe fn __sync() {
    // asm volatile(__SYNC(full, always) ::: "memory");
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn rmb() {
    // asm volatile(__SYNC(rmb, always) ::: "memory");
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Acquire);
}

#[inline(always)]
pub unsafe fn wmb() {
    // asm volatile(__SYNC(wmb, always) ::: "memory");
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
}

#[inline(always)]
pub unsafe fn fast_mb() {
    __sync();
}

#[inline(always)]
pub unsafe fn __fast_iob() {
    // C MIPS inline assembly performs a volatile load from CKSEG1.
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[cfg(feature = "cpu_cavium_octeon")]
#[inline(always)]
pub unsafe fn fast_iob() {}

#[cfg(all(not(feature = "cpu_cavium_octeon"), feature = "sgi_ip28"))]
#[inline(always)]
pub unsafe fn fast_iob() {
    // C MIPS inline assembly loads CKSEG1ADDR(0x1fa00004), executes sync,
    // and loads it again.
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[cfg(all(not(feature = "cpu_cavium_octeon"), not(feature = "sgi_ip28")))]
#[inline(always)]
pub unsafe fn fast_iob() {
    __sync();
    __fast_iob();
}

#[cfg(feature = "cpu_has_wb")]
#[inline(always)]
pub unsafe fn mb() {
    wbflush();
}

#[cfg(feature = "cpu_has_wb")]
#[inline(always)]
pub unsafe fn iob() {
    wbflush();
}

#[cfg(not(feature = "cpu_has_wb"))]
#[inline(always)]
pub unsafe fn mb() {
    fast_mb();
}

#[cfg(not(feature = "cpu_has_wb"))]
#[inline(always)]
pub unsafe fn iob() {
    fast_iob();
}

#[cfg(feature = "weak_ordering")]
#[inline(always)]
pub unsafe fn __smp_mb() { __sync(); }

#[cfg(feature = "weak_ordering")]
#[inline(always)]
pub unsafe fn __smp_rmb() { rmb(); }

#[cfg(feature = "weak_ordering")]
#[inline(always)]
pub unsafe fn __smp_wmb() { wmb(); }

#[cfg(not(feature = "weak_ordering"))]
#[inline(always)]
pub unsafe fn __smp_mb() { barrier(); }

#[cfg(not(feature = "weak_ordering"))]
#[inline(always)]
pub unsafe fn __smp_rmb() { barrier(); }

#[cfg(not(feature = "weak_ordering"))]
#[inline(always)]
pub unsafe fn __smp_wmb() { barrier(); }

/*
 * When LL/SC does imply order, it must also be a compiler barrier to avoid the
 * compiler from reordering where the CPU will not. When it does not imply
 * order, the compiler is also free to reorder across the LL/SC loop and
 * ordering will be done by smp_llsc_mb() and friends.
 */
#[cfg(all(feature = "weak_reordering_beyond_llsc", feature = "smp"))]
pub const __WEAK_LLSC_MB: &str = "sync";

#[cfg(all(feature = "weak_reordering_beyond_llsc", feature = "smp"))]
#[inline(always)]
pub unsafe fn smp_llsc_mb() {
    // __asm__ __volatile__(__stringify(__WEAK_LLSC_MB) : : : "memory");
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[cfg(not(all(feature = "weak_reordering_beyond_llsc", feature = "smp")))]
pub const __WEAK_LLSC_MB: &str = "";

#[cfg(not(all(feature = "weak_reordering_beyond_llsc", feature = "smp")))]
#[inline(always)]
pub unsafe fn smp_llsc_mb() {}

#[cfg(feature = "cpu_cavium_octeon")]
#[inline(always)]
pub unsafe fn smp_mb__before_llsc() { smp_wmb(); }

#[cfg(feature = "cpu_cavium_octeon")]
#[inline(always)]
pub unsafe fn __smp_mb__before_llsc() { __smp_wmb(); }

#[cfg(feature = "cpu_cavium_octeon")]
#[inline(always)]
pub unsafe fn nudge_writes() {
    // MIPS Octeon syncw inline assembly.
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[cfg(not(feature = "cpu_cavium_octeon"))]
#[inline(always)]
pub unsafe fn smp_mb__before_llsc() { smp_llsc_mb(); }

#[cfg(not(feature = "cpu_cavium_octeon"))]
#[inline(always)]
pub unsafe fn __smp_mb__before_llsc() { smp_llsc_mb(); }

#[cfg(not(feature = "cpu_cavium_octeon"))]
#[inline(always)]
pub unsafe fn nudge_writes() { mb(); }

/* In the Loongson3 LL/SC workaround case, the preceding barrier is sufficient. */
#[cfg(feature = "cpu_loongson3_workarounds")]
#[inline(always)]
pub unsafe fn __smp_mb__before_atomic() {}

#[cfg(not(feature = "cpu_loongson3_workarounds"))]
#[inline(always)]
pub unsafe fn __smp_mb__before_atomic() { __smp_mb__before_llsc(); }

#[inline(always)]
pub unsafe fn __smp_mb__after_atomic() { smp_llsc_mb(); }

#[inline(always)]
pub unsafe fn sync_ginv() {
    // asm volatile(__SYNC(ginv, always));
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

// Declarations supplied by the included barrier headers.
unsafe extern "C" {
    pub fn wbflush();
    pub fn barrier();
    pub fn smp_wmb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
