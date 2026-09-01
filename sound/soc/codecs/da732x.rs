// SPDX-License-Identifier: GPL-2.0-only
/*
 * da732x.rs --- Dialog DA732X ALSA SoC Audio Driver
 *
 * Copyright (C) 2012 Dialog Semiconductor GmbH
 *
 * Author: Michal Hajduk <Michal.Hajduk@diasemi.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub reg: c_uint,
    pub shift: c_uint,
    pub dapm: *mut snd_soc_dapm_context,
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
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub base: c_uint,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub driver: *mut snd_soc_dai_driver,
    pub id: c_int,
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
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_opaque {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_opaque,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

pub type snd_soc_bias_level = c_uint;

extern "C" {
    static DA732X_REG_REF1: c_uint;
    static DA732X_REG_BIAS_EN: c_uint;
    static DA732X_REG_BIAS1: c_uint;
    static DA732X_REG_BIAS2: c_uint;
    static DA732X_REG_BIAS3: c_uint;
    static DA732X_REG_BIAS4: c_uint;
    static DA732X_REG_MICBIAS2: c_uint;
    static DA732X_REG_MICBIAS1: c_uint;
    static DA732X_REG_MICDET: c_uint;
    static DA732X_REG_MIC1_PRE: c_uint;
    static DA732X_REG_MIC1: c_uint;
    static DA732X_REG_MIC2_PRE: c_uint;
    static DA732X_REG_MIC2: c_uint;
    static DA732X_REG_AUX1L: c_uint;
    static DA732X_REG_AUX1R: c_uint;
    static DA732X_REG_MIC3_PRE: c_uint;
    static DA732X_REG_MIC3: c_uint;
    static DA732X_REG_INP_PINBIAS: c_uint;
    static DA732X_REG_INP_ZC_EN: c_uint;
    static DA732X_REG_INP_MUX: c_uint;
    static DA732X_REG_HP_DET: c_uint;
    static DA732X_REG_HPL_DAC_OFFSET: c_uint;
    static DA732X_REG_HPL_DAC_OFF_CNTL: c_uint;
    static DA732X_REG_HPL_OUT_OFFSET: c_uint;
    static DA732X_REG_HPL: c_uint;
    static DA732X_REG_HPL_VOL: c_uint;
    static DA732X_REG_HPR_DAC_OFFSET: c_uint;
    static DA732X_REG_HPR_DAC_OFF_CNTL: c_uint;
    static DA732X_REG_HPR_OUT_OFFSET: c_uint;
    static DA732X_REG_HPR: c_uint;
    static DA732X_REG_HPR_VOL: c_uint;
    static DA732X_REG_LIN2: c_uint;
    static DA732X_REG_LIN3: c_uint;
    static DA732X_REG_LIN4: c_uint;
    static DA732X_REG_OUT_ZC_EN: c_uint;
    static DA732X_REG_HP_LIN1_GNDSEL: c_uint;
    static DA732X_REG_CP_HP1: c_uint;
    static DA732X_REG_CP_HP2: c_uint;
    static DA732X_REG_CP_CTRL1: c_uint;
    static DA732X_REG_CP_CTRL2: c_uint;
    static DA732X_REG_CP_CTRL3: c_uint;
    static DA732X_REG_CP_LEVEL_MASK: c_uint;
    static DA732X_REG_CP_DET: c_uint;
    static DA732X_REG_CP_STATUS: c_uint;
    static DA732X_REG_CP_THRESH1: c_uint;
    static DA732X_REG_CP_THRESH2: c_uint;
    static DA732X_REG_CP_THRESH3: c_uint;
    static DA732X_REG_CP_THRESH4: c_uint;
    static DA732X_REG_CP_THRESH5: c_uint;
    static DA732X_REG_CP_THRESH6: c_uint;
    static DA732X_REG_CP_THRESH7: c_uint;
    static DA732X_REG_CP_THRESH8: c_uint;
    static DA732X_REG_PLL_DIV_LO: c_uint;
    static DA732X_REG_PLL_DIV_MID: c_uint;
    static DA732X_REG_PLL_DIV_HI: c_uint;
    static DA732X_REG_PLL_CTRL: c_uint;
    static DA732X_REG_CLK_CTRL: c_uint;
    static DA732X_REG_CLK_DSP: c_uint;
    static DA732X_REG_CLK_EN1: c_uint;
    static DA732X_REG_CLK_EN2: c_uint;
    static DA732X_REG_CLK_EN3: c_uint;
    static DA732X_REG_CLK_EN4: c_uint;
    static DA732X_REG_CLK_EN5: c_uint;
    static DA732X_REG_AIF_MCLK: c_uint;
    static DA732X_REG_AIFA1: c_uint;
    static DA732X_REG_AIFA2: c_uint;
    static DA732X_REG_AIFA3: c_uint;
    static DA732X_REG_AIFB1: c_uint;
    static DA732X_REG_AIFB2: c_uint;
    static DA732X_REG_AIFB3: c_uint;
    static DA732X_REG_PC_CTRL: c_uint;
    static DA732X_REG_DATA_ROUTE: c_uint;
    static DA732X_REG_DSP_CTRL: c_uint;
    static DA732X_REG_CIF_CTRL2: c_uint;
    static DA732X_REG_HANDSHAKE: c_uint;
    static DA732X_REG_SPARE1_OUT: c_uint;
    static DA732X_REG_SPARE2_OUT: c_uint;
    static DA732X_REG_SPARE1_IN: c_uint;
    static DA732X_REG_ADC1_PD: c_uint;
    static DA732X_REG_ADC1_HPF: c_uint;
    static DA732X_REG_ADC1_SEL: c_uint;
    static DA732X_REG_ADC1_EQ12: c_uint;
    static DA732X_REG_ADC1_EQ34: c_uint;
    static DA732X_REG_ADC1_EQ5: c_uint;
    static DA732X_REG_ADC2_PD: c_uint;
    static DA732X_REG_ADC2_HPF: c_uint;
    static DA732X_REG_ADC2_SEL: c_uint;
    static DA732X_REG_ADC2_EQ12: c_uint;
    static DA732X_REG_ADC2_EQ34: c_uint;
    static DA732X_REG_ADC2_EQ5: c_uint;
    static DA732X_REG_DAC1_HPF: c_uint;
    static DA732X_REG_DAC1_L_VOL: c_uint;
    static DA732X_REG_DAC1_R_VOL: c_uint;
    static DA732X_REG_DAC1_SEL: c_uint;
    static DA732X_REG_DAC1_SOFTMUTE: c_uint;
    static DA732X_REG_DAC1_EQ12: c_uint;
    static DA732X_REG_DAC1_EQ34: c_uint;
    static DA732X_REG_DAC1_EQ5: c_uint;
    static DA732X_REG_DAC2_HPF: c_uint;
    static DA732X_REG_DAC2_L_VOL: c_uint;
    static DA732X_REG_DAC2_R_VOL: c_uint;
    static DA732X_REG_DAC2_SEL: c_uint;
    static DA732X_REG_DAC2_SOFTMUTE: c_uint;
    static DA732X_REG_DAC2_EQ12: c_uint;
    static DA732X_REG_DAC2_EQ34: c_uint;
    static DA732X_REG_DAC2_EQ5: c_uint;
    static DA732X_REG_DAC3_HPF: c_uint;
    static DA732X_REG_DAC3_VOL: c_uint;
    static DA732X_REG_DAC3_SEL: c_uint;
    static DA732X_REG_DAC3_SOFTMUTE: c_uint;
    static DA732X_REG_DAC3_EQ12: c_uint;
    static DA732X_REG_DAC3_EQ34: c_uint;
    static DA732X_REG_DAC3_EQ5: c_uint;
    static DA732X_REG_BIQ_BYP: c_uint;
    static DA732X_REG_DMA_CMD: c_uint;
    static DA732X_REG_DMA_ADDR0: c_uint;
    static DA732X_REG_DMA_ADDR1: c_uint;
    static DA732X_REG_DMA_DATA0: c_uint;
    static DA732X_REG_DMA_DATA1: c_uint;
    static DA732X_REG_DMA_DATA2: c_uint;
    static DA732X_REG_DMA_DATA3: c_uint;
    static DA732X_REG_UNLOCK: c_uint;
    static DA732X_REG_ID: c_uint;
    static DA732X_MAX_REG: c_uint;

    static DA732X_MCLK_10MHZ: c_int;
    static DA732X_MCLK_20MHZ: c_int;
    static DA732X_MCLK_40MHZ: c_int;
    static DA732X_MCLK_54MHZ: c_int;
    static DA732X_MCLK_VAL_0_10MHZ: c_int;
    static DA732X_MCLK_VAL_10_20MHZ: c_int;
    static DA732X_MCLK_VAL_20_40MHZ: c_int;
    static DA732X_MCLK_VAL_40_54MHZ: c_int;

    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn msleep(ms: c_uint);
    fn mdelay(ms: c_uint);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SND_SOC_NOPM: c_uint = 0;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 1;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 2;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 3;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 4;
const SND_SOC_DAPM_POST_PMU: c_int = 0x2;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x8;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 10;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;

extern "C" {
    static DA732X_ENABLE_CP: c_int;
    static DA732X_DISABLE_CP: c_int;
    static DA732X_CP_CLK_EN: c_uint;
    static DA732X_HP_CP_EN: c_uint;
    static DA732X_HP_CP_REG: c_uint;
    static DA732X_HP_CP_PULSESKIP: c_uint;
    static DA732X_CP_EN: c_uint;
    static DA732X_CP_CTRL_CPVDD1: c_uint;
    static DA732X_CP_MANAGE_MAGNITUDE: c_uint;
    static DA732X_CP_BOOST: c_uint;
    static DA732X_CP_1MHZ: c_uint;
    static DA732X_CP_CLK_DIS: c_uint;
    static DA732X_HP_CP_DIS: c_uint;
    static DA723X_CP_DIS: c_uint;
    static DA732X_HPF_MODE_SHIFT: c_uint;
    static DA732X_HPF_MUSIC_SHIFT: c_uint;
    static DA732X_HPF_VOICE_SHIFT: c_uint;
    static DA732X_HPF_DISABLED: c_uint;
    static DA732X_HPF_VOICE: c_uint;
    static DA732X_HPF_MUSIC: c_uint;
    static DA732X_HPF_DIS: c_uint;
    static DA732X_HPF_VOICE_EN: c_uint;
    static DA732X_HPF_MUSIC_EN: c_uint;
    static DA732X_HPF_MASK: c_uint;
    static DA732X_ADCA_BB_CLK_EN: c_uint;
    static DA732X_ADCC_BB_CLK_EN: c_uint;
    static DA732X_ADC_RST_MASK: c_uint;
    static DA732X_ADC_SET_ACT: c_uint;
    static DA732X_ADC_PD_MASK: c_uint;
    static DA732X_ADC_ON: c_uint;
    static DA732X_ADC_OFF: c_uint;
    static DA732X_ADC_SET_RST: c_uint;
    static DA732X_OUT_HIZ_EN: c_uint;
    static DA732X_OUT_HIZ_DIS: c_uint;
}

#[repr(C)]
pub struct da732x_priv {
    pub regmap: *mut regmap,
    pub sysclk: c_uint,
    pub pll_en: bool,
}

/*
 * da732x register cache - default settings
 */
