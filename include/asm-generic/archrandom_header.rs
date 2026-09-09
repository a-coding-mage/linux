/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the generic architecture-random header.
// The original header guard is omitted because Rust items are module-scoped.

/// Returns the number of random unsigned longs written to `v`.
///
/// Corresponds to the C `__must_check` attribute and `static inline`
/// declaration.  This generic implementation provides no random values.
pub unsafe fn arch_get_random_longs(
    _v: *mut ::core::ffi::c_ulong,
    _max_longs: usize,
) -> usize {
    0
}

/// Returns the number of random seed unsigned longs written to `v`.
///
/// Corresponds to the C `__must_check` attribute and `static inline`
/// declaration.  This generic implementation provides no random values.
pub unsafe fn arch_get_random_seed_longs(
    _v: *mut ::core::ffi::c_ulong,
    _max_longs: usize,
) -> usize {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
