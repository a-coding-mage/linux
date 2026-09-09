/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency supplied by display_mode_core_structs.h.

extern "C" {
    pub fn dml_core_mode_support(mode_lib: *mut display_mode_lib_st) -> dml_bool_t;
    pub fn dml_core_mode_support_partial(mode_lib: *mut display_mode_lib_st);
    pub fn dml_core_mode_programming(mode_lib: *mut display_mode_lib_st, clk_cfg: *const dml_clk_cfg_st);
    pub fn dml_core_get_row_heights(dpte_row_height: *mut dml_uint_t, meta_row_height: *mut dml_uint_t,
        mode_lib: *const display_mode_lib_st, is_plane1: dml_bool_t, source_pixel_format: dml_source_format_class,
        surface_tiling: dml_swizzle_mode, scan_direction: dml_rotation_angle, pitch: dml_uint_t,
        gpuvm_min_page_size_kbytes: dml_uint_t);
    pub fn dml_get_return_bw_mbps_vm_only(soc: *const soc_bounding_box_st, use_ideal_dram_bw_strobe: dml_bool_t,
        host_vm_enable: dml_bool_t, dcfclk: dml_float_t, fabric_clock: dml_float_t, dram_speed: dml_float_t) -> dml_float_t;
    pub fn dml_get_return_bw_mbps(soc: *const soc_bounding_box_st, use_ideal_dram_bw_strobe: dml_bool_t,
        host_vm_enable: dml_bool_t, dcfclk: dml_float_t, fabric_clock: dml_float_t, dram_speed: dml_float_t) -> dml_float_t;
    pub fn dml_mode_support(mode_lib: *mut display_mode_lib_st, state_idx: dml_uint_t,
        display_cfg: *const dml_display_cfg_st) -> dml_bool_t;
    pub fn dml_mode_programming(mode_lib: *mut display_mode_lib_st, state_idx: dml_uint_t,
        display_cfg: *const dml_display_cfg_st, call_standalone: bool) -> dml_bool_t;
    pub fn dml_mode_support_ex(in_out_params: *mut dml_mode_support_ex_params_st) -> dml_uint_t;
    pub fn dml_get_is_phantom_pipe(mode_lib: *mut display_mode_lib_st, pipe_idx: dml_uint_t) -> dml_bool_t;
}

macro_rules! dml_get_var_decl { ($name:ident, $ty:ty) => { extern "C" { pub fn $name(mode_lib: *mut display_mode_lib_st) -> $ty; } }; }
macro_rules! dml_get_per_surface_var_decl { ($name:ident, $ty:ty) => { extern "C" { pub fn $name(mode_lib: *mut display_mode_lib_st, surface_idx: dml_uint_t) -> $ty; } }; }