static mut da732x_reg_cache: [reg_default; 129] = unsafe {
    [
        reg_default { reg: DA732X_REG_REF1, def: 0x02 },
        reg_default { reg: DA732X_REG_BIAS_EN, def: 0x80 },
        reg_default { reg: DA732X_REG_BIAS1, def: 0x00 },
        reg_default { reg: DA732X_REG_BIAS2, def: 0x00 },
        reg_default { reg: DA732X_REG_BIAS3, def: 0x00 },
        reg_default { reg: DA732X_REG_BIAS4, def: 0x00 },
        reg_default { reg: DA732X_REG_MICBIAS2, def: 0x00 },
        reg_default { reg: DA732X_REG_MICBIAS1, def: 0x00 },
        reg_default { reg: DA732X_REG_MICDET, def: 0x00 },
        reg_default { reg: DA732X_REG_MIC1_PRE, def: 0x01 },
        reg_default { reg: DA732X_REG_MIC1, def: 0x40 },
        reg_default { reg: DA732X_REG_MIC2_PRE, def: 0x01 },
        reg_default { reg: DA732X_REG_MIC2, def: 0x40 },
        reg_default { reg: DA732X_REG_AUX1L, def: 0x75 },
        reg_default { reg: DA732X_REG_AUX1R, def: 0x75 },
        reg_default { reg: DA732X_REG_MIC3_PRE, def: 0x01 },
        reg_default { reg: DA732X_REG_MIC3, def: 0x40 },
        reg_default { reg: DA732X_REG_INP_PINBIAS, def: 0x00 },
        reg_default { reg: DA732X_REG_INP_ZC_EN, def: 0x00 },
        reg_default { reg: DA732X_REG_INP_MUX, def: 0x50 },
        reg_default { reg: DA732X_REG_HP_DET, def: 0x00 },
        reg_default { reg: DA732X_REG_HPL_DAC_OFFSET, def: 0x00 },
        reg_default { reg: DA732X_REG_HPL_DAC_OFF_CNTL, def: 0x00 },
        reg_default { reg: DA732X_REG_HPL_OUT_OFFSET, def: 0x00 },
        reg_default { reg: DA732X_REG_HPL, def: 0x40 },
        reg_default { reg: DA732X_REG_HPL_VOL, def: 0x0F },
        reg_default { reg: DA732X_REG_HPR_DAC_OFFSET, def: 0x00 },
        reg_default { reg: DA732X_REG_HPR_DAC_OFF_CNTL, def: 0x00 },
        reg_default { reg: DA732X_REG_HPR_OUT_OFFSET, def: 0x00 },
        reg_default { reg: DA732X_REG_HPR, def: 0x40 },
        reg_default { reg: DA732X_REG_HPR_VOL, def: 0x0F },
        reg_default { reg: DA732X_REG_LIN2, def: 0x4F },
        reg_default { reg: DA732X_REG_LIN3, def: 0x4F },
        reg_default { reg: DA732X_REG_LIN4, def: 0x4F },
        reg_default { reg: DA732X_REG_OUT_ZC_EN, def: 0x00 },
        reg_default { reg: DA732X_REG_HP_LIN1_GNDSEL, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_HP1, def: 0x0C },
        reg_default { reg: DA732X_REG_CP_HP2, def: 0x03 },
        reg_default { reg: DA732X_REG_CP_CTRL1, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_CTRL2, def: 0x99 },
        reg_default { reg: DA732X_REG_CP_CTRL3, def: 0x25 },
        reg_default { reg: DA732X_REG_CP_LEVEL_MASK, def: 0x3F },
        reg_default { reg: DA732X_REG_CP_DET, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_STATUS, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_THRESH1, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_THRESH2, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_THRESH3, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_THRESH4, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_THRESH5, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_THRESH6, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_THRESH7, def: 0x00 },
        reg_default { reg: DA732X_REG_CP_THRESH8, def: 0x00 },
        reg_default { reg: DA732X_REG_PLL_DIV_LO, def: 0x00 },
        reg_default { reg: DA732X_REG_PLL_DIV_MID, def: 0x00 },
        reg_default { reg: DA732X_REG_PLL_DIV_HI, def: 0x00 },
        reg_default { reg: DA732X_REG_PLL_CTRL, def: 0x02 },
        reg_default { reg: DA732X_REG_CLK_CTRL, def: 0xaa },
        reg_default { reg: DA732X_REG_CLK_DSP, def: 0x07 },
        reg_default { reg: DA732X_REG_CLK_EN1, def: 0x00 },
        reg_default { reg: DA732X_REG_CLK_EN2, def: 0x00 },
        reg_default { reg: DA732X_REG_CLK_EN3, def: 0x00 },
        reg_default { reg: DA732X_REG_CLK_EN4, def: 0x00 },
        reg_default { reg: DA732X_REG_CLK_EN5, def: 0x00 },
        reg_default { reg: DA732X_REG_AIF_MCLK, def: 0x00 },
        reg_default { reg: DA732X_REG_AIFA1, def: 0x02 },
        reg_default { reg: DA732X_REG_AIFA2, def: 0x00 },
        reg_default { reg: DA732X_REG_AIFA3, def: 0x08 },
        reg_default { reg: DA732X_REG_AIFB1, def: 0x02 },
        reg_default { reg: DA732X_REG_AIFB2, def: 0x00 },
        reg_default { reg: DA732X_REG_AIFB3, def: 0x08 },
        reg_default { reg: DA732X_REG_PC_CTRL, def: 0xC0 },
        reg_default { reg: DA732X_REG_DATA_ROUTE, def: 0x00 },
        reg_default { reg: DA732X_REG_DSP_CTRL, def: 0x00 },
        reg_default { reg: DA732X_REG_CIF_CTRL2, def: 0x00 },
        reg_default { reg: DA732X_REG_HANDSHAKE, def: 0x00 },
        reg_default { reg: DA732X_REG_SPARE1_OUT, def: 0x00 },
        reg_default { reg: DA732X_REG_SPARE2_OUT, def: 0x00 },
        reg_default { reg: DA732X_REG_SPARE1_IN, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC1_PD, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC1_HPF, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC1_SEL, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC1_EQ12, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC1_EQ34, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC1_EQ5, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC2_PD, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC2_HPF, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC2_SEL, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC2_EQ12, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC2_EQ34, def: 0x00 },
        reg_default { reg: DA732X_REG_ADC2_EQ5, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC1_HPF, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC1_L_VOL, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC1_R_VOL, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC1_SEL, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC1_SOFTMUTE, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC1_EQ12, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC1_EQ34, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC1_EQ5, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC2_HPF, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC2_L_VOL, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC2_R_VOL, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC2_SEL, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC2_SOFTMUTE, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC2_EQ12, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC2_EQ34, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC2_EQ5, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC3_HPF, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC3_VOL, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC3_SEL, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC3_SOFTMUTE, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC3_EQ12, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC3_EQ34, def: 0x00 },
        reg_default { reg: DA732X_REG_DAC3_EQ5, def: 0x00 },
        reg_default { reg: DA732X_REG_BIQ_BYP, def: 0x00 },
        reg_default { reg: DA732X_REG_DMA_CMD, def: 0x00 },
        reg_default { reg: DA732X_REG_DMA_ADDR0, def: 0x00 },
        reg_default { reg: DA732X_REG_DMA_ADDR1, def: 0x00 },
        reg_default { reg: DA732X_REG_DMA_DATA0, def: 0x00 },
        reg_default { reg: DA732X_REG_DMA_DATA1, def: 0x00 },
        reg_default { reg: DA732X_REG_DMA_DATA2, def: 0x00 },
        reg_default { reg: DA732X_REG_DMA_DATA3, def: 0x00 },
        reg_default { reg: DA732X_REG_UNLOCK, def: 0x00 },
    ]
};

unsafe fn da732x_get_input_div(component: *mut snd_soc_component, sysclk: c_int) -> c_int {
    let val: c_int;

    if sysclk < DA732X_MCLK_10MHZ {
        val = DA732X_MCLK_VAL_0_10MHZ;
    } else if sysclk >= DA732X_MCLK_10MHZ && sysclk < DA732X_MCLK_20MHZ {
        val = DA732X_MCLK_VAL_10_20MHZ;
    } else if sysclk >= DA732X_MCLK_20MHZ && sysclk < DA732X_MCLK_40MHZ {
        val = DA732X_MCLK_VAL_20_40MHZ;
    } else if sysclk >= DA732X_MCLK_40MHZ && sysclk <= DA732X_MCLK_54MHZ {
        val = DA732X_MCLK_VAL_40_54MHZ;
    } else {
        return -EINVAL;
    }

    snd_soc_component_write(component, DA732X_REG_PLL_CTRL, val as c_uint);

    val
}

unsafe fn da732x_set_charge_pump(component: *mut snd_soc_component, state: c_int) {
    if state == DA732X_ENABLE_CP {
        snd_soc_component_write(component, DA732X_REG_CLK_EN2, DA732X_CP_CLK_EN);
        snd_soc_component_write(
            component,
            DA732X_REG_CP_HP2,
            DA732X_HP_CP_EN | DA732X_HP_CP_REG | DA732X_HP_CP_PULSESKIP,
        );
        snd_soc_component_write(
            component,
            DA732X_REG_CP_CTRL1,
            DA732X_CP_EN | DA732X_CP_CTRL_CPVDD1,
        );
        snd_soc_component_write(
            component,
            DA732X_REG_CP_CTRL2,
            DA732X_CP_MANAGE_MAGNITUDE | DA732X_CP_BOOST,
        );
        snd_soc_component_write(component, DA732X_REG_CP_CTRL3, DA732X_CP_1MHZ);
    } else if state == DA732X_DISABLE_CP {
        snd_soc_component_write(component, DA732X_REG_CLK_EN2, DA732X_CP_CLK_DIS);
        snd_soc_component_write(component, DA732X_REG_CP_HP2, DA732X_HP_CP_DIS);
        snd_soc_component_write(component, DA732X_REG_CP_CTRL1, DA723X_CP_DIS);
    } else {
        pr_err(b"Wrong charge pump state\n\0".as_ptr() as *const c_char);
    }
}

