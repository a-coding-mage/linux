// SPDX-License-Identifier: GPL-2.0+
//
// DA7210 ALSA Soc codec driver
//
// Copyright (c) 2009 Dialog Semiconductor
// Written by David Chen <Dajun.chen@diasemi.com>
//
// Copyright (C) 2009 Renesas Solutions Corp.
// Cleanups by Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// Tested on SuperH Ecovec24 board with S16/S24 LE in 48KHz using I2S

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u32 = u32;
type bool_ = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct spi_device {
    pub dev: device,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
type c_long = isize;
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub read_flag_mask: c_int,
    pub write_flag_mask: c_int,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_int,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_pll:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_int,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}
#[repr(C)]
pub struct i2c_driver_driver {
    pub name: *const c_char,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}
#[repr(C)]
pub struct spi_driver_driver {
    pub name: *const c_char,
}
#[repr(C)]
pub struct spi_driver {
    pub driver: spi_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint);
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut da7210_priv;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value)
        -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value)
        -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_register_patch(
        map: *mut regmap,
        regs: *const reg_sequence,
        num_regs: c_int,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn spi_register_driver(driver: *mut spi_driver) -> c_int;
    fn spi_unregister_driver(driver: *mut spi_driver);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_int = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAIFMT_MASTER_MASK: u32 = 0;
const SND_SOC_DAIFMT_CBP_CFP: u32 = 0;
const SND_SOC_DAIFMT_CBC_CFC: u32 = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: u32 = 0;
const SND_SOC_DAIFMT_I2S: u32 = 0;
const SND_SOC_DAIFMT_LEFT_J: u32 = 0;
const SND_SOC_DAIFMT_RIGHT_J: u32 = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;

/* DA7210 register space */
const DA7210_PAGE_CONTROL: c_uint = 0x00;
const DA7210_CONTROL: c_uint = 0x01;
const DA7210_STATUS: c_uint = 0x02;
const DA7210_STARTUP1: c_uint = 0x03;
const DA7210_STARTUP2: c_uint = 0x04;
const DA7210_STARTUP3: c_uint = 0x05;
const DA7210_MIC_L: c_uint = 0x07;
const DA7210_MIC_R: c_uint = 0x08;
const DA7210_AUX1_L: c_uint = 0x09;
const DA7210_AUX1_R: c_uint = 0x0A;
const DA7210_AUX2: c_uint = 0x0B;
const DA7210_IN_GAIN: c_uint = 0x0C;
const DA7210_INMIX_L: c_uint = 0x0D;
const DA7210_INMIX_R: c_uint = 0x0E;
const DA7210_ADC_HPF: c_uint = 0x0F;
const DA7210_ADC: c_uint = 0x10;
const DA7210_ADC_EQ1_2: c_uint = 0x11;
const DA7210_ADC_EQ3_4: c_uint = 0x12;
const DA7210_ADC_EQ5: c_uint = 0x13;
const DA7210_DAC_HPF: c_uint = 0x14;
const DA7210_DAC_L: c_uint = 0x15;
const DA7210_DAC_R: c_uint = 0x16;
const DA7210_DAC_SEL: c_uint = 0x17;
const DA7210_SOFTMUTE: c_uint = 0x18;
const DA7210_DAC_EQ1_2: c_uint = 0x19;
const DA7210_DAC_EQ3_4: c_uint = 0x1A;
const DA7210_DAC_EQ5: c_uint = 0x1B;
const DA7210_OUTMIX_L: c_uint = 0x1C;
const DA7210_OUTMIX_R: c_uint = 0x1D;
const DA7210_OUT1_L: c_uint = 0x1E;
const DA7210_OUT1_R: c_uint = 0x1F;
const DA7210_OUT2: c_uint = 0x20;
const DA7210_HP_L_VOL: c_uint = 0x21;
const DA7210_HP_R_VOL: c_uint = 0x22;
const DA7210_HP_CFG: c_uint = 0x23;
const DA7210_ZERO_CROSS: c_uint = 0x24;
const DA7210_DAI_SRC_SEL: c_uint = 0x25;
const DA7210_DAI_CFG1: c_uint = 0x26;
const DA7210_DAI_CFG3: c_uint = 0x28;
const DA7210_PLL_DIV1: c_uint = 0x29;
const DA7210_PLL_DIV2: c_uint = 0x2A;
const DA7210_PLL_DIV3: c_uint = 0x2B;
const DA7210_PLL: c_uint = 0x2C;
const DA7210_ALC_MAX: c_uint = 0x83;
const DA7210_ALC_MIN: c_uint = 0x84;
const DA7210_ALC_NOIS: c_uint = 0x85;
const DA7210_ALC_ATT: c_uint = 0x86;
const DA7210_ALC_REL: c_uint = 0x87;
const DA7210_ALC_DEL: c_uint = 0x88;
const DA7210_A_HID_UNLOCK: c_uint = 0x8A;
const DA7210_A_TEST_UNLOCK: c_uint = 0x8B;
const DA7210_A_PLL1: c_uint = 0x90;
const DA7210_A_CP_MODE: c_uint = 0xA7;

const DA7210_SC_MST_EN: c_uint = 1 << 0;
const DA7210_MICBIAS_EN: c_uint = 1 << 6;
const DA7210_MIC_L_EN: c_uint = 1 << 7;
const DA7210_MIC_R_EN: c_uint = 1 << 7;
const DA7210_IN_L_EN: c_uint = 1 << 7;
const DA7210_IN_R_EN: c_uint = 1 << 7;
const DA7210_ADC_ALC_EN: c_uint = 1 << 0;
const DA7210_ADC_L_EN: c_uint = 1 << 3;
const DA7210_ADC_R_EN: c_uint = 1 << 7;
const DA7210_VOICE_F0_MASK: c_uint = 0x7 << 4;
const DA7210_VOICE_F0_25: c_uint = 1 << 4;
const DA7210_VOICE_EN: c_uint = 1 << 7;
const DA7210_DAC_L_SRC_DAI_L: c_uint = 4 << 0;
const DA7210_DAC_L_EN: c_uint = 1 << 3;
const DA7210_DAC_R_SRC_DAI_R: c_uint = 5 << 4;
const DA7210_DAC_R_EN: c_uint = 1 << 7;
const DA7210_OUT_L_EN: c_uint = 1 << 7;
const DA7210_OUT_R_EN: c_uint = 1 << 7;
const DA7210_HP_2CAP_MODE: c_uint = 1 << 1;
const DA7210_HP_SENSE_EN: c_uint = 1 << 2;
const DA7210_HP_L_EN: c_uint = 1 << 3;
const DA7210_HP_MODE: c_uint = 1 << 6;
const DA7210_HP_R_EN: c_uint = 1 << 7;
const DA7210_DAI_OUT_L_SRC: c_uint = 6 << 0;
const DA7210_DAI_OUT_R_SRC: c_uint = 7 << 4;
const DA7210_DAI_WORD_S16_LE: c_uint = 0 << 0;
const DA7210_DAI_WORD_S20_3LE: c_uint = 1 << 0;
const DA7210_DAI_WORD_S24_LE: c_uint = 2 << 0;
const DA7210_DAI_WORD_S32_LE: c_uint = 3 << 0;
const DA7210_DAI_FLEN_64BIT: c_uint = 1 << 2;
const DA7210_DAI_MODE_SLAVE: c_uint = 0 << 7;
const DA7210_DAI_MODE_MASTER: c_uint = 1 << 7;
const DA7210_DAI_FORMAT_I2SMODE: c_uint = 0 << 0;
const DA7210_DAI_FORMAT_LEFT_J: c_uint = 1 << 0;
const DA7210_DAI_FORMAT_RIGHT_J: c_uint = 2 << 0;
const DA7210_DAI_OE: c_uint = 1 << 3;
const DA7210_DAI_EN: c_uint = 1 << 7;
const DA7210_PLL_DIV_L_MASK: c_uint = 0xF << 0;
const DA7210_MCLK_RANGE_10_20_MHZ: c_uint = 1 << 4;
const DA7210_PLL_BYP: c_uint = 1 << 6;
const DA7210_PLL_FS_MASK: c_uint = 0xF << 0;
const DA7210_PLL_FS_8000: c_uint = 0x1 << 0;
const DA7210_PLL_FS_11025: c_uint = 0x2 << 0;
const DA7210_PLL_FS_12000: c_uint = 0x3 << 0;
const DA7210_PLL_FS_16000: c_uint = 0x5 << 0;
const DA7210_PLL_FS_22050: c_uint = 0x6 << 0;
const DA7210_PLL_FS_24000: c_uint = 0x7 << 0;
const DA7210_PLL_FS_32000: c_uint = 0x9 << 0;
const DA7210_PLL_FS_44100: c_uint = 0xA << 0;
const DA7210_PLL_FS_48000: c_uint = 0xB << 0;
const DA7210_PLL_FS_88200: c_uint = 0xE << 0;
const DA7210_PLL_FS_96000: c_uint = 0xF << 0;
const DA7210_MCLK_DET_EN: c_uint = 0x1 << 5;
const DA7210_MCLK_SRM_EN: c_uint = 0x1 << 6;
const DA7210_PLL_EN: c_uint = 0x1 << 7;
const DA7210_RAMP_EN: c_uint = 1 << 6;
const DA7210_REG_EN: c_uint = 1 << 0;
const DA7210_BIAS_EN: c_uint = 1 << 2;
const DA7210_NOISE_SUP_EN: c_uint = 1 << 3;
const DA7210_INPGA_L_VOL: c_uint = 0x0F << 0;
const DA7210_INPGA_R_VOL: c_uint = 0xF0 << 0;
const DA7210_AUX1_L_ZC: c_uint = 1 << 0;
const DA7210_AUX1_R_ZC: c_uint = 1 << 1;
const DA7210_HP_L_ZC: c_uint = 1 << 6;
const DA7210_HP_R_ZC: c_uint = 1 << 7;
const DA7210_AUX1_L_VOL: c_uint = 0x3F << 0;
const DA7210_AUX1_L_EN: c_uint = 1 << 7;
const DA7210_AUX1_R_VOL: c_uint = 0x3F << 0;
const DA7210_AUX1_R_EN: c_uint = 1 << 7;
const DA7210_AUX2_EN: c_uint = 1 << 3;
const DA7210_INPGA_MIN_VOL_NS: c_uint = 0x0A; /* 10.5dB */
const DA7210_AUX1_MIN_VOL_NS: c_uint = 0x35; /* 6dB */
const DA7210_OUT1_L_EN: c_uint = 1 << 7;
const DA7210_OUT1_R_EN: c_uint = 1 << 7;
const DA7210_OUT2_OUTMIX_R: c_uint = 1 << 5;
const DA7210_OUT2_OUTMIX_L: c_uint = 1 << 6;
const DA7210_OUT2_EN: c_uint = 1 << 7;

#[repr(C)]
pub struct pll_div {
    pub fref: c_int,
    pub fout: c_int,
    pub div1: u8,
    pub div2: u8,
    pub div3: u8,
    pub mode: u8, /* 0 = slave, 1 = master */
}

/* PLL dividers table */
static da7210_pll_div: [pll_div; 21] = [
    /* for MASTER mode, fs = 44.1Khz */
    pll_div { fref: 12000000, fout: 2822400, div1: 0xE8, div2: 0x6C, div3: 0x2, mode: 1 }, /* MCLK=12Mhz */
    pll_div { fref: 13000000, fout: 2822400, div1: 0xDF, div2: 0x28, div3: 0xC, mode: 1 }, /* MCLK=13Mhz */
    pll_div { fref: 13500000, fout: 2822400, div1: 0xDB, div2: 0x0A, div3: 0xD, mode: 1 }, /* MCLK=13.5Mhz */
    pll_div { fref: 14400000, fout: 2822400, div1: 0xD4, div2: 0x5A, div3: 0x2, mode: 1 }, /* MCLK=14.4Mhz */
    pll_div { fref: 19200000, fout: 2822400, div1: 0xBB, div2: 0x43, div3: 0x9, mode: 1 }, /* MCLK=19.2Mhz */
    pll_div { fref: 19680000, fout: 2822400, div1: 0xB9, div2: 0x6D, div3: 0xA, mode: 1 }, /* MCLK=19.68Mhz */
    pll_div { fref: 19800000, fout: 2822400, div1: 0xB8, div2: 0xFB, div3: 0xB, mode: 1 }, /* MCLK=19.8Mhz */
    /* for MASTER mode, fs = 48Khz */
    pll_div { fref: 12000000, fout: 3072000, div1: 0xF3, div2: 0x12, div3: 0x7, mode: 1 }, /* MCLK=12Mhz */
    pll_div { fref: 13000000, fout: 3072000, div1: 0xE8, div2: 0xFD, div3: 0x5, mode: 1 }, /* MCLK=13Mhz */
    pll_div { fref: 13500000, fout: 3072000, div1: 0xE4, div2: 0x82, div3: 0x3, mode: 1 }, /* MCLK=13.5Mhz */
    pll_div { fref: 14400000, fout: 3072000, div1: 0xDD, div2: 0x3A, div3: 0x0, mode: 1 }, /* MCLK=14.4Mhz */
    pll_div { fref: 19200000, fout: 3072000, div1: 0xC1, div2: 0xEB, div3: 0x8, mode: 1 }, /* MCLK=19.2Mhz */
    pll_div { fref: 19680000, fout: 3072000, div1: 0xBF, div2: 0xEC, div3: 0x0, mode: 1 }, /* MCLK=19.68Mhz */
    pll_div { fref: 19800000, fout: 3072000, div1: 0xBF, div2: 0x70, div3: 0x0, mode: 1 }, /* MCLK=19.8Mhz */
    /* for SLAVE mode with SRM */
    pll_div { fref: 12000000, fout: 2822400, div1: 0xED, div2: 0xBF, div3: 0x5, mode: 0 }, /* MCLK=12Mhz */
    pll_div { fref: 13000000, fout: 2822400, div1: 0xE4, div2: 0x13, div3: 0x0, mode: 0 }, /* MCLK=13Mhz */
    pll_div { fref: 13500000, fout: 2822400, div1: 0xDF, div2: 0xC6, div3: 0x8, mode: 0 }, /* MCLK=13.5Mhz */
    pll_div { fref: 14400000, fout: 2822400, div1: 0xD8, div2: 0xCA, div3: 0x1, mode: 0 }, /* MCLK=14.4Mhz */
    pll_div { fref: 19200000, fout: 2822400, div1: 0xBE, div2: 0x97, div3: 0x9, mode: 0 }, /* MCLK=19.2Mhz */
    pll_div { fref: 19680000, fout: 2822400, div1: 0xBC, div2: 0xAC, div3: 0xD, mode: 0 }, /* MCLK=19.68Mhz */
    pll_div { fref: 19800000, fout: 2822400, div1: 0xBC, div2: 0x35, div3: 0xE, mode: 0 }, /* MCLK=19.8Mhz  */
];

#[repr(C)]
pub enum clk_src {
    DA7210_CLKSRC_MCLK,
}

const DA7210_VERSION: *const c_char = b"0.0.1\0".as_ptr() as *const c_char;

/*
 * The DECLARE_TLV_DB_RANGE, DECLARE_TLV_DB_SCALE, SOC_ENUM_SINGLE_DECL,
 * SOC_* control, and SND_SOC_DAPM_* initializers are Linux ASoC macro data.
 * They are preserved here as dependency-facing static declarations/comments
 * because their concrete Rust layouts are supplied by external kernel headers.
 */
unsafe extern "C" {
    static hp_out_tlv: c_uint;
    static lineout_vol_tlv: c_uint;
    static mono_vol_tlv: c_uint;
    static aux1_vol_tlv: c_uint;
    static eq_gain_tlv: c_uint;
    static adc_eq_master_gain_tlv: c_uint;
    static dac_gain_tlv: c_uint;
    static mic_vol_tlv: c_uint;
    static aux2_vol_tlv: c_uint;
    static inpga_gain_tlv: c_uint;
    static da7210_dac_hpf_cutoff: c_void;
    static da7210_adc_hpf_cutoff: c_void;
    static da7210_dac_vf_cutoff: c_void;
    static da7210_adc_vf_cutoff: c_void;
    static da7210_hp_mode_sel: c_void;
}

static da7210_hpf_cutoff_txt_0: &[u8] = b"Fs/8192*pi\0";
static da7210_hpf_cutoff_txt_1: &[u8] = b"Fs/4096*pi\0";
static da7210_hpf_cutoff_txt_2: &[u8] = b"Fs/2048*pi\0";
static da7210_hpf_cutoff_txt_3: &[u8] = b"Fs/1024*pi\0";
static da7210_hpf_cutoff_txt: [*const c_char; 4] = [
    da7210_hpf_cutoff_txt_0.as_ptr() as *const c_char,
    da7210_hpf_cutoff_txt_1.as_ptr() as *const c_char,
    da7210_hpf_cutoff_txt_2.as_ptr() as *const c_char,
    da7210_hpf_cutoff_txt_3.as_ptr() as *const c_char,
];

static da7210_vf_cutoff_txt: [*const c_char; 8] = [
    b"2.5Hz\0".as_ptr() as *const c_char,
    b"25Hz\0".as_ptr() as *const c_char,
    b"50Hz\0".as_ptr() as *const c_char,
    b"100Hz\0".as_ptr() as *const c_char,
    b"150Hz\0".as_ptr() as *const c_char,
    b"200Hz\0".as_ptr() as *const c_char,
    b"300Hz\0".as_ptr() as *const c_char,
    b"400Hz\0".as_ptr() as *const c_char,
];

static mut da7210_hp_mode_txt: [*const c_char; 2] = [
    b"Class H\0".as_ptr() as *const c_char,
    b"Class G\0".as_ptr() as *const c_char,
];

/* ALC can be enabled only if noise suppression is disabled */
unsafe extern "C" fn da7210_put_alc_sw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);

    if (*core::ptr::addr_of!((*ucontrol).value.integer)).value[0] != 0 {
        /* Check if noise suppression is enabled */
        if snd_soc_component_read(component, DA7210_CONTROL) & DA7210_NOISE_SUP_EN != 0 {
            dev_dbg(
                (*component).dev,
                b"Disable noise suppression to enable ALC\n\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
    }
    /* If all conditions are met or we are actually disabling ALC */
    snd_soc_put_volsw(kcontrol, ucontrol)
}

/* Noise suppression can be enabled only if following conditions are met
 *  ALC disabled
 *  ZC enabled for HP and AUX1 PGA
 *  INPGA_L_VOL and INPGA_R_VOL >= 10.5 dB
 *  AUX1_L_VOL and AUX1_R_VOL >= 6 dB
 */
unsafe extern "C" fn da7210_put_noise_sup_sw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mut val: u8;

    if (*core::ptr::addr_of!((*ucontrol).value.integer)).value[0] != 0 {
        /* Check if ALC is enabled */
        if snd_soc_component_read(component, DA7210_ADC) & DA7210_ADC_ALC_EN != 0 {
            return -EINVAL;
        }

        /* Check ZC for HP and AUX1 PGA */
        if (snd_soc_component_read(component, DA7210_ZERO_CROSS)
            & (DA7210_AUX1_L_ZC | DA7210_AUX1_R_ZC | DA7210_HP_L_ZC | DA7210_HP_R_ZC))
            != (DA7210_AUX1_L_ZC | DA7210_AUX1_R_ZC | DA7210_HP_L_ZC | DA7210_HP_R_ZC)
        {
            return -EINVAL;
        }

        /* Check INPGA_L_VOL and INPGA_R_VOL */
        val = snd_soc_component_read(component, DA7210_IN_GAIN) as u8;
        if (((val as c_uint) & DA7210_INPGA_L_VOL) < DA7210_INPGA_MIN_VOL_NS)
            || ((((val as c_uint) & DA7210_INPGA_R_VOL) >> 4) < DA7210_INPGA_MIN_VOL_NS)
        {
            return -EINVAL;
        }

        /* Check AUX1_L_VOL and AUX1_R_VOL */
        if ((snd_soc_component_read(component, DA7210_AUX1_L) & DA7210_AUX1_L_VOL)
            < DA7210_AUX1_MIN_VOL_NS)
            || ((snd_soc_component_read(component, DA7210_AUX1_R) & DA7210_AUX1_R_VOL)
                < DA7210_AUX1_MIN_VOL_NS)
        {
            return -EINVAL;
        }
    }
    /* If all conditions are met or we are actually disabling Noise sup */
    snd_soc_put_volsw(kcontrol, ucontrol)
}

