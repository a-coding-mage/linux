/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Dreamchip RPP-X1 ISP Driver - Userspace API
 *
 * Copyright (C) 2026 Renesas Electronics Corp.
 * Copyright (C) 2026 Ideas on Board Oy
 * Copyright (C) 2026 Ragnatech AB
 */

/* Dependency: <linux/media/v4l2-isp.h> */

/** Measurement window. */
#[repr(C)]
pub struct rppx1_window {
    pub h_offs: u16,
    pub v_offs: u16,
    pub h_size: u16,
    pub v_size: u16,
}

#[repr(i32)]
pub enum rppx1_meas_chan {
    RPPX1_MEAS_CHAN_SEL0,
    RPPX1_MEAS_CHAN_SEL1,
    RPPX1_MEAS_CHAN_SEL2,
    RPPX1_MEAS_CHAN_SEL3,
    RPPX1_MEAS_CHAN_SEL4,
    RPPX1_MEAS_CHAN_SEL5,
    RPPX1_MEAS_CHAN_SEL6,
    RPPX1_MEAS_CHAN_SEL7,
}

#[repr(i32)]
pub enum rppx1_params_block_type {
    RPPX1_PARAMS_BLOCK_TYPE_WBMEAS_POST,
    RPPX1_PARAMS_BLOCK_TYPE_AWBG_PRE1,
    RPPX1_PARAMS_BLOCK_TYPE_AWBG_PRE2,
    RPPX1_PARAMS_BLOCK_TYPE_AWBG_POST,
    RPPX1_PARAMS_BLOCK_TYPE_EXM_PRE1,
    RPPX1_PARAMS_BLOCK_TYPE_EXM_PRE2,
    RPPX1_PARAMS_BLOCK_TYPE_HIST_PRE1,
    RPPX1_PARAMS_BLOCK_TYPE_HIST_PRE2,
    RPPX1_PARAMS_BLOCK_TYPE_HIST_POST,
    RPPX1_PARAMS_BLOCK_TYPE_BLS_PRE1,
    RPPX1_PARAMS_BLOCK_TYPE_BLS_PRE2,
    RPPX1_PARAMS_BLOCK_TYPE_CCOR_POST,
    RPPX1_PARAMS_BLOCK_TYPE_LSC_PRE1,
    RPPX1_PARAMS_BLOCK_TYPE_LSC_PRE2,
    RPPX1_PARAMS_BLOCK_TYPE_GA_HV,
    RPPX1_PARAMS_BLOCK_TYPE_GA_MV,
    RPPX1_PARAMS_BLOCK_TYPE_LIN_PRE1,
    RPPX1_PARAMS_BLOCK_TYPE_LIN_PRE2,
}

#[repr(i32)]
pub enum rppx1_wbmeas_mode { RPPX1_WBMEAS_MODE_YCBCR, RPPX1_WBMEAS_MODE_RGB }

#[repr(C)]
pub struct rppx1_wbmeas_params {
    pub header: v4l2_isp_params_block_header,
    pub wnd: rppx1_window,
    pub mode: u8,
    pub ymax_cmp: u8,
    pub frames: u8,
    pub reserved: u8,
    pub ref_cr_max_r: u32,
    pub ref_cb_max_b: u32,
    pub min_y_max_g: u32,
    pub max_y: u32,
    pub max_csum: u32,
    pub min_c: u32,
    pub ccor_coeff: [[u16; 3]; 3],
    pub reserved2: u16,
    pub ccor_offs: [u32; 3],
    pub reserved3: u32,
}

#[repr(C)]
pub struct rppx1_awbg_params {
    pub header: v4l2_isp_params_block_header,
    pub gain_red: u32,
    pub gain_green_r: u32,
    pub gain_blue: u32,
    pub gain_green_b: u32,
}

#[repr(i32)]
pub enum rppx1_exm_mode {
    RPPX1_EXP_MEASURING_MODE_DISABLED,
    RPPX1_EXP_MEASURING_MODE_RGB,
    RPPX1_EXP_MEASURING_MODE_BAYER,
}

#[repr(C)]
pub struct rppx1_exm_params {
    pub header: v4l2_isp_params_block_header,
    pub wnd: rppx1_window,
    pub mode: u32,
    pub last_line: u32,
    pub channel_sel: u8,
    pub coeff_r: u8,
    pub coeff_g_gr: u8,
    pub coeff_b: u8,
    pub coeff_gb: u8,
    pub reserved: [u8; 3],
}

pub const RPPX1_HIST_WEIGHT_GRIDS_SIZE: usize = 25;

#[repr(i32)]
pub enum rppx1_hist_mode {
    RPPX1_HIST_MODE_DISABLE,
    RPPX1_HIST_MODE_RGB_COMBINED,
    RPPX1_HIST_MODE_R_HISTOGRAM,
    RPPX1_HIST_MODE_GR_HISTOGRAM,
    RPPX1_HIST_MODE_B_HISTOGRAM,
    RPPX1_HIST_MODE_GB_HISTOGRAM,
}

#[repr(C)]
pub struct rppx1_hist_params {
    pub header: v4l2_isp_params_block_header,
    pub wnd: rppx1_window,
    pub last_line: u32,
    pub v_stepsize: u32,
    pub h_step_inc: u32,
    pub sample_offs: u32,
    pub mode: u8,
    pub channel_sel: u8,
    pub weights: [u8; RPPX1_HIST_WEIGHT_GRIDS_SIZE],
    pub coeff: [u8; 3],
    pub sample_shift: u8,
    pub reserved: u8,
}

#[repr(C)]
pub struct rppx1_bls_fixed { pub a: u32, pub b: u32, pub c: u32, pub d: u32 }

#[repr(i32)]
pub enum rppx1_bls_mode { RPPX1_BLS_MODE_FIXED, RPPX1_BLS_MODE_MEAS }

