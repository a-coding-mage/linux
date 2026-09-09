// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency: dml2_core_shared_types.h

#[repr(C)]
pub struct dml2_dchub_watermark_regs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_display_arb_regs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_per_stream_programming {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_dchub_per_pipe_register_set {
    _private: [u8; 0],
}
#[repr(C)]
pub struct core_plane_support_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct core_stream_support_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dml2_cursor_dlg_regs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct display_configuation_with_meta {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dml2_core_calcs_mode_support_ex(
        in_out_params: *mut dml2_core_calcs_mode_support_ex,
    ) -> u32;
    pub fn dml2_core_calcs_mode_programming_ex(
        in_out_params: *mut dml2_core_calcs_mode_programming_ex,
    ) -> bool;
    pub fn dml2_core_calcs_get_watermarks(
        display_cfg: *const dml2_display_cfg,
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_dchub_watermark_regs,
    );
    pub fn dml2_core_calcs_get_mcif_arb_params(
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_mcif_global_register_set,
    );
    pub fn dml2_core_calcs_get_arb_params(
        display_cfg: *const dml2_display_cfg,
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_display_arb_regs,
    );
    pub fn dml2_core_calcs_get_pipe_regs(
        dml2_display_cfg: *const dml2_display_cfg,
        mode_lib: *mut dml2_core_internal_display_mode_lib,
        out: *mut dml2_dchub_per_pipe_register_set,
        pipe_index: i32,
    );
    pub fn dml2_core_calcs_get_stream_programming(
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_per_stream_programming,
        pipe_index: i32,
    );
    pub fn dml2_core_calcs_get_global_sync_programming(
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_global_sync_programming,
        pipe_index: i32,
    );
    pub fn dml2_core_calcs_get_mcache_allocation(
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_mcache_surface_allocation,
        plane_index: i32,
    );
    pub fn dml2_core_calcs_get_plane_support_info(
        display_cfg: *const dml2_display_cfg,
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut core_plane_support_info,
        plane_index: i32,
    );
    pub fn dml2_core_calcs_get_informative(
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_display_cfg_programming,
    );
    pub fn dml2_core_calcs_get_stream_support_info(
        display_cfg: *const dml2_display_cfg,
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut core_stream_support_info,
        plane_index: i32,
    );
    pub fn dml2_core_calcs_get_mall_allocation(
        mode_lib: *mut dml2_core_internal_display_mode_lib,
        out: *mut u32,
        pipe_index: i32,
    );
    pub fn dml2_core_calcs_get_stream_fams2_programming(
        mode_lib: *const dml2_core_internal_display_mode_lib,
        display_cfg: *const display_configuation_with_meta,
        fams2_base_programming: *mut dmub_cmd_fams2_config,
        fams2_sub_programming: *mut dmub_cmd_fams2_config,
        pstate_method: dml2_pstate_method,
        plane_index: i32,
    );
    pub fn dml2_core_calcs_get_global_fams2_programming(
        mode_lib: *const dml2_core_internal_display_mode_lib,
        display_cfg: *const display_configuation_with_meta,
        fams2_global_config: *mut dmub_cmd_fams2_global_config,
    );
    pub fn dml2_core_calcs_get_per_dwb_params(
        display_cfg: *const dml2_display_cfg,
        mode_lib: *const dml2_core_internal_display_mode_lib,
        out: *mut dml2_mcif_per_pipe_register_set,
        stream_index: i32,
        dwb_index: i32,
    );
    pub fn dml2_core_calcs_get_dpte_row_height(
        dpte_row_height: *mut u32,
        mode_lib: *mut dml2_core_internal_display_mode_lib,
        is_plane1: bool,
        source_pixel_format: dml2_source_format_class,
        surface_tiling: dml2_swizzle_mode,
        scan_direction: dml2_rotation_angle,
        pitch: u32,
        gpuvm_min_page_size_kbytes: u32,
    );
    pub fn dml2_core_calcs_cursor_dlg_reg(
        cursor_dlg_regs: *mut dml2_cursor_dlg_regs,
        p: *const dml2_get_cursor_dlg_reg,
    );
    pub fn dml2_core_internal_bw_type_str(
        bw_type: dml2_core_internal_bw_type,
    ) -> *const ::core::ffi::c_char;
    pub fn dml2_core_internal_soc_state_type_str(
        dml2_core_internal_soc_state_type: dml2_core_internal_soc_state_type,
    ) -> *const ::core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
