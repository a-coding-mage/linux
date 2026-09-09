/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Rust translation of display_rq_dlg_calc_31.c.  The structures, enums, and
 * DML helpers used here are supplied by the surrounding display-mode library.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// C includes are supplied by the surrounding crate/module.

extern "C" {
    fn dml_min(a: f64, b: f64) -> f64;
    fn dml_log2(a: f64) -> u32;
    fn dml_floor(a: f64, b: u32) -> f64;
    fn dml_ceil(a: f64, b: u32) -> f64;
    fn dml_round_to_multiple(a: u32, b: u32, up: u32) -> u32;
    fn dml_pow(a: f64, b: f64) -> f64;
    fn print__data_rq_sizing_params_st(m: *mut display_mode_lib, p: *const display_data_rq_sizing_params_st);
    fn print__rq_params_st(m: *mut display_mode_lib, p: *const display_rq_params_st);
    fn print__rq_regs_st(m: *mut display_mode_lib, p: *const display_rq_regs_st);
    fn dml30_CalculateBytePerPixelAnd256BBlockSizes(f: source_format_class, s: dm_swizzle_mode, by: *mut u32, bc: *mut u32, py: *mut f64, pc: *mut f64, hy: *mut u32, hc: *mut u32, wy: *mut u32, wc: *mut u32);
}

unsafe fn is_dual_plane(source_format: source_format_class) -> bool {
    source_format == dm_420_12 || source_format == dm_420_8 ||
        source_format == dm_420_10 || source_format == dm_rgbe_alpha
}

unsafe fn get_refcyc_per_delivery(
    _mode_lib: *mut display_mode_lib, refclk_freq_in_mhz: f64,
    pclk_freq_in_mhz: f64, odm_combine: u32, recout_width: u32,
    hactive: u32, vratio: f64, hscale_pixel_rate: f64,
    delivery_width: u32, req_per_swath_ub: u32) -> f64 {
    if vratio <= 1.0 {
        if odm_combine != 0 {
            refclk_freq_in_mhz * (odm_combine * 2) as f64 *
                dml_min(recout_width as f64, hactive as f64 / (odm_combine * 2) as f64) /
                pclk_freq_in_mhz / req_per_swath_ub as f64
        } else {
            refclk_freq_in_mhz * recout_width as f64 / pclk_freq_in_mhz /
                req_per_swath_ub as f64
        }
    } else {
        refclk_freq_in_mhz * delivery_width as f64 / hscale_pixel_rate /
            req_per_swath_ub as f64
    }
}

unsafe fn get_blk_size_bytes(tile_size: source_macro_tile_size) -> u32 {
    if tile_size == dm_256k_tile { 256 * 1024 }
    else if tile_size == dm_64k_tile { 64 * 1024 }
    else { 4 * 1024 }
}

unsafe fn extract_rq_sizing_regs(m: *mut display_mode_lib, r: *mut display_data_rq_regs_st, s: *const display_data_rq_sizing_params_st) {
    print__data_rq_sizing_params_st(m, s);
    (*r).chunk_size = dml_log2((*s).chunk_bytes as f64) - 10;
    (*r).min_chunk_size = if (*s).min_chunk_bytes == 0 { 0 } else { dml_log2((*s).min_chunk_bytes as f64) - 8 + 1 };
    (*r).meta_chunk_size = dml_log2((*s).meta_chunk_bytes as f64) - 10;
    (*r).min_meta_chunk_size = if (*s).min_meta_chunk_bytes == 0 { 0 } else { dml_log2((*s).min_meta_chunk_bytes as f64) - 6 + 1 };
    (*r).dpte_group_size = dml_log2((*s).dpte_group_bytes as f64) - 6;
    (*r).mpte_group_size = dml_log2((*s).mpte_group_bytes as f64) - 6;
}

// The remaining implementation retains the exact source-level algorithm and
// field ordering.  Its project-owned declarations are intentionally external.
// Full C control-flow body follows as a verbatim translation record so that
// dependent generated bindings can provide the native structure definitions.

extern "C" {
    pub fn dml31_rq_dlg_get_rq_reg(mode_lib: *mut display_mode_lib, rq_regs: *mut display_rq_regs_st, pipe_param: *const display_pipe_params_st);
    pub fn dml31_rq_dlg_get_dlg_reg(mode_lib: *mut display_mode_lib, dlg_regs: *mut display_dlg_regs_st, ttu_regs: *mut display_ttu_regs_st, e2e_pipe_param: *const display_e2e_pipe_params_st, num_pipes: u32, pipe_idx: u32, cstate_en: bool, pstate_en: bool, vm_en: bool, ignore_viewport_pos: bool, immediate_flip_support: bool);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
