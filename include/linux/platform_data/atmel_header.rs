/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * atmel platform data
 */

/* FIXME: this needs a better location, but gets stuff building again */
/* CONFIG_ATMEL_PM build-time condition preserved as a Rust feature gate. */
#[cfg(CONFIG_ATMEL_PM)]
extern "C" {
    pub fn at91_suspend_entering_slow_clock() -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_ATMEL_PM))]
#[inline]
pub fn at91_suspend_entering_slow_clock() -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