/* Original da7210_snd_controls[] contains SOC_DOUBLE_R_TLV, SOC_SINGLE_TLV,
 * SOC_DOUBLE_TLV, SOC_SINGLE, SOC_DOUBLE, SOC_ENUM, and SOC_SINGLE_EXT
 * initializers for headphone, DAC, lineout, mono, mic, aux, EQ, HPF, mute,
 * zero-cross, ALC, and noise-suppression controls.
 */
static da7210_snd_controls: [snd_kcontrol_new; 0] = [];

/* Original DAPM control arrays contain SOC_DAPM_SINGLE entries for in mixers,
 * out mixers, and mono mixer.
 */
static da7210_dapm_inmixl_controls: [snd_kcontrol_new; 0] = [];
static da7210_dapm_inmixr_controls: [snd_kcontrol_new; 0] = [];
static da7210_dapm_outmixl_controls: [snd_kcontrol_new; 0] = [];
static da7210_dapm_outmixr_controls: [snd_kcontrol_new; 0] = [];
static da7210_dapm_monomix_controls: [snd_kcontrol_new; 0] = [];

/* Original DAPM widgets include input lines MICL/MICR/AUX1L/AUX1R/AUX2,
 * input PGAs, Mic Bias, input mixers, ADCs, DACs, output mixers, output PGAs,
 * line/headphone/mono output PGAs, and outputs OUT1L/OUT1R/HPL/HPR/OUT2.
 */
