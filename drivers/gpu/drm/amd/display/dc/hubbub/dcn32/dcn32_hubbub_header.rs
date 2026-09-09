/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependency supplied by dcn21/dcn21_hubbub.h.
use std::os::raw::{c_int, c_uint};

macro_rules! HUBBUB_MASK_SH_LIST_DCN32 {
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
        HUBBUB_SF!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D, $mask_sh),
        HUBBUB_MASK_SH_LIST_STUTTER!($mask_sh),
        HUBBUB_SF!(DCHUBBUB_GLOBAL_TIMER_CNTL, DCHUBBUB_GLOBAL_TIMER_REFDIV, $mask_sh),
        HUBBUB_SF!(DCN_VM_FB_LOCATION_BASE, FB_BASE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FB_LOCATION_TOP, FB_TOP, $mask_sh),
        HUBBUB_SF!(DCN_VM_FB_OFFSET, FB_OFFSET, $mask_sh),
        HUBBUB_SF!(DCN_VM_AGP_BOT, AGP_BOT, $mask_sh),
        HUBBUB_SF!(DCN_VM_AGP_TOP, AGP_TOP, $mask_sh),
        HUBBUB_SF!(DCN_VM_AGP_BASE, AGP_BASE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_A, DCHUBBUB_ARB_FRAC_URG_BW_NOM_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_B, DCHUBBUB_ARB_FRAC_URG_BW_NOM_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_C, DCHUBBUB_ARB_FRAC_URG_BW_NOM_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_D, DCHUBBUB_ARB_FRAC_URG_BW_NOM_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DEBUG_CTRL_0, DET_DEPTH, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET0_CTRL, DET0_SIZE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET0_CTRL, DET0_SIZE_CURRENT, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET1_CTRL, DET1_SIZE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET1_CTRL, DET1_SIZE_CURRENT, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET2_CTRL, DET2_SIZE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET2_CTRL, DET2_SIZE_CURRENT, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET3_CTRL, DET3_SIZE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_DET3_CTRL, DET3_SIZE_CURRENT, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_COMPBUF_CTRL, COMPBUF_SIZE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_COMPBUF_CTRL, COMPBUF_SIZE_CURRENT, $mask_sh),
        HUBBUB_SF!(COMPBUF_RESERVED_SPACE, COMPBUF_RESERVED_SPACE_64B, $mask_sh),
        HUBBUB_SF!(COMPBUF_RESERVED_SPACE, COMPBUF_RESERVED_SPACE_ZS, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_CNTL, DCHUBBUB_ARB_ALLOW_USR_RETRAINING_FORCE_VALUE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_CNTL, DCHUBBUB_ARB_ALLOW_USR_RETRAINING_FORCE_ENABLE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_CNTL, DCHUBBUB_ARB_DO_NOT_FORCE_ALLOW_USR_RETRAINING_DURING_PSTATE_CHANGE_REQUEST, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_CNTL, DCHUBBUB_ARB_DO_NOT_FORCE_ALLOW_USR_RETRAINING_DURING_PRE_CSTATE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_A, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_B, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_C, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_D, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_A, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_B, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_C, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_D, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_A, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_B, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_C, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_D, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_D, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_ADDR_MSB, DCN_VM_FAULT_ADDR_MSB, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_ADDR_LSB, DCN_VM_FAULT_ADDR_LSB, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_CLEAR, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_MODE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_INTERRUPT_ENABLE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_RANGE_FAULT_DISABLE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_PRQ_FAULT_DISABLE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_STATUS, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_VMID, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_TABLE_LEVEL, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_PIPE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_INTERRUPT_STATUS, $mask_sh),
        HUBBUB_SF!(SDPIF_REQUEST_RATE_LIMIT, SDPIF_REQUEST_RATE_LIMIT, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_CLOCK_CNTL, DISPCLK_R_DCHUBBUB_GATE_DIS, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_CLOCK_CNTL, DCFCLK_R_DCHUBBUB_GATE_DIS, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_SDPIF_CFG0, SDPIF_PORT_CONTROL, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_SDPIF_CFG1, SDPIF_MAX_NUM_OUTSTANDING, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_MEM_PWR_MODE_CTRL, DET_MEM_PWR_LS_MODE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_MALL_CNTL, MALL_PREFETCH_COMPLETE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_MALL_CNTL, MALL_IN_USE, $mask_sh)
    };
}

extern "C" {
    pub fn hubbub32_program_urgent_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub32_program_stutter_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub32_program_pstate_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub32_program_usr_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub32_force_usr_retraining_allow(hubbub: *mut hubbub, allow: bool);
    pub fn hubbub32_force_wm_propagate_to_pipes(hubbub: *mut hubbub);
    pub fn hubbub32_init(hubbub: *mut hubbub);
    pub fn dcn32_program_det_size(hubbub: *mut hubbub, hubp_inst: c_int, det_buffer_size_in_kbyte: c_uint);
    pub fn hubbub32_construct(hubbub2: *mut dcn20_hubbub, ctx: *mut dc_context, hubbub_regs: *const dcn_hubbub_registers, hubbub_shift: *const dcn_hubbub_shift, hubbub_mask: *const dcn_hubbub_mask, det_size_kb: c_int, pixel_chunk_size_kb: c_int, config_return_buffer_size_kb: c_int);
    pub fn hubbub32_set_request_limit(hubbub: *mut hubbub, umc_count: c_int, words_per_umc: c_int);
    pub fn hubbub32_get_mall_en(hubbub: *mut hubbub, mall_in_use: *mut c_uint);
    pub fn dcn32_program_compbuf_size(hubbub: *mut hubbub, compbuf_size_kb: c_uint, safe_to_increase: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
