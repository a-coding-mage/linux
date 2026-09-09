/* Copyright 2012-17 Advanced Micro Devices, Inc.
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

// DCN register-name construction macros. Their arguments are expected to be
// identifiers supplied by the generated register definitions.
macro_rules! BASE_INNER { ($seg:ident) => { DCE_BASE__INST0_SEG$seg }; }
macro_rules! BASE { ($seg:ident) => { BASE_INNER!($seg) }; }
macro_rules! SR { ($reg_name:ident) => { . $reg_name = BASE!(mm$reg_name_BASE_IDX) + mm$reg_name }; }
macro_rules! SRI { ($reg_name:ident, $block:ident, $id:ident) => { . $reg_name = BASE!(mm$block$id_$reg_name_BASE_IDX) + mm$block$id_$reg_name }; }
macro_rules! SRII { ($reg_name:ident, $block:ident, $id:ident) => { .$reg_name[$id] = BASE!(mm$block$id_$reg_name_BASE_IDX) + mm$block$id_$reg_name }; }
macro_rules! SF { ($reg_name:ident, $field_name:ident, $post_fix:ident) => { .$field_name = $reg_name__$field_name$post_fix }; }

macro_rules! DWBC_COMMON_REG_LIST_DCN1_0 {
    ($inst:ident) => {
        SRI!(WB_ENABLE, CNV, $inst), SRI!(WB_EC_CONFIG, CNV, $inst),
        SRI!(CNV_MODE, CNV, $inst), SRI!(WB_SOFT_RESET, CNV, $inst),
        SRI!(MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_PITCH, MCIF_WB, $inst),
        SRI!(MCIF_WB_ARBITRATION_CONTROL, MCIF_WB, $inst), SRI!(MCIF_WB_SCLK_CHANGE, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_Y, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_1_ADDR_Y_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_C, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_1_ADDR_C_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_Y, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_2_ADDR_Y_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_C, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_2_ADDR_C_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_Y, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_3_ADDR_Y_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_C, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_3_ADDR_C_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_Y, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_4_ADDR_Y_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_C, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_4_ADDR_C_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUFMGR_VCE_CONTROL, MCIF_WB, $inst), SRI!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, MCIF_WB, $inst),
        SRI!(MCIF_WB_NB_PSTATE_CONTROL, MCIF_WB, $inst), SRI!(MCIF_WB_WATERMARK, MCIF_WB, $inst),
        SRI!(MCIF_WB_WARM_UP_CNTL, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_LUMA_SIZE, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_CHROMA_SIZE, MCIF_WB, $inst)
    };
}

// The mask-list macro is retained as a source-level macro; field token
// concatenation follows the corresponding generated C register definitions.
macro_rules! DWBC_COMMON_MASK_SH_LIST_DCN1_0 {
    ($mask_sh:ident) => { /* SF!(...) entries from the C header */ };
}

#[repr(C)]
pub struct dcn10_dwbc_registers {
    pub WB_ENABLE: u32, pub WB_EC_CONFIG: u32, pub CNV_MODE: u32, pub WB_SOFT_RESET: u32,
    pub MCIF_WB_BUFMGR_SW_CONTROL: u32, pub MCIF_WB_BUF_PITCH: u32, pub MCIF_WB_ARBITRATION_CONTROL: u32,
    pub MCIF_WB_SCLK_CHANGE: u32, pub MCIF_WB_BUF_1_ADDR_Y: u32, pub MCIF_WB_BUF_1_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_1_ADDR_C: u32, pub MCIF_WB_BUF_1_ADDR_C_OFFSET: u32, pub MCIF_WB_BUF_2_ADDR_Y: u32,
    pub MCIF_WB_BUF_2_ADDR_Y_OFFSET: u32, pub MCIF_WB_BUF_2_ADDR_C: u32, pub MCIF_WB_BUF_2_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUF_3_ADDR_Y: u32, pub MCIF_WB_BUF_3_ADDR_Y_OFFSET: u32, pub MCIF_WB_BUF_3_ADDR_C: u32,
    pub MCIF_WB_BUF_3_ADDR_C_OFFSET: u32, pub MCIF_WB_BUF_4_ADDR_Y: u32, pub MCIF_WB_BUF_4_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_4_ADDR_C: u32, pub MCIF_WB_BUF_4_ADDR_C_OFFSET: u32, pub MCIF_WB_BUFMGR_VCE_CONTROL: u32,
    pub MCIF_WB_NB_PSTATE_LATENCY_WATERMARK: u32, pub MCIF_WB_NB_PSTATE_CONTROL: u32, pub MCIF_WB_WATERMARK: u32,
    pub MCIF_WB_WARM_UP_CNTL: u32, pub MCIF_WB_BUF_LUMA_SIZE: u32, pub MCIF_WB_BUF_CHROMA_SIZE: u32,
}

