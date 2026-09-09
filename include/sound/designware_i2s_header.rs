/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (ST) 2012 Rajeev Kumar (rajeevkumar.linux@gmail.com)
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dw_i2s_dev {
    _private: [u8; 0],
}

pub type dma_addr_t = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dma_slave_buswidth {
    Unknown = 0,
}

/*
 * struct i2s_clk_config_data - represent i2s clk configuration data
 * @chan_nr: number of channel
 * @data_width: number of bits per sample (8/16/24/32 bit)
 * @sample_rate: sampling frequency (8Khz, 16Khz, 32Khz, 44Khz, 48Khz)
 */
#[repr(C)]
pub struct i2s_clk_config_data {
    pub chan_nr: i32,
    pub data_width: u32,
    pub sample_rate: u32,
}

pub const DWC_I2S_PLAY: u32 = 1 << 0;
pub const DWC_I2S_RECORD: u32 = 1 << 1;
pub const DW_I2S_SLAVE: u32 = 1 << 2;
pub const DW_I2S_MASTER: u32 = 1 << 3;

pub const DW_I2S_QUIRK_COMP_REG_OFFSET: u32 = 1 << 0;
pub const DW_I2S_QUIRK_COMP_PARAM1: u32 = 1 << 1;
pub const DW_I2S_QUIRK_16BIT_IDX_OVERRIDE: u32 = 1 << 2;

#[repr(C)]
pub struct i2s_platform_data {
    pub cap: u32,
    pub channel: i32,
    pub snd_fmts: u32,
    pub snd_rates: u32,
    pub quirks: u32,
    pub i2s_reg_comp1: u32,
    pub i2s_reg_comp2: u32,
    pub play_dma_data: *mut c_void,
    pub capture_dma_data: *mut c_void,
    pub filter: Option<unsafe extern "C" fn(chan: *mut dma_chan, slave: *mut c_void) -> bool>,
    pub i2s_clk_cfg: Option<unsafe extern "C" fn(config: *mut i2s_clk_config_data) -> i32>,
    pub i2s_pd_init: Option<unsafe extern "C" fn(dev: *mut dw_i2s_dev) -> i32>,
}

#[repr(C)]
pub struct i2s_dma_data {
    pub data: *mut c_void,
    pub addr: dma_addr_t,
    pub max_burst: u32,
    pub addr_width: dma_slave_buswidth,
    pub filter: Option<unsafe extern "C" fn(chan: *mut dma_chan, slave: *mut c_void) -> bool>,
}

/* I2S DMA registers */
pub const I2S_RXDMA: u32 = 0x01C0;
pub const I2S_TXDMA: u32 = 0x01C8;

pub const TWO_CHANNEL_SUPPORT: u32 = 2; // up to 2.0
pub const FOUR_CHANNEL_SUPPORT: u32 = 4; // up to 3.1
pub const SIX_CHANNEL_SUPPORT: u32 = 6; // up to 5.1
pub const EIGHT_CHANNEL_SUPPORT: u32 = 8; // up to 7.1

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
