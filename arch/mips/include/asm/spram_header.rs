/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_MIPS_SPRAM is a build-time C configuration condition. */
#[cfg(feature = "CONFIG_MIPS_SPRAM")]
extern "C" {
    pub fn spram_config();
}

#[cfg(not(feature = "CONFIG_MIPS_SPRAM"))]
#[inline]
pub fn spram_config() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
