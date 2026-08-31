/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copied from the Linux kernel sources, and also moving code
 * out from tools/perf/perf-sys.h so as to make it be located
 * in a place similar as in the kernel sources.
 *
 * Force strict CPU ordering.
 * And yes, this is required on UP too when we're talking
 * to devices.
 */

unsafe extern "C" {
    fn barrier();
}

#[cfg(target_arch = "x86")]
/*
 * Some non-Intel clones support out of order store. wmb() ceases to be a
 * nop for these.
 */
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("lock; addl $0,0(%esp)", options(nostack, preserves_flags));
    }
}

#[cfg(target_arch = "x86")]
pub unsafe fn rmb() {
    unsafe {
        core::arch::asm!("lock; addl $0,0(%esp)", options(nostack, preserves_flags));
    }
}

#[cfg(target_arch = "x86")]
pub unsafe fn wmb() {
    unsafe {
        core::arch::asm!("lock; addl $0,0(%esp)", options(nostack, preserves_flags));
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("mfence", options(nostack, preserves_flags));
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn rmb() {
    unsafe {
        core::arch::asm!("lfence", options(nostack, preserves_flags));
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn wmb() {
    unsafe {
        core::arch::asm!("sfence", options(nostack, preserves_flags));
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn smp_rmb() {
    unsafe {
        barrier();
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn smp_wmb() {
    unsafe {
        barrier();
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn smp_mb() {
    unsafe {
        core::arch::asm!("lock; addl $0,-132(%rsp)", options(nostack));
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn smp_store_release<T>(p: *mut T, v: T) {
    unsafe {
        barrier();
        core::ptr::write_volatile(p, v);
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn smp_load_acquire<T: Copy>(p: *const T) -> T {
    unsafe {
        let ___p1: T = core::ptr::read_volatile(p);
        barrier();
        ___p1
    }
}
