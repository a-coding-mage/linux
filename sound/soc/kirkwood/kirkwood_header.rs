/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * kirkwood.h
 *
 * (c) 2010 Arnaud Patard <apatard@mandriva.com>
 */

pub const DRV_NAME: &str = "mvebu-audio";

pub const KIRKWOOD_RECORD_WIN: u32 = 0;
pub const KIRKWOOD_PLAYBACK_WIN: u32 = 1;
pub const KIRKWOOD_MAX_AUDIO_WIN: u32 = 2;

pub const fn KIRKWOOD_AUDIO_WIN_BASE_REG(win: u32) -> u32 {
    0xA00 + (win << 3)
}

pub const fn KIRKWOOD_AUDIO_WIN_CTRL_REG(win: u32) -> u32 {
    0xA04 + (win << 3)
}

pub const KIRKWOOD_RECCTL: u32 = 0x1000;
pub const KIRKWOOD_RECCTL_SPDIF_EN: u32 = 1 << 11;
pub const KIRKWOOD_RECCTL_I2S_EN: u32 = 1 << 10;
pub const KIRKWOOD_RECCTL_PAUSE: u32 = 1 << 9;
pub const KIRKWOOD_RECCTL_MUTE: u32 = 1 << 8;
pub const KIRKWOOD_RECCTL_BURST_MASK: u32 = 3 << 5;
pub const KIRKWOOD_RECCTL_BURST_128: u32 = 2 << 5;
pub const KIRKWOOD_RECCTL_BURST_32: u32 = 1 << 5;
pub const KIRKWOOD_RECCTL_MONO: u32 = 1 << 4;
pub const KIRKWOOD_RECCTL_MONO_CHAN_RIGHT: u32 = 1 << 3;
pub const KIRKWOOD_RECCTL_MONO_CHAN_LEFT: u32 = 0 << 3;
pub const KIRKWOOD_RECCTL_SIZE_MASK: u32 = 7 << 0;
pub const KIRKWOOD_RECCTL_SIZE_16: u32 = 7 << 0;
pub const KIRKWOOD_RECCTL_SIZE_16_C: u32 = 3 << 0;
pub const KIRKWOOD_RECCTL_SIZE_20: u32 = 2 << 0;
pub const KIRKWOOD_RECCTL_SIZE_24: u32 = 1 << 0;
pub const KIRKWOOD_RECCTL_SIZE_32: u32 = 0 << 0;

pub const KIRKWOOD_RECCTL_ENABLE_MASK: u32 =
    KIRKWOOD_RECCTL_SPDIF_EN | KIRKWOOD_RECCTL_I2S_EN;

pub const KIRKWOOD_REC_BUF_ADDR: u32 = 0x1004;
pub const KIRKWOOD_REC_BUF_SIZE: u32 = 0x1008;
pub const KIRKWOOD_REC_BYTE_COUNT: u32 = 0x100C;

pub const KIRKWOOD_PLAYCTL: u32 = 0x1100;
pub const KIRKWOOD_PLAYCTL_PLAY_BUSY: u32 = 1 << 16;
pub const KIRKWOOD_PLAYCTL_BURST_MASK: u32 = 3 << 11;
pub const KIRKWOOD_PLAYCTL_BURST_128: u32 = 2 << 11;
pub const KIRKWOOD_PLAYCTL_BURST_32: u32 = 1 << 11;
pub const KIRKWOOD_PLAYCTL_PAUSE: u32 = 1 << 9;
pub const KIRKWOOD_PLAYCTL_SPDIF_MUTE: u32 = 1 << 8;
pub const KIRKWOOD_PLAYCTL_MONO_MASK: u32 = 3 << 5;
pub const KIRKWOOD_PLAYCTL_MONO_BOTH: u32 = 3 << 5;
pub const KIRKWOOD_PLAYCTL_MONO_OFF: u32 = 0 << 5;
pub const KIRKWOOD_PLAYCTL_I2S_MUTE: u32 = 1 << 7;
pub const KIRKWOOD_PLAYCTL_SPDIF_EN: u32 = 1 << 4;
pub const KIRKWOOD_PLAYCTL_I2S_EN: u32 = 1 << 3;
pub const KIRKWOOD_PLAYCTL_SIZE_MASK: u32 = 7 << 0;
pub const KIRKWOOD_PLAYCTL_SIZE_16: u32 = 7 << 0;
pub const KIRKWOOD_PLAYCTL_SIZE_16_C: u32 = 3 << 0;
pub const KIRKWOOD_PLAYCTL_SIZE_20: u32 = 2 << 0;
pub const KIRKWOOD_PLAYCTL_SIZE_24: u32 = 1 << 0;
pub const KIRKWOOD_PLAYCTL_SIZE_32: u32 = 0 << 0;

pub const KIRKWOOD_PLAYCTL_ENABLE_MASK: u32 =
    KIRKWOOD_PLAYCTL_SPDIF_EN | KIRKWOOD_PLAYCTL_I2S_EN;

pub const KIRKWOOD_PLAY_BUF_ADDR: u32 = 0x1104;
pub const KIRKWOOD_PLAY_BUF_SIZE: u32 = 0x1108;
pub const KIRKWOOD_PLAY_BYTE_COUNT: u32 = 0x110C;