#[repr(i32)]
pub enum rppx1_bls_win_en {
    RPPX1_BLS_WIN_EN_OFF,
    RPPX1_BLS_WIN_EN_WIN1,
    RPPX1_BLS_WIN_EN_WIN2,
    RPPX1_BLS_WIN_EN_WIN12,
}

#[repr(C)]
pub struct rppx1_bls_params {
    pub header: v4l2_isp_params_block_header,
    pub window1: rppx1_window,
    pub window2: rppx1_window,
    pub fixed: rppx1_bls_fixed,
    pub mode: u8,
    pub en_windows: u8,
    pub samples: u8,
    pub reserved: [u8; 5],
}

#[repr(C)]
pub struct rppx1_ccor_params {
    pub header: v4l2_isp_params_block_header,
    pub coeff: [[u16; 3]; 3],
    pub reserved: u16,
    pub offset: [u32; 3],
}

pub const RPPX1_LSC_SAMPLES_MAX: usize = 17;
pub const RPPX1_LSC_NUM_SECTORS: usize = 16;

#[repr(C)]
pub struct rppx1_lsc_params {
    pub header: v4l2_isp_params_block_header,
    pub r_data: [[u16; RPPX1_LSC_SAMPLES_MAX]; RPPX1_LSC_SAMPLES_MAX],
    pub gr_data: [[u16; RPPX1_LSC_SAMPLES_MAX]; RPPX1_LSC_SAMPLES_MAX],
    pub gb_data: [[u16; RPPX1_LSC_SAMPLES_MAX]; RPPX1_LSC_SAMPLES_MAX],
    pub b_data: [[u16; RPPX1_LSC_SAMPLES_MAX]; RPPX1_LSC_SAMPLES_MAX],
    pub x_grad: [u16; RPPX1_LSC_NUM_SECTORS],
    pub y_grad: [u16; RPPX1_LSC_NUM_SECTORS],
    pub x_sect_size: [u16; RPPX1_LSC_NUM_SECTORS],
    pub y_sect_size: [u16; RPPX1_LSC_NUM_SECTORS],
}

pub const RPPX1_GA_MAX_SAMPLES: usize = 17;

#[repr(i32)]
pub enum rppx1_ga_seg_mode { RPPX1_GA_SEG_MODE_LOGARITHMIC, RPPX1_GA_SEG_MODE_EQUIDISTANT }

#[repr(C)]
pub struct rppx1_ga_params {
    pub header: v4l2_isp_params_block_header,
    pub gamma_y: [u32; RPPX1_GA_MAX_SAMPLES],
    pub mode: u8,
    pub reserved: [u8; 3],
}

pub const RPPX1_LIN_SAMPLE_POINTS_NUM: usize = 16;
pub const RPPX1_LIN_DEGAMMA_CURVE_NUM: usize = 17;

#[repr(C)]
pub struct rppx1_lin_params {
    pub header: v4l2_isp_params_block_header,
    pub curve_r: [u32; RPPX1_LIN_DEGAMMA_CURVE_NUM],
    pub curve_g: [u32; RPPX1_LIN_DEGAMMA_CURVE_NUM],
    pub curve_b: [u32; RPPX1_LIN_DEGAMMA_CURVE_NUM],
    pub dx: [u8; RPPX1_LIN_SAMPLE_POINTS_NUM],
    pub reserved: u32,
}

pub const RPPX1_PARAMS_MAX_SIZE: usize =
    core::mem::size_of::<rppx1_wbmeas_params>()
    + 3 * core::mem::size_of::<rppx1_awbg_params>()
    + 2 * core::mem::size_of::<rppx1_exm_params>()
    + 3 * core::mem::size_of::<rppx1_hist_params>()
    + 2 * core::mem::size_of::<rppx1_bls_params>()
    + core::mem::size_of::<rppx1_ccor_params>()
    + 2 * core::mem::size_of::<rppx1_lsc_params>()
    + 2 * core::mem::size_of::<rppx1_ga_params>()
    + 2 * core::mem::size_of::<rppx1_lin_params>();

#[repr(i32)]
pub enum rppx1_stats_block_type {
    RPPX1_STATS_BLOCK_TYPE_WBMEAS_POST,
    RPPX1_STATS_BLOCK_TYPE_EXM_PRE1,
    RPPX1_STATS_BLOCK_TYPE_EXM_PRE2,
    RPPX1_STATS_BLOCK_TYPE_HIST_PRE1,
    RPPX1_STATS_BLOCK_TYPE_HIST_PRE2,
    RPPX1_STATS_BLOCK_TYPE_HIST_POST,
}

#[repr(C)]
pub struct rppx1_wbmeas_stats {
    pub header: v4l2_isp_block_header,
    pub cnt: u32,
    pub mean_y_or_g: u32,
    pub mean_cb_or_b: u32,
    pub mean_cr_or_r: u32,
}

pub const RPPX1_EXM_NUM_WIN: usize = 25;

#[repr(C)]
pub struct rppx1_exm_stats {
    pub header: v4l2_isp_block_header,
    pub exp_mean: [u32; RPPX1_EXM_NUM_WIN],
    pub reserved: u32,
}

pub const RPPX1_HIST_NUM_BINS: usize = 32;

#[repr(C)]
pub struct rppx1_hist_stats {
    pub header: v4l2_isp_block_header,
    pub hist_bins: [u32; RPPX1_HIST_NUM_BINS],
}

pub const RPPX1_STATS_MAX_SIZE: usize =
    core::mem::size_of::<rppx1_wbmeas_stats>()
    + 2 * core::mem::size_of::<rppx1_exm_stats>()
    + 3 * core::mem::size_of::<rppx1_hist_stats>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
