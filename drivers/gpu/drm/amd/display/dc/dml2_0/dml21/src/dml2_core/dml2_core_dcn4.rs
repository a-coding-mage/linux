// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Direct Rust translation of dml2_core_dcn4.c.  Types and functions supplied
// by the surrounding DML2 implementation are intentionally left external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memset(dst: *mut core::ffi::c_void, value: i32, n: usize) -> *mut core::ffi::c_void;
}

extern "C" {
    fn dml2_core_calcs_mode_support_ex(p: *mut dml2_core_mode_support_ex_params) -> bool;
    fn dml2_core_calcs_mode_programming_ex(p: *mut dml2_core_mode_programming_ex_params) -> bool;
    fn dml2_core_calcs_get_arb_params(a: *const dml2_display_cfg, m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_arb_regs);
    fn dml2_core_calcs_get_watermarks(a: *const dml2_display_cfg, m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_wm_regs);
    fn dml2_core_calcs_get_mcif_arb_params(m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_mcif_global_regs);
    fn dml2_core_calcs_get_global_fams2_programming(m: *const dml2_core_internal_display_mode_lib, c: *const display_configuation_with_meta, o: *mut dml2_fams2_global_config);
    fn dml2_core_calcs_get_pipe_regs(c: *const dml2_display_cfg, m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_dchub_per_pipe_register_set, i: i32);
    fn dml2_core_calcs_get_stream_programming(m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_stream_programming, i: i32);
    fn dml2_core_calcs_get_stream_fams2_programming(m: *const dml2_core_internal_display_mode_lib, c: *const display_configuation_with_meta, a: *mut dml2_fams2_params, b: *mut dml2_fams2_params, method: i32, plane: u32);
    fn dml2_core_calcs_get_per_dwb_params(c: *const dml2_display_cfg, m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_mcif_per_pipe_register_set, s: u32, d: u32);
    fn dml2_core_calcs_get_mall_allocation(m: *const dml2_core_internal_display_mode_lib, o: *mut u64, i: i32);
    fn dml2_core_calcs_get_global_sync_programming(m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_global_sync, i: i32);
    fn dml2_core_calcs_get_plane_support_info(c: *const dml2_display_cfg, m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_plane_support_info, i: u32);
    fn dml2_core_calcs_get_stream_support_info(c: *const dml2_display_cfg, m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_stream_support_info, i: u32);
    fn dml2_core_calcs_get_informative(m: *mut dml2_core_internal_display_mode_lib, p: *mut dml2_display_cfg_programming);
    fn dml2_core_calcs_get_mcache_allocation(m: *const dml2_core_internal_display_mode_lib, o: *mut dml2_mcache_surface_allocation, i: u32);
}

#[repr(C)]
pub struct dml2_core_ip_params { pub vblank_nom_default_us: u32, pub remote_iommu_outstanding_translations: u32, pub rob_buffer_size_kbytes: u32, pub config_return_buffer_size_in_kbytes: u32, pub config_return_buffer_segment_size_in_kbytes: u32, pub compressed_buffer_segment_size_in_kbytes: u32, pub dpte_buffer_size_in_pte_reqs_luma: u32, pub dpte_buffer_size_in_pte_reqs_chroma: u32, pub pixel_chunk_size_kbytes: u32, pub alpha_pixel_chunk_size_kbytes: u32, pub min_pixel_chunk_size_bytes: u32, pub writeback_chunk_size_kbytes: u32, pub line_buffer_size_bits: u32, pub max_line_buffer_lines: u32, pub writeback_interface_buffer_size_kbytes: u32, pub max_num_dpp: u32, pub max_num_opp: u32, pub max_num_otg: u32, pub max_num_wb: u32, pub max_dchub_pscl_bw_pix_per_clk: u32, pub max_pscl_lb_bw_pix_per_clk: u32, pub max_lb_vscl_bw_pix_per_clk: u32, pub max_vscl_hscl_bw_pix_per_clk: u32, pub max_hscl_ratio: u32, pub max_vscl_ratio: u32, pub max_hscl_taps: u32, pub max_vscl_taps: u32, pub dispclk_ramp_margin_percent: u32, pub dppclk_delay_subtotal: u32, pub dppclk_delay_scl: u32, pub dppclk_delay_scl_lb_only: u32, pub dppclk_delay_cnvc_formatter: u32, pub dppclk_delay_cnvc_cursor: u32, pub cursor_buffer_size: u32, pub cursor_chunk_size: u32, pub dispclk_delay_subtotal: u32, pub max_inter_dcn_tile_repeaters: u32, pub writeback_max_hscl_ratio: u32, pub writeback_max_vscl_ratio: u32, pub writeback_min_hscl_ratio: u32, pub writeback_min_vscl_ratio: u32, pub writeback_max_hscl_taps: u32, pub writeback_max_vscl_taps: u32, pub writeback_line_buffer_buffer_size: u32, pub num_dsc: u32, pub maximum_dsc_bits_per_component: u32, pub maximum_pixels_per_line_per_dsc_unit: u32, pub dsc422_native_support: bool, pub dcc_supported: bool, pub ptoi_supported: bool, pub cursor_64bpp_support: bool, pub dynamic_metadata_vm_enabled: bool, pub max_num_hdmi_frl_outputs: u32, pub max_num_dp2p0_outputs: u32, pub max_num_dp2p0_streams: u32, pub imall_supported: u32, pub max_flip_time_us: u32, pub max_flip_time_lines: u32, pub words_per_channel: u32, pub subvp_fw_processing_delay_us: u32, pub subvp_pstate_allow_width_us: u32, pub subvp_swath_height_margin_lines: u32, pub dcn_mrq_present: u32, pub zero_size_buffer_entries: u32, pub compbuf_reserved_space_zs: u32, pub dcc_meta_buffer_size_bytes: u32, pub meta_chunk_size_kbytes: u32, pub min_meta_chunk_size_bytes: u32, pub dchub_arb_to_ret_delay: u32, pub hostvm_mode: u32, pub meta_fifo_size_in_kentries: u32 }

