/*
 *  font.h -- `Soft' font definitions
 *
 *  Created 1995 by Geert Uytterhoeven
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License.  See the file COPYING in the main directory of this archive
 *  for more details.
 */

/* Translated from the C header; Linux math and type definitions are external dependencies. */

use core::ffi::c_void;

#[repr(C)]
pub struct console_font {
    _private: [u8; 0],
}

/* Glyphs */

#[inline]
pub const fn font_glyph_pitch(width: u32) -> u32 {
    width.wrapping_add(7) / 8
}

#[inline]
pub const fn font_glyph_size(width: u32, vpitch: u32) -> u32 {
    font_glyph_pitch(width).wrapping_mul(vpitch)
}

/* font_data_t and helpers */

pub type font_data_t = u8;

#[inline]
pub unsafe fn font_data_buf(fd: *mut font_data_t) -> *const u8 {
    fd as *const u8
}

extern "C" {
    pub fn font_data_import(
        font: *const console_font,
        vpitch: u32,
        calc_csum: Option<unsafe extern "C" fn(u32, *const c_void, usize) -> u32>,
    ) -> *mut font_data_t;
    pub fn font_data_get(fd: *mut font_data_t);
    pub fn font_data_put(fd: *mut font_data_t) -> bool;
    pub fn font_data_size(fd: *mut font_data_t) -> u32;
    pub fn font_data_glyph_buf(
        fd: *mut font_data_t,
        width: u32,
        vpitch: u32,
        c: u32,
    ) -> *const u8;
    pub fn font_data_is_equal(lhs: *mut font_data_t, rhs: *mut font_data_t) -> bool;
    pub fn font_data_export(
        fd: *mut font_data_t,
        font: *mut console_font,
        vpitch: u32,
    ) -> i32;

    /* font_rotate.c */
    pub fn font_glyph_rotate_90(glyph: *const u8, width: u32, height: u32, out: *mut u8);
    pub fn font_glyph_rotate_180(glyph: *const u8, width: u32, height: u32, out: *mut u8);
    pub fn font_glyph_rotate_270(glyph: *const u8, width: u32, height: u32, out: *mut u8);
    pub fn font_data_rotate(
        fd: *mut font_data_t,
        width: u32,
        height: u32,
        charcount: u32,
        steps: u32,
        buf: *mut u8,
        bufsize: *mut usize,
    ) -> *mut u8;
}

/* Font description */

#[repr(C)]
pub struct font_desc {
    pub idx: i32,
    pub name: *const u8,
    pub width: u32,
    pub height: u32,
    pub charcount: u32,
    pub data: *mut font_data_t,
    pub pref: i32,
}

/* Find a font with a specific name */
extern "C" {
    pub fn find_font(name: *const u8) -> *const font_desc;

    /* Get the default font for a specific screen size */
    pub fn get_default_font(
        xres: i32,
        yres: i32,
        font_w: *mut usize,
        font_h: *mut usize,
    ) -> *const font_desc;

    /* Built-in fonts */
    pub static font_10x18: font_desc;
    pub static font_6x10: font_desc;
    pub static font_6x8: font_desc;
    pub static font_7x14: font_desc;
    pub static font_acorn_8x8: font_desc;
    pub static font_mini_4x6: font_desc;
    pub static font_pearl_8x8: font_desc;
    pub static font_sun_12x22: font_desc;
    pub static font_sun_8x16: font_desc;
    pub static font_ter_10x18: font_desc;
    pub static font_ter_16x32: font_desc;
    pub static font_vga_6x11: font_desc;
    pub static font_vga_8x16: font_desc;
    pub static font_vga_8x8: font_desc;
}

/* Max. length for the name of a predefined font */
pub const MAX_FONT_NAME: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
