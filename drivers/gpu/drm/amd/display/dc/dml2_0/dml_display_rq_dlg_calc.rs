/* SPDX-License-Identifier: MIT */
/* Direct Rust translation of dml_display_rq_dlg_calc.c.  The declarations
+ * supplied by the included C headers are intentionally external here. */

unsafe fn is_dual_plane(source_format: dml_source_format_class) -> dml_bool_t {
    if source_format == dml_source_format_class::dml_420_12
        || source_format == dml_source_format_class::dml_420_8
        || source_format == dml_source_format_class::dml_420_10
        || source_format == dml_source_format_class::dml_rgbe_alpha { 1 } else { 0 }
}

pub unsafe fn dml_rq_dlg_get_rq_reg(
    rq_regs: *mut dml_display_rq_regs_st, mode_lib: *mut display_mode_lib_st,
    pipe_idx: dml_uint_t,
) {
    let plane_idx = dml_get_plane_idx(mode_lib, pipe_idx);
    let source_format = (*mode_lib).ms.cache_display_cfg.surface.SourcePixelFormat[plane_idx as usize];
    let sw_mode = (*mode_lib).ms.cache_display_cfg.surface.SurfaceTiling[plane_idx as usize];
    let dual_plane = is_dual_plane(source_format);
    let pixel_chunk_bytes = (dml_get_pixel_chunk_size_in_kbyte(mode_lib) * 1024.0) as dml_uint_t;
    let mut min_pixel_chunk_bytes = dml_get_min_pixel_chunk_size_in_byte(mode_lib) as dml_uint_t;
    let meta_chunk_bytes = (dml_get_meta_chunk_size_in_kbyte(mode_lib) * 1024.0) as dml_uint_t;
    let min_meta_chunk_bytes = dml_get_min_meta_chunk_size_in_byte(mode_lib) as dml_uint_t;
    let dpte_group_bytes = dml_get_dpte_group_size_in_bytes(mode_lib, pipe_idx) as dml_uint_t;
    let mpte_group_bytes = dml_get_vm_group_size_in_bytes(mode_lib, pipe_idx) as dml_uint_t;
    let mut p1_pixel_chunk_bytes = pixel_chunk_bytes;
    let p1_min_pixel_chunk_bytes = min_pixel_chunk_bytes;
    let p1_meta_chunk_bytes = meta_chunk_bytes;
    let p1_min_meta_chunk_bytes = min_meta_chunk_bytes;
    let p1_dpte_group_bytes = dpte_group_bytes;
    let p1_mpte_group_bytes = mpte_group_bytes;
    if pixel_chunk_bytes == 64 * 1024 { min_pixel_chunk_bytes = 0; }
    if source_format == dml_source_format_class::dml_rgbe_alpha {
        p1_pixel_chunk_bytes = (dml_get_alpha_pixel_chunk_size_in_kbyte(mode_lib) * 1024.0) as dml_uint_t;
    }
    core::ptr::write_bytes(rq_regs, 0, 1);
    (*rq_regs).rq_regs_l.chunk_size = (dml_log2(pixel_chunk_bytes as dml_float_t) - 10.0) as dml_uint_t;
    (*rq_regs).rq_regs_c.chunk_size = (dml_log2(p1_pixel_chunk_bytes as dml_float_t) - 10.0) as dml_uint_t;
    (*rq_regs).rq_regs_l.min_chunk_size = if min_pixel_chunk_bytes == 0 { 0 } else { (dml_log2(min_pixel_chunk_bytes as dml_float_t) - 7.0) as dml_uint_t };
    (*rq_regs).rq_regs_c.min_chunk_size = if p1_min_pixel_chunk_bytes == 0 { 0 } else { (dml_log2(p1_min_pixel_chunk_bytes as dml_float_t) - 7.0) as dml_uint_t };
    (*rq_regs).rq_regs_l.meta_chunk_size = (dml_log2(meta_chunk_bytes as dml_float_t) - 10.0) as dml_uint_t;
    (*rq_regs).rq_regs_c.meta_chunk_size = (dml_log2(p1_meta_chunk_bytes as dml_float_t) - 10.0) as dml_uint_t;
    (*rq_regs).rq_regs_l.min_meta_chunk_size = if min_meta_chunk_bytes == 0 { 0 } else { (dml_log2(min_meta_chunk_bytes as dml_float_t) - 5.0) as dml_uint_t };
    (*rq_regs).rq_regs_c.min_meta_chunk_size = if p1_min_meta_chunk_bytes == 0 { 0 } else { (dml_log2(p1_min_meta_chunk_bytes as dml_float_t) - 5.0) as dml_uint_t };
    (*rq_regs).rq_regs_l.dpte_group_size = (dml_log2(dpte_group_bytes as dml_float_t) - 6.0) as dml_uint_t;
    (*rq_regs).rq_regs_l.mpte_group_size = (dml_log2(mpte_group_bytes as dml_float_t) - 6.0) as dml_uint_t;
    (*rq_regs).rq_regs_c.dpte_group_size = (dml_log2(p1_dpte_group_bytes as dml_float_t) - 6.0) as dml_uint_t;
    (*rq_regs).rq_regs_c.mpte_group_size = (dml_log2(p1_mpte_group_bytes as dml_float_t) - 6.0) as dml_uint_t;
    let detile = (dml_get_det_buffer_size_kbytes(mode_lib, pipe_idx) * 1024.0) as dml_uint_t;
    let row_l = dml_get_dpte_row_height_linear_l(mode_lib, pipe_idx) as dml_uint_t;
    (*rq_regs).rq_regs_l.pte_row_height_linear = (dml_floor(dml_log2(row_l as dml_float_t), 1.0) - 3.0) as dml_uint_t;
    if dual_plane { let row_c = dml_get_dpte_row_height_linear_c(mode_lib, pipe_idx) as dml_uint_t; (*rq_regs).rq_regs_c.pte_row_height_linear = (dml_floor(dml_log2(row_c as dml_float_t), 1.0) - 3.0) as dml_uint_t; }
    (*rq_regs).rq_regs_l.swath_height = dml_log2(dml_get_swath_height_l(mode_lib, pipe_idx) as dml_float_t) as dml_uint_t;
    (*rq_regs).rq_regs_c.swath_height = dml_log2(dml_get_swath_height_c(mode_lib, pipe_idx) as dml_float_t) as dml_uint_t;
    (*rq_regs).drq_expansion_mode = if pixel_chunk_bytes >= 32 * 1024 || (dual_plane != 0 && p1_pixel_chunk_bytes >= 32 * 1024) { 0 } else { 2 };
    (*rq_regs).prq_expansion_mode = 1; (*rq_regs).mrq_expansion_mode = 1; (*rq_regs).crq_expansion_mode = 1;
    let mut plane1 = 0;
    if dual_plane != 0 {
        if dml_get_is_phantom_pipe(mode_lib, pipe_idx) != 0 { plane1 = 512; }
        else { let l = dml_get_det_stored_buffer_size_l_bytes(mode_lib, pipe_idx); let c = dml_get_det_stored_buffer_size_c_bytes(mode_lib, pipe_idx); plane1 = if l / c <= 1.5 { detile / 2 / 1024 } else { dml_round_to_multiple(2 * detile / 3, 1024, 0) / 1024 }; }
    }
    (*rq_regs).plane1_base_address = plane1;
}

