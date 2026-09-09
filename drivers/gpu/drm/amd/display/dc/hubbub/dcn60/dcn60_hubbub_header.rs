// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the corresponding C header hierarchy are intentionally
// not reimplemented here.

pub const DCN6_0_CRB_SIZE_KB: i32 = 2112;
pub const DCN6_0_DEFAULT_DET_SIZE: i32 = 512;
pub const DCN6_0_CRB_SEGMENT_SIZE_KB: i32 = 64;

/// Register-field list for DCN 6.0. `HUBBUB_SF!` is supplied by the
/// architecture-specific register definitions.
#[macro_export]
macro_rules! HUBBUB_MASK_SH_LIST_DCN6_0 {
    ($mask_sh:expr) => {
        [
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
            HUBBUB_SF!(DCHUBBUB_GLOBAL_TIMER_CNTL, DCHUBBUB_GLOBAL_TIMER_REFDIV, $mask_sh),
            HUBBUB_SF!(DCN_VM_FB_LOCATION_BASE, FB_BASE, $mask_sh),
            HUBBUB_SF!(DCN_VM_FB_LOCATION_TOP, FB_TOP, $mask_sh),
            HUBBUB_SF!(DCN_VM_FB_OFFSET, FB_OFFSET, $mask_sh),
            HUBBUB_SF!(DCN_VM_AGP_BOT, AGP_BOT, $mask_sh),
            HUBBUB_SF!(DCN_VM_AGP_TOP, AGP_TOP, $mask_sh),
            HUBBUB_SF!(DCN_VM_AGP_BASE, AGP_BASE, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_DEBUG_CTRL_0, DET_DEPTH, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_DET0_CTRL, DET0_SIZE, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_DET1_CTRL, DET1_SIZE, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_DET2_CTRL, DET2_SIZE, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_DET3_CTRL, DET3_SIZE, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_COMPBUF_CTRL, COMPBUF_SIZE, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_COMPBUF_CTRL, CONFIG_ERROR, $mask_sh),
            HUBBUB_SF!(COMPBUF_RESERVED_SPACE, COMPBUF_RESERVED_SPACE_64B, $mask_sh),
            HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_CLEAR, $mask_sh),
            HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_MODE, $mask_sh),
            HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_INTERRUPT_ENABLE, $mask_sh),
            HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_STATUS, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_CLOCK_CNTL, DISPCLK_R_DCHUBBUB_GATE_DIS, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_CLOCK_CNTL, DCFCLK_R_DCHUBBUB_GATE_DIS, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_SDPIF_CFG0, SDPIF_PORT_CONTROL, $mask_sh),
            HUBBUB_SF!(DCHUBBUB_SDPIF_CFG1, SDPIF_MAX_NUM_OUTSTANDING, $mask_sh),
        ]
    };
}

extern "C" {
    pub fn hubbub60_construct(
        hubbub2: *mut dcn20_hubbub,
        ctx: *mut dc_context,
        hubbub_regs: *const dcn_hubbub_registers,
        hubbub_shift: *const dcn_hubbub_shift,
        hubbub_mask: *const dcn_hubbub_mask,
        det_size_kb: i32,
        pixel_chunk_size_kb: i32,
        config_return_buffer_size_kb: i32,
    );
}

// Types are declared by the included generation-specific headers.
extern "C" {
    pub type dcn20_hubbub;
    pub type dc_context;
    pub type dcn_hubbub_registers;
    pub type dcn_hubbub_shift;
    pub type dcn_hubbub_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
