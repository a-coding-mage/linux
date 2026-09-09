/* SPDX-License-Identifier: GPL-2.0 */

/// Equivalent of the C `static inline` function.
#[inline]
pub unsafe fn arch_get_random_longs(
    _v: *mut core::ffi::c_ulong,
    _max_longs: usize,
) -> usize {
    0
}

unsafe extern "C" {
    pub fn arch_get_random_seed_longs(
        v: *mut core::ffi::c_ulong,
        max_longs: usize,
    ) -> usize;
}

// Preserved build-time condition: CONFIG_PPC_POWERNV.
#[cfg(CONFIG_PPC_POWERNV)]
unsafe extern "C" {
    pub fn pnv_get_random_long(v: *mut core::ffi::c_ulong) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