static da7210_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

/* DAPM audio route definition */
static da7210_audio_map: [snd_soc_dapm_route; 43] = [
    route(b"Mic Left\0", core::ptr::null(), b"MICL\0"),
    route(b"Mic Right\0", core::ptr::null(), b"MICR\0"),
    route(b"Aux1 Left\0", core::ptr::null(), b"AUX1L\0"),
    route(b"Aux1 Right\0", core::ptr::null(), b"AUX1R\0"),
    route(b"Aux2 Mono\0", core::ptr::null(), b"AUX2\0"),
    route(b"In Mixer Left\0", b"Mic Left Switch\0", b"Mic Left\0"),
    route(b"In Mixer Left\0", b"Mic Right Switch\0", b"Mic Right\0"),
    route(b"In Mixer Left\0", b"Aux1 Left Switch\0", b"Aux1 Left\0"),
    route(b"In Mixer Left\0", b"Aux2 Switch\0", b"Aux2 Mono\0"),
    route(b"In Mixer Left\0", b"Outmix Left Switch\0", b"Out Mixer Left\0"),
    route(b"In Mixer Right\0", b"Mic Right Switch\0", b"Mic Right\0"),
    route(b"In Mixer Right\0", b"Mic Left Switch\0", b"Mic Left\0"),
    route(b"In Mixer Right\0", b"Aux1 Right Switch\0", b"Aux1 Right\0"),
    route(b"In Mixer Right\0", b"Aux2 Switch\0", b"Aux2 Mono\0"),
    route(b"In Mixer Right\0", b"Outmix Right Switch\0", b"Out Mixer Right\0"),
    route(b"INPGA Left\0", core::ptr::null(), b"In Mixer Left\0"),
    route(b"ADC Left\0", core::ptr::null(), b"INPGA Left\0"),
    route(b"INPGA Right\0", core::ptr::null(), b"In Mixer Right\0"),
    route(b"ADC Right\0", core::ptr::null(), b"INPGA Right\0"),
    route(b"Out Mixer Left\0", b"Aux1 Left Switch\0", b"Aux1 Left\0"),
    route(b"Out Mixer Left\0", b"Aux2 Switch\0", b"Aux2 Mono\0"),
    route(b"Out Mixer Left\0", b"INPGA Left Switch\0", b"INPGA Left\0"),
    route(b"Out Mixer Left\0", b"INPGA Right Switch\0", b"INPGA Right\0"),
    route(b"Out Mixer Left\0", b"DAC Left Switch\0", b"DAC Left\0"),
    route(b"Out Mixer Right\0", b"Aux1 Right Switch\0", b"Aux1 Right\0"),
    route(b"Out Mixer Right\0", b"Aux2 Switch\0", b"Aux2 Mono\0"),
    route(b"Out Mixer Right\0", b"INPGA Right Switch\0", b"INPGA Right\0"),
    route(b"Out Mixer Right\0", b"INPGA Left Switch\0", b"INPGA Left\0"),
    route(b"Out Mixer Right\0", b"DAC Right Switch\0", b"DAC Right\0"),
    route(b"Mono Mixer\0", b"INPGA Right Switch\0", b"INPGA Right\0"),
    route(b"Mono Mixer\0", b"INPGA Left Switch\0", b"INPGA Left\0"),
    route(b"Mono Mixer\0", b"Outmix Right Switch\0", b"Out Mixer Right\0"),
    route(b"Mono Mixer\0", b"Outmix Left Switch\0", b"Out Mixer Left\0"),
    route(b"OUTPGA Left Enable\0", core::ptr::null(), b"Out Mixer Left\0"),
    route(b"OUTPGA Right Enable\0", core::ptr::null(), b"Out Mixer Right\0"),
    route(b"Out1 Left\0", core::ptr::null(), b"OUTPGA Left Enable\0"),
    route(b"OUT1L\0", core::ptr::null(), b"Out1 Left\0"),
    route(b"Out1 Right\0", core::ptr::null(), b"OUTPGA Right Enable\0"),
    route(b"OUT1R\0", core::ptr::null(), b"Out1 Right\0"),
    route(b"Headphone Left\0", core::ptr::null(), b"OUTPGA Left Enable\0"),
    route(b"HPL\0", core::ptr::null(), b"Headphone Left\0"),
    route(b"Headphone Right\0", core::ptr::null(), b"OUTPGA Right Enable\0"),
    route(b"HPR\0", core::ptr::null(), b"Headphone Right\0"),
    route(b"Out2 Mono\0", core::ptr::null(), b"Mono Mixer\0"),
    route(b"OUT2\0", core::ptr::null(), b"Out2 Mono\0"),
];

