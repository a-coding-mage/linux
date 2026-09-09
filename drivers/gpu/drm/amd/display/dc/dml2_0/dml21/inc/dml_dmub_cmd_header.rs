// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dml_dmub_cmd.h. Linux byte-order/types/string dependencies
// are supplied by the surrounding translation unit.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub mod dml_dmub_cmd {
    macro_rules! DMUB_MAX { ($x:expr, $y:expr) => { if ($x) > ($y) { $x } else { $y } } }
    macro_rules! DMUB_MIN { ($x:expr, $y:expr) => { if ($x) < ($y) { $x } else { $y } } }

    pub const DMUB_MAX_STREAMS: usize = 6;
    pub const DMUB_MAX_PLANES: usize = 6;
    pub const DMUB_MAX_PHANTOM_PLANES: usize = DMUB_MAX_PLANES / 2;

    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_soc_bb_params {
        pub dram_clk_change_blackout_ns:u32, pub dram_clk_change_read_only_ns:u32,
        pub dram_clk_change_write_only_ns:u32, pub fclk_change_blackout_ns:u32,
        pub g7_ppt_blackout_ns:u32, pub stutter_enter_plus_exit_latency_ns:u32,
        pub stutter_exit_latency_ns:u32, pub z8_stutter_enter_plus_exit_latency_ns:u32,
        pub z8_stutter_exit_latency_ns:u32, pub z8_min_idle_time_ns:u32,
        pub type_b_dram_clk_change_blackout_ns:u32, pub type_b_ppt_blackout_ns:u32,
        pub vmin_limit_dispclk_khz:u32, pub vmin_limit_dcfclk_khz:u32,
        pub g7_temperature_read_blackout_ns:u32,
    }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_rect16 { pub x:u16, pub y:u16, pub width:u16, pub height:u16 }
    #[repr(C)] #[derive(Copy, Clone)] pub union fw_assisted_mclk_switch_version { pub ver:u8, pub bits: fw_assisted_mclk_switch_version_bits }
    #[repr(C)] #[derive(Copy, Clone)] pub struct fw_assisted_mclk_switch_version_bits { pub minor:u8, pub major:u8 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_optc_position { pub vpos:u32, pub hpos:u32, pub frame:u32 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct plane_pipe_rect { pub luma:dmub_rect16, pub chroma:dmub_rect16 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct lsdma_outputs {
        pub src_x:[[u16;4];2], pub src_y:[[u16;4];2], pub dst_x:[[u16;4];2], pub dst_y:[[u16;4];2],
        pub width:[[u16;4];2], pub height:[[u16;4];2], pub dst_pitch:[u16;4]
    }
    #[repr(u32)] #[derive(Copy, Clone)] pub enum fams2_stream_type { FAMS2_STREAM_TYPE_NONE=0, FAMS2_STREAM_TYPE_VBLANK=1, FAMS2_STREAM_TYPE_VACTIVE=2, FAMS2_STREAM_TYPE_DRR=3, FAMS2_STREAM_TYPE_SUBVP=4, FAMS2_STREAM_TYPE_ALTERNATE=5 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_alternate_stream_dynamic_state {
        pub earliest_init_tick:u64, pub otg_frame_pending_clear:[u32;3], pub flip_pending_clear_order:[u8;3], pub num_pending_flips:u8,
        pub prefetch_start_line_x1000:[u32;3], pub prefetch_end_line:[u16;3], pub recout_y:[u16;3], pub flip_pending:[u8;3], pub copy_from_earliest:[u8;3],
        pub lsdma_bandwidth_mbps:u16, pub vstartup_line:u16, pub vready_line:u16, pub cursor_size:[u8;3], pub pad:u8,
        pub subvp_start_line_a:[u16;3], pub subvp_height_a:[u16;3], pub subvp_next_start_line_a:[u16;3], pub subvp_next_height_a:[u16;3],
        pub subvp_start_line_b:[u16;3], pub subvp_height_b:[u16;3], pub subvp_next_start_line_b:[u16;3], pub subvp_next_height_b:[u16;3],
        pub subvp_c_start_line_a:[u16;3], pub subvp_c_height_a:[u16;3], pub subvp_c_next_start_line_a:[u16;3], pub subvp_c_next_height_a:[u16;3],
        pub subvp_c_start_line_b:[u16;3], pub subvp_c_height_b:[u16;3], pub subvp_c_next_start_line_b:[u16;3], pub subvp_c_next_height_b:[u16;3],
        pub subvp_position:[u8;3], pub copy_from_primary:[u8;3], pub pad1:[u8;2], pub program_go_line:u32, pub program_go_frame_count:u32,
        pub svp0_start_dst_line:u16, pub svp0_end_dst_line:u16, pub svp1_start_dst_line:u16, pub svp1_end_dst_line:u16, pub lsdma:[lsdma_outputs;2], pub lsdma_c:[lsdma_outputs;2]
    }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_legacy_stream_dynamic_state { pub force_allow_at_vblank:u8, pub pad:[u8;3] }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_subvp_stream_dynamic_state { pub viewport_start_hubp_vline:u16,pub viewport_height_hubp_vlines:u16,pub viewport_start_c_hubp_vline:u16,pub viewport_height_c_hubp_vlines:u16,pub phantom_viewport_height_hubp_vlines:u16,pub phantom_viewport_height_c_hubp_vlines:u16,pub microschedule_start_otg_vline:u16,pub mall_start_otg_vline:u16,pub mall_start_hubp_vline:u16,pub mall_start_c_hubp_vline:u16,pub force_allow_at_vblank_only:u8,pub swath_height:u8,pub swath_height_c:u8,pub pad:u8 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_drr_stream_dynamic_state { pub stretched_vtotal:u16,pub use_cur_vtotal:u8,pub pad:u8 }
    #[repr(C)] pub union dmub_fams2_stream_dynamic_sub_state { pub legacy:dmub_fams2_legacy_stream_dynamic_state, pub subvp:dmub_fams2_subvp_stream_dynamic_state, pub drr:dmub_fams2_drr_stream_dynamic_state, pub alternate:dmub_fams2_alternate_stream_dynamic_state }
    #[repr(C)] pub struct dmub_fams2_stream_dynamic_state { pub ref_tick:u64,pub cur_vtotal:u32,pub adjusted_allow_end_otg_vline:u16,pub pad:[u8;2],pub ref_otg_pos:dmub_optc_position,pub target_otg_pos:dmub_optc_position,pub sub_state:dmub_fams2_stream_dynamic_sub_state }

    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_legacy_stream_static_state { pub vactive_det_fill_delay_otg_vlines:u8,pub programming_delay_otg_vlines:u8 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_subvp_stream_static_state { pub vratio_numerator:u16,pub vratio_denominator:u16,pub phantom_vtotal:u16,pub phantom_vactive:u16,pub config:u8,pub programming_delay_otg_vlines:u8,pub prefetch_to_mall_otg_vlines:u8,pub phantom_otg_inst:u8,pub phantom_pipe_mask:u8,pub phantom_plane_pipe_masks:[u8;DMUB_MAX_PHANTOM_PLANES] }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_drr_stream_static_state { pub nom_stretched_vtotal:u16,pub programming_delay_otg_vlines:u8,pub only_stretch_if_required:u8,pub pad:[u8;2] }
    #[repr(C)] pub union dmub_fams2_stream_static_sub_state { pub legacy:dmub_fams2_legacy_stream_static_state,pub subvp:dmub_fams2_subvp_stream_static_state,pub drr:dmub_fams2_drr_stream_static_state }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_cmd_legacy_stream_static_state { pub vactive_det_fill_delay_otg_vlines:u16,pub programming_delay_otg_vlines:u16,pub disallow_time_us:u32 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_cmd_subvp_stream_static_state { pub vratio_numerator:u16,pub vratio_denominator:u16,pub phantom_vtotal:u16,pub phantom_vactive:u16,pub programming_delay_otg_vlines:u16,pub prefetch_to_mall_otg_vlines:u16,pub config:u8,pub phantom_otg_inst:u8,pub phantom_pipe_mask:u8,pub pad0:u8,pub phantom_plane_pipe_masks:[u8;DMUB_MAX_PHANTOM_PLANES],pub pad1:[u8;4-(DMUB_MAX_PHANTOM_PLANES%4)] }
    #[repr(C)] #[derive(Copy, Clone)] pub struct dmub_fams2_cmd_drr_stream_static_state { pub nom_stretched_vtotal:u16,pub programming_delay_otg_vlines:u16,pub only_stretch_if_required:u8,pub pad:[u8;3] }
    #[repr(C)] pub union dmub_fams2_cmd_stream_static_sub_state { pub legacy:dmub_fams2_cmd_legacy_stream_static_state,pub subvp:dmub_fams2_cmd_subvp_stream_static_state,pub drr:dmub_fams2_cmd_drr_stream_static_state }
    #[repr(C)] pub union dmub_fams2_stream_static_sub_state_v2 { pub legacy:dmub_fams2_cmd_legacy_stream_static_state,pub subvp:dmub_fams2_cmd_subvp_stream_static_state,pub drr:dmub_fams2_cmd_drr_stream_static_state,pub alternate:dmub_fams2_cmd_alternate_stream_static_state }
    #[repr(C)] #[derive(Copy,Clone)] pub struct dmub_fams2_cmd_alternate_stream_static_state { pub total_bytes_to_copy:u32,pub svp0_dst_lines:u16,pub svp1_dst_lines:u16,pub min_lead_dst_lines:u16,pub svp_req_limit:u16,pub fw_delays:u16,pub vstartup_start:u16,pub rec_height:[u16;3],pub viewport_start:[u16;3],pub viewport_size:[u16;3],pub viewport_start_c:[u16;3],pub viewport_size_c:[u16;3],pub surface_pitch:[u16;3],pub surface_pitch_c:[u16;3],pub surface_height:[u16;3],pub surface_height_c:[u16;3],pub element_size:[u8;3],pub element_size_c:[u8;3],pub swizzle_mode:[u8;3],pub vready_offset_lines:u8,pub dst_y_prefetch_x1000:[u16;3],pub total_swaths:[u16;3],pub total_swaths_c:[u16;3],pub prefetch_swaths:[u8;3],pub prefetch_swaths_c:[u8;3],pub swath_height:[u8;3],pub swath_height_c:[u8;3],pub block_256b_width:[u16;3],pub block_256b_height:[u16;3],pub block_256b_width_c:[u16;3],pub block_256b_height_c:[u16;3],pub macro_tile_width:[u16;3],pub macro_tile_width_c:[u16;3],pub config:[u8;3],pub max_cursor_size:u8,pub pre_hdl_delta_x1000:[u16;3],pub pre_hdl_delta_c_x1000:[u16;3],pub rec_hdl_delta_x1000:[u16;3],pub rec_hdl_delta_c_x1000:[u16;3],pub dst_y_per_vm_vblank_x1000:[u16;3],pub dst_y_per_row_vblank_x1000:[u16;3],pub dst_y_after_scaler:[u16;3],pub vinit_prefill:[u16;3],pub vinit_prefill_c:[u16;3],pub vratio_x1000:[u16;3],pub vratio_c_x1000:[u16;3],pub pipe_viewports:[plane_pipe_rect;4],pub pipe_copy_offset:[[u32;4];2],pub pipe_copy_offset_c:[[u32;4];2],pub pipe_copy_addr_47_16:[[u32;4];2],pub pipe_copy_addr_47_16_c:[[u32;4];2],pub pipe_copy_max_size:[[u32;4];2],pub pipe_copy_max_size_c:[[u32;4];2] }
    #[repr(C)] pub struct dmub_fams2_stream_static_state { pub type_:fams2_stream_type,pub otg_vline_time_ns:u32,pub otg_vline_time_ticks:u32,pub htotal:u16,pub vtotal:u16,pub vblank_start:u16,pub vblank_end:u16,pub max_vtotal:u16,pub allow_start_otg_vline:u16,pub allow_end_otg_vline:u16,pub drr_keepout_otg_vline:u16,pub scheduling_delay_otg_vlines:u8,pub contention_delay_otg_vlines:u8,pub vline_int_ack_delay_otg_vlines:u8,pub allow_to_target_delay_otg_vlines:u8,pub config:u8,pub otg_inst:u8,pub pipe_mask:u8,pub num_planes:u8,pub plane_pipe_masks:[u8;DMUB_MAX_PLANES],pub pad:[u8;DMUB_MAX_PLANES%4],pub sub_state:dmub_fams2_stream_static_sub_state }
    #[repr(C)] #[derive(Copy,Clone)] pub struct dmub_fams2_cmd_stream_static_base_state { pub type_:fams2_stream_type,pub otg_vline_time_ns:u32,pub otg_vline_time_ticks:u32,pub htotal:u16,pub vtotal:u16,pub vblank_start:u16,pub vblank_end:u16,pub max_vtotal:u16,pub allow_start_otg_vline:u16,pub allow_end_otg_vline:u16,pub drr_keepout_otg_vline:u16,pub scheduling_delay_otg_vlines:u16,pub contention_delay_otg_vlines:u16,pub vline_int_ack_delay_otg_vlines:u16,pub allow_to_target_delay_otg_vlines:u16,pub config:u8,pub otg_inst:u8,pub pipe_mask:u8,pub num_planes:u8,pub plane_pipe_masks:[u8;DMUB_MAX_PLANES],pub pad:[u8;DMUB_MAX_PLANES%4] }
    #[repr(C)] pub struct dmub_fams2_stream_static_state_v1 { pub base:dmub_fams2_cmd_stream_static_base_state,pub sub_state:dmub_fams2_stream_static_sub_state_v2 }
    #[repr(u32)] pub enum dmub_fams2_allow_delay_check_mode { FAMS2_ALLOW_DELAY_CHECK_NONE=0,FAMS2_ALLOW_DELAY_CHECK_FROM_START=1,FAMS2_ALLOW_DELAY_CHECK_FROM_PREPARE=2 }
    #[repr(C)] #[derive(Copy,Clone)] pub struct dmub_fams2_global_feature_config { pub all:u32 }
    #[repr(C)] #[derive(Copy,Clone)] pub struct dmub_cmd_fams2_global_config { pub max_allow_delay_us:u32,pub lock_wait_time_us:u32,pub num_streams:u32,pub features:dmub_fams2_global_feature_config,pub recovery_timeout_us:u32,pub hwfq_flip_programming_delay_us:u32,pub max_allow_to_target_delta_us:u32 }
    #[repr(C)] pub union dmub_cmd_fams2_config { pub global:dmub_cmd_fams2_global_config,pub stream:dmub_fams2_stream_static_state,pub stream_v1:dmub_cmd_fams2_config_stream_v1 }
    #[repr(C)] pub union dmub_cmd_fams2_config_stream_v1 { pub base:dmub_fams2_cmd_stream_static_base_state,pub sub_state:dmub_fams2_cmd_stream_static_sub_state }
    #[repr(C)] pub struct dmub_fams2_config_v2 { pub global:dmub_cmd_fams2_global_config,pub stream_v1:[dmub_fams2_stream_static_state_v1;DMUB_MAX_STREAMS] }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