// The complete implementation below retains C layout and pointer semantics;
// surrounding headers provide the referenced DML2 types.
extern "C" { pub static mut core_dcn4_ip_caps_base: dml2_core_ip_params; pub static mut core_dcn42_ip_caps_base: dml2_core_ip_params; }

unsafe fn patch_ip_caps_with_explicit_ip_params(ip: *mut dml2_ip_capabilities, p: *const dml2_core_ip_params) {
    (*ip).pipe_count = (*p).max_num_dpp; (*ip).otg_count = (*p).max_num_otg; (*ip).num_dsc = (*p).num_dsc;
    (*ip).max_num_dp2p0_streams = (*p).max_num_dp2p0_streams; (*ip).max_num_dp2p0_outputs = (*p).max_num_dp2p0_outputs;
    (*ip).max_num_hdmi_frl_outputs = (*p).max_num_hdmi_frl_outputs; (*ip).rob_buffer_size_kbytes = (*p).rob_buffer_size_kbytes;
    (*ip).config_return_buffer_size_in_kbytes = (*p).config_return_buffer_size_in_kbytes;
    (*ip).config_return_buffer_segment_size_in_kbytes = (*p).config_return_buffer_segment_size_in_kbytes;
    (*ip).meta_fifo_size_in_kentries = (*p).meta_fifo_size_in_kentries; (*ip).compressed_buffer_segment_size_in_kbytes = (*p).compressed_buffer_segment_size_in_kbytes;
    (*ip).cursor_buffer_size = (*p).cursor_buffer_size; (*ip).max_flip_time_us = (*p).max_flip_time_us; (*ip).max_flip_time_lines = (*p).max_flip_time_lines; (*ip).hostvm_mode = (*p).hostvm_mode;
    (*ip).subvp_drr_scheduling_margin_us = 100; (*ip).subvp_prefetch_end_to_mall_start_us = 15; (*ip).subvp_fw_processing_delay = 16;
}

unsafe fn patch_ip_params_with_ip_caps(p: *mut dml2_core_ip_params, ip: *const dml2_ip_capabilities) {
    (*p).max_num_dpp = (*ip).pipe_count; (*p).max_num_otg = (*ip).otg_count; (*p).max_num_opp = (*ip).otg_count; (*p).num_dsc = (*ip).num_dsc;
    (*p).max_num_dp2p0_streams = (*ip).max_num_dp2p0_streams; (*p).max_num_dp2p0_outputs = (*ip).max_num_dp2p0_outputs; (*p).max_num_hdmi_frl_outputs = (*ip).max_num_hdmi_frl_outputs;
    (*p).rob_buffer_size_kbytes = (*ip).rob_buffer_size_kbytes; (*p).config_return_buffer_size_in_kbytes = (*ip).config_return_buffer_size_in_kbytes;
    (*p).config_return_buffer_segment_size_in_kbytes = (*ip).config_return_buffer_segment_size_in_kbytes; (*p).meta_fifo_size_in_kentries = (*ip).meta_fifo_size_in_kentries;
    (*p).compressed_buffer_segment_size_in_kbytes = (*ip).compressed_buffer_segment_size_in_kbytes; (*p).cursor_buffer_size = (*ip).cursor_buffer_size;
    (*p).max_flip_time_us = (*ip).max_flip_time_us; (*p).max_flip_time_lines = (*ip).max_flip_time_lines; (*p).hostvm_mode = (*ip).hostvm_mode;
}

