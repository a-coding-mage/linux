/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2012
 *
 * Author: Ola Lilja <ola.o.lilja@stericsson.com>,
 *         Roger Nilsson <roger.xr.nilsson@stericsson.com>
 *         for ST-Ericsson.
 */

/* Dependencies from the original header:
 * linux/types.h
 * linux/spinlock.h
 * ux500_msp_i2s.h
 */

pub const UX500_NBR_OF_DAI: u32 = 4;

pub const UX500_I2S_RATES: u32 = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000;

pub const UX500_I2S_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE;

pub const FRAME_PER_SINGLE_SLOT_8_KHZ: u32 = 31;
pub const FRAME_PER_SINGLE_SLOT_16_KHZ: u32 = 124;
pub const FRAME_PER_SINGLE_SLOT_44_1_KHZ: u32 = 63;
pub const FRAME_PER_SINGLE_SLOT_48_KHZ: u32 = 49;
pub const FRAME_PER_2_SLOTS: u32 = 31;
pub const FRAME_PER_8_SLOTS: u32 = 138;
pub const FRAME_PER_16_SLOTS: u32 = 277;

pub const UX500_MSP_INTERNAL_CLOCK_FREQ: u32 = 40000000;
pub const UX500_MSP1_INTERNAL_CLOCK_FREQ: u32 = UX500_MSP_INTERNAL_CLOCK_FREQ;

pub const UX500_MSP_MIN_CHANNELS: u32 = 1;
pub const UX500_MSP_MAX_CHANNELS: u32 = 8;

pub const PLAYBACK_CONFIGURED: u32 = 1;
pub const CAPTURE_CONFIGURED: u32 = 2;

#[repr(C)]
pub enum ux500_msp_clock_id {
    UX500_MSP_MASTER_CLOCK,
}

#[repr(C)]
pub struct ux500_msp_i2s_drvdata {
    pub msp: *mut ux500_msp,
    pub reg_vape: *mut regulator,
    pub fmt: ::core::ffi::c_uint,
    pub tx_mask: ::core::ffi::c_uint,
    pub rx_mask: ::core::ffi::c_uint,
    pub slots: ::core::ffi::c_int,
    pub slot_width: ::core::ffi::c_int,

    /* Clocks */
    pub master_clk: ::core::ffi::c_uint,
    pub clk: *mut clk,
    pub pclk: *mut clk,

    /* Regulators */
    pub vape_opp_constraint: ::core::ffi::c_int,
}

unsafe extern "C" {
    pub fn ux500_msp_dai_set_data_delay(
        dai: *mut snd_soc_dai,
        delay: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