macro_rules! DECLARE_TLV_DB_SCALE {
    ($name:ident, $min:ident, $inc:ident, $mute:expr) => {
        static $name: [c_uint; 4] = [0, unsafe { $min as c_uint }, unsafe { $inc as c_uint }, $mute];
    };
}

extern "C" {
    static DA732X_MIC_PRE_VOL_DB_MIN: c_uint;
    static DA732X_MIC_PRE_VOL_DB_INC: c_uint;
    static DA732X_MIC_VOL_DB_MIN: c_uint;
    static DA732X_MIC_VOL_DB_INC: c_uint;
    static DA732X_AUX_VOL_DB_MIN: c_uint;
    static DA732X_AUX_VOL_DB_INC: c_uint;
    static DA732X_HP_VOL_DB_MIN: c_uint;
    static DA732X_LIN2_VOL_DB_MIN: c_uint;
    static DA732X_LIN2_VOL_DB_INC: c_uint;
    static DA732X_LIN3_VOL_DB_MIN: c_uint;
    static DA732X_LIN3_VOL_DB_INC: c_uint;
    static DA732X_LIN4_VOL_DB_MIN: c_uint;
    static DA732X_LIN4_VOL_DB_INC: c_uint;
    static DA732X_ADC_VOL_DB_MIN: c_uint;
    static DA732X_ADC_VOL_DB_INC: c_uint;
    static DA732X_DAC_VOL_DB_MIN: c_uint;
    static DA732X_DAC_VOL_DB_INC: c_uint;
    static DA732X_EQ_BAND_VOL_DB_MIN: c_uint;
    static DA732X_EQ_BAND_VOL_DB_INC: c_uint;
    static DA732X_EQ_OVERALL_VOL_DB_MIN: c_uint;
    static DA732X_EQ_OVERALL_VOL_DB_INC: c_uint;
}

