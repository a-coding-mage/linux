/*
 * arch/xtensa/platform/xtavnet/include/platform/lcd.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001, 2006 Tensilica Inc.
 */

// The original CONFIG_XTFPGA_LCD preprocessor condition is represented by
// the corresponding Rust configuration feature.
#[cfg(feature = "CONFIG_XTFPGA_LCD")]
extern "C" {
    /* Display string STR at position POS on the LCD. */
    pub fn lcd_disp_at_pos(str_: *mut core::ffi::c_char, pos: u8);

    /* Shift the contents of the LCD display left or right. */
    pub fn lcd_shiftleft();
    pub fn lcd_shiftright();
}

#[cfg(not(feature = "CONFIG_XTFPGA_LCD"))]
#[inline]
pub unsafe fn lcd_disp_at_pos(_str: *mut core::ffi::c_char, _pos: u8) {}

#[cfg(not(feature = "CONFIG_XTFPGA_LCD"))]
#[inline]
pub unsafe fn lcd_shiftleft() {}

#[cfg(not(feature = "CONFIG_XTFPGA_LCD"))]
#[inline]
pub unsafe fn lcd_shiftright() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
