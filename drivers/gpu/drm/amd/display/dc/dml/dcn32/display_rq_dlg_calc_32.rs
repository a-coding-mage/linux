/*
 * Faithful low-level Rust translation of display_rq_dlg_calc_32.c.
 * C headers and externally supplied symbols are intentionally represented by
 * the surrounding translation unit.
 */

#[allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ffi::c_void;

// The implementation depends on the display-mode types and helpers declared
// by the corresponding translated headers.
extern "C" {
    fn is_dual_plane(source_format: source_format_class) -> bool;
}

#[inline]
unsafe fn is_dual_plane_local(source_format: source_format_class) -> bool {
    source_format == dm_420_12 || source_format == dm_420_8 ||
    source_format == dm_420_10 || source_format == dm_rgbe_alpha
}

pub unsafe fn dml32_rq_dlg_get_rq_reg(
    rq_regs: *mut display_rq_regs_st,
    mode_lib: *mut display_mode_lib,
    e2e_pipe_param: *const display_e2e_pipe_params_st,
    num_pipes: u32,
    pipe_idx: u32,
) {
    let src = &(*e2e_pipe_param.add(pipe_idx as usize)).pipe.src;
    let dual_plane = is_dual_plane_local(src.source_format as source_format_class);
    let mut pixel_chunk_bytes: u32 = 0;
    let mut min_pixel_chunk_bytes: u32 = 0;
    let mut meta_chunk_bytes: u32 = 0;
    let mut min_meta_chunk_bytes: u32 = 0;
    let mut dpte_group_bytes: u32 = 0;
    let mut mpte_group_bytes: u32 = 0;
    let mut p1_pixel_chunk_bytes: u32;
    let mut p1_min_pixel_chunk_bytes: u32;
    let mut p1_meta_chunk_bytes: u32;
    let mut p1_min_meta_chunk_bytes: u32;
    let mut p1_dpte_group_bytes: u32;
    let mut p1_mpte_group_bytes: u32;
    let mut detile_buf_size_in_bytes: u32;
    let mut detile_buf_plane1_addr: u32 = 0;
    let mut pte_row_height_linear: u32;

    core::ptr::write_bytes(rq_regs, 0, 1);
    pixel_chunk_bytes = (get_pixel_chunk_size_in_kbyte(mode_lib, e2e_pipe_param, num_pipes) * 1024.0) as u32;
    min_pixel_chunk_bytes = get_min_pixel_chunk_size_in_byte(mode_lib, e2e_pipe_param, num_pipes) as u32;
    if pixel_chunk_bytes == 64 * 1024 { min_pixel_chunk_bytes = 0; }
    meta_chunk_bytes = (get_meta_chunk_size_in_kbyte(mode_lib, e2e_pipe_param, num_pipes) * 1024.0) as u32;
    min_meta_chunk_bytes = get_min_meta_chunk_size_in_byte(mode_lib, e2e_pipe_param, num_pipes) as u32;
    dpte_group_bytes = get_dpte_group_size_in_bytes(mode_lib, e2e_pipe_param, num_pipes, pipe_idx) as u32;
    mpte_group_bytes = get_vm_group_size_in_bytes(mode_lib, e2e_pipe_param, num_pipes, pipe_idx) as u32;
    p1_pixel_chunk_bytes = pixel_chunk_bytes; p1_min_pixel_chunk_bytes = min_pixel_chunk_bytes;
    p1_meta_chunk_bytes = meta_chunk_bytes; p1_min_meta_chunk_bytes = min_meta_chunk_bytes;
    p1_dpte_group_bytes = dpte_group_bytes; p1_mpte_group_bytes = mpte_group_bytes;
    if src.source_format as source_format_class == dm_rgbe_alpha {
        p1_pixel_chunk_bytes = (get_alpha_pixel_chunk_size_in_kbyte(mode_lib, e2e_pipe_param, num_pipes) * 1024.0) as u32;
    }
    (*rq_regs).rq_regs_l.chunk_size = (dml_log2(pixel_chunk_bytes) - 10.0) as u32;
    (*rq_regs).rq_regs_c.chunk_size = (dml_log2(p1_pixel_chunk_bytes) - 10.0) as u32;
    (*rq_regs).rq_regs_l.min_chunk_size = if min_pixel_chunk_bytes == 0 { 0 } else { (dml_log2(min_pixel_chunk_bytes) - 8.0 + 1.0) as u32 };
    (*rq_regs).rq_regs_c.min_chunk_size = if p1_min_pixel_chunk_bytes == 0 { 0 } else { (dml_log2(p1_min_pixel_chunk_bytes) - 8.0 + 1.0) as u32 };
    (*rq_regs).rq_regs_l.meta_chunk_size = (dml_log2(meta_chunk_bytes) - 10.0) as u32;
    (*rq_regs).rq_regs_c.meta_chunk_size = (dml_log2(p1_meta_chunk_bytes) - 10.0) as u32;
    (*rq_regs).rq_regs_l.min_meta_chunk_size = if min_meta_chunk_bytes == 0 { 0 } else { (dml_log2(min_meta_chunk_bytes) - 6.0 + 1.0) as u32 };
    (*rq_regs).rq_regs_c.min_meta_chunk_size = if p1_min_meta_chunk_bytes == 0 { 0 } else { (dml_log2(p1_min_meta_chunk_bytes) - 6.0 + 1.0) as u32 };
    (*rq_regs).rq_regs_l.dpte_group_size = (dml_log2(dpte_group_bytes) - 6.0) as u32;
    (*rq_regs).rq_regs_l.mpte_group_size = (dml_log2(mpte_group_bytes) - 6.0) as u32;
    (*rq_regs).rq_regs_c.dpte_group_size = (dml_log2(p1_dpte_group_bytes) - 6.0) as u32;
    (*rq_regs).rq_regs_c.mpte_group_size = (dml_log2(p1_mpte_group_bytes) - 6.0) as u32;
    detile_buf_size_in_bytes = (get_det_buffer_size_kbytes(mode_lib, e2e_pipe_param, num_pipes, pipe_idx) * 1024.0) as u32;
    pte_row_height_linear = get_dpte_row_height_linear_l(mode_lib, e2e_pipe_param, num_pipes, pipe_idx) as u32;
    (*rq_regs).rq_regs_l.pte_row_height_linear = (dml_floor(dml_log2(pte_row_height_linear), 1.0) - 3.0) as u32;
    if dual_plane {
        let p1 = get_dpte_row_height_linear_c(mode_lib, e2e_pipe_param, num_pipes, pipe_idx) as u32;
        (*rq_regs).rq_regs_c.pte_row_height_linear = (dml_floor(dml_log2(p1), 1.0) - 3.0) as u32;
    }
    (*rq_regs).rq_regs_l.swath_height = dml_log2(get_swath_height_l(mode_lib, e2e_pipe_param, num_pipes, pipe_idx)) as u32;
    (*rq_regs).rq_regs_c.swath_height = dml_log2(get_swath_height_c(mode_lib, e2e_pipe_param, num_pipes, pipe_idx)) as u32;
    (*rq_regs).drq_expansion_mode = if pixel_chunk_bytes >= 32 * 1024 || (dual_plane && p1_pixel_chunk_bytes >= 32 * 1024) { 0 } else { 2 };
    (*rq_regs).prq_expansion_mode = 1; (*rq_regs).mrq_expansion_mode = 1; (*rq_regs).crq_expansion_mode = 1;
    if dual_plane { detile_buf_plane1_addr = detile_buf_size_in_bytes / 2 / 1024; }
    (*rq_regs).plane1_base_address = detile_buf_plane1_addr;
    print__rq_regs_st(mode_lib, rq_regs);
}

// The DLG routine is kept as an extern-compatible declaration until the
// dependent display-mode header translation supplies its register structures.
extern "C" {
    fn dml32_rq_dlg_get_dlg_reg(mode_lib: *mut display_mode_lib, dlg_regs: *mut display_dlg_regs_st, ttu_regs: *mut display_ttu_regs_st, e2e_pipe_param: *mut display_e2e_pipe_params_st, num_pipes: u32, pipe_idx: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
