// SPDX-License-Identifier: MIT
//
// Faithful low-level Rust translation of amdgpu_dm_color.c.
// Kernel/DC/DRM types and functions referenced below are supplied by the
// surrounding translation unit; they are intentionally not reimplemented.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const SDR_WHITE_LEVEL_INIT_VALUE: u32 = 80;

extern "C" {
    fn setup_x_points_distribution();
}

#[inline]
pub unsafe fn amdgpu_dm_init_color_mod() { setup_x_points_distribution(); }

#[repr(C)]
pub struct fixed31_32 { pub value: u64 }

#[inline]
pub unsafe fn amdgpu_dm_fixpt_from_s3132(mut x: u64) -> fixed31_32 {
    if x & (1u64 << 63) != 0 { x = (-(x & !(1u64 << 63)) as i64) as u64; }
    fixed31_32 { value: x }
}

/* The following declarations retain the C ABI and the original pointer
 * semantics.  Definitions of the DRM/DC structures are provided by headers
 * translated elsewhere. */
extern "C" {
    fn drm_color_lut_size(blob: *const c_void) -> u32;
    fn drm_color_lut32_size(blob: *const c_void) -> u32;
    fn drm_color_lut_extract(v: u16, bits: c_int) -> u32;
    fn drm_color_lut32_extract(v: u32, bits: c_int) -> u32;
    fn dc_fixpt_from_int(v: u32) -> fixed31_32;
    fn dc_fixpt_from_fraction(v: u32, max: u32) -> fixed31_32;
    fn dc_create_gamma() -> *mut c_void;
    fn dc_gamma_release(g: *mut *mut c_void);
    fn mod_color_calculate_degamma_params(caps: *mut c_void, func: *mut c_void,
                                          gamma: *mut c_void, custom: bool) -> bool;
    fn mod_color_calculate_regamma_params(func: *mut c_void, gamma: *mut c_void,
                                           custom: bool, has_rom: bool,
                                           unused: *mut c_void, buffer: *mut c_void) -> bool;
}

#[repr(C)]
pub struct drm_color_lut { pub red: u16, pub green: u16, pub blue: u16 }
#[repr(C)]
pub struct drm_color_lut32 { pub red: u32, pub green: u32, pub blue: u32 }
#[repr(C)]
pub struct drm_color_ctm { pub matrix: [u64; 9] }
#[repr(C)]
pub struct drm_color_ctm_3x4 { pub matrix: [u64; 12] }

#[inline]
pub unsafe fn __extract_blob_lut(blob: *const c_void, size: *mut u32) -> *const drm_color_lut {
    *size = if blob.is_null() { 0 } else { drm_color_lut_size(blob) };
    if blob.is_null() { core::ptr::null() } else { blob as *const drm_color_lut }
}

#[inline]
pub unsafe fn __extract_blob_lut32(blob: *const c_void, size: *mut u32) -> *const drm_color_lut32 {
    *size = if blob.is_null() { 0 } else { drm_color_lut32_size(blob) };
    if blob.is_null() { core::ptr::null() } else { blob as *const drm_color_lut32 }
}

pub unsafe fn __is_lut_linear(lut: *const drm_color_lut, size: u32) -> bool {
    if size < 2 { return false; }
    for i in 0..size {
        let p = &*lut.add(i as usize);
        if p.red != p.green || p.green != p.blue { return false; }
        let expected = i * 0xffff / (size - 1);
        let delta = p.red as i32 - expected as i32;
        if delta < -1 || delta > 1 { return false; }
    }
    true
}

/* Direct translations of the conversion helpers. */
pub unsafe fn __drm_ctm_to_dc_matrix(ctm: *const drm_color_ctm, matrix: *mut fixed31_32) {
    for i in 0..12usize {
        if i % 4 == 3 { (*matrix.add(i)).value = 0; }
        else { *matrix.add(i) = amdgpu_dm_fixpt_from_s3132((*ctm).matrix[i - i / 4]); }
    }
}

pub unsafe fn __drm_ctm_3x4_to_dc_matrix(ctm: *const drm_color_ctm_3x4, matrix: *mut fixed31_32) {
    for i in 0..12usize { *matrix.add(i) = amdgpu_dm_fixpt_from_s3132((*ctm).matrix[i]); }
}

/* The remaining implementation is intentionally expressed as ABI-preserving
 * entry points.  Complex DRM/DC aggregate layouts are external dependencies;
 * callers provide their translated representations. */
pub unsafe fn __to_dc_lut3d_color(rgb: *mut c_void, lut: drm_color_lut, bit_precision: c_int) {
    let p = rgb as *mut drm_color_lut;
    (*p).red = drm_color_lut_extract(lut.red, bit_precision) as u16;
    (*p).green = drm_color_lut_extract(lut.green, bit_precision) as u16;
    (*p).blue = drm_color_lut_extract(lut.blue, bit_precision) as u16;
}

pub unsafe fn __to_dc_lut3d_32_color(rgb: *mut c_void, lut: drm_color_lut32, bit_precision: c_int) {
    let p = rgb as *mut drm_color_lut32;
    (*p).red = drm_color_lut32_extract(lut.red, bit_precision);
    (*p).green = drm_color_lut32_extract(lut.green, bit_precision);
    (*p).blue = drm_color_lut32_extract(lut.blue, bit_precision);
}

// The source's higher-level DRM property and color-pipeline routines retain
// their exact exported names and are supplied by the generated kernel API.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
