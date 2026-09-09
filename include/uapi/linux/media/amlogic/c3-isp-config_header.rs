/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/* Copyright (C) 2024 Amlogic, Inc. All rights reserved */

// C dependencies: linux/types.h and linux/media/v4l2-isp.h.
// The v4l2 symbols below are supplied by the surrounding translation unit.

pub const C3_ISP_AE_MAX_ZONES: usize = 17 * 15;
pub const C3_ISP_AF_MAX_ZONES: usize = 17 * 15;
pub const C3_ISP_AWB_MAX_ZONES: usize = 32 * 24;

pub const C3_ISP_AE_MAX_PT_NUM: usize = 18;
pub const C3_ISP_AF_MAX_PT_NUM: usize = 18;
pub const C3_ISP_AWB_MAX_PT_NUM: usize = 33;

#[repr(C)]
pub struct c3_isp_awb_zone_stats {
    pub rg: u16,
    pub bg: u16,
    pub pixel_sum: u32,
}

#[repr(C, align(16))]
pub struct c3_isp_awb_stats {
    pub stats: [c3_isp_awb_zone_stats; C3_ISP_AWB_MAX_ZONES],
}

#[repr(C)]
pub struct c3_isp_ae_zone_stats {
    pub hist0: u16,
    pub hist1: u16,
    pub hist3: u16,
    pub hist4: u16,
}

#[repr(C, align(16))]
pub struct c3_isp_ae_stats {
    pub stats: [c3_isp_ae_zone_stats; C3_ISP_AE_MAX_ZONES],
    pub reserved: [u32; 2],
    pub hist: [u32; 1024],
}

#[repr(C)]
pub struct c3_isp_af_zone_stats {
    pub i2_mat: u16,
    pub i4_mat: u16,
    pub e4_mat: u16,
    // C bitfields: e4_exp:5, i2_exp:5, i4_exp:6.
    pub exponents: u16,
}

#[repr(C, align(16))]
pub struct c3_isp_af_stats {
    pub stats: [c3_isp_af_zone_stats; C3_ISP_AF_MAX_ZONES],
    pub reserved: [u32; 2],
}

#[repr(C)]
pub struct c3_isp_stats_info {
    pub awb: c3_isp_awb_stats,
    pub ae: c3_isp_ae_stats,
    pub af: c3_isp_af_stats,
}

#[repr(i32)]
pub enum c3_isp_params_buffer_version {
    C3_ISP_PARAMS_BUFFER_V0 = V4L2_ISP_PARAMS_VERSION_V0,
}

#[repr(i32)]
pub enum c3_isp_params_block_type {
    C3_ISP_PARAMS_BLOCK_AWB_GAINS,
    C3_ISP_PARAMS_BLOCK_AWB_CONFIG,
    C3_ISP_PARAMS_BLOCK_AE_CONFIG,
    C3_ISP_PARAMS_BLOCK_AF_CONFIG,
    C3_ISP_PARAMS_BLOCK_PST_GAMMA,
    C3_ISP_PARAMS_BLOCK_CCM,
    C3_ISP_PARAMS_BLOCK_CSC,
    C3_ISP_PARAMS_BLOCK_BLC,
    C3_ISP_PARAMS_BLOCK_SENTINEL,
}

pub const C3_ISP_PARAMS_BLOCK_FL_DISABLE: u32 = V4L2_ISP_PARAMS_FL_BLOCK_DISABLE;
pub const C3_ISP_PARAMS_BLOCK_FL_ENABLE: u32 = V4L2_ISP_PARAMS_FL_BLOCK_ENABLE;

pub type c3_isp_params_block_header = v4l2_isp_params_block_header;

#[repr(C, align(8))]
pub struct c3_isp_params_awb_gains {
    pub header: c3_isp_params_block_header,
    pub gr_gain: u16,
    pub r_gain: u16,
    pub b_gain: u16,
    pub gb_gain: u16,
}

#[repr(i32)]
pub enum c3_isp_params_awb_tap_points {
    C3_ISP_AWB_STATS_TAP_OFE = 0,
    C3_ISP_AWB_STATS_TAP_GE,
    C3_ISP_AWB_STATS_TAP_BEFORE_WB,
    C3_ISP_AWB_STATS_TAP_AFTER_WB,
}

