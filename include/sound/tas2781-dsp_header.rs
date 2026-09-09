/* SPDX-License-Identifier: GPL-2.0 */
//
// ALSA SoC Texas Instruments TAS2781 Audio Smart Amplifier
//
// Copyright (C) 2022 - 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2781 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// TAS2781 chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
// Author: Kevin Lu <kevin-lu@ti.com>

pub const MAIN_ALL_DEVICES: u32 = 0x0d;
pub const MAIN_DEVICE_A: u32 = 0x01;
pub const MAIN_DEVICE_B: u32 = 0x08;
pub const MAIN_DEVICE_C: u32 = 0x10;
pub const MAIN_DEVICE_D: u32 = 0x14;
pub const COEFF_DEVICE_A: u32 = 0x03;
pub const COEFF_DEVICE_B: u32 = 0x0a;
pub const COEFF_DEVICE_C: u32 = 0x11;
pub const COEFF_DEVICE_D: u32 = 0x15;
pub const PRE_DEVICE_A: u32 = 0x04;
pub const PRE_DEVICE_B: u32 = 0x0b;
pub const PRE_DEVICE_C: u32 = 0x12;
pub const PRE_DEVICE_D: u32 = 0x16;

pub const PPC3_VERSION_BASE: u32 = 0x4100;
pub const PPC3_VERSION_TAS2781_BASIC_MIN: u32 = 0x14600;
pub const PPC3_VERSION_TAS2781_ALPHA_MIN: u32 = 0x4a00;
pub const PPC3_VERSION_TAS2781_BETA_MIN: u32 = 0x19400;
pub const PPC3_VERSION_TAS5825_BASE: u32 = 0x114200;
pub const TASDEVICE_DEVICE_SUM: u32 = 8;
pub const TASDEVICE_CONFIG_SUM: u32 = 64;
pub const TASDEVICE_MAX_CHANNELS: u32 = 8;

#[repr(i32)]
pub enum tasdevice_dsp_dev_idx {
    TASDEVICE_DSP_TAS_2555 = 0,
    TASDEVICE_DSP_TAS_2555_STEREO,
    TASDEVICE_DSP_TAS_2557_MONO,
    TASDEVICE_DSP_TAS_2557_DUAL_MONO,
    TASDEVICE_DSP_TAS_2559,
    TASDEVICE_DSP_TAS_2563,
    TASDEVICE_DSP_TAS_2563_DUAL_MONO = 7,
    TASDEVICE_DSP_TAS_2563_QUAD,
    TASDEVICE_DSP_TAS_2563_21,
    TASDEVICE_DSP_TAS_2781,
    TASDEVICE_DSP_TAS_2781_DUAL_MONO,
    TASDEVICE_DSP_TAS_2781_21,
    TASDEVICE_DSP_TAS_2781_QUAD,
    TASDEVICE_DSP_TAS_5825_MONO,
    TASDEVICE_DSP_TAS_5825_DUAL,
    TASDEVICE_DSP_TAS_MAX_DEVICE,
}

