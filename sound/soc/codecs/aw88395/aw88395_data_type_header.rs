// SPDX-License-Identifier: GPL-2.0-only
//
// aw883_data_type.h --  The data type of the AW88395 chip
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

use core::ffi::c_char;

pub const PROJECT_NAME_MAX: usize = 24;
pub const CUSTOMER_NAME_MAX: usize = 16;
pub const CFG_VERSION_MAX: usize = 4;
pub const DEV_NAME_MAX: usize = 16;
pub const PROFILE_STR_MAX: usize = 32;

pub const ACF_FILE_ID: u32 = 0x0a15f908;

pub const AW88395_CFG_HDR_VER: u32 = 0x00000001;
pub const AW88395_CFG_HDR_VER_V1: u32 = 0x01000000;

pub const AW88395_DEV_NONE_TYPE_ID: u32 = 0xffffffff;
pub const AW88395_DEV_TYPE_ID: u32 = 0x00000000;
pub const AW88395_SKT_TYPE_ID: u32 = 0x00000001;
pub const AW88395_DEV_DEFAULT_TYPE_ID: u32 = 0x00000002;

pub const ACF_SEC_TYPE_REG: u32 = 0;
pub const ACF_SEC_TYPE_DSP: u32 = 1;
pub const ACF_SEC_TYPE_DSP_CFG: u32 = 2;
pub const ACF_SEC_TYPE_DSP_FW: u32 = 3;
pub const ACF_SEC_TYPE_HDR_REG: u32 = 4;
pub const ACF_SEC_TYPE_HDR_DSP_CFG: u32 = 5;
pub const ACF_SEC_TYPE_HDR_DSP_FW: u32 = 6;
pub const ACF_SEC_TYPE_MULTIPLE_BIN: u32 = 7;
pub const ACF_SEC_TYPE_SKT_PROJECT: u32 = 8;
pub const ACF_SEC_TYPE_DSP_PROJECT: u32 = 9;
pub const ACF_SEC_TYPE_MONITOR: u32 = 10;
pub const ACF_SEC_TYPE_MAX: u32 = 11;

pub const AW88395_DATA_TYPE_REG: u32 = 0;
pub const AW88395_DATA_TYPE_DSP_CFG: u32 = 1;
pub const AW88395_DATA_TYPE_DSP_FW: u32 = 2;
pub const AW88395_DATA_TYPE_MAX: usize = 3;

pub const AW88395_PROFILE_MUSIC: u32 = 0;
pub const AW88395_PROFILE_VOICE: u32 = 1;
pub const AW88395_PROFILE_VOIP: u32 = 2;
pub const AW88395_PROFILE_RINGTONE: u32 = 3;
pub const AW88395_PROFILE_RINGTONE_HS: u32 = 4;
pub const AW88395_PROFILE_LOWPOWER: u32 = 5;
pub const AW88395_PROFILE_BYPASS: u32 = 6;
pub const AW88395_PROFILE_MMI: u32 = 7;
pub const AW88395_PROFILE_FM: u32 = 8;
pub const AW88395_PROFILE_NOTIFICATION: u32 = 9;
pub const AW88395_PROFILE_RECEIVER: u32 = 10;
pub const AW88395_PROFILE_MAX: usize = 11;

pub const AW88395_PROFILE_WAIT: u32 = 0;
pub const AW88395_PROFILE_OK: u32 = 1;

#[repr(C)]
pub struct aw_cfg_hdr {
    pub id: u32,
    pub project: [c_char; PROJECT_NAME_MAX],
    pub custom: [c_char; CUSTOMER_NAME_MAX],
    pub version: [c_char; CFG_VERSION_MAX],
    pub author_id: u32,
    pub ddt_size: u32,
    pub ddt_num: u32,
    pub hdr_offset: u32,
    pub hdr_version: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
pub struct aw_cfg_dde {
    pub type_: u32,
    pub dev_name: [c_char; DEV_NAME_MAX],
    pub dev_index: u16,
    pub dev_bus: u16,
    pub dev_addr: u16,
    pub dev_profile: u16,
    pub data_type: u32,
    pub data_size: u32,
    pub data_offset: u32,
    pub data_crc: u32,
    pub reserved: [u32; 5],
}

#[repr(C)]
pub struct aw_cfg_dde_v1 {
    pub type_: u32,
    pub dev_name: [c_char; DEV_NAME_MAX],
    pub dev_index: u16,
    pub dev_bus: u16,
    pub dev_addr: u16,
    pub dev_profile: u16,
    pub data_type: u32,
    pub data_size: u32,
    pub data_offset: u32,
    pub data_crc: u32,
    pub dev_profile_str: [c_char; PROFILE_STR_MAX],
    pub chip_id: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
pub struct aw_sec_data_desc {
    pub len: u32,
    pub data: *mut u8,
}

#[repr(C)]
pub struct aw_prof_desc {
    pub id: u32,
    pub prof_st: u32,
    pub prf_str: *mut c_char,
    pub fw_ver: u32,
    pub sec_desc: [aw_sec_data_desc; AW88395_DATA_TYPE_MAX],
}

#[repr(C)]
pub struct aw_all_prof_info {
    pub prof_desc: [aw_prof_desc; AW88395_PROFILE_MAX],
}

#[repr(C)]
pub struct aw_prof_info {
    pub count: i32,
    pub prof_type: i32,
    pub prof_name_list: *mut *mut c_char,
    pub prof_desc: *mut aw_prof_desc,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
