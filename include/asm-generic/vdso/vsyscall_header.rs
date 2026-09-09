/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guards and the assembler-only condition are omitted; this file is
 * the non-assembler Rust translation of the header contents.
 *
 * The following types and symbols are supplied by other translation units:
 * `vdso_time_data`, `vdso_rng_data`, `vdso_clock`, `vdso_u_time_data`, and
 * `vdso_u_rng_data`.
 */

#[cfg(not(feature = "arch_get_vdso_u_time_data"))]
#[inline(always)]
pub unsafe fn __arch_get_vdso_u_time_data() -> *const vdso_time_data {
    &raw const vdso_u_time_data
}

#[cfg(not(feature = "arch_get_vdso_u_rng_data"))]
#[inline(always)]
pub unsafe fn __arch_get_vdso_u_rng_data() -> *const vdso_rng_data {
    &raw const vdso_u_rng_data
}

#[cfg(not(feature = "arch_update_vdso_clock"))]
#[inline(always)]
pub unsafe fn __arch_update_vdso_clock(_vc: *mut vdso_clock) {}

#[cfg(not(feature = "arch_sync_vdso_time_data"))]
#[inline(always)]
pub unsafe fn __arch_sync_vdso_time_data(_vdata: *mut vdso_time_data) {}

extern "C" {
    pub static vdso_u_time_data: vdso_time_data;
    pub static vdso_u_rng_data: vdso_rng_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