pub const KIRKWOOD_DCO_CTL: u32 = 0x1204;
pub const KIRKWOOD_DCO_CTL_OFFSET_MASK: u32 = 0xFFF << 2;
pub const KIRKWOOD_DCO_CTL_OFFSET_0: u32 = 0x800 << 2;
pub const KIRKWOOD_DCO_CTL_FREQ_MASK: u32 = 3 << 0;
pub const KIRKWOOD_DCO_CTL_FREQ_11: u32 = 0 << 0;
pub const KIRKWOOD_DCO_CTL_FREQ_12: u32 = 1 << 0;
pub const KIRKWOOD_DCO_CTL_FREQ_24: u32 = 2 << 0;

pub const KIRKWOOD_DCO_SPCR_STATUS: u32 = 0x120c;
pub const KIRKWOOD_DCO_SPCR_STATUS_DCO_LOCK: u32 = 1 << 16;

pub const KIRKWOOD_CLOCKS_CTRL: u32 = 0x1230;
pub const KIRKWOOD_MCLK_SOURCE_MASK: u32 = 3 << 0;
pub const KIRKWOOD_MCLK_SOURCE_DCO: u32 = 0 << 0;
pub const KIRKWOOD_MCLK_SOURCE_EXTCLK: u32 = 3 << 0;

pub const KIRKWOOD_ERR_CAUSE: u32 = 0x1300;
pub const KIRKWOOD_ERR_MASK: u32 = 0x1304;

pub const KIRKWOOD_INT_CAUSE: u32 = 0x1308;
pub const KIRKWOOD_INT_MASK: u32 = 0x130C;
pub const KIRKWOOD_INT_CAUSE_PLAY_BYTES: u32 = 1 << 14;
pub const KIRKWOOD_INT_CAUSE_REC_BYTES: u32 = 1 << 13;
pub const KIRKWOOD_INT_CAUSE_DMA_PLAY_END: u32 = 1 << 7;
pub const KIRKWOOD_INT_CAUSE_DMA_PLAY_3Q: u32 = 1 << 6;
pub const KIRKWOOD_INT_CAUSE_DMA_PLAY_HALF: u32 = 1 << 5;
pub const KIRKWOOD_INT_CAUSE_DMA_PLAY_1Q: u32 = 1 << 4;
pub const KIRKWOOD_INT_CAUSE_DMA_REC_END: u32 = 1 << 3;
pub const KIRKWOOD_INT_CAUSE_DMA_REC_3Q: u32 = 1 << 2;
pub const KIRKWOOD_INT_CAUSE_DMA_REC_HALF: u32 = 1 << 1;
pub const KIRKWOOD_INT_CAUSE_DMA_REC_1Q: u32 = 1 << 0;

pub const KIRKWOOD_REC_BYTE_INT_COUNT: u32 = 0x1310;
pub const KIRKWOOD_PLAY_BYTE_INT_COUNT: u32 = 0x1314;
pub const KIRKWOOD_BYTE_INT_COUNT_MASK: u32 = 0xffffff;

pub const KIRKWOOD_I2S_PLAYCTL: u32 = 0x2508;
pub const KIRKWOOD_I2S_RECCTL: u32 = 0x2408;
pub const KIRKWOOD_I2S_CTL_JUST_MASK: u32 = 0xf << 26;
pub const KIRKWOOD_I2S_CTL_LJ: u32 = 0 << 26;
pub const KIRKWOOD_I2S_CTL_I2S: u32 = 5 << 26;
pub const KIRKWOOD_I2S_CTL_RJ: u32 = 8 << 26;
pub const KIRKWOOD_I2S_CTL_SIZE_MASK: u32 = 3 << 30;
pub const KIRKWOOD_I2S_CTL_SIZE_16: u32 = 3 << 30;
pub const KIRKWOOD_I2S_CTL_SIZE_20: u32 = 2 << 30;
pub const KIRKWOOD_I2S_CTL_SIZE_24: u32 = 1 << 30;
pub const KIRKWOOD_I2S_CTL_SIZE_32: u32 = 0 << 30;

pub const KIRKWOOD_AUDIO_BUF_MAX: u32 = 16 * 1024 * 1024;

/* Theses values come from the marvell alsa driver */
/* need to find where they come from               */
pub const KIRKWOOD_SND_MIN_PERIODS: u32 = 2;
pub const KIRKWOOD_SND_MAX_PERIODS: u32 = 16;
pub const KIRKWOOD_SND_MIN_PERIOD_BYTES: u32 = 256;
pub const KIRKWOOD_SND_MAX_PERIOD_BYTES: u32 = 0x8000;
pub const KIRKWOOD_SND_MAX_BUFFER_BYTES: u32 =
    KIRKWOOD_SND_MAX_PERIOD_BYTES * KIRKWOOD_SND_MAX_PERIODS;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kirkwood_dma_data {
    /* C source: void __iomem *io; */
    pub io: *mut core::ffi::c_void,
    /* C source: void __iomem *pll_config; */
    pub pll_config: *mut core::ffi::c_void,
    /* C source: void __iomem *soc_control; */
    pub soc_control: *mut core::ffi::c_void,
    pub clk: *mut clk,
    pub extclk: *mut clk,
    pub ctl_play: u32,
    pub ctl_rec: u32,
    pub substream_play: *mut snd_pcm_substream,
    pub substream_rec: *mut snd_pcm_substream,
    pub irq: core::ffi::c_int,
    pub burst: core::ffi::c_int,
}

unsafe extern "C" {
    pub static kirkwood_soc_component: snd_soc_component_driver;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
