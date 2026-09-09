// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency intent: uint32_t and other external definitions come from
// dml2_external_lib_deps.h in the C source.

// These types are u32 as they represent actual calculated register values for HW

#[repr(C)]
pub struct dml2_display_dlg_regs {
    pub refcyc_h_blank_end: u32,
    pub dlg_vblank_end: u32,
    pub min_dst_y_next_start: u32,
    pub refcyc_per_htotal: u32,
    pub refcyc_x_after_scaler: u32,
    pub dst_y_after_scaler: u32,
    pub dst_y_prefetch: u32,
    pub dst_y_per_vm_vblank: u32,
    pub dst_y_per_row_vblank: u32,
    pub dst_y_per_vm_flip: u32,
    pub dst_y_per_row_flip: u32,
    pub ref_freq_to_pix_freq: u32,
    pub vratio_prefetch: u32,
    pub vratio_prefetch_c: u32,
    pub refcyc_per_tdlut_group: u32,
    pub refcyc_per_pte_group_vblank_l: u32,
    pub refcyc_per_pte_group_vblank_c: u32,
    pub refcyc_per_pte_group_flip_l: u32,
    pub refcyc_per_pte_group_flip_c: u32,
    pub dst_y_per_pte_row_nom_l: u32,
    pub dst_y_per_pte_row_nom_c: u32,
    pub refcyc_per_pte_group_nom_l: u32,
    pub refcyc_per_pte_group_nom_c: u32,
    pub refcyc_per_line_delivery_pre_l: u32,
    pub refcyc_per_line_delivery_pre_c: u32,
    pub refcyc_per_line_delivery_l: u32,
    pub refcyc_per_line_delivery_c: u32,
    pub refcyc_per_vm_group_vblank: u32,
    pub refcyc_per_vm_group_flip: u32,
    pub refcyc_per_vm_req_vblank: u32,
    pub refcyc_per_vm_req_flip: u32,
    pub dst_y_offset_cur0: u32,
    pub chunk_hdl_adjust_cur0: u32,
    pub vready_after_vcount0: u32,
    pub dst_y_delta_drq_limit: u32,
    pub refcyc_per_vm_dmdata: u32,
    pub dmdata_dl_delta: u32,
    pub dst_y_svp_drq_limit: u32,
    pub force_prefetch_to_vblank: u32,
    pub force_cursor_to_disp_pref: u32,
    // MRQ
    pub refcyc_per_meta_chunk_vblank_l: u32,
    pub refcyc_per_meta_chunk_vblank_c: u32,
    pub refcyc_per_meta_chunk_flip_l: u32,
    pub refcyc_per_meta_chunk_flip_c: u32,
    pub dst_y_per_meta_row_nom_l: u32,
    pub dst_y_per_meta_row_nom_c: u32,
    pub refcyc_per_meta_chunk_nom_l: u32,
    pub refcyc_per_meta_chunk_nom_c: u32,
}

#[repr(C)]
pub struct dml2_display_ttu_regs {
    pub qos_level_low_wm: u32, pub qos_level_high_wm: u32, pub min_ttu_vblank: u32,
    pub qos_level_flip: u32, pub refcyc_per_req_delivery_l: u32,
    pub refcyc_per_req_delivery_c: u32, pub refcyc_per_req_delivery_cur0: u32,
    pub refcyc_per_req_delivery_pre_l: u32, pub refcyc_per_req_delivery_pre_c: u32,
    pub refcyc_per_req_delivery_pre_cur0: u32, pub qos_level_fixed_l: u32,
    pub qos_level_fixed_c: u32, pub qos_level_fixed_cur0: u32,
    pub qos_ramp_disable_l: u32, pub qos_ramp_disable_c: u32,
    pub qos_ramp_disable_cur0: u32,
}

#[repr(C)]
pub struct dml2_display_arb_regs {
    pub max_req_outstanding: u32, pub min_req_outstanding: u32, pub sat_level_us: u32,
    pub hvm_max_qos_commit_threshold: u32, pub hvm_min_req_outstand_commit_threshold: u32,
    pub compbuf_reserved_space_kbytes: u32, pub compbuf_size: u32,
    pub sdpif_request_rate_limit: u32, pub allow_sdpif_rate_limit_when_cstate_req: u32,
    pub dcfclk_deep_sleep_hysteresis: u32, pub pstate_stall_threshold: u32,
}

#[repr(C)]
pub struct dml2_cursor_dlg_regs {
    pub dst_x_offset: u32, // CURSOR0_DST_X_OFFSET
    pub dst_y_offset: u32, // CURSOR0_DST_Y_OFFSET
    pub chunk_hdl_adjust: u32, // CURSOR0_CHUNK_HDL_ADJUST
    pub qos_level_fixed: u32,
    pub qos_ramp_disable: u32,
}

