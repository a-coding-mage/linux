/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Direct Rust translation of dml1_display_rq_dlg_calc.c.  The structures,
 * enums, arithmetic helpers, tracing macros, and assertion facilities used
 * below are supplied by the surrounding DML translation unit.
 */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    fn dml_floor(v: f64, p: i32) -> f64;
    fn dml_ceil(v: f64, p: i32) -> f64;
    fn dml_log2(v: u32) -> u32;
    fn dml_round_to_multiple(v: u32, m: u32, up: i32) -> u32;
    fn dml_pow(v: i32, p: i32) -> f64;
}

/* The following items intentionally retain the C ABI and pointer-oriented
 * interface.  Definitions of the shared DML records and enum constants are
 * provided by the generated companion declarations. */

#[inline]
unsafe fn get_bytes_per_element(source_format: i32, is_chroma: bool) -> u32 {
    let mut ret_val = 1;
    if source_format == dm_444_16 { if !is_chroma { ret_val = 2; } }
    else if source_format == dm_444_32 { if !is_chroma { ret_val = 4; } }
    else if source_format == dm_444_64 { if !is_chroma { ret_val = 8; } }
    else if source_format == dm_420_8 { ret_val = if is_chroma { 2 } else { 1 }; }
    else if source_format == dm_420_10 { ret_val = if is_chroma { 4 } else { 2 }; }
    ret_val
}

#[inline]
fn is_dual_plane(source_format: i32) -> bool {
    source_format == dm_420_8 || source_format == dm_420_10
}

unsafe fn get_blk256_size(w: *mut u32, h: *mut u32, b: u32) {
    match b { 1 => {*w=16;*h=16}, 2 => {*w=16;*h=8}, 4 => {*w=8;*h=8}, 8 => {*w=8;*h=4}, _ => {} }
}

unsafe fn get_blk_size_bytes(tile_size: i32) -> u32 {
    if tile_size == dm_256k_tile { 256 * 1024 }
    else if tile_size == dm_64k_tile { 64 * 1024 } else { 4 * 1024 }
}

/* The remaining implementation is kept as an ABI-faithful low-level block;
 * shared record definitions are intentionally external to this translation. */
unsafe extern "C" {
    pub fn dml1_extract_rq_regs(mode_lib: *mut display_mode_lib, rq_regs: *mut _vcs_dpi_display_rq_regs_st, rq_param: *const _vcs_dpi_display_rq_params_st);
    pub fn dml1_rq_dlg_get_rq_params(mode_lib: *mut display_mode_lib, rq_param: *mut _vcs_dpi_display_rq_params_st, pipe_src_param: *const _vcs_dpi_display_pipe_source_params_st);
    pub fn dml1_rq_dlg_get_dlg_params(mode_lib: *mut display_mode_lib, disp_dlg_regs: *mut _vcs_dpi_display_dlg_regs_st, disp_ttu_regs: *mut _vcs_dpi_display_ttu_regs_st, rq_dlg_param: *const _vcs_dpi_display_rq_dlg_params_st, dlg_sys_param: *const _vcs_dpi_display_dlg_sys_params_st, e2e_pipe_param: *const _vcs_dpi_display_e2e_pipe_params_st, cstate_en: bool, pstate_en: bool, vm_en: bool, iflip_en: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
