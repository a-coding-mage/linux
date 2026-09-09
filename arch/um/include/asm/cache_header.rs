/* SPDX-License-Identifier: GPL-2.0 */

// The original header guard was: __UM_CACHE_H

// The C preprocessor conditions are represented with Cargo feature cfgs.
// CONFIG_X86_L1_CACHE_SHIFT is supplied by the surrounding build.
#[cfg(all(feature = "CONFIG_UML_X86", not(feature = "CONFIG_64BIT")))]
pub const L1_CACHE_SHIFT: usize = CONFIG_X86_L1_CACHE_SHIFT as usize;

#[cfg(all(feature = "CONFIG_UML_X86", feature = "CONFIG_64BIT"))]
pub const L1_CACHE_SHIFT: usize = 6; // Should be 7 on Intel

// XXX: this was taken from x86, now it's completely random. Luckily only
// affects SMP padding.
#[cfg(not(feature = "CONFIG_UML_X86"))]
pub const L1_CACHE_SHIFT: usize = 5;

pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
