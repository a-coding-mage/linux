/* SPDX-License-Identifier: GPL-2.0 */

// Depends on external definitions corresponding to C's u64 and bool.

unsafe extern "C" {
    pub fn cacheline_size() -> i32;
}

/*
 * Some architectures have 'Adjacent Cacheline Prefetch' feature,
 * which performs like the cacheline size being doubled.
 */
#[inline]
pub unsafe fn cl_address(address: u64, double_cl: bool) -> u64 {
    let mut size: u64 = unsafe { cacheline_size() } as u64;

    if double_cl {
        size *= 2;
    }

    /* return the cacheline of the address */
    address & !(size - 1)
}

#[inline]
pub unsafe fn cl_offset(address: u64, double_cl: bool) -> u64 {
    let mut size: u64 = unsafe { cacheline_size() } as u64;

    if double_cl {
        size *= 2;
    }

    /* return the offset inside cacheline */
    address & (size - 1)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