const fn route(sink: &'static [u8], control: *const c_char, source: &'static [u8]) -> snd_soc_dapm_route {
    snd_soc_dapm_route {
        sink: sink.as_ptr() as *const c_char,
        control,
        source: source.as_ptr() as *const c_char,
    }
}

/* Codec private data */
#[repr(C)]
pub struct da7210_priv {
    pub regmap: *mut regmap,
    pub mclk_rate: c_uint,
    pub master: c_int,
}

static da7210_reg_defaults: [reg_default; 52] = [
    reg_default { reg: 0x00, def: 0x00 }, reg_default { reg: 0x01, def: 0x11 },
    reg_default { reg: 0x03, def: 0x00 }, reg_default { reg: 0x04, def: 0x00 },
    reg_default { reg: 0x05, def: 0x00 }, reg_default { reg: 0x06, def: 0x00 },
    reg_default { reg: 0x07, def: 0x00 }, reg_default { reg: 0x08, def: 0x00 },
    reg_default { reg: 0x09, def: 0x00 }, reg_default { reg: 0x0a, def: 0x00 },
    reg_default { reg: 0x0b, def: 0x00 }, reg_default { reg: 0x0c, def: 0x00 },
    reg_default { reg: 0x0d, def: 0x00 }, reg_default { reg: 0x0e, def: 0x00 },
    reg_default { reg: 0x0f, def: 0x08 }, reg_default { reg: 0x10, def: 0x00 },
    reg_default { reg: 0x11, def: 0x00 }, reg_default { reg: 0x12, def: 0x00 },
    reg_default { reg: 0x13, def: 0x00 }, reg_default { reg: 0x14, def: 0x08 },
    reg_default { reg: 0x15, def: 0x10 }, reg_default { reg: 0x16, def: 0x10 },
    reg_default { reg: 0x17, def: 0x54 }, reg_default { reg: 0x18, def: 0x40 },
    reg_default { reg: 0x19, def: 0x00 }, reg_default { reg: 0x1a, def: 0x00 },
    reg_default { reg: 0x1b, def: 0x00 }, reg_default { reg: 0x1c, def: 0x00 },
    reg_default { reg: 0x1d, def: 0x00 }, reg_default { reg: 0x1e, def: 0x00 },
    reg_default { reg: 0x1f, def: 0x00 }, reg_default { reg: 0x20, def: 0x00 },
    reg_default { reg: 0x21, def: 0x00 }, reg_default { reg: 0x22, def: 0x00 },
    reg_default { reg: 0x23, def: 0x02 }, reg_default { reg: 0x24, def: 0x00 },
    reg_default { reg: 0x25, def: 0x76 }, reg_default { reg: 0x26, def: 0x00 },
    reg_default { reg: 0x27, def: 0x00 }, reg_default { reg: 0x28, def: 0x04 },
    reg_default { reg: 0x29, def: 0x00 }, reg_default { reg: 0x2a, def: 0x00 },
    reg_default { reg: 0x2b, def: 0x30 }, reg_default { reg: 0x2c, def: 0x2A },
    reg_default { reg: 0x83, def: 0x00 }, reg_default { reg: 0x84, def: 0x00 },
    reg_default { reg: 0x85, def: 0x00 }, reg_default { reg: 0x86, def: 0x00 },
    reg_default { reg: 0x87, def: 0x00 }, reg_default { reg: 0x88, def: 0x00 },
];

