/*
 * SPDX-License-Identifier: MIT
 *
 * Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.
 */

// Dependency provided by dmub_cmd.h is expected to be available to the
// translation unit containing this header.

pub const END_SWATH_REC: u32 = 0xFFFF;
pub const END_SWATH_PRE: u32 = 0xFFFFF;
pub const NEXT_FRAME_MASK: u32 = 0x80000000;
pub const SWATH_MASK: u32 = 0x7FFFFFFF;
pub const MAX_FRAME_COUNT: u32 = 0xFFFFFF;
pub const PROGRAM_GO_IMMEDIATE: u32 = 0xFFFFFFFF;
pub const MAX_SUBVP_HEIGHT: u32 = 0xFFF;
pub const MAX_SUBVP_START_LINE: u32 = 0xFFFF;

#[repr(C)]
pub struct get_swath_deadlines_params {
    /* inputs */
    pub base: *mut dmub_fams2_cmd_stream_static_base_state,
    pub alternate_static_state: *mut dmub_fams2_cmd_alternate_stream_static_state,
    pub plane_index: u8,
    pub vtotal: u16,
    pub rec_y_start: u16,
    pub chroma_plane: bool,
    /* outputs */
    // caller must allocate its own memory for the output
    pub swath_array: *mut u16,
    pub array_size: *mut u16,
}

#[repr(C)]
pub struct calculate_hubp_start_end_lines_params {
    /* inputs */
    pub base: *mut dmub_fams2_cmd_stream_static_base_state,
    pub alternate_static_state: *mut dmub_fams2_cmd_alternate_stream_static_state,
    pub current_otg_line: u32, // [dst line]
    pub current_frame_count: u32, // reference frame count
    pub otg_pstate_target: u32, // [dst line]
    pub target_frame_count: u32, // target frame count that we expect to assert P-State allow
    pub vtotal: u16,
    pub rec_y_start: u16,
    pub plane_index: u8,
    pub cursor_size: u8,
    pub chroma_plane: bool,

    /* outputs */
    pub svp0_start_line: u16,
    pub svp0_height: u16,
    pub svp0_height_next: u16,
    pub svp1_start_line: u16,
    pub svp1_height: u16,
    pub svp1_height_next: u16,
    pub svp_position: u8,
    pub program_go_line: u32,
    pub program_go_frame_count: u32,
    /* for debug */
    pub svp0_start_dst_line: u16,
    pub svp0_end_dst_line: u16,
    pub svp1_start_dst_line: u16,
    pub svp1_end_dst_line: u16,
}

#[repr(C)]
pub struct calculate_copy_from_primary_params {
    /* inputs */
    pub target_frame: u32,
    pub flip_pending: u32,
    pub flip_pending_clear_frame: u32,
    /* outputs */
    pub copy_from_primary: bool,
}

#[repr(C)]
pub struct svp_params {
    pub start_line: u16,
    pub height: u16,
    pub height_next: u16,
}

#[repr(C)]
pub struct calculate_lsdma_copy_params {
    /* inputs */
    pub base: *mut dmub_fams2_cmd_stream_static_base_state,
    pub alternate_static_state: *mut dmub_fams2_cmd_alternate_stream_static_state,
    pub plane_index: u8,
    // array of 2 for svp0 and svp1
    pub svp: [svp_params; 2],
    // array of 2 for svp0 and svp1
    pub svp_c: [svp_params; 2],

    /* outputs */
    // array of 2 for svp0 and svp1
    pub out: [lsdma_outputs; 2],
    // array of 2 for svp0 and svp1
    pub out_c: [lsdma_outputs; 2],
}

unsafe extern "C" {
    pub fn calculate_lsdma_copy(p: *mut calculate_lsdma_copy_params);

    pub fn calculate_copy_from_primary(p: *mut calculate_copy_from_primary_params);

    pub fn get_swath_deadlines(p: *mut get_swath_deadlines_params);

    pub fn calculate_hubp_start_end_lines(p: *mut calculate_hubp_start_end_lines_params);

    pub fn get_prefetch_start_line_x1000(
        vtotal: u32,
        vblank_end: u16,
        recout_y: u16,
        dst_y_prefetch_x1000: u16,
        prefetch_relative_vblank: u8,
        dst_y_after_scaler: u16,
    ) -> i32;

    pub fn get_prefetch_end_line(
        vtotal: u32,
        vblank_end: u16,
        recout_y: u16,
        prefetch_relative_vblank: u8,
        dst_y_after_scaler: u16,
    ) -> i32;

    pub fn get_effective_vblank_start(
        vblank_start: u16,
        vblank_end: u16,
        recout_y: u16,
        recout_height: u16,
    ) -> u16;

    pub fn in_circular_range(start: u32, end: u32, value: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