DECLARE_TLV_DB_SCALE!(mic_boost_tlv, DA732X_MIC_PRE_VOL_DB_MIN, DA732X_MIC_PRE_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(mic_pga_tlv, DA732X_MIC_VOL_DB_MIN, DA732X_MIC_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(aux_pga_tlv, DA732X_AUX_VOL_DB_MIN, DA732X_AUX_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(hp_pga_tlv, DA732X_HP_VOL_DB_MIN, DA732X_AUX_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(lin2_pga_tlv, DA732X_LIN2_VOL_DB_MIN, DA732X_LIN2_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(lin3_pga_tlv, DA732X_LIN3_VOL_DB_MIN, DA732X_LIN3_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(lin4_pga_tlv, DA732X_LIN4_VOL_DB_MIN, DA732X_LIN4_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(adc_pga_tlv, DA732X_ADC_VOL_DB_MIN, DA732X_ADC_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(dac_pga_tlv, DA732X_DAC_VOL_DB_MIN, DA732X_DAC_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(eq_band_pga_tlv, DA732X_EQ_BAND_VOL_DB_MIN, DA732X_EQ_BAND_VOL_DB_INC, 0);
DECLARE_TLV_DB_SCALE!(eq_overall_tlv, DA732X_EQ_OVERALL_VOL_DB_MIN, DA732X_EQ_OVERALL_VOL_DB_INC, 0);

/* High Pass Filter */
static da732x_hpf_mode: [&[u8]; 3] = [b"Disable\0", b"Music\0", b"Voice\0"];
static da732x_hpf_music: [&[u8]; 4] = [b"1.8Hz\0", b"3.75Hz\0", b"7.5Hz\0", b"15Hz\0"];
static da732x_hpf_voice: [&[u8]; 8] = [
    b"2.5Hz\0",
    b"25Hz\0",
    b"50Hz\0",
    b"100Hz\0",
    b"150Hz\0",
    b"200Hz\0",
    b"300Hz\0",
    b"400Hz\0",
];

/* The SOC_ENUM_* and SOC_* ALSA control constructor macros are external kernel
 * dependencies. Their declarations from da732x.c are preserved here in Rust
 * macro-call form for the translated tables.
 */
macro_rules! SOC_ENUM_SINGLE_DECL { ($($t:tt)*) => {}; }
macro_rules! SOC_SINGLE_RANGE_TLV { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_SINGLE { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_SINGLE_TLV { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DOUBLE_TLV { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DOUBLE { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DOUBLE_R { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DOUBLE_R_TLV { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_ENUM_EXT { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_ENUM { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_ENUM { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }

SOC_ENUM_SINGLE_DECL!(da732x_dac1_hpf_mode_enum, DA732X_REG_DAC1_HPF, DA732X_HPF_MODE_SHIFT, da732x_hpf_mode);
SOC_ENUM_SINGLE_DECL!(da732x_dac2_hpf_mode_enum, DA732X_REG_DAC2_HPF, DA732X_HPF_MODE_SHIFT, da732x_hpf_mode);
SOC_ENUM_SINGLE_DECL!(da732x_dac3_hpf_mode_enum, DA732X_REG_DAC3_HPF, DA732X_HPF_MODE_SHIFT, da732x_hpf_mode);
SOC_ENUM_SINGLE_DECL!(da732x_adc1_hpf_mode_enum, DA732X_REG_ADC1_HPF, DA732X_HPF_MODE_SHIFT, da732x_hpf_mode);
SOC_ENUM_SINGLE_DECL!(da732x_adc2_hpf_mode_enum, DA732X_REG_ADC2_HPF, DA732X_HPF_MODE_SHIFT, da732x_hpf_mode);
SOC_ENUM_SINGLE_DECL!(da732x_dac1_hp_filter_enum, DA732X_REG_DAC1_HPF, DA732X_HPF_MUSIC_SHIFT, da732x_hpf_music);
SOC_ENUM_SINGLE_DECL!(da732x_dac2_hp_filter_enum, DA732X_REG_DAC2_HPF, DA732X_HPF_MUSIC_SHIFT, da732x_hpf_music);
SOC_ENUM_SINGLE_DECL!(da732x_dac3_hp_filter_enum, DA732X_REG_DAC3_HPF, DA732X_HPF_MUSIC_SHIFT, da732x_hpf_music);
SOC_ENUM_SINGLE_DECL!(da732x_adc1_hp_filter_enum, DA732X_REG_ADC1_HPF, DA732X_HPF_MUSIC_SHIFT, da732x_hpf_music);
SOC_ENUM_SINGLE_DECL!(da732x_adc2_hp_filter_enum, DA732X_REG_ADC2_HPF, DA732X_HPF_MUSIC_SHIFT, da732x_hpf_music);
SOC_ENUM_SINGLE_DECL!(da732x_dac1_voice_filter_enum, DA732X_REG_DAC1_HPF, DA732X_HPF_VOICE_SHIFT, da732x_hpf_voice);
SOC_ENUM_SINGLE_DECL!(da732x_dac2_voice_filter_enum, DA732X_REG_DAC2_HPF, DA732X_HPF_VOICE_SHIFT, da732x_hpf_voice);
SOC_ENUM_SINGLE_DECL!(da732x_dac3_voice_filter_enum, DA732X_REG_DAC3_HPF, DA732X_HPF_VOICE_SHIFT, da732x_hpf_voice);
SOC_ENUM_SINGLE_DECL!(da732x_adc1_voice_filter_enum, DA732X_REG_ADC1_HPF, DA732X_HPF_VOICE_SHIFT, da732x_hpf_voice);
SOC_ENUM_SINGLE_DECL!(da732x_adc2_voice_filter_enum, DA732X_REG_ADC2_HPF, DA732X_HPF_VOICE_SHIFT, da732x_hpf_voice);

unsafe extern "C" fn da732x_hpf_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let enum_ctrl = (*kcontrol).private_value as *mut soc_enum;
    let reg = (*enum_ctrl).reg;
    let sel = (*ucontrol).value.enumerated.item[0];
    let bits: c_uint;

    if sel == DA732X_HPF_DISABLED {
        bits = DA732X_HPF_DIS;
    } else if sel == DA732X_HPF_VOICE {
        bits = DA732X_HPF_VOICE_EN;
    } else if sel == DA732X_HPF_MUSIC {
        bits = DA732X_HPF_MUSIC_EN;
    } else {
        return -EINVAL;
    }

    snd_soc_component_update_bits(component, reg, DA732X_HPF_MASK, bits);
    0
}

unsafe extern "C" fn da732x_hpf_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let enum_ctrl = (*kcontrol).private_value as *mut soc_enum;
    let reg = (*enum_ctrl).reg;
    let val = snd_soc_component_read(component, reg) & DA732X_HPF_MASK;

    if val == DA732X_HPF_VOICE_EN {
        (*ucontrol).value.enumerated.item[0] = DA732X_HPF_VOICE;
    } else if val == DA732X_HPF_MUSIC_EN {
        (*ucontrol).value.enumerated.item[0] = DA732X_HPF_MUSIC;
    } else {
        (*ucontrol).value.enumerated.item[0] = DA732X_HPF_DISABLED;
    }

    0
}

/* Input, mixer, DAPM, and route tables translated from the C macro-based ALSA
 * declarations. The concrete expansion of these macros belongs to external
 * ASoC headers.
 */
static da732x_snd_controls: [snd_kcontrol_new; 76] = [
    SOC_SINGLE_RANGE_TLV!("MIC1 Boost Volume", DA732X_REG_MIC1_PRE, DA732X_MICBOOST_SHIFT, DA732X_MICBOOST_MIN, DA732X_MICBOOST_MAX, 0, mic_boost_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC2 Boost Volume", DA732X_REG_MIC2_PRE, DA732X_MICBOOST_SHIFT, DA732X_MICBOOST_MIN, DA732X_MICBOOST_MAX, 0, mic_boost_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC3 Boost Volume", DA732X_REG_MIC3_PRE, DA732X_MICBOOST_SHIFT, DA732X_MICBOOST_MIN, DA732X_MICBOOST_MAX, 0, mic_boost_tlv),
    SOC_SINGLE!("MIC1 Switch", DA732X_REG_MIC1, DA732X_MIC_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_RANGE_TLV!("MIC1 Volume", DA732X_REG_MIC1, DA732X_MIC_VOL_SHIFT, DA732X_MIC_VOL_VAL_MIN, DA732X_MIC_VOL_VAL_MAX, 0, mic_pga_tlv),
    SOC_SINGLE!("MIC2 Switch", DA732X_REG_MIC2, DA732X_MIC_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_RANGE_TLV!("MIC2 Volume", DA732X_REG_MIC2, DA732X_MIC_VOL_SHIFT, DA732X_MIC_VOL_VAL_MIN, DA732X_MIC_VOL_VAL_MAX, 0, mic_pga_tlv),
    SOC_SINGLE!("MIC3 Switch", DA732X_REG_MIC3, DA732X_MIC_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_RANGE_TLV!("MIC3 Volume", DA732X_REG_MIC3, DA732X_MIC_VOL_SHIFT, DA732X_MIC_VOL_VAL_MIN, DA732X_MIC_VOL_VAL_MAX, 0, mic_pga_tlv),
    SOC_SINGLE!("AUX1L Switch", DA732X_REG_AUX1L, DA732X_AUX_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_TLV!("AUX1L Volume", DA732X_REG_AUX1L, DA732X_AUX_VOL_SHIFT, DA732X_AUX_VOL_VAL_MAX, DA732X_NO_INVERT, aux_pga_tlv),
    SOC_SINGLE!("AUX1R Switch", DA732X_REG_AUX1R, DA732X_AUX_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_TLV!("AUX1R Volume", DA732X_REG_AUX1R, DA732X_AUX_VOL_SHIFT, DA732X_AUX_VOL_VAL_MAX, DA732X_NO_INVERT, aux_pga_tlv),
    SOC_DOUBLE_TLV!("ADC1 Volume", DA732X_REG_ADC1_SEL, DA732X_ADCL_VOL_SHIFT, DA732X_ADCR_VOL_SHIFT, DA732X_ADC_VOL_VAL_MAX, DA732X_INVERT, adc_pga_tlv),
    SOC_DOUBLE_TLV!("ADC2 Volume", DA732X_REG_ADC2_SEL, DA732X_ADCL_VOL_SHIFT, DA732X_ADCR_VOL_SHIFT, DA732X_ADC_VOL_VAL_MAX, DA732X_INVERT, adc_pga_tlv),
    SOC_DOUBLE!("Digital Playback DAC12 Switch", DA732X_REG_DAC1_SEL, DA732X_DACL_MUTE_SHIFT, DA732X_DACR_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_DOUBLE_R_TLV!("Digital Playback DAC12 Volume", DA732X_REG_DAC1_L_VOL, DA732X_REG_DAC1_R_VOL, DA732X_DAC_VOL_SHIFT, DA732X_DAC_VOL_VAL_MAX, DA732X_INVERT, dac_pga_tlv),
    SOC_SINGLE!("Digital Playback DAC3 Switch", DA732X_REG_DAC2_SEL, DA732X_DACL_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_TLV!("Digital Playback DAC3 Volume", DA732X_REG_DAC2_L_VOL, DA732X_DAC_VOL_SHIFT, DA732X_DAC_VOL_VAL_MAX, DA732X_INVERT, dac_pga_tlv),
    SOC_SINGLE!("Digital Playback DAC4 Switch", DA732X_REG_DAC2_SEL, DA732X_DACR_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_TLV!("Digital Playback DAC4 Volume", DA732X_REG_DAC2_R_VOL, DA732X_DAC_VOL_SHIFT, DA732X_DAC_VOL_VAL_MAX, DA732X_INVERT, dac_pga_tlv),
    SOC_SINGLE!("Digital Playback DAC5 Switch", DA732X_REG_DAC3_SEL, DA732X_DACL_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_TLV!("Digital Playback DAC5 Volume", DA732X_REG_DAC3_VOL, DA732X_DAC_VOL_SHIFT, DA732X_DAC_VOL_VAL_MAX, DA732X_INVERT, dac_pga_tlv),
    SOC_ENUM_EXT!("DAC1 High Pass Filter Mode", da732x_dac1_hpf_mode_enum, da732x_hpf_get, da732x_hpf_set),
    SOC_ENUM!("DAC1 High Pass Filter", da732x_dac1_hp_filter_enum),
    SOC_ENUM!("DAC1 Voice Filter", da732x_dac1_voice_filter_enum),
    SOC_ENUM_EXT!("DAC2 High Pass Filter Mode", da732x_dac2_hpf_mode_enum, da732x_hpf_get, da732x_hpf_set),
    SOC_ENUM!("DAC2 High Pass Filter", da732x_dac2_hp_filter_enum),
    SOC_ENUM!("DAC2 Voice Filter", da732x_dac2_voice_filter_enum),
    SOC_ENUM_EXT!("DAC3 High Pass Filter Mode", da732x_dac3_hpf_mode_enum, da732x_hpf_get, da732x_hpf_set),
    SOC_ENUM!("DAC3 High Pass Filter", da732x_dac3_hp_filter_enum),
    SOC_ENUM!("DAC3 Filter Mode", da732x_dac3_voice_filter_enum),
    SOC_ENUM_EXT!("ADC1 High Pass Filter Mode", da732x_adc1_hpf_mode_enum, da732x_hpf_get, da732x_hpf_set),
    SOC_ENUM!("ADC1 High Pass Filter", da732x_adc1_hp_filter_enum),
    SOC_ENUM!("ADC1 Voice Filter", da732x_adc1_voice_filter_enum),
    SOC_ENUM_EXT!("ADC2 High Pass Filter Mode", da732x_adc2_hpf_mode_enum, da732x_hpf_get, da732x_hpf_set),
    SOC_ENUM!("ADC2 High Pass Filter", da732x_adc2_hp_filter_enum),
    SOC_ENUM!("ADC2 Voice Filter", da732x_adc2_voice_filter_enum),
    SOC_SINGLE!("ADC1 EQ Switch", DA732X_REG_ADC1_EQ5, DA732X_EQ_EN_SHIFT, DA732X_EQ_EN_MAX, DA732X_NO_INVERT),
    SOC_SINGLE_TLV!("ADC1 EQ Band 1 Volume", DA732X_REG_ADC1_EQ12, DA732X_EQ_BAND1_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ADC1 EQ Band 2 Volume", DA732X_REG_ADC1_EQ12, DA732X_EQ_BAND2_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ADC1 EQ Band 3 Volume", DA732X_REG_ADC1_EQ34, DA732X_EQ_BAND3_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ADC1 EQ Band 4 Volume", DA732X_REG_ADC1_EQ34, DA732X_EQ_BAND4_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ADC1 EQ Band 5 Volume", DA732X_REG_ADC1_EQ5, DA732X_EQ_BAND5_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ADC1 EQ Overall Volume", DA732X_REG_ADC1_EQ5, DA732X_EQ_OVERALL_SHIFT, DA732X_EQ_OVERALL_VOL_VAL_MAX, DA732X_INVERT, eq_overall_tlv),
    SOC_SINGLE!("ADC2 EQ Switch", DA732X_REG_ADC2_EQ5, DA732X_EQ_EN_SHIFT, DA732X_EQ_EN_MAX, DA732X_NO_INVERT),
    SOC_SINGLE_TLV!("ADC2 EQ Band 1 Volume", DA732X_REG_ADC2_EQ12, DA732X_EQ_BAND1_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ADC2 EQ Band 2 Volume", DA732X_REG_ADC2_EQ12, DA732X_EQ_BAND2_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ADC2 EQ Band 3 Volume", DA732X_REG_ADC2_EQ34, DA732X_EQ_BAND3_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ACD2 EQ Band 4 Volume", DA732X_REG_ADC2_EQ34, DA732X_EQ_BAND4_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ACD2 EQ Band 5 Volume", DA732X_REG_ADC2_EQ5, DA732X_EQ_BAND5_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("ADC2 EQ Overall Volume", DA732X_REG_ADC1_EQ5, DA732X_EQ_OVERALL_SHIFT, DA732X_EQ_OVERALL_VOL_VAL_MAX, DA732X_INVERT, eq_overall_tlv),
    SOC_SINGLE!("DAC1 EQ Switch", DA732X_REG_DAC1_EQ5, DA732X_EQ_EN_SHIFT, DA732X_EQ_EN_MAX, DA732X_NO_INVERT),
    SOC_SINGLE_TLV!("DAC1 EQ Band 1 Volume", DA732X_REG_DAC1_EQ12, DA732X_EQ_BAND1_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC1 EQ Band 2 Volume", DA732X_REG_DAC1_EQ12, DA732X_EQ_BAND2_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC1 EQ Band 3 Volume", DA732X_REG_DAC1_EQ34, DA732X_EQ_BAND3_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC1 EQ Band 4 Volume", DA732X_REG_DAC1_EQ34, DA732X_EQ_BAND4_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC1 EQ Band 5 Volume", DA732X_REG_DAC1_EQ5, DA732X_EQ_BAND5_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE!("DAC2 EQ Switch", DA732X_REG_DAC2_EQ5, DA732X_EQ_EN_SHIFT, DA732X_EQ_EN_MAX, DA732X_NO_INVERT),
    SOC_SINGLE_TLV!("DAC2 EQ Band 1 Volume", DA732X_REG_DAC2_EQ12, DA732X_EQ_BAND1_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC2 EQ Band 2 Volume", DA732X_REG_DAC2_EQ12, DA732X_EQ_BAND2_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC2 EQ Band 3 Volume", DA732X_REG_DAC2_EQ34, DA732X_EQ_BAND3_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC2 EQ Band 4 Volume", DA732X_REG_DAC2_EQ34, DA732X_EQ_BAND4_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC2 EQ Band 5 Volume", DA732X_REG_DAC2_EQ5, DA732X_EQ_BAND5_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE!("DAC3 EQ Switch", DA732X_REG_DAC3_EQ5, DA732X_EQ_EN_SHIFT, DA732X_EQ_EN_MAX, DA732X_NO_INVERT),
    SOC_SINGLE_TLV!("DAC3 EQ Band 1 Volume", DA732X_REG_DAC3_EQ12, DA732X_EQ_BAND1_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC3 EQ Band 2 Volume", DA732X_REG_DAC3_EQ12, DA732X_EQ_BAND2_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC3 EQ Band 3 Volume", DA732X_REG_DAC3_EQ34, DA732X_EQ_BAND3_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC3 EQ Band 4 Volume", DA732X_REG_DAC3_EQ34, DA732X_EQ_BAND4_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE_TLV!("DAC3 EQ Band 5 Volume", DA732X_REG_DAC3_EQ5, DA732X_EQ_BAND5_SHIFT, DA732X_EQ_VOL_VAL_MAX, DA732X_INVERT, eq_band_pga_tlv),
    SOC_SINGLE!("Lineout 2 Switch", DA732X_REG_LIN2, DA732X_LOUT_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_TLV!("Lineout 2 Volume", DA732X_REG_LIN2, DA732X_LOUT_VOL_SHIFT, DA732X_LOUT_VOL_VAL_MAX, DA732X_NO_INVERT, lin2_pga_tlv),
    SOC_SINGLE!("Lineout 3 Switch", DA732X_REG_LIN3, DA732X_LOUT_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_TLV!("Lineout 3 Volume", DA732X_REG_LIN3, DA732X_LOUT_VOL_SHIFT, DA732X_LOUT_VOL_VAL_MAX, DA732X_NO_INVERT, lin3_pga_tlv),
    SOC_SINGLE!("Lineout 4 Switch", DA732X_REG_LIN4, DA732X_LOUT_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_SINGLE_TLV!("Lineout 4 Volume", DA732X_REG_LIN4, DA732X_LOUT_VOL_SHIFT, DA732X_LOUT_VOL_VAL_MAX, DA732X_NO_INVERT, lin4_pga_tlv),
    SOC_DOUBLE_R!("Headphone Switch", DA732X_REG_HPR, DA732X_REG_HPL, DA732X_HP_MUTE_SHIFT, DA732X_SWITCH_MAX, DA732X_INVERT),
    SOC_DOUBLE_R_TLV!("Headphone Volume", DA732X_REG_HPL_VOL, DA732X_REG_HPR_VOL, DA732X_HP_VOL_SHIFT, DA732X_HP_VOL_VAL_MAX, DA732X_NO_INVERT, hp_pga_tlv),
];

unsafe extern "C" fn da732x_adc_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_POST_PMU => {
            if (*w).reg == DA732X_REG_ADC1_PD {
                snd_soc_component_update_bits(component, DA732X_REG_CLK_EN3, DA732X_ADCA_BB_CLK_EN, DA732X_ADCA_BB_CLK_EN);
            } else if (*w).reg == DA732X_REG_ADC2_PD {
                snd_soc_component_update_bits(component, DA732X_REG_CLK_EN3, DA732X_ADCC_BB_CLK_EN, DA732X_ADCC_BB_CLK_EN);
            } else {
                return -EINVAL;
            }
            snd_soc_component_update_bits(component, (*w).reg, DA732X_ADC_RST_MASK, DA732X_ADC_SET_ACT);
            snd_soc_component_update_bits(component, (*w).reg, DA732X_ADC_PD_MASK, DA732X_ADC_ON);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, (*w).reg, DA732X_ADC_PD_MASK, DA732X_ADC_OFF);
            snd_soc_component_update_bits(component, (*w).reg, DA732X_ADC_RST_MASK, DA732X_ADC_SET_RST);
            if (*w).reg == DA732X_REG_ADC1_PD {
                snd_soc_component_update_bits(component, DA732X_REG_CLK_EN3, DA732X_ADCA_BB_CLK_EN, 0);
            } else if (*w).reg == DA732X_REG_ADC2_PD {
                snd_soc_component_update_bits(component, DA732X_REG_CLK_EN3, DA732X_ADCC_BB_CLK_EN, 0);
            } else {
                return -EINVAL;
            }
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn da732x_out_pga_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mask = (1u32 << (*w).shift) | DA732X_OUT_HIZ_EN;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            snd_soc_component_update_bits(component, (*w).reg, mask, mask);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, (*w).reg, mask, (1u32 << (*w).shift) | DA732X_OUT_HIZ_DIS);
        }
        _ => return -EINVAL,
    }

    0
}

static adcl_text: [&[u8]; 2] = [b"AUX1L\0", b"MIC1\0"];
static adcr_text: [&[u8]; 3] = [b"AUX1R\0", b"MIC2\0", b"MIC3\0"];
static enable_text: [&[u8]; 2] = [b"Disabled\0", b"Enabled\0"];

SOC_ENUM_SINGLE_DECL!(adc1l_enum, DA732X_REG_INP_MUX, DA732X_ADC1L_MUX_SEL_SHIFT, adcl_text);
static adc1l_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("ADC Route", adc1l_enum);
SOC_ENUM_SINGLE_DECL!(adc1r_enum, DA732X_REG_INP_MUX, DA732X_ADC1R_MUX_SEL_SHIFT, adcr_text);
static adc1r_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("ADC Route", adc1r_enum);
SOC_ENUM_SINGLE_DECL!(adc2l_enum, DA732X_REG_INP_MUX, DA732X_ADC2L_MUX_SEL_SHIFT, adcl_text);
static adc2l_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("ADC Route", adc2l_enum);
SOC_ENUM_SINGLE_DECL!(adc2r_enum, DA732X_REG_INP_MUX, DA732X_ADC2R_MUX_SEL_SHIFT, adcr_text);
static adc2r_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("ADC Route", adc2r_enum);
SOC_ENUM_SINGLE_DECL!(da732x_hp_left_output, DA732X_REG_HPL, DA732X_HP_OUT_DAC_EN_SHIFT, enable_text);
static hpl_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("HPL Switch", da732x_hp_left_output);
SOC_ENUM_SINGLE_DECL!(da732x_hp_right_output, DA732X_REG_HPR, DA732X_HP_OUT_DAC_EN_SHIFT, enable_text);
static hpr_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("HPR Switch", da732x_hp_right_output);
SOC_ENUM_SINGLE_DECL!(da732x_speaker_output, DA732X_REG_LIN3, DA732X_LOUT_DAC_EN_SHIFT, enable_text);
static spk_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("SPK Switch", da732x_speaker_output);
SOC_ENUM_SINGLE_DECL!(da732x_lout4_output, DA732X_REG_LIN4, DA732X_LOUT_DAC_EN_SHIFT, enable_text);
static lout4_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("LOUT4 Switch", da732x_lout4_output);
SOC_ENUM_SINGLE_DECL!(da732x_lout2_output, DA732X_REG_LIN2, DA732X_LOUT_DAC_EN_SHIFT, enable_text);
static lout2_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("LOUT2 Switch", da732x_lout2_output);

/* DAPM widget declarations from the C source are macro constructors supplied by
 * ASoC; the route strings below preserve the source-level topology.
 */
static da732x_dapm_widgets: [snd_soc_dapm_widget_opaque; 0] = [];

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: core::ptr::null(), source: concat!($source, "\0").as_ptr() as *const c_char }
    };
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: concat!($control, "\0").as_ptr() as *const c_char, source: concat!($source, "\0").as_ptr() as *const c_char }
    };
}

static da732x_dapm_routes: [snd_soc_dapm_route; 54] = [
    route!("AUX1L PGA", NULL, "AUX1L"),
    route!("AUX1R PGA", NULL, "AUX1R"),
    route!("MIC1 PGA", NULL, "MIC1"),
    route!("MIC2 PGA", NULL, "MIC2"),
    route!("MIC3 PGA", NULL, "MIC3"),
    route!("ADC1 Left MUX", "MIC1", "MIC1 PGA"),
    route!("ADC1 Left MUX", "AUX1L", "AUX1L PGA"),
    route!("ADC1 Right MUX", "AUX1R", "AUX1R PGA"),
    route!("ADC1 Right MUX", "MIC2", "MIC2 PGA"),
    route!("ADC1 Right MUX", "MIC3", "MIC3 PGA"),
    route!("ADC2 Left MUX", "AUX1L", "AUX1L PGA"),
    route!("ADC2 Left MUX", "MIC1", "MIC1 PGA"),
    route!("ADC2 Right MUX", "AUX1R", "AUX1R PGA"),
    route!("ADC2 Right MUX", "MIC2", "MIC2 PGA"),
    route!("ADC2 Right MUX", "MIC3", "MIC3 PGA"),
    route!("ADC1L", NULL, "ADC1 Supply"),
    route!("ADC1R", NULL, "ADC1 Supply"),
    route!("ADC2L", NULL, "ADC2 Supply"),
    route!("ADC2R", NULL, "ADC2 Supply"),
    route!("ADC1L", NULL, "ADC1 Left MUX"),
    route!("ADC1R", NULL, "ADC1 Right MUX"),
    route!("ADC2L", NULL, "ADC2 Left MUX"),
    route!("ADC2R", NULL, "ADC2 Right MUX"),
    route!("AIFA Output", NULL, "ADC1L"),
    route!("AIFA Output", NULL, "ADC1R"),
    route!("AIFB Output", NULL, "ADC2L"),
    route!("AIFB Output", NULL, "ADC2R"),
    route!("HP Left MUX", "Enabled", "AIFA Input"),
    route!("HP Right MUX", "Enabled", "AIFA Input"),
    route!("Speaker MUX", "Enabled", "AIFB Input"),
    route!("LOUT2 MUX", "Enabled", "AIFB Input"),
    route!("LOUT4 MUX", "Enabled", "AIFB Input"),
    route!("DAC1L", NULL, "DAC1 CLK"),
    route!("DAC1R", NULL, "DAC1 CLK"),
    route!("DAC2L", NULL, "DAC2 CLK"),
    route!("DAC2R", NULL, "DAC2 CLK"),
    route!("DAC3", NULL, "DAC3 CLK"),
    route!("DAC1L", NULL, "HP Left MUX"),
    route!("DAC1R", NULL, "HP Right MUX"),
    route!("DAC2L", NULL, "Speaker MUX"),
    route!("DAC2R", NULL, "LOUT4 MUX"),
    route!("DAC3", NULL, "LOUT2 MUX"),
    route!("HP Left", NULL, "DAC1L"),
    route!("HP Right", NULL, "DAC1R"),
    route!("LIN3", NULL, "DAC2L"),
    route!("LIN4", NULL, "DAC2R"),
    route!("LIN2", NULL, "DAC3"),
    route!("ClassD", NULL, "LIN3"),
    route!("LOUTL", NULL, "LIN2"),
    route!("LOUTR", NULL, "LIN4"),
    route!("HPL", NULL, "HP Left"),
    route!("HPR", NULL, "HP Right"),
    route!("MICBIAS1", NULL, "ADC1 Supply"),
    route!("MICBIAS2", NULL, "ADC2 Supply"),
];

extern "C" {
    static DA732X_AIF_WORD_16: u32;
    static DA732X_AIF_WORD_20: u32;
    static DA732X_AIF_WORD_24: u32;
    static DA732X_AIF_WORD_32: u32;
    static DA732X_SR_8KHZ: u32;
    static DA732X_SR_11_025KHZ: u32;
    static DA732X_SR_12KHZ: u32;
    static DA732X_SR_16KHZ: u32;
    static DA732X_SR_22_05KHZ: u32;
    static DA732X_SR_24KHZ: u32;
    static DA732X_SR_32KHZ: u32;
    static DA732X_SR_44_1KHZ: u32;
    static DA732X_SR_48KHZ: u32;
    static DA732X_SR_88_1KHZ: u32;
    static DA732X_SR_96KHZ: u32;
    static DA732X_AIF_WORD_MASK: u32;
    static DA732X_SR1_MASK: u32;
}

unsafe extern "C" fn da732x_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let mut aif: u32 = 0;
    let reg_aif = (*(*dai).driver).base;
    let fs: u32;

    match params_width(params) {
        16 => aif |= DA732X_AIF_WORD_16,
        20 => aif |= DA732X_AIF_WORD_20,
        24 => aif |= DA732X_AIF_WORD_24,
        32 => aif |= DA732X_AIF_WORD_32,
        _ => return -EINVAL,
    }

    fs = match params_rate(params) {
        8000 => DA732X_SR_8KHZ,
        11025 => DA732X_SR_11_025KHZ,
        12000 => DA732X_SR_12KHZ,
        16000 => DA732X_SR_16KHZ,
        22050 => DA732X_SR_22_05KHZ,
        24000 => DA732X_SR_24KHZ,
        32000 => DA732X_SR_32KHZ,
        44100 => DA732X_SR_44_1KHZ,
        48000 => DA732X_SR_48KHZ,
        88100 => DA732X_SR_88_1KHZ,
        96000 => DA732X_SR_96KHZ,
        _ => return -EINVAL,
    };

    snd_soc_component_update_bits(component, reg_aif, DA732X_AIF_WORD_MASK, aif);
    snd_soc_component_update_bits(component, DA732X_REG_CLK_CTRL, DA732X_SR1_MASK, fs);

    0
}

extern "C" {
    static DA732X_DAI_ID1: c_int;
    static DA732X_DAI_ID2: c_int;
    static DA732X_PC_PULSE_AIFA: u32;
    static DA732X_PC_PULSE_AIFB: u32;
    static DA732X_PC_RESYNC_NOT_AUT: u32;
    static DA732X_PC_SAME: u32;
    static SND_SOC_DAIFMT_MASTER_MASK: u32;
    static SND_SOC_DAIFMT_CBC_CFC: u32;
    static SND_SOC_DAIFMT_CBP_CFP: u32;
    static DA732X_AIF_SLAVE: u32;
    static DA732X_AIFM_FRAME_64: u32;
    static DA732X_AIFM_SRC_SEL_AIFA: u32;
    static DA732X_AIF_CLK_FROM_SRC: u32;
    static DA732X_CLK_GENERATION_AIF_A: u32;
    static SND_SOC_DAIFMT_FORMAT_MASK: u32;
    static SND_SOC_DAIFMT_I2S: u32;
    static SND_SOC_DAIFMT_RIGHT_J: u32;
    static SND_SOC_DAIFMT_LEFT_J: u32;
    static SND_SOC_DAIFMT_DSP_B: u32;
    static DA732X_AIF_I2S_MODE: u32;
    static DA732X_AIF_RIGHT_J_MODE: u32;
    static DA732X_AIF_LEFT_J_MODE: u32;
    static DA732X_AIF_DSP_MODE: u32;
    static SND_SOC_DAIFMT_INV_MASK: u32;
    static SND_SOC_DAIFMT_NB_NF: u32;
    static SND_SOC_DAIFMT_IB_NF: u32;
    static SND_SOC_DAIFMT_IB_IF: u32;
    static SND_SOC_DAIFMT_NB_IF: u32;
    static DA732X_AIF_BCLK_INV: u32;
    static DA732X_AIF_WCLK_INV: u32;
    static DA732X_AIF1_CLK_MASK: u32;
    static DA732X_AIF_MODE_MASK: u32;
}

unsafe extern "C" fn da732x_set_dai_fmt(dai: *mut snd_soc_dai, fmt: u32) -> c_int {
    let component = (*dai).component;
    let aif_mclk: u32;
    let pc_count: u32;
    let reg_aif1: u32;
    let aif1: u32;
    let reg_aif3: u32;
    let mut aif3: u32;

    if (*dai).id == DA732X_DAI_ID1 {
        reg_aif1 = DA732X_REG_AIFA1;
        reg_aif3 = DA732X_REG_AIFA3;
        pc_count = DA732X_PC_PULSE_AIFA | DA732X_PC_RESYNC_NOT_AUT | DA732X_PC_SAME;
    } else if (*dai).id == DA732X_DAI_ID2 {
        reg_aif1 = DA732X_REG_AIFB1;
        reg_aif3 = DA732X_REG_AIFB3;
        pc_count = DA732X_PC_PULSE_AIFB | DA732X_PC_RESYNC_NOT_AUT | DA732X_PC_SAME;
    } else {
        return -EINVAL;
    }

    if (fmt & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBC_CFC {
        aif1 = DA732X_AIF_SLAVE;
        aif_mclk = DA732X_AIFM_FRAME_64 | DA732X_AIFM_SRC_SEL_AIFA;
    } else if (fmt & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
        aif1 = DA732X_AIF_CLK_FROM_SRC;
        aif_mclk = DA732X_CLK_GENERATION_AIF_A;
    } else {
        return -EINVAL;
    }

    let format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    if format == SND_SOC_DAIFMT_I2S {
        aif3 = DA732X_AIF_I2S_MODE;
    } else if format == SND_SOC_DAIFMT_RIGHT_J {
        aif3 = DA732X_AIF_RIGHT_J_MODE;
    } else if format == SND_SOC_DAIFMT_LEFT_J {
        aif3 = DA732X_AIF_LEFT_J_MODE;
    } else if format == SND_SOC_DAIFMT_DSP_B {
        aif3 = DA732X_AIF_DSP_MODE;
    } else {
        return -EINVAL;
    }

    /* Clock inversion */
    if format == SND_SOC_DAIFMT_DSP_B {
        match fmt & SND_SOC_DAIFMT_INV_MASK {
            x if x == SND_SOC_DAIFMT_NB_NF => {}
            x if x == SND_SOC_DAIFMT_IB_NF => aif3 |= DA732X_AIF_BCLK_INV,
            _ => return -EINVAL,
        }
    } else if format == SND_SOC_DAIFMT_I2S || format == SND_SOC_DAIFMT_RIGHT_J || format == SND_SOC_DAIFMT_LEFT_J {
        match fmt & SND_SOC_DAIFMT_INV_MASK {
            x if x == SND_SOC_DAIFMT_NB_NF => {}
            x if x == SND_SOC_DAIFMT_IB_IF => aif3 |= DA732X_AIF_BCLK_INV | DA732X_AIF_WCLK_INV,
            x if x == SND_SOC_DAIFMT_IB_NF => aif3 |= DA732X_AIF_BCLK_INV,
            x if x == SND_SOC_DAIFMT_NB_IF => aif3 |= DA732X_AIF_WCLK_INV,
            _ => return -EINVAL,
        }
    } else {
        return -EINVAL;
    }

    snd_soc_component_write(component, DA732X_REG_AIF_MCLK, aif_mclk);
    snd_soc_component_update_bits(component, reg_aif1, DA732X_AIF1_CLK_MASK, aif1);
    snd_soc_component_update_bits(
        component,
        reg_aif3,
        DA732X_AIF_BCLK_INV | DA732X_AIF_WCLK_INV | DA732X_AIF_MODE_MASK,
        aif3,
    );
    snd_soc_component_write(component, DA732X_REG_PC_CTRL, pc_count);

    0
}

extern "C" {
    static DA732X_SRCCLK_MCLK: c_int;
    static DA732X_PLL_EN: u32;
    static DA732X_PLL_BYPASS: u32;
    static DA732X_1BYTE_SHIFT: u32;
    static DA732X_U8_MASK: u32;
}

unsafe extern "C" fn da732x_set_dai_pll(
    component: *mut snd_soc_component,
    _pll_id: c_int,
    source: c_int,
    _freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let da732x = snd_soc_component_get_drvdata(component) as *mut da732x_priv;
    let fref: c_int;
    let indiv: c_int;
    let div_lo: u8;
    let div_mid: u8;
    let div_hi: u8;
    let mut frac_div: u64;

    /* Disable PLL */
    if freq_out == 0 {
        snd_soc_component_update_bits(component, DA732X_REG_PLL_CTRL, DA732X_PLL_EN, 0);
        (*da732x).pll_en = false;
        return 0;
    }

    if (*da732x).pll_en {
        return -EBUSY;
    }

    if source == DA732X_SRCCLK_MCLK {
        /* Validate Sysclk rate */
        match (*da732x).sysclk {
            11290000 | 12288000 | 22580000 | 24576000 | 45160000 | 49152000 => {
                snd_soc_component_write(component, DA732X_REG_PLL_CTRL, DA732X_PLL_BYPASS);
                return 0;
            }
            _ => {
                dev_err((*component).dev, b"Cannot use PLL Bypass, invalid SYSCLK rate\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
        }
    }

    indiv = da732x_get_input_div(component, (*da732x).sysclk as c_int);
    if indiv < 0 {
        return indiv;
    }

    fref = ((*da732x).sysclk / (1u32 << indiv)) as c_int;
    div_hi = (freq_out / fref as u32) as u8;
    frac_div = ((freq_out % fref as u32) as u64) * 8192u64;
    frac_div /= fref as u64;
    div_mid = ((frac_div >> DA732X_1BYTE_SHIFT) & DA732X_U8_MASK as u64) as u8;
    div_lo = (frac_div & DA732X_U8_MASK as u64) as u8;

    snd_soc_component_write(component, DA732X_REG_PLL_DIV_LO, div_lo as c_uint);
    snd_soc_component_write(component, DA732X_REG_PLL_DIV_MID, div_mid as c_uint);
    snd_soc_component_write(component, DA732X_REG_PLL_DIV_HI, div_hi as c_uint);
    snd_soc_component_update_bits(component, DA732X_REG_PLL_CTRL, DA732X_PLL_EN, DA732X_PLL_EN);

    (*da732x).pll_en = true;
    0
}

unsafe extern "C" fn da732x_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let da732x = snd_soc_component_get_drvdata(component) as *mut da732x_priv;

    (*da732x).sysclk = freq;
    0
}

const DA732X_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const DA732X_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static da732x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(da732x_hw_params),
    set_fmt: Some(da732x_set_dai_fmt),
    set_sysclk: Some(da732x_set_dai_sysclk),
};

static mut da732x_dai: [snd_soc_dai_driver; 2] = unsafe {
    [
        snd_soc_dai_driver {
            name: b"DA732X_AIFA\0".as_ptr() as *const c_char,
            id: DA732X_DAI_ID1,
            base: DA732X_REG_AIFA1,
            playback: snd_soc_pcm_stream {
                stream_name: b"AIFA Playback\0".as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: 2,
                rates: DA732X_RATES,
                formats: DA732X_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"AIFA Capture\0".as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: 2,
                rates: DA732X_RATES,
                formats: DA732X_FORMATS,
            },
            ops: &da732x_dai_ops,
        },
        snd_soc_dai_driver {
            name: b"DA732X_AIFB\0".as_ptr() as *const c_char,
            id: DA732X_DAI_ID2,
            base: DA732X_REG_AIFB1,
            playback: snd_soc_pcm_stream {
                stream_name: b"AIFB Playback\0".as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: 2,
                rates: DA732X_RATES,
                formats: DA732X_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"AIFB Capture\0".as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: 2,
                rates: DA732X_RATES,
                formats: DA732X_FORMATS,
            },
            ops: &da732x_dai_ops,
        },
    ]
};

unsafe extern "C" fn da732x_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == DA732X_REG_HPL_DAC_OFF_CNTL || x == DA732X_REG_HPR_DAC_OFF_CNTL => true,
        _ => false,
    }
}

static mut da732x_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 8,
        val_bits: 8,
        max_register: DA732X_MAX_REG,
        volatile_reg: Some(da732x_volatile),
        reg_defaults: da732x_reg_cache.as_ptr(),
        num_reg_defaults: da732x_reg_cache.len() as c_uint,
        cache_type: REGCACHE_RBTREE,
    }
};

extern "C" {
    static DA732X_HP_DACS: usize;
    static DA732X_DAC_OFFSET_STEP: u8;
    static DA732X_HP_DAC_OFFSET_TRIM_VAL: c_uint;
    static DA732X_HP_DAC_OFF_CALIBRATION: c_uint;
    static DA732X_HP_DAC_OFF_SCALE_STEPS: c_uint;
    static DA732X_WAIT_FOR_STABILIZATION: c_uint;
    static DA732X_HPL_DAC: usize;
    static DA732X_HPR_DAC: usize;
    static DA732X_HP_DAC_OFF_CNTL_COMPO: c_uint;
    static DA732X_HP_DAC_COMPO_SHIFT: u32;
    static DA732X_HP_DAC_OFF_MASK: c_uint;
    static DA732X_HP_AMPS: usize;
    static DA732X_OUTPUT_OFFSET_STEP: u8;
    static DA732X_HPL_AMP: usize;
    static DA732X_HPR_AMP: usize;
    static DA732X_HP_OUT_TRIM_VAL: u8;
    static DA732X_HP_OUT_COMP: c_uint;
    static DA732X_HP_OUT_EN: c_uint;
    static DA732X_HP_OUT_COMPO: c_uint;
    static DA732X_HP_OUT_COMPO_SHIFT: u32;
}

unsafe fn da732x_dac_offset_adjust(component: *mut snd_soc_component) {
    let mut offset = [0u8; 2];
    let mut sign = [0u8; 2];
    let mut step: u8 = DA732X_DAC_OFFSET_STEP;

    /* Initialize DAC offset calibration circuits and registers */
    snd_soc_component_write(component, DA732X_REG_HPL_DAC_OFFSET, DA732X_HP_DAC_OFFSET_TRIM_VAL);
    snd_soc_component_write(component, DA732X_REG_HPR_DAC_OFFSET, DA732X_HP_DAC_OFFSET_TRIM_VAL);
    snd_soc_component_write(component, DA732X_REG_HPL_DAC_OFF_CNTL, DA732X_HP_DAC_OFF_CALIBRATION | DA732X_HP_DAC_OFF_SCALE_STEPS);
    snd_soc_component_write(component, DA732X_REG_HPR_DAC_OFF_CNTL, DA732X_HP_DAC_OFF_CALIBRATION | DA732X_HP_DAC_OFF_SCALE_STEPS);

    /* Wait for voltage stabilization */
    msleep(DA732X_WAIT_FOR_STABILIZATION);

    /* Check DAC offset sign */
    sign[DA732X_HPL_DAC] = (snd_soc_component_read(component, DA732X_REG_HPL_DAC_OFF_CNTL) & DA732X_HP_DAC_OFF_CNTL_COMPO) as u8;
    sign[DA732X_HPR_DAC] = (snd_soc_component_read(component, DA732X_REG_HPR_DAC_OFF_CNTL) & DA732X_HP_DAC_OFF_CNTL_COMPO) as u8;

    /* Binary search DAC offset values (both channels at once) */
    offset[DA732X_HPL_DAC] = sign[DA732X_HPL_DAC] << DA732X_HP_DAC_COMPO_SHIFT;
    offset[DA732X_HPR_DAC] = sign[DA732X_HPR_DAC] << DA732X_HP_DAC_COMPO_SHIFT;

    while step != 0 {
        offset[DA732X_HPL_DAC] |= step;
        offset[DA732X_HPR_DAC] |= step;
        snd_soc_component_write(component, DA732X_REG_HPL_DAC_OFFSET, (!offset[DA732X_HPL_DAC] as c_uint) & DA732X_HP_DAC_OFF_MASK);
        snd_soc_component_write(component, DA732X_REG_HPR_DAC_OFFSET, (!offset[DA732X_HPR_DAC] as c_uint) & DA732X_HP_DAC_OFF_MASK);

        msleep(DA732X_WAIT_FOR_STABILIZATION);

        if (((snd_soc_component_read(component, DA732X_REG_HPL_DAC_OFF_CNTL) & DA732X_HP_DAC_OFF_CNTL_COMPO) as u8) ^ sign[DA732X_HPL_DAC]) != 0 {
            offset[DA732X_HPL_DAC] &= !step;
        }
        if (((snd_soc_component_read(component, DA732X_REG_HPR_DAC_OFF_CNTL) & DA732X_HP_DAC_OFF_CNTL_COMPO) as u8) ^ sign[DA732X_HPR_DAC]) != 0 {
            offset[DA732X_HPR_DAC] &= !step;
        }

        step >>= 1;
    }

    /* Write final DAC offsets to registers */
    snd_soc_component_write(component, DA732X_REG_HPL_DAC_OFFSET, (!offset[DA732X_HPL_DAC] as c_uint) & DA732X_HP_DAC_OFF_MASK);
    snd_soc_component_write(component, DA732X_REG_HPR_DAC_OFFSET, (!offset[DA732X_HPR_DAC] as c_uint) & DA732X_HP_DAC_OFF_MASK);

    /* End DAC calibration mode */
    snd_soc_component_write(component, DA732X_REG_HPL_DAC_OFF_CNTL, DA732X_HP_DAC_OFF_SCALE_STEPS);
    snd_soc_component_write(component, DA732X_REG_HPR_DAC_OFF_CNTL, DA732X_HP_DAC_OFF_SCALE_STEPS);
}

unsafe fn da732x_output_offset_adjust(component: *mut snd_soc_component) {
    let mut offset = [0u8; 2];
    let mut sign = [0u8; 2];
    let mut step: u8 = DA732X_OUTPUT_OFFSET_STEP;

    offset[DA732X_HPL_AMP] = DA732X_HP_OUT_TRIM_VAL;
    offset[DA732X_HPR_AMP] = DA732X_HP_OUT_TRIM_VAL;

    /* Initialize output offset calibration circuits and registers  */
    snd_soc_component_write(component, DA732X_REG_HPL_OUT_OFFSET, DA732X_HP_OUT_TRIM_VAL as c_uint);
    snd_soc_component_write(component, DA732X_REG_HPR_OUT_OFFSET, DA732X_HP_OUT_TRIM_VAL as c_uint);
    snd_soc_component_write(component, DA732X_REG_HPL, DA732X_HP_OUT_COMP | DA732X_HP_OUT_EN);
    snd_soc_component_write(component, DA732X_REG_HPR, DA732X_HP_OUT_COMP | DA732X_HP_OUT_EN);

    /* Wait for voltage stabilization */
    msleep(DA732X_WAIT_FOR_STABILIZATION);

    /* Check output offset sign */
    sign[DA732X_HPL_AMP] = (snd_soc_component_read(component, DA732X_REG_HPL) & DA732X_HP_OUT_COMPO) as u8;
    sign[DA732X_HPR_AMP] = (snd_soc_component_read(component, DA732X_REG_HPR) & DA732X_HP_OUT_COMPO) as u8;

    snd_soc_component_write(component, DA732X_REG_HPL, DA732X_HP_OUT_COMP | ((sign[DA732X_HPL_AMP] as c_uint) >> DA732X_HP_OUT_COMPO_SHIFT) | DA732X_HP_OUT_EN);
    snd_soc_component_write(component, DA732X_REG_HPR, DA732X_HP_OUT_COMP | ((sign[DA732X_HPR_AMP] as c_uint) >> DA732X_HP_OUT_COMPO_SHIFT) | DA732X_HP_OUT_EN);

    /* Binary search output offset values (both channels at once) */
    while step != 0 {
        offset[DA732X_HPL_AMP] |= step;
        offset[DA732X_HPR_AMP] |= step;
        snd_soc_component_write(component, DA732X_REG_HPL_OUT_OFFSET, offset[DA732X_HPL_AMP] as c_uint);
        snd_soc_component_write(component, DA732X_REG_HPR_OUT_OFFSET, offset[DA732X_HPR_AMP] as c_uint);

        msleep(DA732X_WAIT_FOR_STABILIZATION);

        if (((snd_soc_component_read(component, DA732X_REG_HPL) & DA732X_HP_OUT_COMPO) as u8) ^ sign[DA732X_HPL_AMP]) != 0 {
            offset[DA732X_HPL_AMP] &= !step;
        }
        if (((snd_soc_component_read(component, DA732X_REG_HPR) & DA732X_HP_OUT_COMPO) as u8) ^ sign[DA732X_HPR_AMP]) != 0 {
            offset[DA732X_HPR_AMP] &= !step;
        }

        step >>= 1;
    }

    /* Write final DAC offsets to registers */
    snd_soc_component_write(component, DA732X_REG_HPL_OUT_OFFSET, offset[DA732X_HPL_AMP] as c_uint);
    snd_soc_component_write(component, DA732X_REG_HPR_OUT_OFFSET, offset[DA732X_HPR_AMP] as c_uint);
}

extern "C" {
    static DA732X_SOFTMUTE_EN: c_uint;
    static DA732X_GAIN_RAMPED: c_uint;
    static DA732X_16_SAMPLES: c_uint;
    static DA732X_DACL_EN: c_uint;
    static DA732X_DACR_EN: c_uint;
    static DA732X_DACL_SDM: c_uint;
    static DA732X_DACR_SDM: c_uint;
    static DA732X_DACL_MUTE: c_uint;
    static DA732X_DACR_MUTE: c_uint;
    static DA732X_HP_OUT_DAC_EN: c_uint;
    static DA732X_HP_OUT_MUTE: c_uint;
    static DA732X_DACS_DIS: c_uint;
    static DA732X_HP_DIS: c_uint;
}

unsafe fn da732x_hp_dc_offset_cancellation(component: *mut snd_soc_component) {
    /* Make sure that we have Soft Mute enabled */
    snd_soc_component_write(component, DA732X_REG_DAC1_SOFTMUTE, DA732X_SOFTMUTE_EN | DA732X_GAIN_RAMPED | DA732X_16_SAMPLES);
    snd_soc_component_write(component, DA732X_REG_DAC1_SEL, DA732X_DACL_EN | DA732X_DACR_EN | DA732X_DACL_SDM | DA732X_DACR_SDM | DA732X_DACL_MUTE | DA732X_DACR_MUTE);
    snd_soc_component_write(component, DA732X_REG_HPL, DA732X_HP_OUT_DAC_EN | DA732X_HP_OUT_MUTE | DA732X_HP_OUT_EN);
    snd_soc_component_write(component, DA732X_REG_HPR, DA732X_HP_OUT_EN | DA732X_HP_OUT_MUTE | DA732X_HP_OUT_DAC_EN);

    da732x_dac_offset_adjust(component);
    da732x_output_offset_adjust(component);

    snd_soc_component_write(component, DA732X_REG_DAC1_SEL, DA732X_DACS_DIS);
    snd_soc_component_write(component, DA732X_REG_HPL, DA732X_HP_DIS);
    snd_soc_component_write(component, DA732X_REG_HPR, DA732X_HP_DIS);
}

extern "C" {
    static DA732X_BIAS_BOOST_MASK: c_uint;
    static DA732X_BIAS_BOOST_100PC: c_uint;
    static DA732X_VMID_FASTCHG: c_uint;
    static DA732X_BIAS_EN: c_uint;
    static DA732X_STARTUP_DELAY: c_uint;
    static DA732X_REFBUFX2_EN: c_uint;
    static DA732X_BYPASS_DSP: c_uint;
    static DA732X_DIGITAL_EN: c_uint;
    static DA732X_HP_DRIVER_EN: c_uint;
    static DA732X_HP_GATE_LOW: c_uint;
    static DA732X_HP_LOOP_GAIN_CTRL: c_uint;
    static DA732X_HP_OUT_GNDSEL: c_uint;
    static DA732X_SYS3_CLK_EN: c_uint;
    static DA732X_PC_CLK_EN: c_uint;
    static DA732X_MIC1_PRE_ZC_EN: c_uint;
    static DA732X_MIC1_ZC_EN: c_uint;
    static DA732X_MIC2_PRE_ZC_EN: c_uint;
    static DA732X_MIC2_ZC_EN: c_uint;
    static DA732X_AUXL_ZC_EN: c_uint;
    static DA732X_AUXR_ZC_EN: c_uint;
    static DA732X_MIC3_PRE_ZC_EN: c_uint;
    static DA732X_MIC3_ZC_EN: c_uint;
    static DA732X_HPL_ZC_EN: c_uint;
    static DA732X_HPR_ZC_EN: c_uint;
    static DA732X_LIN2_ZC_EN: c_uint;
    static DA732X_LIN3_ZC_EN: c_uint;
    static DA732X_LIN4_ZC_EN: c_uint;
    static DA732X_BIAS_BOOST_50PC: c_uint;
    static DA732X_BIAS_DIS: c_uint;
}

unsafe extern "C" fn da732x_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let da732x = snd_soc_component_get_drvdata(component) as *mut da732x_priv;
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        SND_SOC_BIAS_ON => {
            snd_soc_component_update_bits(component, DA732X_REG_BIAS_EN, DA732X_BIAS_BOOST_MASK, DA732X_BIAS_BOOST_100PC);
        }
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                /* Init Codec */
                snd_soc_component_write(component, DA732X_REG_REF1, DA732X_VMID_FASTCHG);
                snd_soc_component_write(component, DA732X_REG_BIAS_EN, DA732X_BIAS_EN);
                mdelay(DA732X_STARTUP_DELAY);

                /* Disable Fast Charge and enable DAC ref voltage */
                snd_soc_component_write(component, DA732X_REG_REF1, DA732X_REFBUFX2_EN);

                /* Enable bypass DSP routing */
                snd_soc_component_write(component, DA732X_REG_DATA_ROUTE, DA732X_BYPASS_DSP);

                /* Enable Digital subsystem */
                snd_soc_component_write(component, DA732X_REG_DSP_CTRL, DA732X_DIGITAL_EN);
                snd_soc_component_write(component, DA732X_REG_SPARE1_OUT, DA732X_HP_DRIVER_EN | DA732X_HP_GATE_LOW | DA732X_HP_LOOP_GAIN_CTRL);
                snd_soc_component_write(component, DA732X_REG_HP_LIN1_GNDSEL, DA732X_HP_OUT_GNDSEL);

                da732x_set_charge_pump(component, DA732X_ENABLE_CP);
                snd_soc_component_write(component, DA732X_REG_CLK_EN1, DA732X_SYS3_CLK_EN | DA732X_PC_CLK_EN);

                /* Enable Zero Crossing */
                snd_soc_component_write(component, DA732X_REG_INP_ZC_EN, DA732X_MIC1_PRE_ZC_EN | DA732X_MIC1_ZC_EN | DA732X_MIC2_PRE_ZC_EN | DA732X_MIC2_ZC_EN | DA732X_AUXL_ZC_EN | DA732X_AUXR_ZC_EN | DA732X_MIC3_PRE_ZC_EN | DA732X_MIC3_ZC_EN);
                snd_soc_component_write(component, DA732X_REG_OUT_ZC_EN, DA732X_HPL_ZC_EN | DA732X_HPR_ZC_EN | DA732X_LIN2_ZC_EN | DA732X_LIN3_ZC_EN | DA732X_LIN4_ZC_EN);

                da732x_hp_dc_offset_cancellation(component);
                regcache_cache_only((*da732x).regmap, false);
                regcache_sync((*da732x).regmap);
            } else {
                snd_soc_component_update_bits(component, DA732X_REG_BIAS_EN, DA732X_BIAS_BOOST_MASK, DA732X_BIAS_BOOST_50PC);
                snd_soc_component_update_bits(component, DA732X_REG_PLL_CTRL, DA732X_PLL_EN, 0);
                (*da732x).pll_en = false;
            }
        }
        SND_SOC_BIAS_OFF => {
            regcache_cache_only((*da732x).regmap, true);
            da732x_set_charge_pump(component, DA732X_DISABLE_CP);
            snd_soc_component_update_bits(component, DA732X_REG_BIAS_EN, DA732X_BIAS_EN, DA732X_BIAS_DIS);
            (*da732x).pll_en = false;
        }
        _ => {}
    }

    0
}

static soc_component_dev_da732x: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(da732x_set_bias_level),
    controls: da732x_snd_controls.as_ptr(),
    num_controls: da732x_snd_controls.len() as c_uint,
    dapm_widgets: da732x_dapm_widgets.as_ptr(),
    num_dapm_widgets: da732x_dapm_widgets.len() as c_uint,
    dapm_routes: da732x_dapm_routes.as_ptr(),
    num_dapm_routes: da732x_dapm_routes.len() as c_uint,
    set_pll: Some(da732x_set_dai_pll),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

extern "C" {
    static DA732X_ID_MAJOR_MASK: c_uint;
    static DA732X_ID_MINOR_MASK: c_uint;
}

unsafe extern "C" fn da732x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let da732x: *mut da732x_priv;
    let mut reg: c_uint = 0;
    let mut ret: c_int;

    da732x = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<da732x_priv>(), GFP_KERNEL) as *mut da732x_priv;
    if da732x.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, da732x as *mut c_void);

    (*da732x).regmap = devm_regmap_init_i2c(i2c, &raw const da732x_regmap);
    if IS_ERR((*da732x).regmap as *const c_void) {
        ret = PTR_ERR((*da732x).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, b"Failed to initialize regmap\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = regmap_read((*da732x).regmap, DA732X_REG_ID, &mut reg);
    if ret < 0 {
        dev_err(&mut (*i2c).dev, b"Failed to read ID register: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    dev_info(
        &mut (*i2c).dev,
        b"Revision: %d.%d\n\0".as_ptr() as *const c_char,
        (reg & DA732X_ID_MAJOR_MASK) >> 4,
        reg & DA732X_ID_MINOR_MASK,
    );

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_da732x,
        da732x_dai.as_mut_ptr(),
        da732x_dai.len() as c_int,
    );
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to register component.\n\0".as_ptr() as *const c_char);
    }

    ret
}

static da732x_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'd' as c_char, b'a' as c_char, b'7' as c_char, b'3' as c_char, b'2' as c_char, b'0' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, da732x_i2c_id); */

static da732x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"da7320\0".as_ptr() as *const c_char,
    },
    probe: Some(da732x_i2c_probe),
    id_table: da732x_i2c_id.as_ptr(),
};

/* module_i2c_driver(da732x_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC DA732X driver"); */
/* MODULE_AUTHOR("Michal Hajduk <michal.hajduk@diasemi.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
