/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit: dc_dsc.h,
// dc_hw_types.h, and dc_types.h. No other headers are required here.

#[repr(C)]
pub struct dsc_config {
    pub pic_width: u32,
    pub pic_height: u32,
    pub pixel_encoding: dc_pixel_encoding,
    pub color_depth: dc_color_depth, // Bits per component
    pub is_odm: bool,
    pub dc_dsc_cfg: dc_dsc_config,
    pub dsc_padding: u32,
}

#[repr(C)]
pub struct dsc_optc_config {
    pub slice_width: u32, // Slice width in pixels
    pub bytes_per_pixel: u32, // Bytes per pixel in u3.28 format
    pub is_pixel_format_444: bool, // 'true' if pixel format is 'RGB 444' or 'Simple YCbCr 4:2:2' (4:2:2 upsampled to 4:4:4)'
}

#[repr(C)]
pub struct dcn_dsc_state {
    pub dsc_clock_en: u32,
    pub dsc_slice_width: u32,
    pub dsc_bits_per_pixel: u32,
    pub dsc_slice_height: u32,
    pub dsc_pic_width: u32,
    pub dsc_pic_height: u32,
    pub dsc_slice_bpg_offset: u32,
    pub dsc_chunk_size: u32,
    pub dsc_fw_en: u32,
    pub dsc_opp_source: u32,
    pub dsc_block_pred_enable: u32,
    pub dsc_line_buf_depth: u32,
    pub dsc_version_minor: u32,
    pub dsc_rc_buffer_size: u32,
    pub dsc_simple_422: u32,
}

#[repr(C)]
pub struct dcn_dsc_reg_state {
    pub dsc_top_control: u32,
    pub dscc_interrupt_control_status: u32,
}

// DSC encoder capabilities. These differ from the DPCD DSC caps because they
// are based on AMD DSC encoder caps.
#[repr(C)]
pub struct dsc_enc_slice_caps_bits {
    pub raw: u8,
}

impl dsc_enc_slice_caps_bits {
    pub const NUM_SLICES_1: u8 = 1 << 0;
    pub const NUM_SLICES_2: u8 = 1 << 1;
    pub const NUM_SLICES_3: u8 = 1 << 2; // Not per DSC spec, but supported by our encoder
    pub const NUM_SLICES_4: u8 = 1 << 3;
    pub const NUM_SLICES_8: u8 = 1 << 4;
    pub const NUM_SLICES_12: u8 = 1 << 5;
    pub const NUM_SLICES_16: u8 = 1 << 6;
}

#[repr(C)]
pub union dsc_enc_slice_caps {
    pub bits: dsc_enc_slice_caps_bits,
    pub raw: u8,
}

#[repr(C)]
pub struct dsc_enc_caps {
    pub dsc_version: u8,
    pub slice_caps: dsc_enc_slice_caps,
    pub lb_bit_depth: i32,
    pub is_block_pred_supported: bool,
    pub color_formats: dsc_color_formats,
    pub color_depth: dsc_color_depth,
    pub max_total_throughput_mps: i32, // Maximum total throughput with all the slices combined
    pub max_slice_width: i32,
    pub bpp_increment_div: u32, // bpp increment divisor, e.g. if 16, it's 1/16th of a bit
    pub is_frl: bool,
    pub is_vic_all_bpp: bool,
    pub total_chunk_kbytes: u32,
    pub num_lanes: u32,
    pub frl_rate: u32,
    pub edp_sink_max_bits_per_pixel: u32,
    pub is_dp: bool,
}

#[repr(C)]
pub struct dsc_funcs {
    pub dsc_get_enc_caps: Option<unsafe extern "C" fn(dsc_enc_caps: *mut dsc_enc_caps, pixel_clock_100Hz: i32)>,
    pub dsc_read_state: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor, s: *mut dcn_dsc_state)>,
    pub dsc_read_reg_state: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor, dccg_reg_state: *mut dcn_dsc_reg_state)>,
    pub dsc_validate_stream: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config) -> bool>,
    pub dsc_set_config: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config, dsc_optc_cfg: *mut dsc_optc_config)>,
    pub dsc_get_packed_pps: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config, dsc_packed_pps: *mut u8) -> bool>,
    pub dsc_enable: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor, opp_pipe: i32)>,
    pub dsc_disable: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor)>,
    pub dsc_disconnect: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor)>,
    pub dsc_wait_disconnect_pending_clear: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor)>,
    pub dsc_get_single_enc_caps: Option<unsafe extern "C" fn(dsc_enc_caps: *mut dsc_enc_caps, max_dscclk_khz: u32)>,
    pub set_fgcg: Option<unsafe extern "C" fn(dsc: *mut display_stream_compressor, enable: bool)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
