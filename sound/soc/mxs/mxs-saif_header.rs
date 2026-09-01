/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011 Freescale Semiconductor, Inc. All Rights Reserved.
 */

/* C header dependency: "mxs-pcm.h" */

pub const SAIF_CTRL: u32 = 0x0;
pub const SAIF_STAT: u32 = 0x10;
pub const SAIF_DATA: u32 = 0x20;
pub const SAIF_VERSION: u32 = 0x30;

/* SAIF_CTRL */
pub const BM_SAIF_CTRL_SFTRST: u32 = 0x80000000;
pub const BM_SAIF_CTRL_CLKGATE: u32 = 0x40000000;
pub const BP_SAIF_CTRL_BITCLK_MULT_RATE: u32 = 27;
pub const BM_SAIF_CTRL_BITCLK_MULT_RATE: u32 = 0x38000000;
pub const fn BF_SAIF_CTRL_BITCLK_MULT_RATE(v: u32) -> u32 {
    (v << 27) & BM_SAIF_CTRL_BITCLK_MULT_RATE
}
pub const BM_SAIF_CTRL_BITCLK_BASE_RATE: u32 = 0x04000000;
pub const BM_SAIF_CTRL_FIFO_ERROR_IRQ_EN: u32 = 0x02000000;
pub const BM_SAIF_CTRL_FIFO_SERVICE_IRQ_EN: u32 = 0x01000000;
pub const BP_SAIF_CTRL_RSRVD2: u32 = 21;
pub const BM_SAIF_CTRL_RSRVD2: u32 = 0x00E00000;

pub const BP_SAIF_CTRL_DMAWAIT_COUNT: u32 = 16;
pub const BM_SAIF_CTRL_DMAWAIT_COUNT: u32 = 0x001F0000;
pub const fn BF_SAIF_CTRL_DMAWAIT_COUNT(v: u32) -> u32 {
    (v << 16) & BM_SAIF_CTRL_DMAWAIT_COUNT
}
pub const BP_SAIF_CTRL_CHANNEL_NUM_SELECT: u32 = 14;
pub const BM_SAIF_CTRL_CHANNEL_NUM_SELECT: u32 = 0x0000C000;
pub const fn BF_SAIF_CTRL_CHANNEL_NUM_SELECT(v: u32) -> u32 {
    (v << 14) & BM_SAIF_CTRL_CHANNEL_NUM_SELECT
}
pub const BM_SAIF_CTRL_LRCLK_PULSE: u32 = 0x00002000;
pub const BM_SAIF_CTRL_BIT_ORDER: u32 = 0x00001000;
pub const BM_SAIF_CTRL_DELAY: u32 = 0x00000800;
pub const BM_SAIF_CTRL_JUSTIFY: u32 = 0x00000400;
pub const BM_SAIF_CTRL_LRCLK_POLARITY: u32 = 0x00000200;
pub const BM_SAIF_CTRL_BITCLK_EDGE: u32 = 0x00000100;
pub const BP_SAIF_CTRL_WORD_LENGTH: u32 = 4;
pub const BM_SAIF_CTRL_WORD_LENGTH: u32 = 0x000000F0;
pub const fn BF_SAIF_CTRL_WORD_LENGTH(v: u32) -> u32 {
    (v << 4) & BM_SAIF_CTRL_WORD_LENGTH
}
pub const BM_SAIF_CTRL_BITCLK_48XFS_ENABLE: u32 = 0x00000008;
pub const BM_SAIF_CTRL_SLAVE_MODE: u32 = 0x00000004;
pub const BM_SAIF_CTRL_READ_MODE: u32 = 0x00000002;
pub const BM_SAIF_CTRL_RUN: u32 = 0x00000001;

