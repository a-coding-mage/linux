/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Definitions specific to SMP platforms.
 *
 * Copyright (C) 2013 ARM Ltd.
 */

// Dependency intent: linux/cpumask.h, asm/smp.h, and asm/types.h provide the
// declarations and constants referenced by this header.

#[repr(C)]
pub struct mpidr_hash {
    pub mask: u64,
    pub shift_aff: [u32; 4],
    pub bits: u32,
}

extern "C" {
    pub static mut mpidr_hash: mpidr_hash;
    pub static nr_cpu_ids: i32;
    pub fn cpu_logical_map(cpu: i32) -> u64;
}

// -EINVAL, supplied by the kernel errno definitions.
extern "C" {
    pub static EINVAL: i32;
}

#[inline]
pub unsafe fn mpidr_hash_size() -> u32 {
    1u32 << mpidr_hash.bits
}

/*
 * Retrieve logical cpu index corresponding to a given MPIDR.Aff*
 *  - mpidr: MPIDR.Aff* bits to be used for the look-up
 *
 * Returns the cpu logical index or -EINVAL on look-up error
 */
#[inline]
pub unsafe fn get_logical_index(mpidr: u64) -> i32 {
    let mut cpu: i32 = 0;
    while cpu < nr_cpu_ids {
        if cpu_logical_map(cpu) == mpidr {
            return cpu;
        }
        cpu += 1;
    }
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
