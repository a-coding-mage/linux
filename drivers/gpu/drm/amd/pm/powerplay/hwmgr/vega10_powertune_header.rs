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
 */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum vega10_pt_config_reg_type {
    VEGA10_CONFIGREG_MMR = 0,
    VEGA10_CONFIGREG_SMC_IND,
    VEGA10_CONFIGREG_DIDT_IND,
    VEGA10_CONFIGREG_CACHE,
    VEGA10_CONFIGREG_MAX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum vega10_didt_config_reg_type {
    VEGA10_CONFIGREG_DIDT = 0,
    VEGA10_CONFIGREG_GCCAC,
    VEGA10_CONFIGREG_SECAC,
}

/* PowerContainment Features */
pub const POWERCONTAINMENT_FEATURE_DTE: u32 = 0x00000001;
pub const POWERCONTAINMENT_FEATURE_TDCLimit: u32 = 0x00000002;
pub const POWERCONTAINMENT_FEATURE_PkgPwrLimit: u32 = 0x00000004;

#[repr(C)]
pub struct vega10_pt_config_reg {
    pub offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub value: u32,
    pub type_: vega10_pt_config_reg_type,
}

#[repr(C)]
pub struct vega10_didt_config_reg {
    pub offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub value: u32,
}

#[repr(C)]
pub struct vega10_pt_defaults {
    pub SviLoadLineEn: u8,
    pub SviLoadLineVddC: u8,
    pub TDC_VDDC_ThrottleReleaseLimitPerc: u8,
    pub TDC_MAWt: u8,
    pub TdcWaterfallCtl: u8,
    pub DTEAmbientTempBase: u8,
}

/* External type supplied by another translation unit. */
pub enum pp_hwmgr {}

extern "C" {
    pub fn vega10_initialize_power_tune_defaults(hwmgr: *mut pp_hwmgr);
    pub fn vega10_populate_bapm_parameters_in_dpm_table(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega10_populate_pm_fuses(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega10_enable_smc_cac(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega10_enable_power_containment(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega10_set_power_limit(hwmgr: *mut pp_hwmgr, n: u32) -> i32;
    pub fn vega10_power_control_set_level(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega10_disable_power_containment(hwmgr: *mut pp_hwmgr) -> i32;

    pub fn vega10_enable_didt_config(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega10_disable_didt_config(hwmgr: *mut pp_hwmgr) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
