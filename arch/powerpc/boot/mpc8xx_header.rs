/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: `u32` is supplied by the translated `types.h`.

extern "C" {
    pub fn mpc8xx_set_clocks(sysclk: u32);

    pub fn mpc885_get_clock(crystal: u32) -> u32;
    pub fn mpc885_fixup_clocks(crystal: u32) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