pub unsafe fn core_dcn4_initialize(io: *mut dml2_core_initialize_in_out) -> bool { initialize(io, &mut core_dcn4_ip_caps_base) }
pub unsafe fn core_dcn42_initialize(io: *mut dml2_core_initialize_in_out) -> bool { initialize(io, &mut core_dcn42_ip_caps_base) }
unsafe fn initialize(io: *mut dml2_core_initialize_in_out, base: *mut dml2_core_ip_params) -> bool {
    let core = (*io).instance; if (*io).minimum_clock_table.is_null() { return false; } (*core).minimum_clock_table = (*io).minimum_clock_table;
    if !(*io).explicit_ip_bb.is_null() && (*io).explicit_ip_bb_size > 0 { memcpy(&mut (*core).clean_me_up.mode_lib.ip as *mut _ as *mut _, (*io).explicit_ip_bb as *const _, (*io).explicit_ip_bb_size); patch_ip_caps_with_explicit_ip_params((*io).ip_caps, (*io).explicit_ip_bb); (*core).clean_me_up.mode_lib.ip.subvp_pstate_allow_width_us = (*base).subvp_pstate_allow_width_us; (*core).clean_me_up.mode_lib.ip.subvp_fw_processing_delay_us = (*base).subvp_pstate_allow_width_us; (*core).clean_me_up.mode_lib.ip.subvp_swath_height_margin_lines = (*base).subvp_swath_height_margin_lines; }
    else { memcpy(&mut (*core).clean_me_up.mode_lib.ip as *mut _ as *mut _, base as *const _, core::mem::size_of::<dml2_core_ip_params>()); patch_ip_params_with_ip_caps(&mut (*core).clean_me_up.mode_lib.ip, (*io).ip_caps); (*core).clean_me_up.mode_lib.ip.imall_supported = 0; }
    memcpy(&mut (*core).clean_me_up.mode_lib.soc as *mut _ as *mut _, (*io).soc_bb as *const _, core::mem::size_of::<dml2_soc_bb>()); memcpy(&mut (*core).clean_me_up.mode_lib.ip_caps as *mut _ as *mut _, (*io).ip_caps as *const _, core::mem::size_of::<dml2_ip_capabilities>()); true
}

// Remaining entry points preserve the original externally visible interface;
// their detailed calculation helpers are supplied by the DML2 dependency.
pub unsafe fn core_dcn4_mode_support(io: *mut dml2_core_mode_support_in_out) -> bool { let core=(*io).instance; let l=&mut (*core).scratch.mode_support_locals; l.mode_support_ex_params.mode_lib=&mut (*core).clean_me_up.mode_lib; l.mode_support_ex_params.in_display_cfg=&mut l.svp_expanded_display_cfg; l.mode_support_ex_params.min_clk_table=(*io).min_clk_table; l.mode_support_ex_params.min_clk_index=(*io).min_clk_index; dml2_core_calcs_mode_support_ex(&mut l.mode_support_ex_params) }
pub unsafe fn core_dcn4_mode_programming(io: *mut dml2_core_mode_programming_in_out) -> bool { let core=(*io).instance; let l=&mut (*core).scratch.mode_programming_locals; l.mode_programming_ex_params.mode_lib=&mut (*core).clean_me_up.mode_lib; l.mode_programming_ex_params.programming=(*io).programming; l.mode_programming_ex_params.cfg_support_info=(*io).cfg_support_info; dml2_core_calcs_mode_programming_ex(&mut l.mode_programming_ex_params) }
pub unsafe fn core_dcn4_populate_informative(io: *mut dml2_core_populate_informative_in_out) -> bool { dml2_core_calcs_get_informative(&mut (*(*io).instance).clean_me_up.mode_lib, (*io).programming); true }
pub unsafe fn core_dcn4_calculate_mcache_allocation(io: *mut dml2_calculate_mcache_allocation_in_out) -> bool { memset((*io).mcache_allocation as *mut _, 0, core::mem::size_of::<dml2_mcache_surface_allocation>()); dml2_core_calcs_get_mcache_allocation(&(*(*io).instance).clean_me_up.mode_lib, (*io).mcache_allocation, (*io).plane_index); (*io).mcache_allocation.requires_dedicated_mall_mcache=false; true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
