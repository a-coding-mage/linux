/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency supplied by the corresponding C/Rust translation unit:
// #include "dcn32/dcn32_hubbub.h"

pub const DCN4_01_CRB_SIZE_KB: i32 = 1344;
pub const DCN4_01_DEFAULT_DET_SIZE: i32 = 320;
pub const DCN4_01_CRB_SEGMENT_SIZE_KB: i32 = 64;

// C macro preserved as a Rust macro; HUBBUB_SF is supplied by dependencies.
macro_rules! HUBBUB_MASK_SH_LIST_DCN4_01 {
    ($mask_sh:expr) => {
        HUBBUB_SF!(DCHUBBUB_GLOBAL_TIMER_CNTL, DCHUBBUB_GLOBAL_TIMER_ENABLE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_SOFT_RESET, DCHUBBUB_GLOBAL_SOFT_RESET, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_WATERMARK_CHANGE_CNTL, DCHUBBUB_ARB_WATERMARK_CHANGE_REQUEST, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_WATERMARK_CHANGE_CNTL, DCHUBBUB_ARB_WATERMARK_CHANGE_DONE_INTERRUPT_DISABLE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DRAM_STATE_CNTL, DCHUBBUB_ARB_ALLOW_SELF_REFRESH_FORCE_VALUE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DRAM_STATE_CNTL, DCHUBBUB_ARB_ALLOW_SELF_REFRESH_FORCE_ENABLE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DRAM_STATE_CNTL, DCHUBBUB_ARB_ALLOW_PSTATE_CHANGE_FORCE_VALUE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DRAM_STATE_CNTL, DCHUBBUB_ARB_ALLOW_PSTATE_CHANGE_FORCE_ENABLE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_SAT_LEVEL, DCHUBBUB_ARB_SAT_LEVEL, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DF_REQ_OUTSTAND, DCHUBBUB_ARB_MIN_REQ_OUTSTAND, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DF_REQ_OUTSTAND, DCHUBBUB_ARB_MAX_REQ_OUTSTAND, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK1_A, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK1_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK1_A, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK1_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK1_B, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK1_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK1_B, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK1_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK2_A, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK2_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK2_A, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK2_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK2_B, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK2_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK2_B, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK2_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK3_A, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK3_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK3_A, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK3_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK3_B, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK3_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK3_B, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK3_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_GLOBAL_TIMER_CNTL, DCHUBBUB_GLOBAL_TIMER_REFDIV, $mask_sh),
        HUBBUB_SF!(DCN_VM_FB_LOCATION_BASE, FB_BASE, $mask_sh), HUBBUB_SF!(DCN_VM_FB_LOCATION_TOP, FB_TOP, $mask_sh), HUBBUB_SF!(DCN_VM_FB_OFFSET, FB_OFFSET, $mask_sh),
        HUBBUB_SF!(DCN_VM_AGP_BOT, AGP_BOT, $mask_sh), HUBBUB_SF!(DCN_VM_AGP_TOP, AGP_TOP, $mask_sh), HUBBUB_SF!(DCN_VM_AGP_BASE, AGP_BASE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_A, DCHUBBUB_ARB_FRAC_URG_BW_NOM_A, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_B, DCHUBBUB_ARB_FRAC_URG_BW_NOM_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_META_TRIP_A, DCHUBBUB_ARB_REFCYC_PER_META_TRIP_A, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_META_TRIP_B, DCHUBBUB_ARB_REFCYC_PER_META_TRIP_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DEBUG_CTRL_0, DET_DEPTH, $mask_sh), HUBBUB_SF!(DCHUBBUB_DET0_CTRL, DET0_SIZE, $mask_sh), HUBBUB_SF!(DCHUBBUB_DET0_CTRL, DET0_SIZE_CURRENT, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET1_CTRL, DET1_SIZE, $mask_sh), HUBBUB_SF!(DCHUBBUB_DET1_CTRL, DET1_SIZE_CURRENT, $mask_sh), HUBBUB_SF!(DCHUBBUB_DET2_CTRL, DET2_SIZE, $mask_sh), HUBBUB_SF!(DCHUBBUB_DET2_CTRL, DET2_SIZE_CURRENT, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET3_CTRL, DET3_SIZE, $mask_sh), HUBBUB_SF!(DCHUBBUB_DET3_CTRL, DET3_SIZE_CURRENT, $mask_sh), HUBBUB_SF!(DCHUBBUB_COMPBUF_CTRL, COMPBUF_SIZE, $mask_sh), HUBBUB_SF!(DCHUBBUB_COMPBUF_CTRL, COMPBUF_SIZE_CURRENT, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_COMPBUF_CTRL, CONFIG_ERROR, $mask_sh), HUBBUB_SF!(COMPBUF_RESERVED_SPACE, COMPBUF_RESERVED_SPACE_64B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_CNTL, DCHUBBUB_ARB_ALLOW_USR_RETRAINING_FORCE_VALUE, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_CNTL, DCHUBBUB_ARB_ALLOW_USR_RETRAINING_FORCE_ENABLE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_A, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_A, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_B, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_A, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_A, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_B, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_A, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_A, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_B, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_MALL_A, DCHUBBUB_ARB_FRAC_URG_BW_MALL_A, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_MALL_B, DCHUBBUB_ARB_FRAC_URG_BW_MALL_B, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_ADDR_MSB, DCN_VM_FAULT_ADDR_MSB, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_ADDR_LSB, DCN_VM_FAULT_ADDR_LSB, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_CLEAR, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_MODE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_INTERRUPT_ENABLE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_RANGE_FAULT_DISABLE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_PRQ_FAULT_DISABLE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_STATUS, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_VMID, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_TABLE_LEVEL, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_PIPE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_INTERRUPT_STATUS, $mask_sh),
        HUBBUB_SF!(SDPIF_REQUEST_RATE_LIMIT, SDPIF_REQUEST_RATE_LIMIT, $mask_sh), HUBBUB_SF!(DCHUBBUB_CLOCK_CNTL, DISPCLK_R_DCHUBBUB_GATE_DIS, $mask_sh), HUBBUB_SF!(DCHUBBUB_CLOCK_CNTL, DCFCLK_R_DCHUBBUB_GATE_DIS, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_SDPIF_CFG0, SDPIF_PORT_CONTROL, $mask_sh), HUBBUB_SF!(DCHUBBUB_SDPIF_CFG1, SDPIF_MAX_NUM_OUTSTANDING, $mask_sh), HUBBUB_SF!(DCHUBBUB_MEM_PWR_MODE_CTRL, DET_MEM_PWR_LS_MODE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_TIMEOUT_DETECTION_CTRL1, DCHUBBUB_TIMEOUT_ERROR_STATUS, $mask_sh), HUBBUB_SF!(DCHUBBUB_TIMEOUT_DETECTION_CTRL1, DCHUBBUB_TIMEOUT_REQ_STALL_THRESHOLD, $mask_sh), HUBBUB_SF!(DCHUBBUB_TIMEOUT_DETECTION_CTRL2, DCHUBBUB_TIMEOUT_PSTATE_STALL_THRESHOLD, $mask_sh), HUBBUB_SF!(DCHUBBUB_TIMEOUT_DETECTION_CTRL2, DCHUBBUB_TIMEOUT_DETECTION_EN, $mask_sh), HUBBUB_SF!(DCHUBBUB_TIMEOUT_DETECTION_CTRL2, DCHUBBUB_TIMEOUT_TIMER_RESET, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_CTRL_STATUS, ROB_UNDERFLOW_STATUS, $mask_sh), HUBBUB_SF!(DCHUBBUB_CTRL_STATUS, ROB_OVERFLOW_STATUS, $mask_sh), HUBBUB_SF!(DCHUBBUB_CTRL_STATUS, ROB_OVERFLOW_CLEAR, $mask_sh), HUBBUB_SF!(DCHUBBUB_CTRL_STATUS, DCHUBBUB_HW_DEBUG, $mask_sh), HUBBUB_SF!(DCHUBBUB_CTRL_STATUS, CSTATE_SWATH_CHK_GOOD_MODE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_MALL_CNTL, MALL_PREFETCH_COMPLETE, $mask_sh), HUBBUB_SF!(DCHUBBUB_ARB_MALL_CNTL, MALL_IN_USE, $mask_sh)
    };
}