dml_get_var_decl!(dml_get_wm_urgent, dml_float_t);
dml_get_var_decl!(dml_get_wm_stutter_exit, dml_float_t);
dml_get_var_decl!(dml_get_wm_stutter_enter_exit, dml_float_t);
dml_get_var_decl!(dml_get_wm_memory_trip, dml_float_t);
dml_get_var_decl!(dml_get_wm_dram_clock_change, dml_float_t);
dml_get_var_decl!(dml_get_wm_z8_stutter_enter_exit, dml_float_t);
dml_get_var_decl!(dml_get_wm_z8_stutter, dml_float_t);
dml_get_var_decl!(dml_get_urgent_latency, dml_float_t);
dml_get_var_decl!(dml_get_clk_dcf_deepsleep, dml_float_t);
dml_get_var_decl!(dml_get_wm_fclk_change, dml_float_t);
dml_get_var_decl!(dml_get_wm_usr_retraining, dml_float_t);
dml_get_var_decl!(dml_get_wm_writeback_dram_clock_change, dml_float_t);
dml_get_var_decl!(dml_get_wm_writeback_urgent, dml_float_t);
dml_get_var_decl!(dml_get_stutter_efficiency_no_vblank, dml_float_t);
dml_get_var_decl!(dml_get_stutter_efficiency, dml_float_t);
dml_get_var_decl!(dml_get_stutter_efficiency_z8, dml_float_t);
dml_get_var_decl!(dml_get_stutter_num_bursts_z8, dml_float_t);
dml_get_var_decl!(dml_get_stutter_period, dml_float_t);
dml_get_var_decl!(dml_get_stutter_efficiency_z8_bestcase, dml_float_t);
dml_get_var_decl!(dml_get_stutter_num_bursts_z8_bestcase, dml_float_t);
dml_get_var_decl!(dml_get_stutter_period_bestcase, dml_float_t);
dml_get_var_decl!(dml_get_urgent_extra_latency, dml_float_t);
dml_get_var_decl!(dml_get_fclk_change_latency, dml_float_t);
dml_get_var_decl!(dml_get_nonurgent_latency, dml_float_t);
dml_get_var_decl!(dml_get_dispclk_calculated, dml_float_t);
dml_get_var_decl!(dml_get_total_data_read_bw, dml_float_t);
dml_get_var_decl!(dml_get_return_bw, dml_float_t);
dml_get_var_decl!(dml_get_return_dram_bw, dml_float_t);
dml_get_var_decl!(dml_get_tcalc, dml_float_t);
dml_get_var_decl!(dml_get_fraction_of_urgent_bandwidth, dml_float_t);
dml_get_var_decl!(dml_get_fraction_of_urgent_bandwidth_imm_flip, dml_float_t);
dml_get_var_decl!(dml_get_comp_buffer_size_kbytes, dml_uint_t);
dml_get_var_decl!(dml_get_pixel_chunk_size_in_kbyte, dml_uint_t);
dml_get_var_decl!(dml_get_alpha_pixel_chunk_size_in_kbyte, dml_uint_t);
dml_get_var_decl!(dml_get_meta_chunk_size_in_kbyte, dml_uint_t);
dml_get_var_decl!(dml_get_min_pixel_chunk_size_in_byte, dml_uint_t);
dml_get_var_decl!(dml_get_min_meta_chunk_size_in_byte, dml_uint_t);
dml_get_var_decl!(dml_get_total_immediate_flip_bytes, dml_uint_t);

dml_get_per_surface_var_decl!(dml_get_dsc_delay, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dppclk_calculated, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dscclk_calculated, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_min_ttu_vblank_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_vratio_prefetch_l, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_vratio_prefetch_c, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_x_after_scaler, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_after_scaler, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_per_vm_vblank, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_per_row_vblank, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_prefetch, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_per_vm_flip, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_per_row_flip, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_per_pte_row_nom_l, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_per_pte_row_nom_c, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_per_meta_row_nom_l, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dst_y_per_meta_row_nom_c, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_vm_group_vblank_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_vm_group_flip_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_vm_req_vblank_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_vm_req_flip_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_vm_dmdata_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_dmdata_dl_delta_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_line_delivery_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_line_delivery_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_line_delivery_pre_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_line_delivery_pre_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_req_delivery_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_req_delivery_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_req_delivery_pre_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_req_delivery_pre_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_cursor_req_delivery_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_cursor_req_delivery_pre_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_meta_chunk_nom_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_meta_chunk_nom_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_meta_chunk_vblank_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_meta_chunk_vblank_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_meta_chunk_flip_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_meta_chunk_flip_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_pte_group_nom_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_pte_group_nom_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_pte_group_vblank_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_pte_group_vblank_c_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_pte_group_flip_l_in_us, dml_float_t);
dml_get_per_surface_var_decl!(dml_get_refcyc_per_pte_group_flip_c_in_us, dml_float_t);

dml_get_per_surface_var_decl!(dml_get_dpte_group_size_in_bytes, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_vm_group_size_in_bytes, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_swath_height_l, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_swath_height_c, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dpte_row_height_l, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dpte_row_height_c, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dpte_row_height_linear_l, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dpte_row_height_linear_c, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_meta_row_height_l, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_meta_row_height_c, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_vstartup_calculated, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_vupdate_offset, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_vupdate_width, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_vready_offset, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_vready_at_or_after_vsync, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_min_dst_y_next_start, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_det_stored_buffer_size_l_bytes, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_det_stored_buffer_size_c_bytes, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_use_mall_for_static_screen, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_surface_size_for_mall, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dcc_max_uncompressed_block_l, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dcc_max_uncompressed_block_c, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dcc_max_compressed_block_l, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dcc_max_compressed_block_c, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dcc_independent_block_l, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dcc_independent_block_c, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_max_active_dram_clock_change_latency_supported, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_pte_buffer_mode, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_bigk_fragment_size, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_dpte_bytes_per_row, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_meta_bytes_per_row, dml_uint_t);
dml_get_per_surface_var_decl!(dml_get_det_buffer_size_kbytes, dml_uint_t);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
