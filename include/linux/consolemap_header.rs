/* SPDX-License-Identifier: GPL-2.0 */
/*
 * consolemap.h
 *
 * Interface between console.c, selection.c  and consolemap.c
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum translation_map {
    LAT1_MAP,
    GRAF_MAP,
    IBMPC_MAP,
    USER_MAP,
}

pub const FIRST_MAP: translation_map = translation_map::LAT1_MAP;
pub const LAST_MAP: translation_map = translation_map::USER_MAP;

#[repr(C)]
pub struct vc_data {
    _private: [u8; 0],
}

/* CONFIG_CONSOLE_TRANSLATIONS is a build-time condition from the C header. */
#[cfg(feature = "CONFIG_CONSOLE_TRANSLATIONS")]
extern "C" {
    pub fn inverse_translate(conp: *const vc_data, glyph: u16, use_unicode: bool) -> u16;
    pub fn set_translate(m: translation_map, vc: *mut vc_data) -> *mut u16;
    pub fn conv_uni_to_pc(conp: *mut vc_data, ucs: i64) -> i32;
    pub fn conv_8bit_to_uni(c: u8) -> u32;
    pub fn conv_uni_to_8bit(uni: u32) -> i32;
    pub fn console_map_init();
    pub fn ucs_get_width(cp: u32) -> u32;
    pub fn ucs_recompose(base: u32, mark: u32) -> u32;
    pub fn ucs_get_fallback(cp: u32) -> u32;
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn inverse_translate(_conp: *const vc_data, glyph: u16, _use_unicode: bool) -> u16 {
    glyph
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn set_translate(_m: translation_map, _vc: *mut vc_data) -> *mut u16 {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn conv_uni_to_pc(_conp: *mut vc_data, ucs: i64) -> i32 {
    if ucs > 0xff { -1 } else { ucs as i32 }
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn conv_8bit_to_uni(c: u8) -> u32 {
    c as u32
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn conv_uni_to_8bit(uni: u32) -> i32 {
    (uni & 0xff) as i32
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn console_map_init() {}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn ucs_get_width(_cp: u32) -> u32 {
    1
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn ucs_recompose(_base: u32, _mark: u32) -> u32 {
    0
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
#[inline]
pub unsafe fn ucs_get_fallback(_cp: u32) -> u32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
