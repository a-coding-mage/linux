/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependencies: include/grph_object_id.h and bios_parser_interface.h.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fbc_compress_ratio {
    FBC_COMPRESS_RATIO_INVALID = 0,
    FBC_COMPRESS_RATIO_1TO1 = 1,
    FBC_COMPRESS_RATIO_2TO1 = 2,
    FBC_COMPRESS_RATIO_4TO1 = 4,
    FBC_COMPRESS_RATIO_8TO1 = 8,
}

#[repr(C)]
pub union fbc_physical_address {
    pub addr: fbc_physical_address_addr,
    pub quad_part: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbc_physical_address_addr {
    pub low_part: u32,
    pub high_part: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compr_addr_and_pitch_params {
    // enum controller_id controller_id;
    pub inst: u32,
    pub source_view_width: u32,
    pub source_view_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fbc_hw_max_resolution_supported {
    FBC_MAX_X = 3840,
    FBC_MAX_Y = 2400,
    FBC_MAX_X_SG = 1920,
    FBC_MAX_Y_SG = 1080,
}

#[repr(C)]
pub struct compressor_funcs {
    pub power_up_fbc: Option<unsafe extern "C" fn(cp: *mut compressor)>,
    pub enable_fbc: Option<unsafe extern "C" fn(
        cp: *mut compressor,
        params: *mut compr_addr_and_pitch_params,
    )>,
    pub disable_fbc: Option<unsafe extern "C" fn(cp: *mut compressor)>,
    pub set_fbc_invalidation_triggers:
        Option<unsafe extern "C" fn(cp: *mut compressor, fbc_trigger: u32)>,
    pub surface_address_and_pitch: Option<unsafe extern "C" fn(
        cp: *mut compressor,
        params: *mut compr_addr_and_pitch_params,
    )>,
    pub is_fbc_enabled_in_hw: Option<unsafe extern "C" fn(
        cp: *mut compressor,
        fbc_mapped_crtc_id: *mut u32,
    ) -> bool>,
}

#[repr(C)]
pub union compressor_options {
    pub raw: u32,
    pub bits: compressor_options_bits,
}

// C bitfields; the low-order bits correspond to the fields in declaration order.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compressor_options_bits {
    pub value: u32,
}

impl compressor_options_bits {
    pub const FBC_SUPPORT: u32 = 1 << 0;
    pub const FB_POOL: u32 = 1 << 1;
    pub const DYNAMIC_ALLOC: u32 = 1 << 2;
    pub const LPT_SUPPORT: u32 = 1 << 3;
    pub const LPT_MC_CONFIG: u32 = 1 << 4;
    pub const DUMMY_BACKEND: u32 = 1 << 5;
    pub const CLK_GATING_DISABLED: u32 = 1 << 6;
}

#[repr(C)]
pub struct compressor {
    pub ctx: *mut dc_context,
    // CONTROLLER_ID_D0 + instance, CONTROLLER_ID_UNDEFINED = 0
    pub attached_inst: u32,
    pub is_enabled: bool,
    pub funcs: *const compressor_funcs,
    pub options: compressor_options,
    pub compr_surface_address: fbc_physical_address,
    pub embedded_panel_h_size: u32,
    pub embedded_panel_v_size: u32,
    pub memory_bus_width: u32,
    pub banks_num: u32,
    pub raw_size: u32,
    pub channel_interleave_size: u32,
    pub dram_channels_num: u32,
    pub allocated_size: u32,
    pub preferred_requested_size: u32,
    pub lpt_channels_num: u32,
    pub min_compress_ratio: fbc_compress_ratio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbc_input_info {
    pub dynamic_fbc_buffer_alloc: bool,
    pub source_view_width: libc::c_uint,
    pub source_view_height: libc::c_uint,
    pub num_of_active_targets: libc::c_uint,
}

#[repr(C)]
pub union fbc_requested_compressed_size_flags_union {
    pub bits: fbc_requested_compressed_size_flags_bits,
    pub flags: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbc_requested_compressed_size_flags_bits {
    pub value: libc::c_uint,
}

impl fbc_requested_compressed_size_flags_bits {
    pub const PREFERRED_MUST_BE_FRAMEBUFFER_POOL: libc::c_uint = 1 << 0;
    pub const MIN_MUST_BE_FRAMEBUFFER_POOL: libc::c_uint = 1 << 1;
}

#[repr(C)]
pub struct fbc_requested_compressed_size {
    // Above preferedSize must be allocated in FB pool
    pub preferred_size: libc::c_uint,
    pub preferred_size_alignment: libc::c_uint,
    // Above minSize must be allocated in FB pool
    pub min_size: libc::c_uint,
    pub min_size_alignment: libc::c_uint,
    pub flags_union: fbc_requested_compressed_size_flags_union,
}

// Supplied by bios_parser_interface.h.
#[repr(C)]
pub struct dc_context;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