#[repr(C, align(8))]
pub struct c3_isp_params_awb_config {
    pub header: c3_isp_params_block_header,
    pub tap_point: u8,
    pub satur_vald: u8,
    pub horiz_zones_num: u8,
    pub vert_zones_num: u8,
    pub rg_min: u16,
    pub rg_max: u16,
    pub bg_min: u16,
    pub bg_max: u16,
    pub rg_low: u16,
    pub rg_high: u16,
    pub bg_low: u16,
    pub bg_high: u16,
    pub zone_weight: [u8; C3_ISP_AWB_MAX_ZONES],
    pub horiz_coord: [u16; C3_ISP_AWB_MAX_PT_NUM],
    pub vert_coord: [u16; C3_ISP_AWB_MAX_PT_NUM],
}

#[repr(i32)]
pub enum c3_isp_params_ae_tap_points {
    C3_ISP_AE_STATS_TAP_GE = 0,
    C3_ISP_AE_STATS_TAP_MLS,
}

#[repr(C, align(8))]
pub struct c3_isp_params_ae_config {
    pub header: c3_isp_params_block_header,
    pub tap_point: u8,
    pub horiz_zones_num: u8,
    pub vert_zones_num: u8,
    pub zone_weight: [u8; C3_ISP_AE_MAX_ZONES],
    pub horiz_coord: [u16; C3_ISP_AE_MAX_PT_NUM],
    pub vert_coord: [u16; C3_ISP_AE_MAX_PT_NUM],
    pub reserved: [u16; 3],
}

#[repr(i32)]
pub enum c3_isp_params_af_tap_points {
    C3_ISP_AF_STATS_TAP_SNR = 0,
    C3_ISP_AF_STATS_TAP_DMS,
}

#[repr(C, align(8))]
pub struct c3_isp_params_af_config {
    pub header: c3_isp_params_block_header,
    pub tap_point: u8,
    pub horiz_zones_num: u8,
    pub vert_zones_num: u8,
    pub reserved: [u8; 5],
    pub horiz_coord: [u16; C3_ISP_AF_MAX_PT_NUM],
    pub vert_coord: [u16; C3_ISP_AF_MAX_PT_NUM],
}

#[repr(C, align(8))]
pub struct c3_isp_params_pst_gamma {
    pub header: c3_isp_params_block_header,
    pub lut: [u16; 129],
    pub reserved: [u16; 3],
}

#[repr(C, align(8))]
pub struct c3_isp_params_ccm {
    pub header: c3_isp_params_block_header,
    pub matrix: [[i16; 3]; 3],
    pub reserved: [u16; 3],
}

#[repr(C, align(8))]
pub struct c3_isp_params_csc {
    pub header: c3_isp_params_block_header,
    pub matrix: [[i16; 3]; 3],
    pub reserved: [u16; 3],
}

#[repr(C)]
pub struct c3_isp_params_blc {
    pub header: c3_isp_params_block_header,
    pub gr_ofst: u16,
    pub r_ofst: u16,
    pub b_ofst: u16,
    pub gb_ofst: u16,
}

pub const C3_ISP_PARAMS_MAX_SIZE: usize =
    core::mem::size_of::<c3_isp_params_awb_gains>()
    + core::mem::size_of::<c3_isp_params_awb_config>()
    + core::mem::size_of::<c3_isp_params_ae_config>()
    + core::mem::size_of::<c3_isp_params_af_config>()
    + core::mem::size_of::<c3_isp_params_pst_gamma>()
    + core::mem::size_of::<c3_isp_params_ccm>()
    + core::mem::size_of::<c3_isp_params_csc>()
    + core::mem::size_of::<c3_isp_params_blc>();

#[repr(C)]
pub struct c3_isp_params_cfg {
    pub version: u32,
    pub data_size: u32,
    pub data: [u8; C3_ISP_PARAMS_MAX_SIZE],
}

// Under __KERNEL__, C performs a static assertion that the leading header is
// type-convertible to struct v4l2_isp_params_buffer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
