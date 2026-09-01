/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2018 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

/* Dependency intent from C include: "gx-formatter.h" */

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct clk_bulk_data {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct of_phandle_args {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub enum aiu_clk_ids {
    PCLK = 0,
    AOCLK,
    MCLK,
    MIXER,
}

#[repr(C)]
pub struct aiu_interface {
    pub clks: *mut clk_bulk_data,
    pub clk_num: ::core::ffi::c_uint,
    pub irq: ::core::ffi::c_int,
    pub iface: gx_iface,
}

#[repr(C)]
pub struct aiu_platform_data {
    pub has_acodec: bool,
    pub has_clk_ctrl_more_i2s_div: bool,
}

#[repr(C)]
pub struct aiu {
    pub spdif_mclk: *mut clk,
    pub i2s: aiu_interface,
    pub spdif: aiu_interface,
    pub platform: *const aiu_platform_data,
}

pub const AIU_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE;

unsafe extern "C" {
    pub fn aiu_of_xlate_dai_name(
        component: *mut snd_soc_component,
        args: *const of_phandle_args,
        dai_name: *mut *const ::core::ffi::c_char,
        component_id: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn aiu_hdmi_ctrl_register_component(dev: *mut device) -> ::core::ffi::c_int;
    pub fn aiu_acodec_ctrl_register_component(dev: *mut device) -> ::core::ffi::c_int;

    pub fn aiu_fifo_i2s_dai_probe(dai: *mut snd_soc_dai) -> ::core::ffi::c_int;
    pub fn aiu_fifo_spdif_dai_probe(dai: *mut snd_soc_dai) -> ::core::ffi::c_int;

    pub static aiu_fifo_i2s_dai_ops: snd_soc_dai_ops;
    pub static aiu_fifo_spdif_dai_ops: snd_soc_dai_ops;
    pub static aiu_encoder_i2s_dai_ops: snd_soc_dai_ops;
    pub static aiu_encoder_spdif_dai_ops: snd_soc_dai_ops;
    pub static aiu_formatter_i2s_ops: gx_formatter_ops;
}

pub const AIU_IEC958_BPF: u32 = 0x000;
pub const AIU_958_MISC: u32 = 0x010;
pub const AIU_IEC958_DCU_FF_CTRL: u32 = 0x01c;
pub const AIU_958_CHSTAT_L0: u32 = 0x020;
pub const AIU_958_CHSTAT_L1: u32 = 0x024;
pub const AIU_958_CTRL: u32 = 0x028;
pub const AIU_I2S_SOURCE_DESC: u32 = 0x034;
pub const AIU_I2S_DAC_CFG: u32 = 0x040;
pub const AIU_I2S_SYNC: u32 = 0x044;
pub const AIU_I2S_MISC: u32 = 0x048;
pub const AIU_RST_SOFT: u32 = 0x054;
pub const AIU_CLK_CTRL: u32 = 0x058;
pub const AIU_CLK_CTRL_MORE: u32 = 0x064;
pub const AIU_CODEC_DAC_LRCLK_CTRL: u32 = 0x0a0;
pub const AIU_HDMI_CLK_DATA_CTRL: u32 = 0x0a8;
pub const AIU_ACODEC_CTRL: u32 = 0x0b0;
pub const AIU_958_CHSTAT_R0: u32 = 0x0c0;
pub const AIU_958_CHSTAT_R1: u32 = 0x0c4;
pub const AIU_MEM_I2S_START: u32 = 0x180;
pub const AIU_MEM_I2S_MASKS: u32 = 0x18c;
pub const AIU_MEM_I2S_CONTROL: u32 = 0x190;
pub const AIU_MEM_IEC958_START: u32 = 0x194;
pub const AIU_MEM_IEC958_CONTROL: u32 = 0x1a4;
pub const AIU_MEM_I2S_BUF_CNTL: u32 = 0x1d8;
pub const AIU_MEM_IEC958_BUF_CNTL: u32 = 0x1fc;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
