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

// Dependencies supplied by the surrounding translation unit:
// dal_types.h, dc_hw_types.h

pub const DWB_SW_V2: u32 = 1;
pub const DWB_MCIF_BUF_COUNT: u32 = 4;

#[repr(C)]
pub struct mcif_wb {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_sw_version {
    dwb_ver_1_0 = 1,
    dwb_ver_2_0 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_source {
    dwb_src_scl = 0, // for DCE7x/9x, DCN won't support.
    dwb_src_blnd,    // for DCE7x/9x
    dwb_src_fmt,     // for DCE7x/9x
    dwb_src_otg0 = 0x100, // for DCN1.x/DCN2.x, register: mmDWB_SOURCE_SELECT
    dwb_src_otg1,    // for DCN1.x/DCN2.x
    dwb_src_otg2,    // for DCN1.x/DCN2.x
    dwb_src_otg3,    // for DCN1.x/DCN2.x
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_pipe {
    dwb_pipe0 = 0,
    dwb_pipe1,
    dwb_pipe_max_num,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_frame_capture_enable {
    DWB_FRAME_CAPTURE_DISABLE = 0,
    DWB_FRAME_CAPTURE_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum wbscl_coef_filter_type_sel {
    WBSCL_COEF_LUMA_VERT_FILTER = 0,
    WBSCL_COEF_CHROMA_VERT_FILTER = 1,
    WBSCL_COEF_LUMA_HORZ_FILTER = 2,
    WBSCL_COEF_CHROMA_HORZ_FILTER = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_boundary_mode {
    DWBSCL_BOUNDARY_MODE_EDGE = 0,
    DWBSCL_BOUNDARY_MODE_BLACK = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_output_csc_mode {
    DWB_OUTPUT_CSC_DISABLE = 0,
    DWB_OUTPUT_CSC_COEF_A = 1,
    DWB_OUTPUT_CSC_COEF_B = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_ogam_lut_mode {
    DWB_OGAM_MODE_BYPASS,
    DWB_OGAM_RAMA_LUT,
    DWB_OGAM_RAMB_LUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_color_volume {
    DWB_SRGB_BT709 = 0, // SDR
    DWB_PQ = 1,         // HDR
    DWB_HLG = 2,        // HDR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwb_color_space {
    DWB_SRGB = 0,   // SDR
    DWB_BT709 = 1,  // SDR
    DWB_BT2020 = 2, // HDR
}

#[repr(C)]
pub struct dwb_efc_hdr_metadata {
    pub chromaticity_green_x: u32,
    pub chromaticity_green_y: u32,
    pub chromaticity_blue_x: u32,
    pub chromaticity_blue_y: u32,
    pub chromaticity_red_x: u32,
    pub chromaticity_red_y: u32,
    pub chromaticity_white_point_x: u32,
    pub chromaticity_white_point_y: u32,
    pub min_luminance: u32,
    pub max_luminance: u32,
    pub maximum_content_light_level: u32,
    pub maximum_frame_average_light_level: u32,
}

#[repr(C)]
pub struct dwb_efc_display_settings {
    pub inputColorVolume: u32,
    pub inputColorSpace: u32,
    pub inputBitDepthMinus8: u32,
    pub hdr_metadata: dwb_efc_hdr_metadata,
    pub dwbOutputBlack: u32, // 0 - Normal, 1 - Output Black
}

#[repr(C)]
pub struct dwb_warmup_params {
    pub warmup_en: bool,    // false: normal mode, true: enable pattern generator
    pub warmup_mode: bool,  // false: 420, true: 444
    pub warmup_depth: bool, // false: 8bit, true: 10bit
    pub warmup_data: i32,
    pub warmup_width: i32,
    pub warmup_height: i32,
}

#[repr(C)]
pub struct dwb_caps {
    pub hw_version: dce_version,
    pub sw_version: dwb_sw_version,
    pub reserved: [u32; 6],
    pub adapter_id: u32,
    pub num_pipes: u32,
    // C bitfields support_dwb:1, support_ogam:1, support_wbscl:1,
    // support_ocsc:1, support_stereo:1, support_4k_120p:1.
    pub caps: u32,
    pub reserved2: [u32; 10],
}

#[repr(C)]
pub struct dwbc {
    pub funcs: *const dwbc_funcs,
    pub ctx: *mut dc_context,
    pub inst: i32,
    pub mcif: *mut mcif_wb,
    pub status: bool,
    pub inputSrcSelect: i32,
    pub dwb_output_black: bool,
    pub tf: dc_transfer_func_predefined,
    pub output_color_space: dc_color_space,
    pub dwb_is_efc_transition: bool,
    pub dwb_is_drc: bool,
    pub wb_src_plane_inst: i32, // hubp, mpcc, inst
    pub mask_id: u32,
    pub otg_inst: i32,
    pub mvc_cfg: bool,
    pub params: dc_dwb_params,
}

#[repr(C)]
pub struct dwbc_funcs {
    pub get_caps: Option<unsafe extern "C" fn(*mut dwbc, *mut dwb_caps) -> bool>,
    pub enable: Option<unsafe extern "C" fn(*mut dwbc, *mut dc_dwb_params) -> bool>,
    pub disable: Option<unsafe extern "C" fn(*mut dwbc) -> bool>,
    pub update: Option<unsafe extern "C" fn(*mut dwbc, *mut dc_dwb_params) -> bool>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut dwbc) -> bool>,
    pub set_fc_enable: Option<unsafe extern "C" fn(*mut dwbc, dwb_frame_capture_enable)>,
    pub dwb_set_scaler: Option<unsafe extern "C" fn(*mut dwbc, *mut dc_dwb_params)>,
    pub set_stereo: Option<unsafe extern "C" fn(*mut dwbc, *mut dwb_stereo_params)>,
    pub set_new_content: Option<unsafe extern "C" fn(*mut dwbc, bool)>,
    pub set_warmup: Option<unsafe extern "C" fn(*mut dwbc, *mut dwb_warmup_params)>,
    pub dwb_get_mcifbuf_line: Option<unsafe extern "C" fn(*mut dwbc, *mut u32, *mut u32, *mut u32) -> bool>,
    // Preserved build-time condition: CONFIG_DRM_AMD_DC_FP.
    pub dwb_program_output_csc: Option<unsafe extern "C" fn(*mut dwbc, dc_color_space, dwb_output_csc_mode)>,
    pub dwb_ogam_set_output_transfer_func: Option<unsafe extern "C" fn(*mut dwbc, *const dc_transfer_func) -> bool>,
    // TODO: merge with output_transfer_func?
    pub dwb_ogam_set_input_transfer_func: Option<unsafe extern "C" fn(*mut dwbc, *const dc_transfer_func) -> bool>,
    pub get_drr_time_stamp: Option<unsafe extern "C" fn(*mut dwbc, *mut u32)>,
    pub get_dwb_status: Option<unsafe extern "C" fn(*mut dwbc) -> bool>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