extern "C" {
    pub fn hubbub401_program_urgent_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: u32, safe_to_lower: bool) -> bool;
    pub fn hubbub401_program_stutter_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub401_program_pstate_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub401_program_usr_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub401_dcc_support_swizzle(swizzle: swizzle_mode_addr3_values, plane_pitch: c_uint, bytes_per_element: c_uint, segment_order_horz: *mut segment_order, segment_order_vert: *mut segment_order) -> bool;
    pub fn hubbub401_dcc_support_pixel_format(format: surface_pixel_format, plane0_bpe: *mut c_uint, plane1_bpe: *mut c_uint) -> bool;
    pub fn hubbub401_get_blk256_size(blk256_width: *mut c_uint, blk256_height: *mut c_uint, bytes_per_element: c_uint);
    pub fn hubbub401_det_request_size(detile_buf_size: c_uint, format: surface_pixel_format, p0_height: c_uint, p0_width: c_uint, p0_bpe: c_uint, p1_height: c_uint, p1_width: c_uint, p1_bpe: c_uint, p0_req128_horz_wc: *mut bool, p0_req128_vert_wc: *mut bool, p1_req128_horz_wc: *mut bool, p1_req128_vert_wc: *mut bool);
    pub fn hubbub401_get_dcc_compression_cap(hubbub: *mut hubbub, input: *const dc_dcc_surface_param, output: *mut dc_surface_dcc_cap) -> bool;
    pub fn dcn401_program_arbiter(hubbub: *mut hubbub, arb_regs: *mut dml2_display_arb_regs, safe_to_lower: bool) -> bool;
    pub fn hubbub401_construct(hubbub2: *mut dcn20_hubbub, ctx: *mut dc_context, hubbub_regs: *const dcn_hubbub_registers, hubbub_shift: *const dcn_hubbub_shift, hubbub_mask: *const dcn_hubbub_mask, det_size_kb: c_int, pixel_chunk_size_kb: c_int, config_return_buffer_size_kb: c_int);
    pub fn dcn401_program_det_segments(hubbub: *mut hubbub, hubp_inst: c_int, det_buffer_size_seg: c_uint);
    pub fn dcn401_program_compbuf_segments(hubbub: *mut hubbub, compbuf_size_seg: c_uint, safe_to_increase: bool);
    pub fn dcn401_wait_for_det_update(hubbub: *mut hubbub, hubp_inst: c_int);
    pub fn dcn401_init_crb(hubbub: *mut hubbub);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