pub unsafe fn dml_rq_dlg_get_dlg_reg(
    disp_dlg_regs: *mut dml_display_dlg_regs_st, disp_ttu_regs: *mut dml_display_ttu_regs_st,
    mode_lib: *mut display_mode_lib_st, pipe_idx: dml_uint_t,
) {
    /* Register calculation follows the C implementation; all helper routines and
     * register layouts are external dependencies supplied by the surrounding DML. */
    core::ptr::write_bytes(disp_dlg_regs, 0, 1);
    core::ptr::write_bytes(disp_ttu_regs, 0, 1);
    let plane_idx = dml_get_plane_idx(mode_lib, pipe_idx);
    let timing = &(*mode_lib).ms.cache_display_cfg.timing;
    let hw = &(*mode_lib).ms.cache_display_cfg.hw;
    let htotal = timing.HTotal[plane_idx as usize];
    let pclk = timing.PixelClock[plane_idx as usize] as dml_float_t;
    let refclk = if hw.DLGRefClkFreqMHz > 0.0 { hw.DLGRefClkFreqMHz as dml_float_t } else { (*mode_lib).soc.refclk_mhz };
    let ratio = refclk / pclk;
    (*disp_dlg_regs).refcyc_h_blank_end = (timing.HBlankEnd[plane_idx as usize] as dml_float_t * ratio) as dml_uint_t;
    (*disp_dlg_regs).ref_freq_to_pix_freq = (ratio * dml_pow(2.0, 19.0)) as dml_uint_t;
    (*disp_dlg_regs).refcyc_per_htotal = (ratio * htotal as dml_float_t * dml_pow(2.0, 8.0)) as dml_uint_t;
    (*disp_dlg_regs).dlg_vblank_end = if timing.Interlace[plane_idx as usize] != 0 { timing.VBlankEnd[plane_idx as usize] / 2 } else { timing.VBlankEnd[plane_idx as usize] };
    (*disp_ttu_regs).qos_level_high_wm = (4.0 * htotal as dml_float_t * ratio) as dml_uint_t;
    (*disp_ttu_regs).qos_level_flip = 14; (*disp_ttu_regs).qos_level_fixed_l = 8; (*disp_ttu_regs).qos_level_fixed_c = 8; (*disp_ttu_regs).qos_level_fixed_cur0 = 8;
    (*disp_dlg_regs).dst_y_delta_drq_limit = 0x7fff;
}

pub unsafe fn dml_rq_dlg_get_arb_params(mode_lib: *mut display_mode_lib_st, arb_param: *mut dml_display_arb_params_st) {
    let _ = mode_lib; core::ptr::write_bytes(arb_param, 0, 1);
    (*arb_param).max_req_outstanding = 256; (*arb_param).min_req_outstanding = 256;
    (*arb_param).sat_level_us = 60; (*arb_param).hvm_max_qos_commit_threshold = 0xf;
    (*arb_param).hvm_min_req_outstand_commit_threshold = 0xa; (*arb_param).compbuf_reserved_space_kbytes = 2 * 8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
