/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::c_void;

extern "C" {
    pub fn pxa_smemc_set_pcmcia_timing(sock: i32, mcmem: u32, mcatt: u32, mcio: u32);
    pub fn pxa_smemc_set_pcmcia_socket(nr: i32);
    pub fn pxa2xx_smemc_get_sdram_rows() -> i32;
    pub fn pxa3xx_smemc_get_memclkdiv() -> u32;
    pub fn pxa_smemc_get_mdrefr() -> *mut c_void;
}

/*
 * Once fully converted to the clock framework, all these functions should be
 * removed, and replaced with a clk_get(NULL, "core").
 *
 * The original CONFIG_PXA25x conditional is represented by the corresponding
 * Rust feature.  Enable the feature when the kernel configuration symbol is
 * enabled.
 */
#[cfg(feature = "CONFIG_PXA25x")]
extern "C" {
    pub fn pxa25x_get_clk_frequency_khz(x: i32) -> u32;
}

#[cfg(not(feature = "CONFIG_PXA25x"))]
#[inline]
pub const fn pxa25x_get_clk_frequency_khz(_x: i32) -> u32 {
    0
}

/* The original CONFIG_PXA27x conditional is represented by the corresponding
 * Rust feature. */
#[cfg(feature = "CONFIG_PXA27x")]
extern "C" {
    pub fn pxa27x_get_clk_frequency_khz(x: i32) -> u32;
}

#[cfg(not(feature = "CONFIG_PXA27x"))]
#[inline]
pub const fn pxa27x_get_clk_frequency_khz(_x: i32) -> u32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
