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
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
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

/* interface to PPLIB/SMU to setup clocks and pstate requirements on SoC */

#[repr(C)]
pub enum pp_smu_ver {
    PP_SMU_UNSUPPORTED,
    PP_SMU_VER_RV,
    PP_SMU_VER_NV,
    PP_SMU_VER_RN,
    PP_SMU_VER_VG,
    PP_SMU_VER_MAX,
}

#[repr(C)]
pub struct pp_smu {
    pub ver: pp_smu_ver,
    pub pp: *const core::ffi::c_void,
    /* interim extra handle for backwards compatibility as some existing
     * functionality not yet implemented by ppsmu */
    pub dm: *const core::ffi::c_void,
}

#[repr(C)]
pub enum pp_smu_status {
    PP_SMU_RESULT_UNDEFINED = 0,
    PP_SMU_RESULT_OK = 1,
    PP_SMU_RESULT_FAIL,
    PP_SMU_RESULT_UNSUPPORTED,
}

pub const PP_SMU_WM_SET_RANGE_CLK_UNCONSTRAINED_MIN: u32 = 0x0;
pub const PP_SMU_WM_SET_RANGE_CLK_UNCONSTRAINED_MAX: u32 = 0xFFFF;

#[repr(C)]
pub enum wm_type {
    WM_TYPE_PSTATE_CHG = 0,
    WM_TYPE_RETRAINING = 1,
}

#[repr(C)]
pub struct pp_smu_wm_set_range {
    pub min_fill_clk_mhz: u16,
    pub max_fill_clk_mhz: u16,
    pub min_drain_clk_mhz: u16,
    pub max_drain_clk_mhz: u16,
    pub wm_inst: u8,
    pub wm_type: u8,
}

pub const MAX_WATERMARK_SETS: usize = 4;

#[repr(C)]
pub struct pp_smu_wm_range_sets {
    pub num_reader_wm_sets: core::ffi::c_uint,
    pub reader_wm_sets: [pp_smu_wm_set_range; MAX_WATERMARK_SETS],
    pub num_writer_wm_sets: core::ffi::c_uint,
    pub writer_wm_sets: [pp_smu_wm_set_range; MAX_WATERMARK_SETS],
}

#[repr(C)]
pub struct pp_smu_funcs_rv {
    pub pp_smu: pp_smu,
    pub set_display_count: Option<unsafe extern "C" fn(*mut pp_smu, i32)>,
    pub set_wm_ranges: Option<unsafe extern "C" fn(*mut pp_smu, *mut pp_smu_wm_range_sets)>,
    pub set_hard_min_dcfclk_by_freq: Option<unsafe extern "C" fn(*mut pp_smu, i32)>,
    pub set_min_deep_sleep_dcfclk: Option<unsafe extern "C" fn(*mut pp_smu, i32)>,
    pub set_hard_min_fclk_by_freq: Option<unsafe extern "C" fn(*mut pp_smu, i32)>,
    pub set_hard_min_socclk_by_freq: Option<unsafe extern "C" fn(*mut pp_smu, i32)>,
    pub set_pme_wa_enable: Option<unsafe extern "C" fn(*mut pp_smu)>,
}

#[repr(C)]
pub enum pp_smu_nv_clock_id {
    PP_SMU_NV_DISPCLK,
    PP_SMU_NV_PHYCLK,
    PP_SMU_NV_PIXELCLK,
}

#[repr(C)]
pub struct pp_smu_nv_clock_table {
    // voltage managed SMU, freq set by driver
    pub displayClockInKhz: core::ffi::c_uint,
    pub dppClockInKhz: core::ffi::c_uint,
    pub phyClockInKhz: core::ffi::c_uint,
    pub pixelClockInKhz: core::ffi::c_uint,
    pub dscClockInKhz: core::ffi::c_uint,
    // freq/voltage managed by SMU
    pub fabricClockInKhz: core::ffi::c_uint,
    pub socClockInKhz: core::ffi::c_uint,
    pub dcfClockInKhz: core::ffi::c_uint,
    pub uClockInKhz: core::ffi::c_uint,
}

