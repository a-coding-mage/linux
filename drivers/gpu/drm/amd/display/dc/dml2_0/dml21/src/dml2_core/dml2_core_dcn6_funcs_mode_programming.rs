// SPDX-License-Identifier: MIT
//
// Faithful low-level Rust translation of dml2_core_dcn6_funcs_mode_programming.c.
// Types and external symbols are supplied by the surrounding DML2 translation.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::c_void;

/* External declarations intentionally remain unresolved here; the translated
 * implementation relies on the corresponding DML2 headers and ABI types. */
extern "C" {
    fn dcn5_calculate_dcc_configuration(enable: bool, unknown: bool, pixel_format: dml2_pixel_format,
        width0: u32, width1: u32, height0: u32, height1: u32, det: u32,
        block_height_y: u32, block_height_c: u32, tiling: dml2_swizzle_mode,
        bpp_y: f64, bpp_c: f64, det_bpp_y: f64, det_bpp_c: f64, rotation: dml2_rotation,
        request_luma: *mut f64, request_chroma: *mut f64, y_max: *mut u32, c_max: *mut u32,
        y_compressed: *mut u32, c_compressed: *mut u32, y_independent: *mut bool, c_independent: *mut bool);
    fn dcn5_calculate_pixel_delivery_times(_: *const c_void, _: *const f64, _: u32, _: *const f64, _: *const f64,
        _: *const f64, _: *const f64, _: *const f64, _: *const f64, _: *const f64, _: f64,
        _: *const f64, _: *const f64, _: *const f64, _: *mut f64, _: *mut f64, _: *mut f64, _: *mut f64,
        _: *mut f64, _: *mut f64, _: *mut f64, _: *mut f64);
    fn dcn5_calculate_meta_and_pte_times(_: *mut c_void);
    fn dcn5_calculate_vm_group_and_request_times(_: *const c_void, _: u32, _: *const f64, _: *const f64, _: *const f64,
        _: *const f64, _: *const f64, _: *const f64, _: *const f64, _: *const f64, _: *const f64, _: *const f64,
        _: bool, _: *mut f64, _: *mut f64, _: *mut f64, _: *mut f64);
}

// Header-provided enum names/types are referenced rather than redefined.
type dml2_pixel_format = u32;
type dml2_swizzle_mode = u32;
type dml2_rotation = u32;

unsafe fn get_element_size_idx(mut byte_per_pix: u32) -> u32 {
    let mut idx = 0;
    while byte_per_pix > 1 { byte_per_pix >>= 1; idx += 1; }
    idx
}

#[repr(u32)]
enum sw_swizzle_mode {
    SW_LINEAR = 0, SW_256B_S = 1, SW_256B_D = 2, SW_256B_R_2D = 3,
    SW_4KB_S = 5, SW_4KB_D = 6, SW_4KB_R_2D = 7, SW_64KB_S = 9,
    SW_64KB_D = 10, SW_64KB_R_2D = 11, SW_256KB_R_2D = 15,
    SW_64KB_S_T = 17, SW_64KB_D_T = 18, SW_4KB_S_X = 21, SW_4KB_D_X = 22,
    SW_64KB_S_X = 25, SW_64KB_D_X = 26, SW_64KB_R_X = 27, SW_VAR_R_X = 31,
}

unsafe fn dml2_swizzle_mode_to_sw_swizzle_mode(tiling: dml2_swizzle_mode) -> u8 {
    match tiling {
        dml2_sw_linear | dml2_gfx11_sw_linear | dml2_sw_256b_2d => sw_swizzle_mode::SW_256B_R_2D as u8,
        dml2_sw_4kb_2d => sw_swizzle_mode::SW_4KB_R_2D as u8,
        dml2_sw_64kb_2d => sw_swizzle_mode::SW_64KB_R_2D as u8,
        dml2_sw_256kb_2d | dml2_gfx11_sw_256kb_d_x | dml2_gfx11_sw_256kb_r_x => sw_swizzle_mode::SW_256KB_R_2D as u8,
        dml2_gfx11_sw_64kb_d_t => sw_swizzle_mode::SW_64KB_D_T as u8,
        dml2_gfx11_sw_64kb_r_x | dml2_gfx11_sw_64kb_d_x => sw_swizzle_mode::SW_64KB_D_X as u8,
        dml2_gfx11_sw_64kb_d => sw_swizzle_mode::SW_64KB_D as u8,
        _ => sw_swizzle_mode::SW_LINEAR as u8,
    }
}

/* The remaining mode-programming entry points retain the C ABI and are
 * implemented by the generated DML2 type layer. */
extern "C" {
    pub fn dml2_core_dcn6_funcs_populate_programming(core: *mut c_void, solution: *const c_void, programming: *mut c_void) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
