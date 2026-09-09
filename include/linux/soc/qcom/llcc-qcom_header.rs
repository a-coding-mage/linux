/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2017-2018, The Linux Foundation. All rights reserved.
 *
 */

// Dependency: Linux kernel platform-device and related types are supplied externally.

pub const LLCC_CPUSS: u32 = 1;
pub const LLCC_VIDSC0: u32 = 2;
pub const LLCC_VIDSC1: u32 = 3;
pub const LLCC_ROTATOR: u32 = 4;
pub const LLCC_VOICE: u32 = 5;
pub const LLCC_AUDIO: u32 = 6;
pub const LLCC_MDMHPGRW: u32 = 7;
pub const LLCC_MDM: u32 = 8;
pub const LLCC_MODHW: u32 = 9;
pub const LLCC_CMPT: u32 = 10;
pub const LLCC_GPUHTW: u32 = 11;
pub const LLCC_GPU: u32 = 12;
pub const LLCC_MMUHWT: u32 = 13;
pub const LLCC_CMPTDMA: u32 = 15;
pub const LLCC_DISP: u32 = 16;
pub const LLCC_VIDFW: u32 = 17;
pub const LLCC_CAMFW: u32 = 18;
pub const LLCC_MDMHPFX: u32 = 20;
pub const LLCC_MDMPNG: u32 = 21;
pub const LLCC_AUDHW: u32 = 22;
pub const LLCC_NPU: u32 = 23;
pub const LLCC_WLHW: u32 = 24;
pub const LLCC_PIMEM: u32 = 25;
pub const LLCC_ECC: u32 = 26;
pub const LLCC_CVP: u32 = 28;
pub const LLCC_MODPE: u32 = 29;
pub const LLCC_APTCM: u32 = 30;
pub const LLCC_WRCACHE: u32 = 31;
pub const LLCC_CVPFW: u32 = 32;
pub const LLCC_CPUSS1: u32 = 33;
pub const LLCC_CAMEXP0: u32 = 34;
pub const LLCC_CPUMTE: u32 = 35;
pub const LLCC_CPUHWT: u32 = 36;
pub const LLCC_MDMCLAD2: u32 = 37;
pub const LLCC_CAMEXP1: u32 = 38;
pub const LLCC_CMPTHCP: u32 = 39;
pub const LLCC_LCPDARE: u32 = 40;
pub const LLCC_AENPU: u32 = 45;
pub const LLCC_ISLAND1: u32 = 46;
pub const LLCC_ISLAND2: u32 = 47;
pub const LLCC_ISLAND3: u32 = 48;
pub const LLCC_ISLAND4: u32 = 49;
pub const LLCC_CAMEXP2: u32 = 50;
pub const LLCC_CAMEXP3: u32 = 51;
pub const LLCC_CAMEXP4: u32 = 52;
pub const LLCC_DISP_WB: u32 = 53;
pub const LLCC_DISP_1: u32 = 54;
pub const LLCC_VIEYE: u32 = 57;
pub const LLCC_VIDPTH: u32 = 58;
pub const LLCC_GPUMV: u32 = 59;
pub const LLCC_EVA_LEFT: u32 = 60;
pub const LLCC_EVA_RIGHT: u32 = 61;
pub const LLCC_EVAGAIN: u32 = 62;
pub const LLCC_VIPTH: u32 = 63;
pub const LLCC_VIDVSP: u32 = 64;
pub const LLCC_DISP_LEFT: u32 = 65;
pub const LLCC_DISP_RIGHT: u32 = 66;
pub const LLCC_EVCS_LEFT: u32 = 67;
pub const LLCC_EVCS_RIGHT: u32 = 68;
pub const LLCC_SPAD: u32 = 69;
pub const LLCC_VIDDEC: u32 = 70;
pub const LLCC_CAMOFE: u32 = 71;
pub const LLCC_CAMRTIP: u32 = 72;
pub const LLCC_CAMSRTIP: u32 = 73;
pub const LLCC_CAMRTRF: u32 = 74;
pub const LLCC_CAMSRTRF: u32 = 75;
pub const LLCC_OOBM_NS: u32 = 81;
pub const LLCC_OOBM_S: u32 = 82;
pub const LLCC_VIDEO_APV: u32 = 83;
pub const LLCC_COMPUTE1: u32 = 87;
pub const LLCC_CPUSS_OPP: u32 = 88;
pub const LLCC_CPUSSMPAM: u32 = 89;
pub const LLCC_VIDSC_VSP1: u32 = 91;
pub const LLCC_CAM_IPE_STROV: u32 = 92;
pub const LLCC_CAM_OFE_STROV: u32 = 93;
pub const LLCC_CPUSS_HEU: u32 = 94;
pub const LLCC_PCIE_TCU: u32 = 97;
pub const LLCC_MDM_PNG_FIXED: u32 = 100;