#[repr(C)]
pub struct pp_smu_funcs_nv {
    pub pp_smu: pp_smu,
    pub set_display_count: Option<unsafe extern "C" fn(*mut pp_smu, i32) -> pp_smu_status>,
    pub set_hard_min_dcfclk_by_freq: Option<unsafe extern "C" fn(*mut pp_smu, i32) -> pp_smu_status>,
    pub set_min_deep_sleep_dcfclk: Option<unsafe extern "C" fn(*mut pp_smu, i32) -> pp_smu_status>,
    pub set_hard_min_uclk_by_freq: Option<unsafe extern "C" fn(*mut pp_smu, i32) -> pp_smu_status>,
    pub set_hard_min_socclk_by_freq: Option<unsafe extern "C" fn(*mut pp_smu, i32) -> pp_smu_status>,
    pub set_pme_wa_enable: Option<unsafe extern "C" fn(*mut pp_smu) -> pp_smu_status>,
    pub set_voltage_by_freq: Option<unsafe extern "C" fn(*mut pp_smu, pp_smu_nv_clock_id, i32) -> pp_smu_status>,
    pub set_wm_ranges: Option<unsafe extern "C" fn(*mut pp_smu, *mut pp_smu_wm_range_sets) -> pp_smu_status>,
    pub get_maximum_sustainable_clocks: Option<unsafe extern "C" fn(*mut pp_smu, *mut pp_smu_nv_clock_table) -> pp_smu_status>,
    pub get_uclk_dpm_states: Option<unsafe extern "C" fn(*mut pp_smu, *mut core::ffi::c_uint, *mut core::ffi::c_uint) -> pp_smu_status>,
    pub set_pstate_handshake_support: Option<unsafe extern "C" fn(*mut pp_smu, bool) -> pp_smu_status>,
}

pub const PP_SMU_NUM_SOCCLK_DPM_LEVELS: usize = 8;
pub const PP_SMU_NUM_DCFCLK_DPM_LEVELS: usize = 8;
pub const PP_SMU_NUM_FCLK_DPM_LEVELS: usize = 4;
pub const PP_SMU_NUM_MEMCLK_DPM_LEVELS: usize = 4;
pub const PP_SMU_NUM_DCLK_DPM_LEVELS: usize = 8;
pub const PP_SMU_NUM_VCLK_DPM_LEVELS: usize = 8;
pub const PP_SMU_NUM_VPECLK_DPM_LEVELS: usize = 8;

#[repr(C)]
pub struct dpm_clock { pub Freq: u32, pub Vol: u32 }

#[repr(C)]
pub struct dpm_clocks {
    pub DcfClocks: [dpm_clock; PP_SMU_NUM_DCFCLK_DPM_LEVELS],
    pub SocClocks: [dpm_clock; PP_SMU_NUM_SOCCLK_DPM_LEVELS],
    pub FClocks: [dpm_clock; PP_SMU_NUM_FCLK_DPM_LEVELS],
    pub MemClocks: [dpm_clock; PP_SMU_NUM_MEMCLK_DPM_LEVELS],
    pub VClocks: [dpm_clock; PP_SMU_NUM_VCLK_DPM_LEVELS],
    pub DClocks: [dpm_clock; PP_SMU_NUM_DCLK_DPM_LEVELS],
    pub VPEClocks: [dpm_clock; PP_SMU_NUM_VPECLK_DPM_LEVELS],
}

#[repr(C)]
pub struct pp_smu_funcs_rn {
    pub pp_smu: pp_smu,
    pub set_wm_ranges: Option<unsafe extern "C" fn(*mut pp_smu, *mut pp_smu_wm_range_sets) -> pp_smu_status>,
    pub get_dpm_clock_table: Option<unsafe extern "C" fn(*mut pp_smu, *mut dpm_clocks) -> pp_smu_status>,
}

#[repr(C)]
pub struct pp_smu_funcs_vgh {
    pub pp_smu: pp_smu,
    // TODO: Check whether this is moved to DAL, and remove as needed
    pub set_wm_ranges: Option<unsafe extern "C" fn(*mut pp_smu, *mut pp_smu_wm_range_sets) -> pp_smu_status>,
    // TODO: Check whether this is moved to DAL, and remove as needed
    pub get_dpm_clock_table: Option<unsafe extern "C" fn(*mut pp_smu, *mut dpm_clocks) -> pp_smu_status>,
    pub notify_smu_timeout: Option<unsafe extern "C" fn(*mut pp_smu) -> pp_smu_status>,
}

#[repr(C)]
pub union pp_smu_funcs_union {
    pub rv_funcs: core::mem::ManuallyDrop<pp_smu_funcs_rv>,
    pub nv_funcs: core::mem::ManuallyDrop<pp_smu_funcs_nv>,
    pub rn_funcs: core::mem::ManuallyDrop<pp_smu_funcs_rn>,
    pub vgh_funcs: core::mem::ManuallyDrop<pp_smu_funcs_vgh>,
}

#[repr(C)]
pub struct pp_smu_funcs {
    pub ctx: pp_smu,
    pub funcs: pp_smu_funcs_union,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
