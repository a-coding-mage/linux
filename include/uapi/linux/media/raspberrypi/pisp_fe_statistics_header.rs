/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * RP1 PiSP Front End statistics definitions
 *
 * Copyright (C) 2021 - Raspberry Pi Ltd.
 *
 */

// Translated from the C header; Linux fixed-width types map to Rust u32/u64.

pub const PISP_FLOATING_STATS_NUM_ZONES: usize = 4;
pub const PISP_AGC_STATS_NUM_BINS: usize = 1024;
pub const PISP_AGC_STATS_SIZE: usize = 16;
pub const PISP_AGC_STATS_NUM_ZONES: usize = PISP_AGC_STATS_SIZE * PISP_AGC_STATS_SIZE;
pub const PISP_AGC_STATS_NUM_ROW_SUMS: usize = 512;

#[repr(C, packed)]
pub struct pisp_agc_statistics_zone {
    pub Y_sum: u64,
    pub counted: u32,
    pub pad: u32,
}

#[repr(C, packed)]
pub struct pisp_agc_statistics {
    pub row_sums: [u32; PISP_AGC_STATS_NUM_ROW_SUMS],
    /*
     * 32-bits per bin means an image (just less than) 16384x16384 pixels
     * in size can weight every pixel from 0 to 15.
     */
    pub histogram: [u32; PISP_AGC_STATS_NUM_BINS],
    pub floating: [pisp_agc_statistics_zone; PISP_FLOATING_STATS_NUM_ZONES],
}

pub const PISP_AWB_STATS_SIZE: usize = 32;
pub const PISP_AWB_STATS_NUM_ZONES: usize = PISP_AWB_STATS_SIZE * PISP_AWB_STATS_SIZE;

#[repr(C, packed)]
pub struct pisp_awb_statistics_zone {
    pub R_sum: u32,
    pub G_sum: u32,
    pub B_sum: u32,
    pub counted: u32,
}

#[repr(C, packed)]
pub struct pisp_awb_statistics {
    pub zones: [pisp_awb_statistics_zone; PISP_AWB_STATS_NUM_ZONES],
    pub floating: [pisp_awb_statistics_zone; PISP_FLOATING_STATS_NUM_ZONES],
}

pub const PISP_CDAF_STATS_SIZE: usize = 8;
pub const PISP_CDAF_STATS_NUM_FOMS: usize = PISP_CDAF_STATS_SIZE * PISP_CDAF_STATS_SIZE;

#[repr(C, packed)]
pub struct pisp_cdaf_statistics {
    pub foms: [u64; PISP_CDAF_STATS_NUM_FOMS],
    pub floating: [u64; PISP_FLOATING_STATS_NUM_ZONES],
}

#[repr(C, packed)]
pub struct pisp_statistics {
    pub awb: pisp_awb_statistics,
    pub agc: pisp_agc_statistics,
    pub cdaf: pisp_cdaf_statistics,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