unsafe extern "C" fn da7210_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        DA7210_A_HID_UNLOCK | DA7210_A_TEST_UNLOCK | DA7210_A_PLL1 | DA7210_A_CP_MODE => false,
        _ => true,
    }
}

unsafe extern "C" fn da7210_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        DA7210_STATUS => true,
        _ => false,
    }
}

/* Set PCM DAI word length. */
unsafe extern "C" fn da7210_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let da7210 = snd_soc_component_get_drvdata(component);
    let mut dai_cfg1: u32;
    let fs: u32;
    let sysclk: u32;

    snd_soc_component_write(component, DA7210_DAI_SRC_SEL, DA7210_DAI_OUT_R_SRC | DA7210_DAI_OUT_L_SRC);
    snd_soc_component_write(component, DA7210_DAI_CFG3, DA7210_DAI_OE | DA7210_DAI_EN);

    dai_cfg1 = 0xFC & snd_soc_component_read(component, DA7210_DAI_CFG1);

    match params_width(params) {
        16 => dai_cfg1 |= DA7210_DAI_WORD_S16_LE,
        20 => dai_cfg1 |= DA7210_DAI_WORD_S20_3LE,
        24 => dai_cfg1 |= DA7210_DAI_WORD_S24_LE,
        32 => dai_cfg1 |= DA7210_DAI_WORD_S32_LE,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, DA7210_DAI_CFG1, dai_cfg1);

    match params_rate(params) {
        8000 => { fs = DA7210_PLL_FS_8000; sysclk = 3072000; }
        11025 => { fs = DA7210_PLL_FS_11025; sysclk = 2822400; }
        12000 => { fs = DA7210_PLL_FS_12000; sysclk = 3072000; }
        16000 => { fs = DA7210_PLL_FS_16000; sysclk = 3072000; }
        22050 => { fs = DA7210_PLL_FS_22050; sysclk = 2822400; }
        32000 => { fs = DA7210_PLL_FS_32000; sysclk = 3072000; }
        44100 => { fs = DA7210_PLL_FS_44100; sysclk = 2822400; }
        48000 => { fs = DA7210_PLL_FS_48000; sysclk = 3072000; }
        88200 => { fs = DA7210_PLL_FS_88200; sysclk = 2822400; }
        96000 => { fs = DA7210_PLL_FS_96000; sysclk = 3072000; }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, DA7210_STARTUP1, DA7210_SC_MST_EN, 0);
    snd_soc_component_update_bits(component, DA7210_PLL, DA7210_PLL_FS_MASK, fs);

    if (*da7210).mclk_rate != 0 && (*da7210).mclk_rate != sysclk {
        snd_soc_component_update_bits(component, DA7210_PLL_DIV3, DA7210_PLL_BYP, 0);
        if (*da7210).master == 0 {
            snd_soc_component_update_bits(
                component,
                DA7210_PLL,
                DA7210_MCLK_SRM_EN | DA7210_MCLK_DET_EN,
                DA7210_MCLK_SRM_EN | DA7210_MCLK_DET_EN,
            );
        }
    } else {
        snd_soc_component_update_bits(component, DA7210_PLL, DA7210_MCLK_DET_EN, DA7210_MCLK_DET_EN);
        snd_soc_component_update_bits(component, DA7210_PLL_DIV3, DA7210_PLL_BYP, DA7210_PLL_BYP);
    }
    snd_soc_component_update_bits(component, DA7210_STARTUP1, DA7210_SC_MST_EN, DA7210_SC_MST_EN);
    0
}

