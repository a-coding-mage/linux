// SPDX-License-Identifier: GPL-2.0-only
//
// aw88395_lib.h  -- ACF bin parsing and check library file for aw88395
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

pub const CHECK_REGISTER_NUM_OFFSET: u32 = 4;
pub const VALID_DATA_LEN: u32 = 4;
pub const VALID_DATA_ADDR: u32 = 4;
pub const PARSE_DSP_REG_NUM: u32 = 4;
pub const REG_DATA_BYTP_LEN: u32 = 8;
pub const CHECK_DSP_REG_NUM: u32 = 12;
pub const DSP_VALID_DATA_LEN: u32 = 12;
pub const DSP_VALID_DATA_ADDR: u32 = 12;
pub const PARSE_SOC_APP_NUM: u32 = 8;
pub const CHECK_SOC_APP_NUM: u32 = 12;
pub const APP_DOWNLOAD_ADDR: u32 = 4;
pub const APP_VALID_DATA_LEN: u32 = 12;
pub const APP_VALID_DATA_ADDR: u32 = 12;
pub const BIN_NUM_MAX: usize = 100;
pub const HEADER_LEN: u32 = 60;
pub const BIN_DATA_TYPE_OFFSET: u32 = 8;
pub const DATA_LEN: u32 = 44;
pub const VALID_DATA_ADDR_OFFSET: u32 = 60;
pub const START_ADDR_OFFSET: u32 = 64;

pub const AW88395_FW_CHECK_PART: u32 = 10;
pub const HDADER_LEN: u32 = 60;

pub const HEADER_VERSION_OFFSET: u32 = 4;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bin_header_version_enum {
    HEADER_VERSION_V1 = 0x01000000,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum data_type_enum {
    DATA_TYPE_REGISTER = 0x00000000,
    DATA_TYPE_DSP_REG = 0x00000010,
    DATA_TYPE_DSP_CFG = 0x00000011,
    DATA_TYPE_SOC_REG = 0x00000020,
    DATA_TYPE_SOC_APP = 0x00000021,
    DATA_TYPE_DSP_FW = 0x00000022,
    DATA_TYPE_MULTI_BINS = 0x00002000,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum data_version_enum {
    DATA_VERSION_V1 = 0x00000001,
    DATA_VERSION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bin_header_info {
    pub check_sum: u32,
    pub header_ver: u32,
    pub bin_data_type: u32,
    pub bin_data_ver: u32,
    pub bin_data_len: u32,
    pub ui_ver: u32,
    pub chip_type: [u8; 8],
    pub reg_byte_len: u32,
    pub data_byte_len: u32,
    pub device_addr: u32,
    pub valid_data_len: u32,
    pub valid_data_addr: u32,

    pub reg_num: u32,
    pub reg_data_byte_len: u32,
    pub download_addr: u32,
    pub app_version: u32,
    pub header_len: u32,
}

#[repr(C)]
pub struct bin_container {
    pub len: u32,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct aw_bin {
    pub p_addr: *mut u8,
    pub all_bin_parse_num: u32,
    pub multi_bin_parse_num: u32,
    pub single_bin_parse_num: u32,
    pub header_info: [bin_header_info; BIN_NUM_MAX],
    pub info: bin_container,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