/* SAIF_STAT */
pub const BM_SAIF_STAT_PRESENT: u32 = 0x80000000;
pub const BP_SAIF_STAT_RSRVD2: u32 = 17;
pub const BM_SAIF_STAT_RSRVD2: u32 = 0x7FFE0000;
pub const fn BF_SAIF_STAT_RSRVD2(v: u32) -> u32 {
    (v << 17) & BM_SAIF_STAT_RSRVD2
}
pub const BM_SAIF_STAT_DMA_PREQ: u32 = 0x00010000;
pub const BP_SAIF_STAT_RSRVD1: u32 = 7;
pub const BM_SAIF_STAT_RSRVD1: u32 = 0x0000FF80;
pub const fn BF_SAIF_STAT_RSRVD1(v: u32) -> u32 {
    (v << 7) & BM_SAIF_STAT_RSRVD1
}

pub const BM_SAIF_STAT_FIFO_UNDERFLOW_IRQ: u32 = 0x00000040;
pub const BM_SAIF_STAT_FIFO_OVERFLOW_IRQ: u32 = 0x00000020;
pub const BM_SAIF_STAT_FIFO_SERVICE_IRQ: u32 = 0x00000010;
pub const BP_SAIF_STAT_RSRVD0: u32 = 1;
pub const BM_SAIF_STAT_RSRVD0: u32 = 0x0000000E;
pub const fn BF_SAIF_STAT_RSRVD0(v: u32) -> u32 {
    (v << 1) & BM_SAIF_STAT_RSRVD0
}
pub const BM_SAIF_STAT_BUSY: u32 = 0x00000001;

/* SAFI_DATA */
pub const BP_SAIF_DATA_PCM_RIGHT: u32 = 16;
pub const BM_SAIF_DATA_PCM_RIGHT: u32 = 0xFFFF0000;
pub const fn BF_SAIF_DATA_PCM_RIGHT(v: u32) -> u32 {
    (v << 16) & BM_SAIF_DATA_PCM_RIGHT
}
pub const BP_SAIF_DATA_PCM_LEFT: u32 = 0;
pub const BM_SAIF_DATA_PCM_LEFT: u32 = 0x0000FFFF;
pub const fn BF_SAIF_DATA_PCM_LEFT(v: u32) -> u32 {
    (v << 0) & BM_SAIF_DATA_PCM_LEFT
}

/* SAIF_VERSION */
pub const BP_SAIF_VERSION_MAJOR: u32 = 24;
pub const BM_SAIF_VERSION_MAJOR: u32 = 0xFF000000;
pub const fn BF_SAIF_VERSION_MAJOR(v: u32) -> u32 {
    (v << 24) & BM_SAIF_VERSION_MAJOR
}
pub const BP_SAIF_VERSION_MINOR: u32 = 16;
pub const BM_SAIF_VERSION_MINOR: u32 = 0x00FF0000;
pub const fn BF_SAIF_VERSION_MINOR(v: u32) -> u32 {
    (v << 16) & BM_SAIF_VERSION_MINOR
}
pub const BP_SAIF_VERSION_STEP: u32 = 0;
pub const BM_SAIF_VERSION_STEP: u32 = 0x0000FFFF;
pub const fn BF_SAIF_VERSION_STEP(v: u32) -> u32 {
    (v << 0) & BM_SAIF_VERSION_STEP
}

pub const MXS_SAIF_MCLK: u32 = 0;

#[repr(C)]
pub struct mxs_saif {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub mclk: ::core::ffi::c_uint,
    pub mclk_in_use: ::core::ffi::c_uint,
    pub base: *mut ::core::ffi::c_void,
    pub id: ::core::ffi::c_uint,
    pub master_id: ::core::ffi::c_uint,
    pub cur_rate: ::core::ffi::c_uint,
    pub ongoing: ::core::ffi::c_uint,

    pub fifo_underrun: u32,
    pub fifo_overrun: u32,

    pub state: mxs_saif_state,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum mxs_saif_state {
    MXS_SAIF_STATE_STOPPED,
    MXS_SAIF_STATE_RUNNING,
}

extern "C" {
    pub fn mxs_saif_put_mclk(saif_id: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn mxs_saif_get_mclk(
        saif_id: ::core::ffi::c_uint,
        mclk: ::core::ffi::c_uint,
        rate: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
