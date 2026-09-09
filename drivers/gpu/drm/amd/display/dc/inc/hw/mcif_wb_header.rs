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
// dc_hw_types.h and dml2_0/dml21/inc/dml_top_dchub_registers.h

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mmhubbub_wbif_mode {
    PACKED_444 = 0,
    PACKED_444_FP16 = 1,
    PLANAR_420_8BPC = 2,
    PLANAR_420_10BPC = 3,
}

#[repr(C)]
pub struct mcif_arb_params_legacy {
    pub time_per_pixel: ::core::ffi::c_uint,
    pub cli_watermark: [::core::ffi::c_uint; 4],
    pub pstate_watermark: [::core::ffi::c_uint; 4],
    pub arbitration_slice: ::core::ffi::c_uint,
    pub slice_lines: ::core::ffi::c_uint,
    pub max_scaled_time: ::core::ffi::c_uint,
    pub dram_speed_change_duration: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct mcif_arb_params_dcn4x {
    pub global_regs: dml2_mcif_global_register_set,
    pub inst_regs: dml2_mcif_per_pipe_register_set,
}

#[repr(C)]
pub union mcif_arb_params_union {
    pub legacy: ::core::mem::ManuallyDrop<mcif_arb_params_legacy>,
    pub dcn4x: ::core::mem::ManuallyDrop<mcif_arb_params_dcn4x>,
}

#[repr(C)]
pub struct mcif_arb_params {
    pub data: mcif_arb_params_union,
}

#[repr(C)]
pub struct mcif_irq_params {
    pub sw_int_en: ::core::ffi::c_uint,
    pub sw_slice_int_en: ::core::ffi::c_uint,
    pub sw_overrun_int_en: ::core::ffi::c_uint,
    pub vce_int_en: ::core::ffi::c_uint,
    pub vce_slice_int_en: ::core::ffi::c_uint,
}

/* / - mcif_wb_frame_dump_info is the info of the dumping WB data */
#[repr(C)]
pub struct mcif_wb_frame_dump_info {
    pub size: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub luma_pitch: ::core::ffi::c_uint,
    pub chroma_pitch: ::core::ffi::c_uint,
    pub format: dwb_scaler_mode,
}

#[repr(C)]
pub struct mcif_wb {
    pub funcs: *const mcif_wb_funcs,
    pub ctx: *mut dc_context,
    pub inst: ::core::ffi::c_int,
}

#[repr(C)]
pub struct mcif_wb_funcs {
    pub warmup_mcif: Option<unsafe extern "C" fn(*mut mcif_wb, *mut mcif_warmup_params)>,
    pub enable_mcif: Option<unsafe extern "C" fn(*mut mcif_wb)>,
    pub disable_mcif: Option<unsafe extern "C" fn(*mut mcif_wb)>,
    pub config_mcif_buf: Option<unsafe extern "C" fn(*mut mcif_wb, *mut mcif_buf_params, ::core::ffi::c_uint)>,
    pub config_mcif_arb: Option<unsafe extern "C" fn(*mut mcif_wb, *mut mcif_arb_params)>,
    pub config_mcif_irq: Option<unsafe extern "C" fn(*mut mcif_wb, *mut mcif_irq_params)>,
    pub dump_frame: Option<unsafe extern "C" fn(
        *mut mcif_wb,
        *mut mcif_buf_params,
        dwb_scaler_mode,
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
        *mut mcif_wb_frame_dump_info,
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
    )>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