#[repr(C)]
pub struct tasdevice_fw_fixed_hdr { pub fwsize: u32, pub ppcver: u32, pub drv_ver: u32 }
#[repr(C)]
pub struct tasdevice_dspfw_hdr { pub fixed_hdr: tasdevice_fw_fixed_hdr, pub device_family: u16, pub device: u16, pub ndev: u8 }
#[repr(C)]
pub struct tasdev_blk {
    pub nr_retry: i32, pub type_: u32, pub is_pchksum_present: u8, pub pchksum: u8,
    pub is_ychksum_present: u8, pub ychksum: u8, pub nr_cmds: u32, pub blk_size: u32,
    pub nr_subblocks: u32, pub dev_idx: u8, pub data: *mut u8,
}
#[repr(C)]
pub struct tasdevice_data { pub name: [i8; 64], pub nr_blk: u32, pub dev_blks: *mut tasdev_blk }
#[repr(C)]
pub struct tasdevice_prog { pub prog_size: u32, pub dev_data: tasdevice_data }
#[repr(C)]
pub struct tasdevice_config { pub cfg_size: u32, pub name: [i8; 64], pub dev_data: tasdevice_data }
#[repr(C)]
pub struct tasdevice_calibration { pub dev_data: tasdevice_data }
#[repr(C)]
pub struct fct_param_address {
    pub thr: [u8; 3], pub thr2: [u8; 3], pub plt_flg: [u8; 3], pub sin_gn: [u8; 3],
    pub sin_gn2: [u8; 3], pub r0_reg: [u8; 3], pub tf_reg: [u8; 3],
    pub a1_reg: [u8; 3], pub a2_reg: [u8; 3],
}
#[repr(C)]
pub struct tasdevice_fw {
    pub fw_hdr: tasdevice_dspfw_hdr, pub nr_programs: u16, pub programs: *mut tasdevice_prog,
    pub nr_configurations: u16, pub configs: *mut tasdevice_config, pub nr_calibrations: u16,
    pub calibrations: *mut tasdevice_calibration, pub fct_par_addr: fct_param_address,
    pub dev: *mut device,
}

#[repr(i32)]
pub enum tasdevice_fw_state { TASDEVICE_DSP_FW_PENDING, TASDEVICE_DSP_FW_FAIL, TASDEVICE_RCA_FW_OK, TASDEVICE_DSP_FW_ALL_OK }
#[repr(i32)]
pub enum tasdevice_bin_blk_type { TASDEVICE_BIN_BLK_COEFF = 1, TASDEVICE_BIN_BLK_POST_POWER_UP, TASDEVICE_BIN_BLK_PRE_SHUTDOWN, TASDEVICE_BIN_BLK_PRE_POWER_UP, TASDEVICE_BIN_BLK_POST_SHUTDOWN }

#[repr(C)]
pub struct tasdevice_rca_hdr { pub img_sz: u32, pub checksum: u32, pub binary_version_num: u32, pub drv_fw_version: u32, pub plat_type: u8, pub dev_family: u8, pub reserve: u8, pub ndev: u8, pub devs: [u8; 8], pub nconfig: u32, pub config_size: [u32; 64] }
#[repr(C)]
pub struct tasdev_blk_data { pub dev_idx: u8, pub block_type: u8, pub yram_checksum: u16, pub block_size: u32, pub n_subblks: u32, pub regdata: *mut u8 }
#[repr(C)]
pub struct tasdevice_config_info { pub nblocks: u32, pub real_nblocks: u32, pub active_dev: u8, pub blk_data: *mut *mut tasdev_blk_data }
#[repr(C)]
pub struct tasdevice_rca { pub fw_hdr: tasdevice_rca_hdr, pub ncfgs: i32, pub cfg_info: *mut *mut tasdevice_config_info, pub profile_cfg_id: i32, pub capture_profile_id: i32, pub init_profile_id: i32 }

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct firmware { _private: [u8; 0] }

extern "C" {
    pub fn tasdevice_select_cfg_blk(context: *mut core::ffi::c_void, conf_no: i32, block_type: u8);
    pub fn tasdevice_config_info_remove(context: *mut core::ffi::c_void);
    pub fn tasdevice_dsp_remove(context: *mut core::ffi::c_void);
    pub fn tasdevice_dsp_parser(context: *mut core::ffi::c_void) -> i32;
    pub fn tasdevice_rca_parser(context: *mut core::ffi::c_void, fmw: *const firmware) -> i32;
    pub fn tasdevice_calbin_remove(context: *mut core::ffi::c_void);
    pub fn tasdevice_select_tuningprm_cfg(context: *mut core::ffi::c_void, prm: i32, cfg_no: i32, rca_conf_no: i32) -> i32;
    pub fn tasdevice_prmg_load(context: *mut core::ffi::c_void, prm_no: i32) -> i32;
    pub fn tasdevice_tuning_switch(context: *mut core::ffi::c_void, state: i32, is_cap: bool);
    pub fn tas2781_load_calibration(context: *mut core::ffi::c_void, file_name: *mut i8, i: u16) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