/* Set DAI mode and Format */
unsafe extern "C" fn da7210_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: u32) -> c_int {
    let component = (*codec_dai).component;
    let da7210 = snd_soc_component_get_drvdata(component);
    let mut dai_cfg1: u32 = 0x7f & snd_soc_component_read(component, DA7210_DAI_CFG1);
    let mut dai_cfg3: u32 = 0xfc & snd_soc_component_read(component, DA7210_DAI_CFG3);

    if (snd_soc_component_read(component, DA7210_PLL) & DA7210_PLL_EN) != 0
        && (snd_soc_component_read(component, DA7210_PLL_DIV3) & DA7210_PLL_BYP) == 0
    {
        return -EINVAL;
    }

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            (*da7210).master = 1;
            dai_cfg1 |= DA7210_DAI_MODE_MASTER;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            (*da7210).master = 0;
            dai_cfg1 |= DA7210_DAI_MODE_SLAVE;
        }
        _ => return -EINVAL,
    }

    /* FIXME
     *
     * It support I2S only now
     */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => dai_cfg3 |= DA7210_DAI_FORMAT_I2SMODE,
        SND_SOC_DAIFMT_LEFT_J => dai_cfg3 |= DA7210_DAI_FORMAT_LEFT_J,
        SND_SOC_DAIFMT_RIGHT_J => dai_cfg3 |= DA7210_DAI_FORMAT_RIGHT_J,
        _ => return -EINVAL,
    }

    /* FIXME
     *
     * It support 64bit data transmission only now
     */
    dai_cfg1 |= DA7210_DAI_FLEN_64BIT;

    snd_soc_component_write(component, DA7210_DAI_CFG1, dai_cfg1);
    snd_soc_component_write(component, DA7210_DAI_CFG3, dai_cfg3);
    0
}

unsafe extern "C" fn da7210_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let mute_reg: u8 = (snd_soc_component_read(component, DA7210_DAC_HPF) & 0xFB) as u8;

    if mute != 0 {
        snd_soc_component_write(component, DA7210_DAC_HPF, (mute_reg | 0x4) as c_uint);
    } else {
        snd_soc_component_write(component, DA7210_DAC_HPF, mute_reg as c_uint);
    }
    0
}

const DA7210_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

unsafe extern "C" fn da7210_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let da7210 = snd_soc_component_get_drvdata(component);

    match clk_id {
        x if x == clk_src::DA7210_CLKSRC_MCLK as c_int => match freq {
            12000000 | 13000000 | 13500000 | 14400000 | 19200000 | 19680000 | 19800000 => {
                (*da7210).mclk_rate = freq;
                0
            }
            _ => {
                dev_err((*codec_dai).dev, b"Unsupported MCLK value %d\n\0".as_ptr() as *const c_char, freq);
                -EINVAL
            }
        },
        _ => {
            dev_err((*codec_dai).dev, b"Unknown clock source %d\n\0".as_ptr() as *const c_char, clk_id);
            -EINVAL
        }
    }
}

/**
 * da7210_set_dai_pll	:Configure the codec PLL
 * @codec_dai: pointer to codec DAI
 * @pll_id: da7210 has only one pll, so pll_id is always zero
 * @source: clock source
 * @fref: MCLK frequency, should be < 20MHz
 * @fout: FsDM value, Refer page 44 & 45 of datasheet
 *
 * Note: Supported PLL input frequencies are 12MHz, 13MHz, 13.5MHz, 14.4MHz,
 *       19.2MHz, 19.6MHz and 19.8MHz
 *
 * Return: Zero for success, negative error code for error
 */
unsafe extern "C" fn da7210_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    fref: c_uint,
    mut fout: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let da7210 = snd_soc_component_get_drvdata(component);
    let mut pll_div1: u8 = 0;
    let mut pll_div2: u8 = 0;
    let mut pll_div3: u8 = 0;
    let mut cnt: usize = 0;

    /* In slave mode, there is only one set of divisors */
    if (*da7210).master == 0 {
        fout = 2822400;
    }

    /* Search pll div array for correct divisors */
    while cnt < da7210_pll_div.len() {
        /* check fref, mode  and fout */
        if fref as c_int == da7210_pll_div[cnt].fref
            && (*da7210).master == da7210_pll_div[cnt].mode as c_int
            && fout as c_int == da7210_pll_div[cnt].fout
        {
            /* all match, pick up divisors */
            pll_div1 = da7210_pll_div[cnt].div1;
            pll_div2 = da7210_pll_div[cnt].div2;
            pll_div3 = da7210_pll_div[cnt].div3;
            break;
        }
        cnt += 1;
    }
    if cnt >= da7210_pll_div.len() {
        dev_err((*codec_dai).dev, b"Unsupported PLL input frequency %d\n\0".as_ptr() as *const c_char, fref);
        return -EINVAL;
    }

    snd_soc_component_update_bits(component, DA7210_STARTUP1, DA7210_SC_MST_EN, 0);
    snd_soc_component_write(component, DA7210_PLL_DIV1, pll_div1 as c_uint);
    snd_soc_component_write(component, DA7210_PLL_DIV2, pll_div2 as c_uint);
    snd_soc_component_update_bits(component, DA7210_PLL_DIV3, DA7210_PLL_DIV_L_MASK, pll_div3 as c_uint);
    snd_soc_component_update_bits(component, DA7210_PLL, DA7210_PLL_EN, DA7210_PLL_EN);
    snd_soc_component_update_bits(component, DA7210_STARTUP1, DA7210_SC_MST_EN, DA7210_SC_MST_EN);
    0
}

