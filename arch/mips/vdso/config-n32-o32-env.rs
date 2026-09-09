// SPDX-License-Identifier: GPL-2.0
/*
 * Configuration file for O32 and N32 binaries.
 * Note: To be included before lib/vdso/gettimeofday.c
 */

// Equivalent of:
// #if defined(CONFIG_MIPS32_O32) || defined(CONFIG_MIPS32_N32)
//
// In case of a 32 bit VDSO for a 64 bit kernel fake a 32 bit kernel
// configuration. CONFIG_64BIT is intentionally undefined in this
// configuration.
#[cfg(any(feature = "CONFIG_MIPS32_O32", feature = "CONFIG_MIPS32_N32"))]
pub const BUILD_VDSO32: bool = true;

#[cfg(any(feature = "CONFIG_MIPS32_O32", feature = "CONFIG_MIPS32_N32"))]
pub const CONFIG_32BIT: u32 = 1;

#[cfg(any(feature = "CONFIG_MIPS32_O32", feature = "CONFIG_MIPS32_N32"))]
pub const CONFIG_GENERIC_ATOMIC64: u32 = 1;

#[cfg(any(feature = "CONFIG_MIPS32_O32", feature = "CONFIG_MIPS32_N32"))]
pub const BUILD_VDSO32_64: bool = true;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