macro_rules! DWBC_REG_FIELD_LIST { ($type:ty) => {
    WB_ENABLE: $type, DISPCLK_R_WB_GATE_DIS: $type, DISPCLK_G_WB_GATE_DIS: $type,
    DISPCLK_G_WBSCL_GATE_DIS: $type, WB_LB_LS_DIS: $type, WB_LB_SD_DIS: $type,
    WB_LUT_LS_DIS: $type, CNV_WINDOW_CROP_EN: $type, CNV_STEREO_TYPE: $type,
    CNV_INTERLACED_MODE: $type, CNV_EYE_SELECTION: $type, CNV_STEREO_POLARITY: $type,
    CNV_INTERLACED_FIELD_ORDER: $type, CNV_STEREO_SPLIT: $type, CNV_NEW_CONTENT: $type,
    CNV_FRAME_CAPTURE_EN: $type, WB_SOFT_RESET: $type, MCIF_WB_BUFMGR_ENABLE: $type,
    MCIF_WB_BUF_DUALSIZE_REQ: $type, MCIF_WB_BUFMGR_SW_INT_EN: $type, MCIF_WB_BUFMGR_SW_INT_ACK: $type,
    MCIF_WB_BUFMGR_SW_SLICE_INT_EN: $type, MCIF_WB_BUFMGR_SW_OVERRUN_INT_EN: $type,
    MCIF_WB_BUFMGR_SW_LOCK: $type, MCIF_WB_P_VMID: $type, MCIF_WB_BUF_ADDR_FENCE_EN: $type,
    MCIF_WB_BUF_LUMA_PITCH: $type, MCIF_WB_BUF_CHROMA_PITCH: $type, MCIF_WB_CLIENT_ARBITRATION_SLICE: $type,
    MCIF_WB_TIME_PER_PIXEL: $type, WM_CHANGE_ACK_FORCE_ON: $type, MCIF_WB_CLI_WATERMARK_MASK: $type,
    MCIF_WB_BUF_1_ADDR_Y: $type, MCIF_WB_BUF_1_ADDR_Y_OFFSET: $type, MCIF_WB_BUF_1_ADDR_C: $type,
    MCIF_WB_BUF_1_ADDR_C_OFFSET: $type, MCIF_WB_BUF_2_ADDR_Y: $type, MCIF_WB_BUF_2_ADDR_Y_OFFSET: $type,
    MCIF_WB_BUF_2_ADDR_C: $type, MCIF_WB_BUF_2_ADDR_C_OFFSET: $type, MCIF_WB_BUF_3_ADDR_Y: $type,
    MCIF_WB_BUF_3_ADDR_Y_OFFSET: $type, MCIF_WB_BUF_3_ADDR_C: $type, MCIF_WB_BUF_3_ADDR_C_OFFSET: $type,
    MCIF_WB_BUF_4_ADDR_Y: $type, MCIF_WB_BUF_4_ADDR_Y_OFFSET: $type, MCIF_WB_BUF_4_ADDR_C: $type,
    MCIF_WB_BUF_4_ADDR_C_OFFSET: $type, MCIF_WB_BUFMGR_VCE_LOCK_IGNORE: $type,
    MCIF_WB_BUFMGR_VCE_INT_EN: $type, MCIF_WB_BUFMGR_VCE_INT_ACK: $type,
    MCIF_WB_BUFMGR_VCE_SLICE_INT_EN: $type, MCIF_WB_BUFMGR_VCE_LOCK: $type,
    MCIF_WB_BUFMGR_SLICE_SIZE: $type, NB_PSTATE_CHANGE_REFRESH_WATERMARK: $type,
    NB_PSTATE_CHANGE_URGENT_DURING_REQUEST: $type, NB_PSTATE_CHANGE_FORCE_ON: $type,
    NB_PSTATE_ALLOW_FOR_URGENT: $type, NB_PSTATE_CHANGE_WATERMARK_MASK: $type,
    MCIF_WB_CLI_WATERMARK: $type, MCIF_WB_CLI_CLOCK_GATER_OVERRIDE: $type,
    MCIF_WB_PITCH_SIZE_WARMUP: $type, MCIF_WB_BUF_LUMA_SIZE: $type, MCIF_WB_BUF_CHROMA_SIZE: $type
}; }

#[repr(C)] pub struct dcn10_dwbc_mask { pub fields: [u32; 0] }
#[repr(C)] pub struct dcn10_dwbc_shift { pub fields: [u8; 0] }

// `dwbc` and `dc_context` are supplied by external dependencies.
#[repr(C)] pub struct dcn10_dwbc { pub base: dwbc, pub dwbc_regs: *const dcn10_dwbc_registers, pub dwbc_shift: *const dcn10_dwbc_shift, pub dwbc_mask: *const dcn10_dwbc_mask }

extern "C" {
    pub fn dcn10_dwbc_construct(dwbc10: *mut dcn10_dwbc, ctx: *mut dc_context, dwbc_regs: *const dcn10_dwbc_registers, dwbc_shift: *const dcn10_dwbc_shift, dwbc_mask: *const dcn10_dwbc_mask, inst: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