#[repr(C)]
pub struct llcc_slice_desc {
    pub slice_id: u32,
    pub uid: u32,
    pub slice_size: usize,
    pub refcount: refcount_t,
}

#[repr(C)]
pub struct llcc_edac_reg_data {
    pub name: *mut std::ffi::c_char,
    pub reg_cnt: u32,
    pub count_mask: u32,
    pub ways_mask: u32,
    pub count_shift: u8,
    pub ways_shift: u8,
}

#[repr(C)]
pub struct llcc_edac_reg_offset {
    pub trp_ecc_error_status0: u32,
    pub trp_ecc_error_status1: u32,
    pub trp_ecc_sb_err_syn0: u32,
    pub trp_ecc_db_err_syn0: u32,
    pub trp_ecc_error_cntr_clear: u32,
    pub trp_interrupt_0_status: u32,
    pub trp_interrupt_0_clear: u32,
    pub trp_interrupt_0_enable: u32,
    pub cmn_status0: u32,
    pub cmn_interrupt_0_enable: u32,
    pub cmn_interrupt_2_enable: u32,
    pub drp_ecc_error_cfg: u32,
    pub drp_ecc_error_cntr_clear: u32,
    pub drp_interrupt_status: u32,
    pub drp_interrupt_clear: u32,
    pub drp_interrupt_enable: u32,
    pub drp_ecc_error_status0: u32,
    pub drp_ecc_error_status1: u32,
    pub drp_ecc_sb_err_syn0: u32,
    pub drp_ecc_db_err_syn0: u32,
}

#[repr(C)]
pub struct llcc_drv_data {
    pub dev: *mut device,
    pub regmaps: *mut *mut regmap,
    pub bcast_regmap: *mut regmap,
    pub bcast_and_regmap: *mut regmap,
    pub cfg: *const llcc_slice_config,
    pub edac_reg_offset: *const llcc_edac_reg_offset,
    pub lock: mutex,
    pub cfg_size: u32,
    pub num_banks: u32,
    pub ecc_irq: i32,
    pub ecc_irq_configured: bool,
    pub version: u32,
    pub desc: *mut llcc_slice_desc,
}

// The following declarations are enabled when CONFIG_QCOM_LLCC is enabled.
#[cfg(feature = "qcom_llcc")]
extern "C" {
    pub fn llcc_slice_getd(uid: u32) -> *mut llcc_slice_desc;
    pub fn llcc_slice_putd(desc: *mut llcc_slice_desc);
    pub fn llcc_get_slice_id(desc: *mut llcc_slice_desc) -> i32;
    pub fn llcc_get_slice_size(desc: *mut llcc_slice_desc) -> usize;
    pub fn llcc_slice_activate(desc: *mut llcc_slice_desc) -> i32;
    pub fn llcc_slice_deactivate(desc: *mut llcc_slice_desc) -> i32;
}

// When CONFIG_QCOM_LLCC is disabled, the C header supplies these static inline fallbacks.
#[cfg(not(feature = "qcom_llcc"))]
pub unsafe fn llcc_slice_getd(_uid: u32) -> *mut llcc_slice_desc { std::ptr::null_mut() }

#[cfg(not(feature = "qcom_llcc"))]
pub unsafe fn llcc_slice_putd(_desc: *mut llcc_slice_desc) {}

#[cfg(not(feature = "qcom_llcc"))]
pub unsafe fn llcc_get_slice_id(_desc: *mut llcc_slice_desc) -> i32 { -22 }

#[cfg(not(feature = "qcom_llcc"))]
pub unsafe fn llcc_get_slice_size(_desc: *mut llcc_slice_desc) -> usize { 0 }

#[cfg(not(feature = "qcom_llcc"))]
pub unsafe fn llcc_slice_activate(_desc: *mut llcc_slice_desc) -> i32 { -22 }

#[cfg(not(feature = "qcom_llcc"))]
pub unsafe fn llcc_slice_deactivate(_desc: *mut llcc_slice_desc) -> i32 { -22 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
