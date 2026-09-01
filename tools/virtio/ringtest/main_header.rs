// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Red Hat, Inc.
 * Author: Michael S. Tsirkin <mst@redhat.com>
 *
 * Common macros and functions for ring benchmarking.
 */

use core::ffi::{c_int, c_uint, c_void};
use core::ptr;
use core::sync::atomic::{compiler_fence, fence, Ordering};

unsafe extern "C" {
    pub static mut param: c_int;
    pub static mut do_exit: bool;
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline]
pub unsafe fn wait_cycles(cycles: u64) {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_rdtsc;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_rdtsc;

    let t = unsafe { _rdtsc() };
    while unsafe { _rdtsc() }.wrapping_sub(t) < cycles {}
}

#[cfg(target_arch = "s390x")]
#[inline]
pub unsafe fn wait_cycles(cycles: u64) {
    unsafe {
        core::arch::asm!("0: brctg {0},0b", in(reg) cycles, options(nostack, preserves_flags));
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "s390x")))]
#[inline]
pub unsafe fn wait_cycles(_cycles: u64) {
    std::process::exit(5);
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub const VMEXIT_CYCLES: u64 = 500;
#[cfg(target_arch = "s390x")]
pub const VMEXIT_CYCLES: u64 = 200;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "s390x")))]
pub const VMEXIT_CYCLES: u64 = 0;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub const VMENTRY_CYCLES: u64 = 500;
#[cfg(target_arch = "s390x")]
pub const VMENTRY_CYCLES: u64 = 200;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "s390x")))]
pub const VMENTRY_CYCLES: u64 = 0;

#[inline]
pub unsafe fn vmexit() {
    if unsafe { !do_exit } {
        return;
    }

    unsafe { wait_cycles(VMEXIT_CYCLES) };
}

#[inline]
pub unsafe fn vmentry() {
    if unsafe { !do_exit } {
        return;
    }

    unsafe { wait_cycles(VMENTRY_CYCLES) };
}

/* implemented by ring */
unsafe extern "C" {
    pub fn alloc_ring();
    /* guest side */
    pub fn add_inbuf(arg1: c_uint, arg2: *mut c_void, arg3: *mut c_void) -> c_int;
    pub fn get_buf(arg1: *mut c_uint, arg2: *mut *mut c_void) -> *mut c_void;
    pub fn disable_call();
    pub fn used_empty() -> bool;
    pub fn enable_call() -> bool;
    pub fn kick_available();
    /* host side */
    pub fn disable_kick();
    pub fn avail_empty() -> bool;
    pub fn enable_kick() -> bool;
    pub fn use_buf(arg1: *mut c_uint, arg2: *mut *mut c_void) -> bool;
    pub fn call_used();
}

/* implemented by main */
unsafe extern "C" {
    pub static mut do_sleep: bool;
    pub fn kick();
    pub fn wait_for_kick();
    pub fn call();
    pub fn wait_for_call();

    pub static mut ring_size: c_uint;
}

/* Compiler barrier - similar to what Linux uses */
#[inline]
pub fn barrier() {
    compiler_fence(Ordering::SeqCst);
}

/* Is there a portable way to do this? */
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline]
pub unsafe fn cpu_relax() {
    unsafe {
        core::arch::asm!("rep; nop", options(nostack, preserves_flags));
    }
}

#[cfg(target_arch = "s390x")]
#[inline]
pub fn cpu_relax() {
    barrier();
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn cpu_relax() {
    unsafe {
        core::arch::asm!("yield", options(nostack, preserves_flags));
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "s390x", target_arch = "aarch64")))]
#[inline]
pub fn cpu_relax() {
    assert!(false);
}

unsafe extern "C" {
    pub static mut do_relax: bool;
}

