/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_DEBUG_UNCOMPRESS is a build-time condition from the original header.
#[cfg(CONFIG_DEBUG_UNCOMPRESS)]
unsafe extern "C" {
    pub fn putc(c: core::ffi::c_int);
}

#[cfg(not(CONFIG_DEBUG_UNCOMPRESS))]
#[inline]
pub fn putc(_c: core::ffi::c_int) {}

#[inline]
pub fn flush() {}

#[inline]
pub fn arch_decomp_setup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