#[repr(C)]
pub struct dml2_display_plane_rq_regs {
    pub chunk_size: u32, pub min_chunk_size: u32, pub dpte_group_size: u32,
    pub mpte_group_size: u32, pub swath_height: u32, pub pte_row_height_linear: u32,
    // MRQ
    pub meta_chunk_size: u32, pub min_meta_chunk_size: u32,
}

#[repr(C)]
pub struct dml2_display_rq_regs {
    pub rq_regs_l: dml2_display_plane_rq_regs,
    pub rq_regs_c: dml2_display_plane_rq_regs,
    pub drq_expansion_mode: u32, pub prq_expansion_mode: u32, pub crq_expansion_mode: u32,
    pub plane1_base_address: u32, pub unbounded_request_enabled: u32,
    pub pte_buffer_mode: bool, pub force_one_row_for_frame: bool,
    // MRQ
    pub mrq_expansion_mode: u32,
}

#[repr(C)]
pub struct dml2_display_mcache_regs { pub mcache_id_first: u32, pub mcache_id_second: u32, pub split_location: u32 }

#[repr(C)]
pub struct dml2_hubp_pipe_mcache_regs {
    pub main: dml2_hubp_pipe_mcache_regs_group,
    pub mall: dml2_hubp_pipe_mcache_regs_group,
}

#[repr(C)]
pub struct dml2_hubp_pipe_mcache_regs_group {
    pub p0: dml2_display_mcache_regs,
    pub p1: dml2_display_mcache_regs,
}

#[repr(C)]
pub struct dml2_dchub_per_pipe_register_set {
    pub rq_regs: dml2_display_rq_regs, pub ttu_regs: dml2_display_ttu_regs,
    pub dlg_regs: dml2_display_dlg_regs, pub det_size: u32,
}

#[repr(C)]
pub struct dml2_mcif_per_pipe_register_set {
    pub time_per_pixel: ::core::ffi::c_uint, // U6.6 format
    pub arbitration_slice: ::core::ffi::c_uint,
    pub slice_lines: ::core::ffi::c_uint,
    pub max_scaled_time_ns: ::core::ffi::c_uint,
}

#[repr(C)]
pub union dml2_dchub_watermark_temp_read_or_ppt { pub temp_read_or_ppt: u32, pub temp_read: u32 }
#[repr(C)]
pub union dml2_dchub_watermark_usr_or_buffer_fullness { pub usr: u32, pub buffer_fullness: u32 }

#[repr(C)]
pub struct dml2_dchub_watermark_regs {
    /* watermarks */
    pub urgent: u32, pub sr_enter: u32, pub sr_exit: u32, pub sr_enter_z8: u32,
    pub sr_exit_z8: u32, pub sr_enter_low_power: u32, pub sr_exit_low_power: u32,
    pub uclk_pstate: u32, pub fclk_pstate: u32,
    pub temp_read_or_ppt: dml2_dchub_watermark_temp_read_or_ppt,
    pub ppt: u32, pub usr: dml2_dchub_watermark_usr_or_buffer_fullness,
    /* qos */
    pub refcyc_per_trip_to_mem: u32, pub refcyc_per_meta_trip_to_mem: u32,
    pub frac_urg_bw_flip: u32, pub frac_urg_bw_nom: u32, pub frac_urg_bw_mall: u32,
}

#[repr(C)]
pub enum dml2_dchub_watermark_reg_set_index {
    DML2_DCHUB_WATERMARK_SET_A = 0,
    DML2_DCHUB_WATERMARK_SET_B = 1,
    DML2_DCHUB_WATERMARK_SET_C = 2,
    DML2_DCHUB_WATERMARK_SET_D = 3,
    DML2_DCHUB_WATERMARK_SET_NUM = 4,
}

#[repr(C)]
pub struct dml2_dchub_global_register_set {
    pub arb_regs: dml2_display_arb_regs,
    pub wm_regs: [dml2_dchub_watermark_regs; DML2_DCHUB_WATERMARK_SET_NUM as usize],
    pub num_watermark_sets: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dml2_mcif_watermark_regs {
    /* watermarks */
    pub urgent: u32, // (CLI)
    pub uclk_pstate: u32, pub fclk_pstate: u32, pub temp_read_or_ppt: u32,
}

#[repr(C)]
pub struct dml2_mcif_global_register_set {
    pub wm_regs: [dml2_mcif_watermark_regs; DML2_DCHUB_WATERMARK_SET_NUM as usize],
    pub num_watermark_sets: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
