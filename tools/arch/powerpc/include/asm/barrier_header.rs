/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copied from the kernel sources:
 *
 * Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
 */

/*
 * Memory barrier.
 * The sync instruction guarantees that all memory accesses initiated
 * by this processor have been performed (with respect to all other
 * mechanisms that access memory).  The eieio instruction is a barrier
 * providing an ordering (separately) for (a) cacheable stores and (b)
 * loads and stores to non-cacheable memory (e.g. I/O devices).
 *
 * mb() prevents loads and stores being reordered across this point.
 * rmb() prevents loads being reordered across this point.
 * wmb() prevents stores being reordered across this point.
 *
 * *mb() variants without smp_ prefix must order all types of memory
 * operations with one another. sync is the only instruction sufficient
 * to do this.
 */
#[inline(always)]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("sync", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn rmb() {
    unsafe {
        core::arch::asm!("sync", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn wmb() {
    unsafe {
        core::arch::asm!("sync", options(nostack, preserves_flags));
    }
}

/* C condition: defined(__powerpc64__) */
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn smp_lwsync() {
    unsafe {
        core::arch::asm!("lwsync", options(nostack, preserves_flags));
    }
}

/* C condition: defined(__powerpc64__) */
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn smp_store_release<T>(p: *mut T, v: T) {
    unsafe {
        smp_lwsync();
        core::ptr::write_volatile(p, v);
    }
}

/* C condition: defined(__powerpc64__) */
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn smp_load_acquire<T: Copy>(p: *const T) -> T {
    unsafe {
        let ___p1 = core::ptr::read_volatile(p);
        smp_lwsync();
        ___p1
    }
}
