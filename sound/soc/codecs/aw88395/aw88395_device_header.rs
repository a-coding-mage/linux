// SPDX-License-Identifier: GPL-2.0-only
//
// aw88395_device.h --  AW88395 function for ALSA Audio Driver
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ushort};

// Dependencies from:
// #include "aw88395.h"
// #include "aw88395_data_type.h"
// #include "aw88395_lib.h"

pub const AW88395_DEV_DEFAULT_CH: c_int = 0;
pub const AW88395_DEV_DSP_CHECK_MAX: c_int = 5;
// #define AW88395_DSP_I2C_WRITES
pub const AW88395_MAX_RAM_WRITE_BYTE_SIZE: c_int = 128;
pub const AW88395_DSP_ODD_NUM_BIT_TEST: c_int = 0x5555;
pub const AW88395_DSP_EVEN_NUM_BIT_TEST: c_int = 0xAAAA;
pub const AW88395_DSP_ST_CHECK_MAX: c_int = 2;
pub const AW88395_FADE_IN_OUT_DEFAULT: c_int = 0;
pub const AW88395_CALI_RE_MAX: c_int = 15000;
pub const AW88395_CALI_RE_MIN: c_int = 4000;

#[inline]
pub const fn AW88395_CALI_DELAY_CACL(value: c_uint) -> c_uint {
    value.wrapping_mul(32).wrapping_div(48)
}

#[inline]
pub const fn AW88395_DSP_RE_TO_SHOW_RE(re: c_uint, shift: c_uint) -> c_uint {
    re.wrapping_mul(1000) >> shift
}

#[inline]
pub const fn AW88395_SHOW_RE_TO_DSP_RE(re: c_uint, shift: c_uint) -> c_uint {
    (re << shift).wrapping_div(1000)
}

pub const AW88395_ACF_FILE: &[u8; 16] = b"aw88395_acf.bin\0";
pub const AW88395_DEV_SYSST_CHECK_MAX: c_int = 10;

pub const AW88395_DEV_VDSEL_DAC: c_int = 0;
pub const AW88395_DEV_VDSEL_VSENSE: c_int = 1;

pub const AW88395_DSP_CRC_NA: c_int = 0;
pub const AW88395_DSP_CRC_OK: c_int = 1;

pub const AW88395_DSP_FW_UPDATE_OFF: c_int = 0;
pub const AW88395_DSP_FW_UPDATE_ON: c_int = 1;

pub const AW88395_FORCE_UPDATE_OFF: c_int = 0;
pub const AW88395_FORCE_UPDATE_ON: c_int = 1;

pub const AW88395_1000_US: c_int = 1000;
pub const AW88395_2000_US: c_int = 2000;
pub const AW88395_3000_US: c_int = 3000;
pub const AW88395_4000_US: c_int = 4000;
pub const AW88395_5000_US: c_int = 5000;
pub const AW88395_10000_US: c_int = 10000;
pub const AW88395_100000_US: c_int = 100000;

