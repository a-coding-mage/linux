/* SPDX-License-Identifier: GPL-2.0 */

/*
 * If you have a cycle counter, return the value here.
 */
pub type cycles_t = ::core::ffi::c_ulong;

/* C conditional: provide this only when get_cycles is not supplied externally. */
#[inline]
pub fn get_cycles() -> cycles_t {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
