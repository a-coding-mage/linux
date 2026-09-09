/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Kernel interface for the s390 arch_random_* functions
 *
 * Copyright IBM Corp. 2017, 2022
 *
 * Author: Harald Freudenberger <freude@de.ibm.com>
 *
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/static_key.h, linux/preempt.h, linux/atomic.h, and asm/cpacf.h.

extern "C" {
    pub static mut s390_arch_random_available: StaticKey;
    pub static mut s390_arch_random_counter: Atomic64;

    pub fn static_branch_likely(key: *const StaticKey) -> bool;
    pub fn in_task() -> bool;
    pub fn cpacf_trng(r1: *mut core::ffi::c_void, r2: u32, output: *mut u8, len: usize);
    pub fn atomic64_add(i: i64, v: *mut Atomic64);
}

// External kernel types supplied by the corresponding dependencies.
pub type StaticKey = core::ffi::c_void;
pub type Atomic64 = core::ffi::c_void;

pub unsafe fn arch_get_random_longs(v: *mut usize, max_longs: usize) -> usize {
    let _ = v;
    let _ = max_longs;
    0
}

pub unsafe fn arch_get_random_seed_longs(v: *mut usize, max_longs: usize) -> usize {
    if static_branch_likely(&s390_arch_random_available as *const StaticKey)
        && in_task()
    {
        cpacf_trng(
            core::ptr::null_mut(),
            0,
            v as *mut u8,
            max_longs * core::mem::size_of::<usize>(),
        );
        atomic64_add(
            (max_longs * core::mem::size_of::<usize>()) as i64,
            &mut s390_arch_random_counter as *mut Atomic64,
        );
        return max_longs;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