pub const AW88395_DEV_TYPE_OK: c_int = 0;
pub const AW88395_DEV_TYPE_NONE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AW88395_DEV_STATUS {
    AW88395_DEV_PW_OFF = 0,
    AW88395_DEV_PW_ON = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AW88395_DEV_FW_STATUS {
    AW88395_DEV_FW_FAILED = 0,
    AW88395_DEV_FW_OK = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AW88395_DEV_MEMCLK {
    AW88395_DEV_MEMCLK_OSC = 0,
    AW88395_DEV_MEMCLK_PLL = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AW88395_DEV_DSP_CFG {
    AW88395_DEV_DSP_WORK = 0,
    AW88395_DEV_DSP_BYPASS = 1,
}

pub const AW_DSP_16_DATA: c_int = 0;
pub const AW_DSP_32_DATA: c_int = 1;

pub const AW88395_NOT_RCV_MODE: c_int = 0;
pub const AW88395_RCV_MODE: c_int = 1;

#[repr(C)]
pub struct aw_profctrl_desc {
    pub cur_mode: c_uint,
}

pub const CALI_RESULT_NORMAL: c_int = 0;
pub const CALI_RESULT_ERROR: c_int = 1;

#[repr(C)]
pub struct aw_volume_desc {
    pub init_volume: c_uint,
    pub mute_volume: c_uint,
    pub ctl_volume: c_uint,
    pub max_volume: c_uint,
}

#[repr(C)]
pub struct aw_dsp_mem_desc {
    pub dsp_madd_reg: c_uint,
    pub dsp_mdat_reg: c_uint,
    pub dsp_fw_base_addr: c_uint,
    pub dsp_cfg_base_addr: c_uint,
}

#[repr(C)]
pub struct aw_vmax_desc {
    pub init_vmax: c_uint,
}

#[repr(C)]
pub struct aw_cali_delay_desc {
    pub delay: c_uint,
}

pub const AW_CALI_CFG_NUM: usize = 4;

#[repr(C)]
pub struct cali_cfg {
    pub data: [u32; AW_CALI_CFG_NUM],
}

#[repr(C)]
pub struct aw_cali_backup_desc {
    pub dsp_ng_cfg: c_uint,
    pub dsp_lp_cfg: c_uint,
}

#[repr(C)]
pub struct aw_cali_desc {
    pub cali_re: u32,
    pub ra: u32,
    pub cali_switch: bool,
    pub cali_running: bool,
    pub cali_result: u16,
    pub store_vol: u16,
    pub cali_cfg: cali_cfg,
    pub backup_info: aw_cali_backup_desc,
}

#[repr(C)]
pub struct aw_container {
    pub len: c_int,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct aw_device {
    pub status: c_int,
    pub dsp_lock: mutex,

    pub prof_cur: c_uchar,
    pub prof_index: c_uchar,
    pub dsp_crc_st: c_uchar,
    pub dsp_cfg: c_uchar,
    pub chip_id: u16,

    pub channel: c_uint,
    pub fade_step: c_uint,
    pub prof_data_type: c_uint,

    pub i2c: *mut i2c_client,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub acf: *mut c_char,

    pub dsp_fw_len: u32,
    pub dsp_cfg_len: u32,
    pub platform: u8,
    pub fw_status: u8,

    pub fade_in_time: c_uint,
    pub fade_out_time: c_uint,

    pub prof_info: aw_prof_info,
    pub crc_dsp_cfg: aw_sec_data_desc,
    pub profctrl_desc: aw_profctrl_desc,
    pub volume_desc: aw_volume_desc,
    pub dsp_mem_desc: aw_dsp_mem_desc,
    pub vmax_desc: aw_vmax_desc,

    pub cali_delay_desc: aw_cali_delay_desc,
    pub cali_desc: aw_cali_desc,
}

unsafe extern "C" {
    pub fn aw88395_init(
        aw_dev: *mut *mut aw_device,
        i2c: *mut i2c_client,
        regmap: *mut regmap,
    ) -> c_int;
    pub fn aw88395_dev_init(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    pub fn aw88395_dev_start(aw_dev: *mut aw_device) -> c_int;
    pub fn aw88395_dev_stop(aw_dev: *mut aw_device) -> c_int;
    pub fn aw88395_dev_fw_update(
        aw_dev: *mut aw_device,
        up_dsp_fw_en: bool,
        force_up_en: bool,
    ) -> c_int;

    pub fn aw88395_dev_set_volume(aw_dev: *mut aw_device, set_vol: c_ushort);
    pub fn aw88395_dev_get_prof_data(
        aw_dev: *mut aw_device,
        index: c_int,
        prof_desc: *mut *mut aw_prof_desc,
    ) -> c_int;
    pub fn aw88395_dev_get_prof_name(
        aw_dev: *mut aw_device,
        index: c_int,
        prof_name: *mut *mut c_char,
    ) -> c_int;
    pub fn aw88395_dev_set_profile_index(aw_dev: *mut aw_device, index: c_int) -> c_int;
    pub fn aw88395_dev_get_profile_index(aw_dev: *mut aw_device) -> c_int;
    pub fn aw88395_dev_get_profile_count(aw_dev: *mut aw_device) -> c_int;
    pub fn aw88395_dev_load_acf_check(
        aw_dev: *mut aw_device,
        aw_cfg: *mut aw_container,
    ) -> c_int;
    pub fn aw88395_dev_cfg_load(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    pub fn aw88395_dev_mute(aw_dev: *mut aw_device, is_mute: bool);
    pub fn aw_dev_dsp_write(
        aw_dev: *mut aw_device,
        dsp_addr: c_ushort,
        dsp_data: c_uint,
        data_type: c_uchar,
    ) -> c_int;
    pub fn aw_dev_dsp_read(
        aw_dev: *mut aw_device,
        dsp_addr: c_ushort,
        dsp_data: *mut c_uint,
        data_type: c_uchar,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
