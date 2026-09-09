/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 */

// Translated from __BCM47XX_SPROM_H.
// Dependencies corresponding to <linux/errno.h>, <linux/types.h>, and
// <linux/vmalloc.h> are supplied by other translation units.

#[repr(C)]
pub struct ssb_sprom {
    _private: [u8; 0],
}

#[cfg(CONFIG_BCM47XX_SPROM)]
extern "C" {
    pub fn bcm47xx_fill_sprom(
        sprom: *mut ssb_sprom,
        prefix: *const core::ffi::c_char,
        fallback: bool,
    );
    pub fn bcm47xx_sprom_register_fallbacks() -> core::ffi::c_int;
}

#[cfg(not(CONFIG_BCM47XX_SPROM))]
#[inline]
pub fn bcm47xx_fill_sprom(
    _sprom: *mut ssb_sprom,
    _prefix: *const core::ffi::c_char,
    _fallback: bool,
) {
}

#[cfg(not(CONFIG_BCM47XX_SPROM))]
#[inline]
pub fn bcm47xx_sprom_register_fallbacks() -> core::ffi::c_int {
    // ENOTSUPP is provided by the translated Linux errno dependency.
    -ENOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