/* DAI operations */
static da7210_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(da7210_hw_params),
    set_fmt: Some(da7210_set_dai_fmt),
    set_sysclk: Some(da7210_set_dai_sysclk),
    set_pll: Some(da7210_set_dai_pll),
    mute_stream: Some(da7210_mute),
    no_capture_mute: 1,
};

static mut da7210_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"da7210-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: DA7210_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: DA7210_FORMATS,
    },
    ops: &da7210_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn da7210_probe(component: *mut snd_soc_component) -> c_int {
    let da7210 = snd_soc_component_get_drvdata(component);

    dev_info((*component).dev, b"DA7210 Audio Codec %s\n\0".as_ptr() as *const c_char, DA7210_VERSION);

    (*da7210).mclk_rate = 0; /* This will be set from set_sysclk() */
    (*da7210).master = 0; /* This will be set from set_fmt() */

    snd_soc_component_write(component, DA7210_CONTROL, DA7210_REG_EN | DA7210_BIAS_EN);

    /* ADC settings */
    snd_soc_component_write(component, DA7210_MIC_L, DA7210_MIC_L_EN | DA7210_MICBIAS_EN);
    snd_soc_component_write(component, DA7210_MIC_R, DA7210_MIC_R_EN);
    snd_soc_component_write(component, DA7210_INMIX_L, DA7210_IN_L_EN);
    snd_soc_component_write(component, DA7210_INMIX_R, DA7210_IN_R_EN);
    snd_soc_component_write(component, DA7210_ADC, DA7210_ADC_L_EN | DA7210_ADC_R_EN);

    /* DAC settings */
    snd_soc_component_write(
        component,
        DA7210_DAC_SEL,
        DA7210_DAC_L_SRC_DAI_L | DA7210_DAC_L_EN | DA7210_DAC_R_SRC_DAI_R | DA7210_DAC_R_EN,
    );
    snd_soc_component_write(component, DA7210_OUTMIX_L, DA7210_OUT_L_EN);
    snd_soc_component_write(component, DA7210_OUTMIX_R, DA7210_OUT_R_EN);
    snd_soc_component_write(
        component,
        DA7210_HP_CFG,
        DA7210_HP_2CAP_MODE | DA7210_HP_SENSE_EN | DA7210_HP_L_EN | DA7210_HP_MODE | DA7210_HP_R_EN,
    );
    snd_soc_component_write(component, DA7210_SOFTMUTE, DA7210_RAMP_EN);

    /*
     * For DA7210 codec, there are two ways to enable/disable analog IOs
     * and ADC/DAC,
     * (1) Using "Enable Bit" of register associated with that IO
     * (or ADC/DAC)
     *	e.g. Mic Left can be enabled using bit 7 of MIC_L(0x7) reg
     *
     * (2) Using "Standby Bit" of STARTUP2 or STARTUP3 register
     *	e.g. Mic left can be put to STANDBY using bit 0 of STARTUP3(0x5)
     *
     * Out of these two methods, the one using STANDBY bits is preferred
     * way to enable/disable individual blocks. This is because STANDBY
     * registers are part of system controller which allows system power
     * up/down in a controlled, pop-free manner. Also, as per application
     * note of DA7210, STANDBY register bits are only effective if a
     * particular IO (or ADC/DAC) is already enabled using enable/disable
     * register bits. Keeping these things in mind, current DAPM
     * implementation manipulates only STANDBY bits.
     *
     * Overall implementation can be outlined as below,
     *
     * - "Enable bit" of an IO or ADC/DAC is used to enable it in probe()
     * - "STANDBY bit" is controlled by DAPM
     */
    snd_soc_component_write(component, DA7210_OUT1_L, DA7210_OUT1_L_EN);
    snd_soc_component_write(component, DA7210_OUT1_R, DA7210_OUT1_R_EN);
    snd_soc_component_write(component, DA7210_OUT2, DA7210_OUT2_EN | DA7210_OUT2_OUTMIX_L | DA7210_OUT2_OUTMIX_R);
    snd_soc_component_write(component, DA7210_AUX1_L, DA7210_AUX1_L_EN);
    snd_soc_component_write(component, DA7210_AUX1_R, DA7210_AUX1_R_EN);
    snd_soc_component_write(component, DA7210_AUX2, DA7210_AUX2_EN);
    snd_soc_component_write(component, DA7210_PLL_DIV3, DA7210_MCLK_RANGE_10_20_MHZ | DA7210_PLL_BYP);
    /* Diable PLL and bypass it */
    snd_soc_component_write(component, DA7210_PLL, DA7210_PLL_FS_48000);
    snd_soc_component_write(component, DA7210_STARTUP1, DA7210_SC_MST_EN);

    dev_info((*component).dev, b"DA7210 Audio Codec %s\n\0".as_ptr() as *const c_char, DA7210_VERSION);
    0
}

static soc_component_dev_da7210: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(da7210_probe),
    controls: da7210_snd_controls.as_ptr(),
    num_controls: da7210_snd_controls.len() as c_uint,
    dapm_widgets: da7210_dapm_widgets.as_ptr(),
    num_dapm_widgets: da7210_dapm_widgets.len() as c_uint,
    dapm_routes: da7210_audio_map.as_ptr(),
    num_dapm_routes: da7210_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

/* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
#[cfg(CONFIG_I2C)]
static da7210_regmap_i2c_patch: [reg_sequence; 8] = [
    reg_sequence { reg: DA7210_STARTUP1, def: 0x00 },
    reg_sequence { reg: DA7210_PLL_DIV3, def: DA7210_MCLK_RANGE_10_20_MHZ },
    reg_sequence { reg: DA7210_A_HID_UNLOCK, def: 0x8B },
    reg_sequence { reg: DA7210_A_TEST_UNLOCK, def: 0xB4 },
    reg_sequence { reg: DA7210_A_PLL1, def: 0x01 },
    reg_sequence { reg: DA7210_A_CP_MODE, def: 0x7C },
    reg_sequence { reg: DA7210_A_HID_UNLOCK, def: 0x00 },
    reg_sequence { reg: DA7210_A_TEST_UNLOCK, def: 0x00 },
];

#[cfg(CONFIG_I2C)]
static da7210_regmap_config_i2c: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    read_flag_mask: 0,
    write_flag_mask: 0,
    reg_defaults: da7210_reg_defaults.as_ptr(),
    num_reg_defaults: da7210_reg_defaults.len() as c_uint,
    volatile_reg: Some(da7210_volatile_register),
    readable_reg: Some(da7210_readable_register),
    cache_type: REGCACHE_RBTREE,
};

#[cfg(CONFIG_I2C)]
unsafe extern "C" fn da7210_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let da7210: *mut da7210_priv;
    let mut ret: c_int;

    da7210 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<da7210_priv>(), GFP_KERNEL) as *mut da7210_priv;
    if da7210.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, da7210 as *mut c_void);
    (*da7210).regmap = devm_regmap_init_i2c(i2c, &da7210_regmap_config_i2c);
    if IS_ERR((*da7210).regmap as *const c_void) {
        ret = PTR_ERR((*da7210).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, b"regmap_init() failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regmap_register_patch((*da7210).regmap, da7210_regmap_i2c_patch.as_ptr(), da7210_regmap_i2c_patch.len() as c_int);
    if ret != 0 {
        dev_warn(&mut (*i2c).dev, b"Failed to apply regmap patch: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_da7210, &mut da7210_dai, 1);
    if ret < 0 {
        dev_err(&mut (*i2c).dev, b"Failed to register component: %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret
}

#[cfg(CONFIG_I2C)]
static da7210_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'd' as c_char, b'a' as c_char, b'7' as c_char, b'2' as c_char, b'1' as c_char, b'0' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, da7210_i2c_id); */

#[cfg(CONFIG_I2C)]
static mut da7210_i2c_driver: i2c_driver = i2c_driver {
    driver: i2c_driver_driver { name: b"da7210\0".as_ptr() as *const c_char },
    probe: Some(da7210_i2c_probe),
    id_table: da7210_i2c_id.as_ptr(),
};

/* Original C condition: #if defined(CONFIG_SPI_MASTER) */
#[cfg(CONFIG_SPI_MASTER)]
static da7210_regmap_spi_patch: [reg_sequence; 12] = [
    reg_sequence { reg: DA7210_AUX2, def: 0x00 },
    reg_sequence { reg: DA7210_AUX2, def: 0x00 },
    reg_sequence { reg: DA7210_STARTUP1, def: 0x00 },
    reg_sequence { reg: DA7210_PLL_DIV3, def: DA7210_MCLK_RANGE_10_20_MHZ },
    reg_sequence { reg: DA7210_PAGE_CONTROL, def: 0x80 },
    reg_sequence { reg: DA7210_A_HID_UNLOCK, def: 0x8B },
    reg_sequence { reg: DA7210_A_TEST_UNLOCK, def: 0xB4 },
    reg_sequence { reg: DA7210_A_PLL1, def: 0x01 },
    reg_sequence { reg: DA7210_A_CP_MODE, def: 0x7C },
    reg_sequence { reg: DA7210_A_HID_UNLOCK, def: 0x00 },
    reg_sequence { reg: DA7210_A_TEST_UNLOCK, def: 0x00 },
    reg_sequence { reg: DA7210_PAGE_CONTROL, def: 0x00 },
];

#[cfg(CONFIG_SPI_MASTER)]
static da7210_regmap_config_spi: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    read_flag_mask: 0x01,
    write_flag_mask: 0x00,
    reg_defaults: da7210_reg_defaults.as_ptr(),
    num_reg_defaults: da7210_reg_defaults.len() as c_uint,
    volatile_reg: Some(da7210_volatile_register),
    readable_reg: Some(da7210_readable_register),
    cache_type: REGCACHE_RBTREE,
};

#[cfg(CONFIG_SPI_MASTER)]
unsafe extern "C" fn da7210_spi_probe(spi: *mut spi_device) -> c_int {
    let da7210: *mut da7210_priv;
    let mut ret: c_int;

    da7210 = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<da7210_priv>(), GFP_KERNEL) as *mut da7210_priv;
    if da7210.is_null() {
        return -ENOMEM;
    }

    spi_set_drvdata(spi, da7210 as *mut c_void);
    (*da7210).regmap = devm_regmap_init_spi(spi, &da7210_regmap_config_spi);
    if IS_ERR((*da7210).regmap as *const c_void) {
        ret = PTR_ERR((*da7210).regmap as *const c_void);
        dev_err(&mut (*spi).dev, b"Failed to register regmap: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regmap_register_patch((*da7210).regmap, da7210_regmap_spi_patch.as_ptr(), da7210_regmap_spi_patch.len() as c_int);
    if ret != 0 {
        dev_warn(&mut (*spi).dev, b"Failed to apply regmap patch: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret = devm_snd_soc_register_component(&mut (*spi).dev, &soc_component_dev_da7210, &mut da7210_dai, 1);
    ret
}

#[cfg(CONFIG_SPI_MASTER)]
static mut da7210_spi_driver: spi_driver = spi_driver {
    driver: spi_driver_driver { name: b"da7210\0".as_ptr() as *const c_char },
    probe: Some(da7210_spi_probe),
};

unsafe extern "C" fn da7210_modinit() -> c_int {
    let mut ret: c_int = 0;
    /* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
    #[cfg(CONFIG_I2C)]
    {
        ret = i2c_add_driver(&mut da7210_i2c_driver);
        if ret != 0 {
            return ret;
        }
    }
    /* Original C condition: #if defined(CONFIG_SPI_MASTER) */
    #[cfg(CONFIG_SPI_MASTER)]
    {
        ret = spi_register_driver(&mut da7210_spi_driver);
        if ret != 0 {
            printk(b"Failed to register da7210 SPI driver: %d\n\0".as_ptr() as *const c_char, ret);
        }
    }
    ret
}
/* module_init(da7210_modinit); */

unsafe extern "C" fn da7210_exit() {
    /* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
    #[cfg(CONFIG_I2C)]
    {
        i2c_del_driver(&mut da7210_i2c_driver);
    }
    /* Original C condition: #if defined(CONFIG_SPI_MASTER) */
    #[cfg(CONFIG_SPI_MASTER)]
    {
        spi_unregister_driver(&mut da7210_spi_driver);
    }
}
/* module_exit(da7210_exit); */

/* MODULE_DESCRIPTION("ASoC DA7210 driver"); */
/* MODULE_AUTHOR("David Chen, Kuninori Morimoto"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
