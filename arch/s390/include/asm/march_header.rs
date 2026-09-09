/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm/march.h.
// The original include and header guard are C-only constructs.

pub const MARCH_HAS_Z10_FEATURES: i32 = 1;

// The following configuration conditions correspond to the original
// CONFIG_HAVE_MARCH_* preprocessor symbols.  The __DECOMPRESSOR guard is
// preserved as the absence of the `decompressor` Cargo configuration.
#[cfg(not(feature = "decompressor"))]
pub mod non_decompressor {
    #[cfg(feature = "CONFIG_HAVE_MARCH_Z196_FEATURES")]
    pub const MARCH_HAS_Z196_FEATURES: i32 = 1;

    #[cfg(feature = "CONFIG_HAVE_MARCH_ZEC12_FEATURES")]
    pub const MARCH_HAS_ZEC12_FEATURES: i32 = 1;

    #[cfg(feature = "CONFIG_HAVE_MARCH_Z13_FEATURES")]
    pub const MARCH_HAS_Z13_FEATURES: i32 = 1;

    #[cfg(feature = "CONFIG_HAVE_MARCH_Z14_FEATURES")]
    pub const MARCH_HAS_Z14_FEATURES: i32 = 1;

    #[cfg(feature = "CONFIG_HAVE_MARCH_Z15_FEATURES")]
    pub const MARCH_HAS_Z15_FEATURES: i32 = 1;

    #[cfg(feature = "CONFIG_HAVE_MARCH_Z16_FEATURES")]
    pub const MARCH_HAS_Z16_FEATURES: i32 = 1;

    #[cfg(feature = "CONFIG_HAVE_MARCH_Z17_FEATURES")]
    pub const MARCH_HAS_Z17_FEATURES: i32 = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
