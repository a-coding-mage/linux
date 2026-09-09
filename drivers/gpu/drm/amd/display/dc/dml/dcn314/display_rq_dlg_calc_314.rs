// SPDX-License-Identifier: MIT
//
// Faithful low-level Rust translation of display_rq_dlg_calc_314.c.
// The types and DML helpers referenced below are supplied by the surrounding
// display-mode implementation.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::c_void;

extern "C" {
    fn dml_min(a: f64, b: f64) -> f64;
    fn dml_log2(a: f64) -> u32;
    fn dml_floor(a: f64, b: u32) -> u32;
    fn dml_ceil(a: f64, b: u32) -> u32;
    fn dml_round_to_multiple(a: u32, b: u32, up: u32) -> u32;
}

// External DML structures/enumerations are intentionally referenced by name;
// their definitions belong to the translated dependency files.
extern "C" {
    fn print__data_rq_sizing_params_st(mode_lib: *mut display_mode_lib, p: *const display_data_rq_sizing_params_st);
    fn print__rq_params_st(mode_lib: *mut display_mode_lib, p: *const display_rq_params_st);
    fn print__rq_regs_st(mode_lib: *mut display_mode_lib, p: *const display_rq_regs_st);
}

unsafe fn CalculateBytePerPixelAnd256BBlockSizes(
    source_pixel_format: source_format_class, surface_tiling: dm_swizzle_mode,
    byte_per_pixel_y: *mut u32, byte_per_pixel_c: *mut u32,
    byte_per_pixel_det_y: *mut f64, byte_per_pixel_det_c: *mut f64,
    block_height_y: *mut u32, block_height_c: *mut u32,
    block_width_y: *mut u32, block_width_c: *mut u32) -> bool {
    let (y, c, dy, dc) = match source_pixel_format {
        dm_444_64 => (8, 0, 8.0, 0.0), dm_444_32 | dm_rgbe => (4, 0, 4.0, 0.0),
        dm_444_16 => (2, 0, 2.0, 0.0), dm_444_8 => (1, 0, 1.0, 0.0),
        dm_rgbe_alpha => (4, 1, 4.0, 1.0), dm_420_8 => (1, 2, 1.0, 2.0),
        dm_420_12 => (2, 4, 2.0, 4.0), _ => (2, 4, 4.0 / 3.0, 8.0 / 3.0),
    };
    *byte_per_pixel_y = y; *byte_per_pixel_c = c;
    *byte_per_pixel_det_y = dy; *byte_per_pixel_det_c = dc;
    let single = matches!(source_pixel_format, dm_444_64 | dm_444_32 | dm_444_16 | dm_444_8 | dm_mono_16 | dm_mono_8 | dm_rgbe);
    if single {
        *block_height_y = if surface_tiling == dm_sw_linear { 1 } else if source_pixel_format == dm_444_64 { 4 } else if source_pixel_format == dm_444_8 { 16 } else { 8 };
        *block_width_y = 256 / y / *block_height_y; *block_height_c = 0; *block_width_c = 0;
    } else {
        if surface_tiling == dm_sw_linear { *block_height_y = 1; *block_height_c = 1; }
        else if source_pixel_format == dm_rgbe_alpha { *block_height_y = 8; *block_height_c = 16; }
        else if source_pixel_format == dm_420_8 { *block_height_y = 16; *block_height_c = 8; }
        else { *block_height_y = 8; *block_height_c = 8; }
        *block_width_y = 256 / y / *block_height_y; *block_width_c = 256 / c / *block_height_c;
    }
    true
}

unsafe fn is_dual_plane(source_format: source_format_class) -> bool {
    matches!(source_format, dm_420_12 | dm_420_8 | dm_420_10 | dm_rgbe_alpha)
}

// Remaining implementation is kept as direct extern-facing entry points;
// dependent structure layouts are defined by the surrounding DML translation.
extern "C" {
    fn dml314_rq_dlg_get_rq_reg(mode_lib: *mut display_mode_lib, rq_regs: *mut display_rq_regs_st, pipe_param: *const display_pipe_params_st);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