#[inline]
pub unsafe fn busy_wait() {
    if unsafe { do_relax } {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"))]
        unsafe {
            cpu_relax();
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
        cpu_relax();
    } else {
        /* prevent compiler from removing busy loops */
        barrier();
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline]
pub unsafe fn smp_mb() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("lock; addl $0,-132(%rsp)", options(nostack));
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::asm!("lock; addl $0,-132(%esp)", options(nostack));
    }
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn smp_mb() {
    unsafe {
        core::arch::asm!("dmb ish", options(nostack, preserves_flags));
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
#[inline]
pub fn smp_mb() {
    /*
     * Not using __ATOMIC_SEQ_CST since gcc docs say they are only synchronized
     * with other __ATOMIC_SEQ_CST calls.
     */
    fence(Ordering::SeqCst);
}

/*
 * This abuses the atomic builtins for thread fences, and
 * adds a compiler barrier.
 */
#[inline]
pub fn smp_release() {
    barrier();
    fence(Ordering::Release);
}

#[inline]
pub fn smp_acquire() {
    fence(Ordering::Acquire);
    barrier();
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "s390x"))]
#[inline]
pub fn smp_wmb() {
    barrier();
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn smp_wmb() {
    unsafe {
        core::arch::asm!("dmb ishst", options(nostack, preserves_flags));
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "s390x", target_arch = "aarch64")))]
#[inline]
pub fn smp_wmb() {
    smp_release();
}

#[inline(always)]
pub unsafe fn __read_once_size(p: *const c_void, res: *mut c_void, size: c_int) {
    match size {
        1 => unsafe {
            *(res as *mut u8) = ptr::read_volatile(p as *const u8);
        },
        2 => unsafe {
            *(res as *mut u16) = ptr::read_volatile(p as *const u16);
        },
        4 => unsafe {
            *(res as *mut c_uint) = ptr::read_volatile(p as *const c_uint);
        },
        8 => unsafe {
            *(res as *mut u64) = ptr::read_volatile(p as *const u64);
        },
        _ => unsafe {
            barrier();
            ptr::copy_nonoverlapping(p as *const u8, res as *mut u8, size as usize);
            barrier();
        },
    }
}

#[inline(always)]
pub unsafe fn __write_once_size(p: *mut c_void, res: *mut c_void, size: c_int) {
    match size {
        1 => unsafe {
            ptr::write_volatile(p as *mut u8, *(res as *mut u8));
        },
        2 => unsafe {
            ptr::write_volatile(p as *mut u16, *(res as *mut u16));
        },
        4 => unsafe {
            ptr::write_volatile(p as *mut c_uint, *(res as *mut c_uint));
        },
        8 => unsafe {
            ptr::write_volatile(p as *mut u64, *(res as *mut u64));
        },
        _ => unsafe {
            barrier();
            ptr::copy_nonoverlapping(res as *const u8, p as *mut u8, size as usize);
            barrier();
        },
    }
}

#[cfg(target_arch = "alpha")]
#[inline]
pub unsafe fn READ_ONCE<T: Copy>(x: *const T) -> T {
    let mut val = core::mem::MaybeUninit::<T>::uninit();
    unsafe {
        __read_once_size(
            x as *const c_void,
            val.as_mut_ptr() as *mut c_void,
            core::mem::size_of::<T>() as c_int,
        );
        smp_mb();
        val.assume_init()
    }
}

#[cfg(not(target_arch = "alpha"))]
#[inline]
pub unsafe fn READ_ONCE<T: Copy>(x: *const T) -> T {
    let mut val = core::mem::MaybeUninit::<T>::uninit();
    unsafe {
        __read_once_size(
            x as *const c_void,
            val.as_mut_ptr() as *mut c_void,
            core::mem::size_of::<T>() as c_int,
        );
        val.assume_init()
    }
}

#[inline]
pub unsafe fn WRITE_ONCE<T: Copy>(x: *mut T, val: T) -> T {
    let mut tmp = val;
    unsafe {
        __write_once_size(
            x as *mut c_void,
            &mut tmp as *mut T as *mut c_void,
            core::mem::size_of::<T>() as c_int,
        );
    }
    tmp
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
