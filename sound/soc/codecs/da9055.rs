// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DA9055 ALSA Soc codec driver
 *
 * Copyright (c) 2012 Dialog Semiconductor
 *
 * Tested on (Samsung SMDK6410 board + DA9055 EVB) using I2S and I2C
 * Written by David Chen <david.chen@diasemi.com> and
 * Ashish Chavan <ashish.chavan@kpitcummins.com>
 */

/* Linux/ALSA dependencies from the original C includes are expected externally:
 * linux/delay.h, linux/i2c.h, linux/regmap.h, linux/slab.h, linux/module.h,
 * linux/of.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/initval.h,
 * sound/tlv.h, sound/da9055.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u32 = u32;
type bool_t = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct da9055_platform_data {
    pub micbias_source: c_int,
    pub micbias: c_int,
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
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
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
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
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
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub cache_type: c_uint,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

unsafe extern "C" {
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
}

/* DA9055 register space */

/* Status Registers */
const DA9055_STATUS1: c_uint = 0x02;
const DA9055_PLL_STATUS: c_uint = 0x03;
const DA9055_AUX_L_GAIN_STATUS: c_uint = 0x04;
const DA9055_AUX_R_GAIN_STATUS: c_uint = 0x05;
const DA9055_MIC_L_GAIN_STATUS: c_uint = 0x06;
const DA9055_MIC_R_GAIN_STATUS: c_uint = 0x07;
const DA9055_MIXIN_L_GAIN_STATUS: c_uint = 0x08;
const DA9055_MIXIN_R_GAIN_STATUS: c_uint = 0x09;
const DA9055_ADC_L_GAIN_STATUS: c_uint = 0x0A;
const DA9055_ADC_R_GAIN_STATUS: c_uint = 0x0B;
const DA9055_DAC_L_GAIN_STATUS: c_uint = 0x0C;
const DA9055_DAC_R_GAIN_STATUS: c_uint = 0x0D;
const DA9055_HP_L_GAIN_STATUS: c_uint = 0x0E;
const DA9055_HP_R_GAIN_STATUS: c_uint = 0x0F;
const DA9055_LINE_GAIN_STATUS: c_uint = 0x10;

/* System Initialisation Registers */
const DA9055_CIF_CTRL: c_uint = 0x20;
const DA9055_DIG_ROUTING_AIF: c_uint = 0x21;
const DA9055_SR: c_uint = 0x22;
const DA9055_REFERENCES: c_uint = 0x23;
const DA9055_PLL_FRAC_TOP: c_uint = 0x24;
const DA9055_PLL_FRAC_BOT: c_uint = 0x25;
const DA9055_PLL_INTEGER: c_uint = 0x26;
const DA9055_PLL_CTRL: c_uint = 0x27;
const DA9055_AIF_CLK_MODE: c_uint = 0x28;
const DA9055_AIF_CTRL: c_uint = 0x29;
const DA9055_DIG_ROUTING_DAC: c_uint = 0x2A;
const DA9055_ALC_CTRL1: c_uint = 0x2B;

/* Input - Gain, Select and Filter Registers */
const DA9055_AUX_L_GAIN: c_uint = 0x30;
const DA9055_AUX_R_GAIN: c_uint = 0x31;
const DA9055_MIXIN_L_SELECT: c_uint = 0x32;
const DA9055_MIXIN_R_SELECT: c_uint = 0x33;
const DA9055_MIXIN_L_GAIN: c_uint = 0x34;
const DA9055_MIXIN_R_GAIN: c_uint = 0x35;
const DA9055_ADC_L_GAIN: c_uint = 0x36;
const DA9055_ADC_R_GAIN: c_uint = 0x37;
const DA9055_ADC_FILTERS1: c_uint = 0x38;
const DA9055_MIC_L_GAIN: c_uint = 0x39;
const DA9055_MIC_R_GAIN: c_uint = 0x3A;

/* Output - Gain, Select and Filter Registers */
const DA9055_DAC_FILTERS5: c_uint = 0x40;
const DA9055_DAC_FILTERS2: c_uint = 0x41;
const DA9055_DAC_FILTERS3: c_uint = 0x42;
const DA9055_DAC_FILTERS4: c_uint = 0x43;
const DA9055_DAC_FILTERS1: c_uint = 0x44;
const DA9055_DAC_L_GAIN: c_uint = 0x45;
const DA9055_DAC_R_GAIN: c_uint = 0x46;
const DA9055_CP_CTRL: c_uint = 0x47;
const DA9055_HP_L_GAIN: c_uint = 0x48;
const DA9055_HP_R_GAIN: c_uint = 0x49;
const DA9055_LINE_GAIN: c_uint = 0x4A;
const DA9055_MIXOUT_L_SELECT: c_uint = 0x4B;
const DA9055_MIXOUT_R_SELECT: c_uint = 0x4C;

/* System Controller Registers */
const DA9055_SYSTEM_MODES_INPUT: c_uint = 0x50;
const DA9055_SYSTEM_MODES_OUTPUT: c_uint = 0x51;

/* Control Registers */
const DA9055_AUX_L_CTRL: c_uint = 0x60;
const DA9055_AUX_R_CTRL: c_uint = 0x61;
const DA9055_MIC_BIAS_CTRL: c_uint = 0x62;
const DA9055_MIC_L_CTRL: c_uint = 0x63;
const DA9055_MIC_R_CTRL: c_uint = 0x64;
const DA9055_MIXIN_L_CTRL: c_uint = 0x65;
const DA9055_MIXIN_R_CTRL: c_uint = 0x66;
const DA9055_ADC_L_CTRL: c_uint = 0x67;
const DA9055_ADC_R_CTRL: c_uint = 0x68;
const DA9055_DAC_L_CTRL: c_uint = 0x69;
const DA9055_DAC_R_CTRL: c_uint = 0x6A;
const DA9055_HP_L_CTRL: c_uint = 0x6B;
const DA9055_HP_R_CTRL: c_uint = 0x6C;
const DA9055_LINE_CTRL: c_uint = 0x6D;
const DA9055_MIXOUT_L_CTRL: c_uint = 0x6E;
const DA9055_MIXOUT_R_CTRL: c_uint = 0x6F;

/* Configuration Registers */
const DA9055_LDO_CTRL: c_uint = 0x90;
const DA9055_IO_CTRL: c_uint = 0x91;
const DA9055_GAIN_RAMP_CTRL: c_uint = 0x92;
const DA9055_MIC_CONFIG: c_uint = 0x93;
const DA9055_PC_COUNT: c_uint = 0x94;
const DA9055_CP_VOL_THRESHOLD1: c_uint = 0x95;
const DA9055_CP_DELAY: c_uint = 0x96;
const DA9055_CP_DETECTOR: c_uint = 0x97;
const DA9055_AIF_OFFSET: c_uint = 0x98;
const DA9055_DIG_CTRL: c_uint = 0x99;
const DA9055_ALC_CTRL2: c_uint = 0x9A;
const DA9055_ALC_CTRL3: c_uint = 0x9B;
const DA9055_ALC_NOISE: c_uint = 0x9C;
const DA9055_ALC_TARGET_MIN: c_uint = 0x9D;
const DA9055_ALC_TARGET_MAX: c_uint = 0x9E;
const DA9055_ALC_GAIN_LIMITS: c_uint = 0x9F;
const DA9055_ALC_ANA_GAIN_LIMITS: c_uint = 0xA0;
const DA9055_ALC_ANTICLIP_CTRL: c_uint = 0xA1;
const DA9055_ALC_ANTICLIP_LEVEL: c_uint = 0xA2;
const DA9055_ALC_OFFSET_OP2M_L: c_uint = 0xA6;
const DA9055_ALC_OFFSET_OP2U_L: c_uint = 0xA7;
const DA9055_ALC_OFFSET_OP2M_R: c_uint = 0xAB;
const DA9055_ALC_OFFSET_OP2U_R: c_uint = 0xAC;
const DA9055_ALC_CIC_OP_LVL_CTRL: c_uint = 0xAD;
const DA9055_ALC_CIC_OP_LVL_DATA: c_uint = 0xAE;
const DA9055_DAC_NG_SETUP_TIME: c_uint = 0xAF;
const DA9055_DAC_NG_OFF_THRESHOLD: c_uint = 0xB0;
const DA9055_DAC_NG_ON_THRESHOLD: c_uint = 0xB1;
const DA9055_DAC_NG_CTRL: c_uint = 0xB2;

/* SR bit fields */
const DA9055_SR_8000: c_uint = 0x1 << 0;
const DA9055_SR_11025: c_uint = 0x2 << 0;
const DA9055_SR_12000: c_uint = 0x3 << 0;
const DA9055_SR_16000: c_uint = 0x5 << 0;
const DA9055_SR_22050: c_uint = 0x6 << 0;
const DA9055_SR_24000: c_uint = 0x7 << 0;
const DA9055_SR_32000: c_uint = 0x9 << 0;
const DA9055_SR_44100: c_uint = 0xA << 0;
const DA9055_SR_48000: c_uint = 0xB << 0;
const DA9055_SR_88200: c_uint = 0xE << 0;
const DA9055_SR_96000: c_uint = 0xF << 0;

/* REFERENCES bit fields */
const DA9055_BIAS_EN: c_uint = 1 << 3;
const DA9055_VMID_EN: c_uint = 1 << 7;

/* PLL_CTRL bit fields */
const DA9055_PLL_INDIV_10_20_MHZ: c_uint = 1 << 2;
const DA9055_PLL_SRM_EN: c_uint = 1 << 6;
const DA9055_PLL_EN: c_uint = 1 << 7;

/* AIF_CLK_MODE bit fields */
const DA9055_AIF_BCLKS_PER_WCLK_32: c_uint = 0 << 0;
const DA9055_AIF_BCLKS_PER_WCLK_64: c_uint = 1 << 0;
const DA9055_AIF_BCLKS_PER_WCLK_128: c_uint = 2 << 0;
const DA9055_AIF_BCLKS_PER_WCLK_256: c_uint = 3 << 0;
const DA9055_AIF_CLK_EN_SLAVE_MODE: c_uint = 0 << 7;
const DA9055_AIF_CLK_EN_MASTER_MODE: c_uint = 1 << 7;

/* AIF_CTRL bit fields */
const DA9055_AIF_FORMAT_I2S_MODE: c_uint = 0 << 0;
const DA9055_AIF_FORMAT_LEFT_J: c_uint = 1 << 0;
const DA9055_AIF_FORMAT_RIGHT_J: c_uint = 2 << 0;
const DA9055_AIF_FORMAT_DSP: c_uint = 3 << 0;
const DA9055_AIF_WORD_S16_LE: c_uint = 0 << 2;
const DA9055_AIF_WORD_S20_3LE: c_uint = 1 << 2;
const DA9055_AIF_WORD_S24_LE: c_uint = 2 << 2;
const DA9055_AIF_WORD_S32_LE: c_uint = 3 << 2;

/* MIC_L_CTRL bit fields */
const DA9055_MIC_L_MUTE_EN: c_uint = 1 << 6;

/* MIC_R_CTRL bit fields */
const DA9055_MIC_R_MUTE_EN: c_uint = 1 << 6;

/* MIXIN_L_CTRL bit fields */
const DA9055_MIXIN_L_MIX_EN: c_uint = 1 << 3;

/* MIXIN_R_CTRL bit fields */
const DA9055_MIXIN_R_MIX_EN: c_uint = 1 << 3;

/* ADC_L_CTRL bit fields */
const DA9055_ADC_L_EN: c_uint = 1 << 7;

/* ADC_R_CTRL bit fields */
const DA9055_ADC_R_EN: c_uint = 1 << 7;

/* DAC_L_CTRL bit fields */
const DA9055_DAC_L_MUTE_EN: c_uint = 1 << 6;

/* DAC_R_CTRL bit fields */
const DA9055_DAC_R_MUTE_EN: c_uint = 1 << 6;

/* HP_L_CTRL bit fields */
const DA9055_HP_L_AMP_OE: c_uint = 1 << 3;

/* HP_R_CTRL bit fields */
const DA9055_HP_R_AMP_OE: c_uint = 1 << 3;

/* LINE_CTRL bit fields */
const DA9055_LINE_AMP_OE: c_uint = 1 << 3;

/* MIXOUT_L_CTRL bit fields */
const DA9055_MIXOUT_L_MIX_EN: c_uint = 1 << 3;

/* MIXOUT_R_CTRL bit fields */
const DA9055_MIXOUT_R_MIX_EN: c_uint = 1 << 3;

/* MIC bias select bit fields */
const DA9055_MICBIAS2_EN: c_uint = 1 << 6;

/* ALC_CIC_OP_LEVEL_CTRL bit fields */
const DA9055_ALC_DATA_MIDDLE: c_uint = 2 << 0;
const DA9055_ALC_DATA_TOP: c_uint = 3 << 0;
const DA9055_ALC_CIC_OP_CHANNEL_LEFT: c_uint = 0 << 7;
const DA9055_ALC_CIC_OP_CHANNEL_RIGHT: c_uint = 1 << 7;

const DA9055_AIF_BCLK_MASK: c_uint = 3 << 0;
const DA9055_AIF_CLK_MODE_MASK: c_uint = 1 << 7;
const DA9055_AIF_FORMAT_MASK: c_uint = 3 << 0;
const DA9055_AIF_WORD_LENGTH_MASK: c_uint = 3 << 2;
const DA9055_GAIN_RAMPING_EN: c_uint = 1 << 5;
const DA9055_MICBIAS_LEVEL_MASK: c_uint = 3 << 4;

const DA9055_ALC_OFFSET_15_8: c_int = 0x00FF00;
const DA9055_ALC_OFFSET_17_16: c_int = 0x030000;
const DA9055_ALC_AVG_ITERATIONS: u8 = 5;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SND_SOC_NOPM: c_uint = 0;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 3;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 1;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 2;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const DA9055_MICBIAS_2_2V: c_int = 0;
const DA9055_MICBIAS_2_1V: c_int = 1;
const DA9055_MICBIAS_1_8V: c_int = 2;
const DA9055_MICBIAS_1_6V: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
struct pll_div {
    fref: c_int,
    fout: c_int,
    frac_top: u8,
    frac_bot: u8,
    integer: u8,
    mode: u8, /* 0 = slave, 1 = master */
}

/* PLL divisor table */
static da9055_pll_div: [pll_div; 27] = [
    /* for MASTER mode, fs = 44.1Khz and its harmonics */
    pll_div { fref: 11289600, fout: 2822400, frac_top: 0x00, frac_bot: 0x00, integer: 0x20, mode: 1 }, /* MCLK=11.2896Mhz */
    pll_div { fref: 12000000, fout: 2822400, frac_top: 0x03, frac_bot: 0x61, integer: 0x1E, mode: 1 }, /* MCLK=12Mhz */
    pll_div { fref: 12288000, fout: 2822400, frac_top: 0x0C, frac_bot: 0xCC, integer: 0x1D, mode: 1 }, /* MCLK=12.288Mhz */
    pll_div { fref: 13000000, fout: 2822400, frac_top: 0x19, frac_bot: 0x45, integer: 0x1B, mode: 1 }, /* MCLK=13Mhz */
    pll_div { fref: 13500000, fout: 2822400, frac_top: 0x18, frac_bot: 0x56, integer: 0x1A, mode: 1 }, /* MCLK=13.5Mhz */
    pll_div { fref: 14400000, fout: 2822400, frac_top: 0x02, frac_bot: 0xD0, integer: 0x19, mode: 1 }, /* MCLK=14.4Mhz */
    pll_div { fref: 19200000, fout: 2822400, frac_top: 0x1A, frac_bot: 0x1C, integer: 0x12, mode: 1 }, /* MCLK=19.2Mhz */
    pll_div { fref: 19680000, fout: 2822400, frac_top: 0x0B, frac_bot: 0x6D, integer: 0x12, mode: 1 }, /* MCLK=19.68Mhz */
    pll_div { fref: 19800000, fout: 2822400, frac_top: 0x07, frac_bot: 0xDD, integer: 0x12, mode: 1 }, /* MCLK=19.8Mhz */
    /* for MASTER mode, fs = 48Khz and its harmonics */
    pll_div { fref: 11289600, fout: 3072000, frac_top: 0x1A, frac_bot: 0x8E, integer: 0x22, mode: 1 }, /* MCLK=11.2896Mhz */
    pll_div { fref: 12000000, fout: 3072000, frac_top: 0x18, frac_bot: 0x93, integer: 0x20, mode: 1 }, /* MCLK=12Mhz */
    pll_div { fref: 12288000, fout: 3072000, frac_top: 0x00, frac_bot: 0x00, integer: 0x20, mode: 1 }, /* MCLK=12.288Mhz */
    pll_div { fref: 13000000, fout: 3072000, frac_top: 0x07, frac_bot: 0xEA, integer: 0x1E, mode: 1 }, /* MCLK=13Mhz */
    pll_div { fref: 13500000, fout: 3072000, frac_top: 0x04, frac_bot: 0x11, integer: 0x1D, mode: 1 }, /* MCLK=13.5Mhz */
    pll_div { fref: 14400000, fout: 3072000, frac_top: 0x09, frac_bot: 0xD0, integer: 0x1B, mode: 1 }, /* MCLK=14.4Mhz */
    pll_div { fref: 19200000, fout: 3072000, frac_top: 0x0F, frac_bot: 0x5C, integer: 0x14, mode: 1 }, /* MCLK=19.2Mhz */
    pll_div { fref: 19680000, fout: 3072000, frac_top: 0x1F, frac_bot: 0x60, integer: 0x13, mode: 1 }, /* MCLK=19.68Mhz */
    pll_div { fref: 19800000, fout: 3072000, frac_top: 0x1B, frac_bot: 0x80, integer: 0x13, mode: 1 }, /* MCLK=19.8Mhz */
    /* for SLAVE mode with SRM */
    pll_div { fref: 11289600, fout: 2822400, frac_top: 0x0D, frac_bot: 0x47, integer: 0x21, mode: 0 }, /* MCLK=11.2896Mhz */
    pll_div { fref: 12000000, fout: 2822400, frac_top: 0x0D, frac_bot: 0xFA, integer: 0x1F, mode: 0 }, /* MCLK=12Mhz */
    pll_div { fref: 12288000, fout: 2822400, frac_top: 0x16, frac_bot: 0x66, integer: 0x1E, mode: 0 }, /* MCLK=12.288Mhz */
    pll_div { fref: 13000000, fout: 2822400, frac_top: 0x00, frac_bot: 0x98, integer: 0x1D, mode: 0 }, /* MCLK=13Mhz */
    pll_div { fref: 13500000, fout: 2822400, frac_top: 0x1E, frac_bot: 0x33, integer: 0x1B, mode: 0 }, /* MCLK=13.5Mhz */
    pll_div { fref: 14400000, fout: 2822400, frac_top: 0x06, frac_bot: 0x50, integer: 0x1A, mode: 0 }, /* MCLK=14.4Mhz */
    pll_div { fref: 19200000, fout: 2822400, frac_top: 0x14, frac_bot: 0xBC, integer: 0x13, mode: 0 }, /* MCLK=19.2Mhz */
    pll_div { fref: 19680000, fout: 2822400, frac_top: 0x05, frac_bot: 0x66, integer: 0x13, mode: 0 }, /* MCLK=19.68Mhz */
    pll_div { fref: 19800000, fout: 2822400, frac_top: 0x01, frac_bot: 0xAE, integer: 0x13, mode: 0 }, /* MCLK=19.8Mhz  */
];

#[repr(C)]
#[derive(Copy, Clone)]
enum clk_src {
    DA9055_CLKSRC_MCLK,
}

/* Gain and Volume: these ALSA TLV and SOC enum/control macros are supplied by
 * sound/tlv.h and sound/soc.h in the source tree. They are preserved here as
 * Rust macro-style dependency invocations carrying the original arguments.
 */
DECLARE_TLV_DB_RANGE!(aux_vol_tlv,
    0x0, 0x10, TLV_DB_SCALE_ITEM!(-5400, 0, 0),
    /* -54dB to 15dB */
    0x11, 0x3f, TLV_DB_SCALE_ITEM!(-5400, 150, 0)
);
DECLARE_TLV_DB_RANGE!(digital_gain_tlv,
    0x0, 0x07, TLV_DB_SCALE_ITEM!(TLV_DB_GAIN_MUTE, 0, 1),
    /* -78dB to 12dB */
    0x08, 0x7f, TLV_DB_SCALE_ITEM!(-7800, 75, 0)
);
DECLARE_TLV_DB_RANGE!(alc_analog_gain_tlv,
    0x0, 0x0, TLV_DB_SCALE_ITEM!(TLV_DB_GAIN_MUTE, 0, 1),
    /* 0dB to 36dB */
    0x01, 0x07, TLV_DB_SCALE_ITEM!(0, 600, 0)
);
DECLARE_TLV_DB_SCALE!(mic_vol_tlv, -600, 600, 0);
DECLARE_TLV_DB_SCALE!(mixin_gain_tlv, -450, 150, 0);
DECLARE_TLV_DB_SCALE!(eq_gain_tlv, -1050, 150, 0);
DECLARE_TLV_DB_SCALE!(hp_vol_tlv, -5700, 100, 0);
DECLARE_TLV_DB_SCALE!(lineout_vol_tlv, -4800, 100, 0);
DECLARE_TLV_DB_SCALE!(alc_threshold_tlv, -9450, 150, 0);
DECLARE_TLV_DB_SCALE!(alc_gain_tlv, 0, 600, 0);

/* ADC and DAC high pass filter cutoff value */
static da9055_hpf_cutoff_txt: [*const c_char; 4] = [
    c"Fs/24000".as_ptr(), c"Fs/12000".as_ptr(), c"Fs/6000".as_ptr(), c"Fs/3000".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_dac_hpf_cutoff, DA9055_DAC_FILTERS1, 4, da9055_hpf_cutoff_txt);
SOC_ENUM_SINGLE_DECL!(da9055_adc_hpf_cutoff, DA9055_ADC_FILTERS1, 4, da9055_hpf_cutoff_txt);

/* ADC and DAC voice mode (8kHz) high pass cutoff value */
static da9055_vf_cutoff_txt: [*const c_char; 8] = [
    c"2.5Hz".as_ptr(), c"25Hz".as_ptr(), c"50Hz".as_ptr(), c"100Hz".as_ptr(),
    c"150Hz".as_ptr(), c"200Hz".as_ptr(), c"300Hz".as_ptr(), c"400Hz".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_dac_vf_cutoff, DA9055_DAC_FILTERS1, 0, da9055_vf_cutoff_txt);
SOC_ENUM_SINGLE_DECL!(da9055_adc_vf_cutoff, DA9055_ADC_FILTERS1, 0, da9055_vf_cutoff_txt);

/* Gain ramping rate value */
static da9055_gain_ramping_txt: [*const c_char; 4] = [
    c"nominal rate".as_ptr(), c"nominal rate * 4".as_ptr(),
    c"nominal rate * 8".as_ptr(), c"nominal rate / 8".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_gain_ramping_rate, DA9055_GAIN_RAMP_CTRL, 0, da9055_gain_ramping_txt);

/* DAC noise gate setup/ramp and soft mute values */
static da9055_dac_ng_setup_time_txt: [*const c_char; 4] = [
    c"256 samples".as_ptr(), c"512 samples".as_ptr(), c"1024 samples".as_ptr(), c"2048 samples".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_dac_ng_setup_time, DA9055_DAC_NG_SETUP_TIME, 0, da9055_dac_ng_setup_time_txt);
static da9055_dac_ng_rampup_txt: [*const c_char; 2] = [c"0.02 ms/dB".as_ptr(), c"0.16 ms/dB".as_ptr()];
SOC_ENUM_SINGLE_DECL!(da9055_dac_ng_rampup_rate, DA9055_DAC_NG_SETUP_TIME, 2, da9055_dac_ng_rampup_txt);
static da9055_dac_ng_rampdown_txt: [*const c_char; 2] = [c"0.64 ms/dB".as_ptr(), c"20.48 ms/dB".as_ptr()];
SOC_ENUM_SINGLE_DECL!(da9055_dac_ng_rampdown_rate, DA9055_DAC_NG_SETUP_TIME, 3, da9055_dac_ng_rampdown_txt);
static da9055_dac_soft_mute_rate_txt: [*const c_char; 7] = [
    c"1".as_ptr(), c"2".as_ptr(), c"4".as_ptr(), c"8".as_ptr(), c"16".as_ptr(), c"32".as_ptr(), c"64".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_dac_soft_mute_rate, DA9055_DAC_FILTERS5, 4, da9055_dac_soft_mute_rate_txt);

/* DAC and MIC routing selects */
static da9055_dac_src_txt: [*const c_char; 4] = [
    c"ADC output left".as_ptr(), c"ADC output right".as_ptr(), c"AIF input left".as_ptr(), c"AIF input right".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_dac_l_src, DA9055_DIG_ROUTING_DAC, 0, da9055_dac_src_txt);
SOC_ENUM_SINGLE_DECL!(da9055_dac_r_src, DA9055_DIG_ROUTING_DAC, 4, da9055_dac_src_txt);
static da9055_mic_l_src_txt: [*const c_char; 4] = [
    c"MIC1_P_N".as_ptr(), c"MIC1_P".as_ptr(), c"MIC1_N".as_ptr(), c"MIC2_L".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_mic_l_src, DA9055_MIXIN_L_SELECT, 4, da9055_mic_l_src_txt);
static da9055_mic_r_src_txt: [*const c_char; 3] = [
    c"MIC2_R_L".as_ptr(), c"MIC2_R".as_ptr(), c"MIC2_L".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_mic_r_src, DA9055_MIXIN_R_SELECT, 4, da9055_mic_r_src_txt);

/* ALC rate selects */
static da9055_signal_tracking_rate_txt: [*const c_char; 4] = [
    c"1/4".as_ptr(), c"1/16".as_ptr(), c"1/256".as_ptr(), c"1/65536".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_integ_attack_rate, DA9055_ALC_CTRL3, 4, da9055_signal_tracking_rate_txt);
SOC_ENUM_SINGLE_DECL!(da9055_integ_release_rate, DA9055_ALC_CTRL3, 6, da9055_signal_tracking_rate_txt);
static da9055_attack_rate_txt: [*const c_char; 13] = [
    c"44/fs".as_ptr(), c"88/fs".as_ptr(), c"176/fs".as_ptr(), c"352/fs".as_ptr(), c"704/fs".as_ptr(),
    c"1408/fs".as_ptr(), c"2816/fs".as_ptr(), c"5632/fs".as_ptr(), c"11264/fs".as_ptr(),
    c"22528/fs".as_ptr(), c"45056/fs".as_ptr(), c"90112/fs".as_ptr(), c"180224/fs".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_attack_rate, DA9055_ALC_CTRL2, 0, da9055_attack_rate_txt);
static da9055_release_rate_txt: [*const c_char; 11] = [
    c"176/fs".as_ptr(), c"352/fs".as_ptr(), c"704/fs".as_ptr(), c"1408/fs".as_ptr(), c"2816/fs".as_ptr(),
    c"5632/fs".as_ptr(), c"11264/fs".as_ptr(), c"22528/fs".as_ptr(), c"45056/fs".as_ptr(),
    c"90112/fs".as_ptr(), c"180224/fs".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_release_rate, DA9055_ALC_CTRL2, 4, da9055_release_rate_txt);
static da9055_hold_time_txt: [*const c_char; 16] = [
    c"62/fs".as_ptr(), c"124/fs".as_ptr(), c"248/fs".as_ptr(), c"496/fs".as_ptr(), c"992/fs".as_ptr(),
    c"1984/fs".as_ptr(), c"3968/fs".as_ptr(), c"7936/fs".as_ptr(), c"15872/fs".as_ptr(),
    c"31744/fs".as_ptr(), c"63488/fs".as_ptr(), c"126976/fs".as_ptr(), c"253952/fs".as_ptr(),
    c"507904/fs".as_ptr(), c"1015808/fs".as_ptr(), c"2031616/fs".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(da9055_hold_time, DA9055_ALC_CTRL3, 0, da9055_hold_time_txt);

unsafe extern "C" fn da9055_get_alc_data(component: *mut snd_soc_component, reg_val: u8) -> c_int {
    let mut mid_data: c_int;
    let mut top_data: c_int;
    let mut sum: c_int = 0;
    let mut iteration: u8 = 0;

    while iteration < DA9055_ALC_AVG_ITERATIONS {
        /* Select the left or right channel and capture data */
        snd_soc_component_write(component, DA9055_ALC_CIC_OP_LVL_CTRL, reg_val as c_uint);

        /* Select middle 8 bits for read back from data register */
        snd_soc_component_write(component, DA9055_ALC_CIC_OP_LVL_CTRL, (reg_val as c_uint) | DA9055_ALC_DATA_MIDDLE);
        mid_data = snd_soc_component_read(component, DA9055_ALC_CIC_OP_LVL_DATA) as c_int;

        /* Select top 8 bits for read back from data register */
        snd_soc_component_write(component, DA9055_ALC_CIC_OP_LVL_CTRL, (reg_val as c_uint) | DA9055_ALC_DATA_TOP);
        top_data = snd_soc_component_read(component, DA9055_ALC_CIC_OP_LVL_DATA) as c_int;

        sum += (mid_data << 8) | (top_data << 16);
        iteration = iteration.wrapping_add(1);
    }

    sum / DA9055_ALC_AVG_ITERATIONS as c_int
}

unsafe extern "C" fn da9055_put_alc_sw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mut reg_val: u8;
    let mut adc_left: u8;
    let mut adc_right: u8;
    let mut mic_left: u8;
    let mut mic_right: u8;
    let mut avg_left_data: c_int;
    let mut avg_right_data: c_int;
    let mut offset_l: c_int;
    let mut offset_r: c_int;

    if (*ucontrol).value.integer.value[0] != 0 {
        /*
         * While enabling ALC (or ALC sync mode), calibration of the DC
         * offsets must be done first
         */

        /* Save current values from Mic control registers */
        mic_left = snd_soc_component_read(component, DA9055_MIC_L_CTRL) as u8;
        mic_right = snd_soc_component_read(component, DA9055_MIC_R_CTRL) as u8;

        /* Mute Mic PGA Left and Right */
        snd_soc_component_update_bits(component, DA9055_MIC_L_CTRL, DA9055_MIC_L_MUTE_EN, DA9055_MIC_L_MUTE_EN);
        snd_soc_component_update_bits(component, DA9055_MIC_R_CTRL, DA9055_MIC_R_MUTE_EN, DA9055_MIC_R_MUTE_EN);

        /* Save current values from ADC control registers */
        adc_left = snd_soc_component_read(component, DA9055_ADC_L_CTRL) as u8;
        adc_right = snd_soc_component_read(component, DA9055_ADC_R_CTRL) as u8;

        /* Enable ADC Left and Right */
        snd_soc_component_update_bits(component, DA9055_ADC_L_CTRL, DA9055_ADC_L_EN, DA9055_ADC_L_EN);
        snd_soc_component_update_bits(component, DA9055_ADC_R_CTRL, DA9055_ADC_R_EN, DA9055_ADC_R_EN);

        /* Calculate average for Left and Right data */
        avg_left_data = da9055_get_alc_data(component, DA9055_ALC_CIC_OP_CHANNEL_LEFT as u8);
        avg_right_data = da9055_get_alc_data(component, DA9055_ALC_CIC_OP_CHANNEL_RIGHT as u8);

        /* Calculate DC offset */
        offset_l = -avg_left_data;
        offset_r = -avg_right_data;

        reg_val = ((offset_l & DA9055_ALC_OFFSET_15_8) >> 8) as u8;
        snd_soc_component_write(component, DA9055_ALC_OFFSET_OP2M_L, reg_val as c_uint);
        reg_val = ((offset_l & DA9055_ALC_OFFSET_17_16) >> 16) as u8;
        snd_soc_component_write(component, DA9055_ALC_OFFSET_OP2U_L, reg_val as c_uint);

        reg_val = ((offset_r & DA9055_ALC_OFFSET_15_8) >> 8) as u8;
        snd_soc_component_write(component, DA9055_ALC_OFFSET_OP2M_R, reg_val as c_uint);
        reg_val = ((offset_r & DA9055_ALC_OFFSET_17_16) >> 16) as u8;
        snd_soc_component_write(component, DA9055_ALC_OFFSET_OP2U_R, reg_val as c_uint);

        /* Restore original values of ADC control registers */
        snd_soc_component_write(component, DA9055_ADC_L_CTRL, adc_left as c_uint);
        snd_soc_component_write(component, DA9055_ADC_R_CTRL, adc_right as c_uint);

        /* Restore original values of Mic control registers */
        snd_soc_component_write(component, DA9055_MIC_L_CTRL, mic_left as c_uint);
        snd_soc_component_write(component, DA9055_MIC_R_CTRL, mic_right as c_uint);
    }

    snd_soc_put_volsw(kcontrol, ucontrol)
}

/* ALSA kcontrol arrays and DAPM widget/control arrays from the original C file
 * are translated as dependency macro invocations. These preserve the complete
 * source-level arguments and ordering from the C macro initializers.
 */
snd_controls_array!(da9055_snd_controls, snd_kcontrol_new, [
    SOC_DOUBLE_R_TLV!("Mic Volume", DA9055_MIC_L_GAIN, DA9055_MIC_R_GAIN, 0, 0x7, 0, mic_vol_tlv),
    SOC_DOUBLE_R_TLV!("Aux Volume", DA9055_AUX_L_GAIN, DA9055_AUX_R_GAIN, 0, 0x3f, 0, aux_vol_tlv),
    SOC_DOUBLE_R_TLV!("Mixin PGA Volume", DA9055_MIXIN_L_GAIN, DA9055_MIXIN_R_GAIN, 0, 0xf, 0, mixin_gain_tlv),
    SOC_DOUBLE_R_TLV!("ADC Volume", DA9055_ADC_L_GAIN, DA9055_ADC_R_GAIN, 0, 0x7f, 0, digital_gain_tlv),
    SOC_DOUBLE_R_TLV!("DAC Volume", DA9055_DAC_L_GAIN, DA9055_DAC_R_GAIN, 0, 0x7f, 0, digital_gain_tlv),
    SOC_DOUBLE_R_TLV!("Headphone Volume", DA9055_HP_L_GAIN, DA9055_HP_R_GAIN, 0, 0x3f, 0, hp_vol_tlv),
    SOC_SINGLE_TLV!("Lineout Volume", DA9055_LINE_GAIN, 0, 0x3f, 0, lineout_vol_tlv),
    SOC_SINGLE!("DAC EQ Switch", DA9055_DAC_FILTERS4, 7, 1, 0),
    SOC_SINGLE_TLV!("DAC EQ1 Volume", DA9055_DAC_FILTERS2, 0, 0xf, 0, eq_gain_tlv),
    SOC_SINGLE_TLV!("DAC EQ2 Volume", DA9055_DAC_FILTERS2, 4, 0xf, 0, eq_gain_tlv),
    SOC_SINGLE_TLV!("DAC EQ3 Volume", DA9055_DAC_FILTERS3, 0, 0xf, 0, eq_gain_tlv),
    SOC_SINGLE_TLV!("DAC EQ4 Volume", DA9055_DAC_FILTERS3, 4, 0xf, 0, eq_gain_tlv),
    SOC_SINGLE_TLV!("DAC EQ5 Volume", DA9055_DAC_FILTERS4, 0, 0xf, 0, eq_gain_tlv),
    SOC_SINGLE!("ADC HPF Switch", DA9055_ADC_FILTERS1, 7, 1, 0),
    SOC_ENUM!("ADC HPF Cutoff", da9055_adc_hpf_cutoff),
    SOC_SINGLE!("ADC Voice Mode Switch", DA9055_ADC_FILTERS1, 3, 1, 0),
    SOC_ENUM!("ADC Voice Cutoff", da9055_adc_vf_cutoff),
    SOC_SINGLE!("DAC HPF Switch", DA9055_DAC_FILTERS1, 7, 1, 0),
    SOC_ENUM!("DAC HPF Cutoff", da9055_dac_hpf_cutoff),
    SOC_SINGLE!("DAC Voice Mode Switch", DA9055_DAC_FILTERS1, 3, 1, 0),
    SOC_ENUM!("DAC Voice Cutoff", da9055_dac_vf_cutoff),
    SOC_DOUBLE_R!("Mic Switch", DA9055_MIC_L_CTRL, DA9055_MIC_R_CTRL, 6, 1, 0),
    SOC_DOUBLE_R!("Aux Switch", DA9055_AUX_L_CTRL, DA9055_AUX_R_CTRL, 6, 1, 0),
    SOC_DOUBLE_R!("Mixin PGA Switch", DA9055_MIXIN_L_CTRL, DA9055_MIXIN_R_CTRL, 6, 1, 0),
    SOC_DOUBLE_R!("ADC Switch", DA9055_ADC_L_CTRL, DA9055_ADC_R_CTRL, 6, 1, 0),
    SOC_DOUBLE_R!("Headphone Switch", DA9055_HP_L_CTRL, DA9055_HP_R_CTRL, 6, 1, 0),
    SOC_SINGLE!("Lineout Switch", DA9055_LINE_CTRL, 6, 1, 0),
    SOC_SINGLE!("DAC Soft Mute Switch", DA9055_DAC_FILTERS5, 7, 1, 0),
    SOC_ENUM!("DAC Soft Mute Rate", da9055_dac_soft_mute_rate),
    SOC_DOUBLE_R!("Aux ZC Switch", DA9055_AUX_L_CTRL, DA9055_AUX_R_CTRL, 4, 1, 0),
    SOC_DOUBLE_R!("Mixin PGA ZC Switch", DA9055_MIXIN_L_CTRL, DA9055_MIXIN_R_CTRL, 4, 1, 0),
    SOC_DOUBLE_R!("Headphone ZC Switch", DA9055_HP_L_CTRL, DA9055_HP_R_CTRL, 4, 1, 0),
    SOC_SINGLE!("Lineout ZC Switch", DA9055_LINE_CTRL, 4, 1, 0),
    SOC_DOUBLE_R!("Aux Gain Ramping Switch", DA9055_AUX_L_CTRL, DA9055_AUX_R_CTRL, 5, 1, 0),
    SOC_DOUBLE_R!("Mixin Gain Ramping Switch", DA9055_MIXIN_L_CTRL, DA9055_MIXIN_R_CTRL, 5, 1, 0),
    SOC_DOUBLE_R!("ADC Gain Ramping Switch", DA9055_ADC_L_CTRL, DA9055_ADC_R_CTRL, 5, 1, 0),
    SOC_DOUBLE_R!("DAC Gain Ramping Switch", DA9055_DAC_L_CTRL, DA9055_DAC_R_CTRL, 5, 1, 0),
    SOC_DOUBLE_R!("Headphone Gain Ramping Switch", DA9055_HP_L_CTRL, DA9055_HP_R_CTRL, 5, 1, 0),
    SOC_SINGLE!("Lineout Gain Ramping Switch", DA9055_LINE_CTRL, 5, 1, 0),
    SOC_ENUM!("Gain Ramping Rate", da9055_gain_ramping_rate),
    SOC_SINGLE!("DAC NG Switch", DA9055_DAC_NG_CTRL, 7, 1, 0),
    SOC_SINGLE!("DAC NG ON Threshold", DA9055_DAC_NG_ON_THRESHOLD, 0, 0x7, 0),
    SOC_SINGLE!("DAC NG OFF Threshold", DA9055_DAC_NG_OFF_THRESHOLD, 0, 0x7, 0),
    SOC_ENUM!("DAC NG Setup Time", da9055_dac_ng_setup_time),
    SOC_ENUM!("DAC NG Rampup Rate", da9055_dac_ng_rampup_rate),
    SOC_ENUM!("DAC NG Rampdown Rate", da9055_dac_ng_rampdown_rate),
    SOC_SINGLE!("DAC Left Invert", DA9055_DIG_CTRL, 3, 1, 0),
    SOC_SINGLE!("DAC Right Invert", DA9055_DIG_CTRL, 7, 1, 0),
    SOC_DOUBLE_R!("DMIC Switch", DA9055_MIXIN_L_SELECT, DA9055_MIXIN_R_SELECT, 7, 1, 0),
    SOC_DOUBLE_EXT!("ALC Switch", DA9055_ALC_CTRL1, 3, 7, 1, 0, snd_soc_get_volsw, da9055_put_alc_sw),
    SOC_SINGLE_EXT!("ALC Sync Mode Switch", DA9055_ALC_CTRL1, 1, 1, 0, snd_soc_get_volsw, da9055_put_alc_sw),
    SOC_SINGLE!("ALC Offset Switch", DA9055_ALC_CTRL1, 0, 1, 0),
    SOC_SINGLE!("ALC Anticlip Mode Switch", DA9055_ALC_ANTICLIP_CTRL, 7, 1, 0),
    SOC_SINGLE!("ALC Anticlip Level", DA9055_ALC_ANTICLIP_LEVEL, 0, 0x7f, 0),
    SOC_SINGLE_TLV!("ALC Min Threshold Volume", DA9055_ALC_TARGET_MIN, 0, 0x3f, 1, alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Max Threshold Volume", DA9055_ALC_TARGET_MAX, 0, 0x3f, 1, alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Noise Threshold Volume", DA9055_ALC_NOISE, 0, 0x3f, 1, alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Max Gain Volume", DA9055_ALC_GAIN_LIMITS, 4, 0xf, 0, alc_gain_tlv),
    SOC_SINGLE_TLV!("ALC Max Attenuation Volume", DA9055_ALC_GAIN_LIMITS, 0, 0xf, 0, alc_gain_tlv),
    SOC_SINGLE_TLV!("ALC Min Analog Gain Volume", DA9055_ALC_ANA_GAIN_LIMITS, 0, 0x7, 0, alc_analog_gain_tlv),
    SOC_SINGLE_TLV!("ALC Max Analog Gain Volume", DA9055_ALC_ANA_GAIN_LIMITS, 4, 0x7, 0, alc_analog_gain_tlv),
    SOC_ENUM!("ALC Attack Rate", da9055_attack_rate),
    SOC_ENUM!("ALC Release Rate", da9055_release_rate),
    SOC_ENUM!("ALC Hold Time", da9055_hold_time),
    /* Rate at which input signal envelope is tracked as the signal gets larger */
    SOC_ENUM!("ALC Integ Attack Rate", da9055_integ_attack_rate),
    /* Rate at which input signal envelope is tracked as the signal gets smaller */
    SOC_ENUM!("ALC Integ Release Rate", da9055_integ_release_rate),
]);

SOC_DAPM_ENUM_DECL!(da9055_mic_l_mux_controls, "Route", da9055_mic_l_src);
SOC_DAPM_ENUM_DECL!(da9055_mic_r_mux_controls, "Route", da9055_mic_r_src);
SOC_DAPM_ENUM_DECL!(da9055_dac_l_mux_controls, "Route", da9055_dac_l_src);
SOC_DAPM_ENUM_DECL!(da9055_dac_r_mux_controls, "Route", da9055_dac_r_src);
dapm_controls_array!(da9055_dapm_mixinl_controls, [
    SOC_DAPM_SINGLE!("Aux Left Switch", DA9055_MIXIN_L_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("Mic Left Switch", DA9055_MIXIN_L_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("Mic Right Switch", DA9055_MIXIN_L_SELECT, 2, 1, 0),
]);
dapm_controls_array!(da9055_dapm_mixinr_controls, [
    SOC_DAPM_SINGLE!("Aux Right Switch", DA9055_MIXIN_R_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("Mic Right Switch", DA9055_MIXIN_R_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("Mic Left Switch", DA9055_MIXIN_R_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Left Switch", DA9055_MIXIN_R_SELECT, 3, 1, 0),
]);
dapm_controls_array!(da9055_dapm_mixoutl_controls, [
    SOC_DAPM_SINGLE!("Aux Left Switch", DA9055_MIXOUT_L_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Left Switch", DA9055_MIXOUT_L_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Right Switch", DA9055_MIXOUT_L_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("DAC Left Switch", DA9055_MIXOUT_L_SELECT, 3, 1, 0),
    SOC_DAPM_SINGLE!("Aux Left Invert Switch", DA9055_MIXOUT_L_SELECT, 4, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Left Invert Switch", DA9055_MIXOUT_L_SELECT, 5, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Right Invert Switch", DA9055_MIXOUT_L_SELECT, 6, 1, 0),
]);
dapm_controls_array!(da9055_dapm_mixoutr_controls, [
    SOC_DAPM_SINGLE!("Aux Right Switch", DA9055_MIXOUT_R_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Right Switch", DA9055_MIXOUT_R_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Left Switch", DA9055_MIXOUT_R_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("DAC Right Switch", DA9055_MIXOUT_R_SELECT, 3, 1, 0),
    SOC_DAPM_SINGLE!("Aux Right Invert Switch", DA9055_MIXOUT_R_SELECT, 4, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Right Invert Switch", DA9055_MIXOUT_R_SELECT, 5, 1, 0),
    SOC_DAPM_SINGLE!("Mixin Left Invert Switch", DA9055_MIXOUT_R_SELECT, 6, 1, 0),
]);
SOC_DAPM_SINGLE_DECL!(da9055_dapm_hp_l_control, "Switch", DA9055_HP_L_CTRL, 3, 1, 0);
SOC_DAPM_SINGLE_DECL!(da9055_dapm_hp_r_control, "Switch", DA9055_HP_R_CTRL, 3, 1, 0);
SOC_DAPM_SINGLE_DECL!(da9055_dapm_lineout_control, "Switch", DA9055_LINE_CTRL, 3, 1, 0);

dapm_widgets_array!(da9055_dapm_widgets, [
    SND_SOC_DAPM_INPUT!("MIC1"), SND_SOC_DAPM_INPUT!("MIC2"), SND_SOC_DAPM_INPUT!("AUXL"), SND_SOC_DAPM_INPUT!("AUXR"),
    SND_SOC_DAPM_MUX!("Mic Left Source", SND_SOC_NOPM, 0, 0, &da9055_mic_l_mux_controls),
    SND_SOC_DAPM_MUX!("Mic Right Source", SND_SOC_NOPM, 0, 0, &da9055_mic_r_mux_controls),
    SND_SOC_DAPM_PGA!("Mic Left", DA9055_MIC_L_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mic Right", DA9055_MIC_R_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Aux Left", DA9055_AUX_L_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Aux Right", DA9055_AUX_R_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIXIN Left", DA9055_MIXIN_L_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIXIN Right", DA9055_MIXIN_R_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", DA9055_MIC_BIAS_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("AIF", DA9055_AIF_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Charge Pump", DA9055_CP_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("In Mixer Left", SND_SOC_NOPM, 0, 0, &da9055_dapm_mixinl_controls[0], ARRAY_SIZE!(da9055_dapm_mixinl_controls)),
    SND_SOC_DAPM_MIXER!("In Mixer Right", SND_SOC_NOPM, 0, 0, &da9055_dapm_mixinr_controls[0], ARRAY_SIZE!(da9055_dapm_mixinr_controls)),
    SND_SOC_DAPM_ADC!("ADC Left", "Capture", DA9055_ADC_L_CTRL, 7, 0),
    SND_SOC_DAPM_ADC!("ADC Right", "Capture", DA9055_ADC_R_CTRL, 7, 0),
    SND_SOC_DAPM_MUX!("DAC Left Source", SND_SOC_NOPM, 0, 0, &da9055_dac_l_mux_controls),
    SND_SOC_DAPM_MUX!("DAC Right Source", SND_SOC_NOPM, 0, 0, &da9055_dac_r_mux_controls),
    SND_SOC_DAPM_AIF_IN!("AIFIN Left", "Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("AIFIN Right", "Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("DAC Left", "Playback", DA9055_DAC_L_CTRL, 7, 0),
    SND_SOC_DAPM_DAC!("DAC Right", "Playback", DA9055_DAC_R_CTRL, 7, 0),
    SND_SOC_DAPM_MIXER!("Out Mixer Left", SND_SOC_NOPM, 0, 0, &da9055_dapm_mixoutl_controls[0], ARRAY_SIZE!(da9055_dapm_mixoutl_controls)),
    SND_SOC_DAPM_MIXER!("Out Mixer Right", SND_SOC_NOPM, 0, 0, &da9055_dapm_mixoutr_controls[0], ARRAY_SIZE!(da9055_dapm_mixoutr_controls)),
    SND_SOC_DAPM_SWITCH!("Headphone Left Enable", SND_SOC_NOPM, 0, 0, &da9055_dapm_hp_l_control),
    SND_SOC_DAPM_SWITCH!("Headphone Right Enable", SND_SOC_NOPM, 0, 0, &da9055_dapm_hp_r_control),
    SND_SOC_DAPM_SWITCH!("Lineout Enable", SND_SOC_NOPM, 0, 0, &da9055_dapm_lineout_control),
    SND_SOC_DAPM_PGA!("MIXOUT Left", DA9055_MIXOUT_L_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIXOUT Right", DA9055_MIXOUT_R_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Lineout", DA9055_LINE_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Headphone Left", DA9055_HP_L_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Headphone Right", DA9055_HP_R_CTRL, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("HPL"), SND_SOC_DAPM_OUTPUT!("HPR"), SND_SOC_DAPM_OUTPUT!("LINE"),
]);

static da9055_audio_map: [snd_soc_dapm_route; 54] = [
    route!("Mic Left Source", "MIC1_P_N", "MIC1"), route!("Mic Left Source", "MIC1_P", "MIC1"),
    route!("Mic Left Source", "MIC1_N", "MIC1"), route!("Mic Left Source", "MIC2_L", "MIC2"),
    route!("Mic Right Source", "MIC2_R_L", "MIC2"), route!("Mic Right Source", "MIC2_R", "MIC2"),
    route!("Mic Right Source", "MIC2_L", "MIC2"), route!("Mic Left", NULL, "Mic Left Source"),
    route!("Mic Right", NULL, "Mic Right Source"), route!("Aux Left", NULL, "AUXL"),
    route!("Aux Right", NULL, "AUXR"), route!("In Mixer Left", "Mic Left Switch", "Mic Left"),
    route!("In Mixer Left", "Mic Right Switch", "Mic Right"), route!("In Mixer Left", "Aux Left Switch", "Aux Left"),
    route!("In Mixer Right", "Mic Right Switch", "Mic Right"), route!("In Mixer Right", "Mic Left Switch", "Mic Left"),
    route!("In Mixer Right", "Aux Right Switch", "Aux Right"), route!("In Mixer Right", "Mixin Left Switch", "MIXIN Left"),
    route!("MIXIN Left", NULL, "In Mixer Left"), route!("ADC Left", NULL, "MIXIN Left"),
    route!("MIXIN Right", NULL, "In Mixer Right"), route!("ADC Right", NULL, "MIXIN Right"),
    route!("ADC Left", NULL, "AIF"), route!("ADC Right", NULL, "AIF"),
    route!("AIFIN Left", NULL, "AIF"), route!("AIFIN Right", NULL, "AIF"),
    route!("DAC Left Source", "ADC output left", "ADC Left"), route!("DAC Left Source", "ADC output right", "ADC Right"),
    route!("DAC Left Source", "AIF input left", "AIFIN Left"), route!("DAC Left Source", "AIF input right", "AIFIN Right"),
    route!("DAC Right Source", "ADC output left", "ADC Left"), route!("DAC Right Source", "ADC output right", "ADC Right"),
    route!("DAC Right Source", "AIF input left", "AIFIN Left"), route!("DAC Right Source", "AIF input right", "AIFIN Right"),
    route!("DAC Left", NULL, "DAC Left Source"), route!("DAC Right", NULL, "DAC Right Source"),
    route!("Out Mixer Left", "Aux Left Switch", "Aux Left"), route!("Out Mixer Left", "Mixin Left Switch", "MIXIN Left"),
    route!("Out Mixer Left", "Mixin Right Switch", "MIXIN Right"), route!("Out Mixer Left", "Aux Left Invert Switch", "Aux Left"),
    route!("Out Mixer Left", "Mixin Left Invert Switch", "MIXIN Left"), route!("Out Mixer Left", "Mixin Right Invert Switch", "MIXIN Right"),
    route!("Out Mixer Left", "DAC Left Switch", "DAC Left"), route!("Out Mixer Right", "Aux Right Switch", "Aux Right"),
    route!("Out Mixer Right", "Mixin Right Switch", "MIXIN Right"), route!("Out Mixer Right", "Mixin Left Switch", "MIXIN Left"),
    route!("Out Mixer Right", "Aux Right Invert Switch", "Aux Right"), route!("Out Mixer Right", "Mixin Right Invert Switch", "MIXIN Right"),
    route!("Out Mixer Right", "Mixin Left Invert Switch", "MIXIN Left"), route!("Out Mixer Right", "DAC Right Switch", "DAC Right"),
    route!("MIXOUT Left", NULL, "Out Mixer Left"), route!("Headphone Left Enable", "Switch", "MIXOUT Left"),
    route!("Headphone Left", NULL, "Headphone Left Enable"), route!("Headphone Left", NULL, "Charge Pump"),
    route!("HPL", NULL, "Headphone Left"), route!("MIXOUT Right", NULL, "Out Mixer Right"),
    route!("Headphone Right Enable", "Switch", "MIXOUT Right"), route!("Headphone Right", NULL, "Headphone Right Enable"),
    route!("Headphone Right", NULL, "Charge Pump"), route!("HPR", NULL, "Headphone Right"),
    route!("MIXOUT Right", NULL, "Out Mixer Right"), route!("Lineout Enable", "Switch", "MIXOUT Right"),
    route!("Lineout", NULL, "Lineout Enable"), route!("LINE", NULL, "Lineout"),
];

/* Codec private data */
#[repr(C)]
struct da9055_priv {
    regmap: *mut regmap,
    mclk_rate: c_uint,
    master: c_int,
    pdata: *mut da9055_platform_data,
}

static da9055_reg_defaults: [reg_default; 76] = [
    reg_default { reg: 0x21, def: 0x10 }, reg_default { reg: 0x22, def: 0x0A },
    reg_default { reg: 0x23, def: 0x00 }, reg_default { reg: 0x24, def: 0x00 },
    reg_default { reg: 0x25, def: 0x00 }, reg_default { reg: 0x26, def: 0x00 },
    reg_default { reg: 0x27, def: 0x0C }, reg_default { reg: 0x28, def: 0x01 },
    reg_default { reg: 0x29, def: 0x08 }, reg_default { reg: 0x2A, def: 0x32 },
    reg_default { reg: 0x2B, def: 0x00 }, reg_default { reg: 0x30, def: 0x35 },
    reg_default { reg: 0x31, def: 0x35 }, reg_default { reg: 0x32, def: 0x00 },
    reg_default { reg: 0x33, def: 0x00 }, reg_default { reg: 0x34, def: 0x03 },
    reg_default { reg: 0x35, def: 0x03 }, reg_default { reg: 0x36, def: 0x6F },
    reg_default { reg: 0x37, def: 0x6F }, reg_default { reg: 0x38, def: 0x80 },
    reg_default { reg: 0x39, def: 0x01 }, reg_default { reg: 0x3A, def: 0x01 },
    reg_default { reg: 0x40, def: 0x00 }, reg_default { reg: 0x41, def: 0x88 },
    reg_default { reg: 0x42, def: 0x88 }, reg_default { reg: 0x43, def: 0x08 },
    reg_default { reg: 0x44, def: 0x80 }, reg_default { reg: 0x45, def: 0x6F },
    reg_default { reg: 0x46, def: 0x6F }, reg_default { reg: 0x47, def: 0x61 },
    reg_default { reg: 0x48, def: 0x35 }, reg_default { reg: 0x49, def: 0x35 },
    reg_default { reg: 0x4A, def: 0x35 }, reg_default { reg: 0x4B, def: 0x00 },
    reg_default { reg: 0x4C, def: 0x00 }, reg_default { reg: 0x60, def: 0x44 },
    reg_default { reg: 0x61, def: 0x44 }, reg_default { reg: 0x62, def: 0x00 },
    reg_default { reg: 0x63, def: 0x40 }, reg_default { reg: 0x64, def: 0x40 },
    reg_default { reg: 0x65, def: 0x40 }, reg_default { reg: 0x66, def: 0x40 },
    reg_default { reg: 0x67, def: 0x40 }, reg_default { reg: 0x68, def: 0x40 },
    reg_default { reg: 0x69, def: 0x48 }, reg_default { reg: 0x6A, def: 0x40 },
    reg_default { reg: 0x6B, def: 0x41 }, reg_default { reg: 0x6C, def: 0x40 },
    reg_default { reg: 0x6D, def: 0x40 }, reg_default { reg: 0x6E, def: 0x10 },
    reg_default { reg: 0x6F, def: 0x10 }, reg_default { reg: 0x90, def: 0x80 },
    reg_default { reg: 0x92, def: 0x02 }, reg_default { reg: 0x93, def: 0x00 },
    reg_default { reg: 0x99, def: 0x00 }, reg_default { reg: 0x9A, def: 0x00 },
    reg_default { reg: 0x9B, def: 0x00 }, reg_default { reg: 0x9C, def: 0x3F },
    reg_default { reg: 0x9D, def: 0x00 }, reg_default { reg: 0x9E, def: 0x3F },
    reg_default { reg: 0x9F, def: 0xFF }, reg_default { reg: 0xA0, def: 0x71 },
    reg_default { reg: 0xA1, def: 0x00 }, reg_default { reg: 0xA2, def: 0x00 },
    reg_default { reg: 0xA6, def: 0x00 }, reg_default { reg: 0xA7, def: 0x00 },
    reg_default { reg: 0xAB, def: 0x00 }, reg_default { reg: 0xAC, def: 0x00 },
    reg_default { reg: 0xAD, def: 0x00 }, reg_default { reg: 0xAF, def: 0x08 },
    reg_default { reg: 0xB0, def: 0x00 }, reg_default { reg: 0xB1, def: 0x00 },
    reg_default { reg: 0xB2, def: 0x00 },
];

unsafe extern "C" fn da9055_volatile_register(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        DA9055_STATUS1 | DA9055_PLL_STATUS | DA9055_AUX_L_GAIN_STATUS |
        DA9055_AUX_R_GAIN_STATUS | DA9055_MIC_L_GAIN_STATUS |
        DA9055_MIC_R_GAIN_STATUS | DA9055_MIXIN_L_GAIN_STATUS |
        DA9055_MIXIN_R_GAIN_STATUS | DA9055_ADC_L_GAIN_STATUS |
        DA9055_ADC_R_GAIN_STATUS | DA9055_DAC_L_GAIN_STATUS |
        DA9055_DAC_R_GAIN_STATUS | DA9055_HP_L_GAIN_STATUS |
        DA9055_HP_R_GAIN_STATUS | DA9055_LINE_GAIN_STATUS |
        DA9055_ALC_CIC_OP_LVL_DATA => true,
        _ => false,
    }
}

/* Set DAI word length */
unsafe extern "C" fn da9055_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let da9055 = snd_soc_component_get_drvdata(component) as *mut da9055_priv;
    let aif_ctrl: u8;
    let fs: u8;
    let sysclk: u32;

    match params_width(params) {
        16 => aif_ctrl = DA9055_AIF_WORD_S16_LE as u8,
        20 => aif_ctrl = DA9055_AIF_WORD_S20_3LE as u8,
        24 => aif_ctrl = DA9055_AIF_WORD_S24_LE as u8,
        32 => aif_ctrl = DA9055_AIF_WORD_S32_LE as u8,
        _ => return -EINVAL,
    }

    /* Set AIF format */
    snd_soc_component_update_bits(component, DA9055_AIF_CTRL, DA9055_AIF_WORD_LENGTH_MASK, aif_ctrl as c_uint);

    match params_rate(params) {
        8000 => { fs = DA9055_SR_8000 as u8; sysclk = 3072000; }
        11025 => { fs = DA9055_SR_11025 as u8; sysclk = 2822400; }
        12000 => { fs = DA9055_SR_12000 as u8; sysclk = 3072000; }
        16000 => { fs = DA9055_SR_16000 as u8; sysclk = 3072000; }
        22050 => { fs = DA9055_SR_22050 as u8; sysclk = 2822400; }
        32000 => { fs = DA9055_SR_32000 as u8; sysclk = 3072000; }
        44100 => { fs = DA9055_SR_44100 as u8; sysclk = 2822400; }
        48000 => { fs = DA9055_SR_48000 as u8; sysclk = 3072000; }
        88200 => { fs = DA9055_SR_88200 as u8; sysclk = 2822400; }
        96000 => { fs = DA9055_SR_96000 as u8; sysclk = 3072000; }
        _ => return -EINVAL,
    }

    if (*da9055).mclk_rate != 0 {
        /* PLL Mode, Write actual FS */
        snd_soc_component_write(component, DA9055_SR, fs as c_uint);
    } else {
        /*
         * Non-PLL Mode
         * When PLL is bypassed, chip assumes constant MCLK of
         * 12.288MHz and uses sample rate value to divide this MCLK
         * to derive its sys clk. As sys clk has to be 256 * Fs, we
         * need to write constant sample rate i.e. 48KHz.
         */
        snd_soc_component_write(component, DA9055_SR, DA9055_SR_48000);
    }

    if (*da9055).mclk_rate != 0 && (*da9055).mclk_rate != sysclk {
        /* PLL Mode */
        if (*da9055).master == 0 {
            /* PLL slave mode, enable PLL and also SRM */
            snd_soc_component_update_bits(component, DA9055_PLL_CTRL,
                DA9055_PLL_EN | DA9055_PLL_SRM_EN, DA9055_PLL_EN | DA9055_PLL_SRM_EN);
        } else {
            /* PLL master mode, only enable PLL */
            snd_soc_component_update_bits(component, DA9055_PLL_CTRL, DA9055_PLL_EN, DA9055_PLL_EN);
        }
    } else {
        /* Non PLL Mode, disable PLL */
        snd_soc_component_update_bits(component, DA9055_PLL_CTRL, DA9055_PLL_EN, 0);
    }

    0
}

/* Set DAI mode and Format */
unsafe extern "C" fn da9055_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let da9055 = snd_soc_component_get_drvdata(component) as *mut da9055_priv;
    let mut aif_clk_mode: u8;
    let aif_ctrl: u8;
    let mode: u8;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            /* DA9055 in I2S Master Mode */
            mode = 1;
            aif_clk_mode = DA9055_AIF_CLK_EN_MASTER_MODE as u8;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            /* DA9055 in I2S Slave Mode */
            mode = 0;
            aif_clk_mode = DA9055_AIF_CLK_EN_SLAVE_MODE as u8;
        }
        _ => return -EINVAL,
    }

    /* Don't allow change of mode if PLL is enabled */
    if (snd_soc_component_read(component, DA9055_PLL_CTRL) & DA9055_PLL_EN) != 0
        && (*da9055).master != mode as c_int
    {
        return -EINVAL;
    }

    (*da9055).master = mode as c_int;

    /* Only I2S is supported */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => aif_ctrl = DA9055_AIF_FORMAT_I2S_MODE as u8,
        SND_SOC_DAIFMT_LEFT_J => aif_ctrl = DA9055_AIF_FORMAT_LEFT_J as u8,
        SND_SOC_DAIFMT_RIGHT_J => aif_ctrl = DA9055_AIF_FORMAT_RIGHT_J as u8,
        SND_SOC_DAIFMT_DSP_A => aif_ctrl = DA9055_AIF_FORMAT_DSP as u8,
        _ => return -EINVAL,
    }

    /* By default only 32 BCLK per WCLK is supported */
    aif_clk_mode |= DA9055_AIF_BCLKS_PER_WCLK_32 as u8;

    snd_soc_component_update_bits(component, DA9055_AIF_CLK_MODE,
        DA9055_AIF_CLK_MODE_MASK | DA9055_AIF_BCLK_MASK, aif_clk_mode as c_uint);
    snd_soc_component_update_bits(component, DA9055_AIF_CTRL, DA9055_AIF_FORMAT_MASK, aif_ctrl as c_uint);
    0
}

unsafe extern "C" fn da9055_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;

    if mute != 0 {
        snd_soc_component_update_bits(component, DA9055_DAC_L_CTRL, DA9055_DAC_L_MUTE_EN, DA9055_DAC_L_MUTE_EN);
        snd_soc_component_update_bits(component, DA9055_DAC_R_CTRL, DA9055_DAC_R_MUTE_EN, DA9055_DAC_R_MUTE_EN);
    } else {
        snd_soc_component_update_bits(component, DA9055_DAC_L_CTRL, DA9055_DAC_L_MUTE_EN, 0);
        snd_soc_component_update_bits(component, DA9055_DAC_R_CTRL, DA9055_DAC_R_MUTE_EN, 0);
    }

    0
}

const DA9055_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

unsafe extern "C" fn da9055_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let da9055 = snd_soc_component_get_drvdata(component) as *mut da9055_priv;

    match clk_id {
        x if x == clk_src::DA9055_CLKSRC_MCLK as c_int => match freq {
            11289600 | 12000000 | 12288000 | 13000000 | 13500000 |
            14400000 | 19200000 | 19680000 | 19800000 => {
                (*da9055).mclk_rate = freq;
                0
            }
            _ => {
                dev_err((*codec_dai).dev, c"Unsupported MCLK value %d\n".as_ptr(), freq);
                -EINVAL
            }
        },
        _ => {
            dev_err((*codec_dai).dev, c"Unknown clock source %d\n".as_ptr(), clk_id);
            -EINVAL
        }
    }
}

/*
 * da9055_set_dai_pll	: Configure the codec PLL
 * @param codec_dai	: Pointer to codec DAI
 * @param pll_id	: da9055 has only one pll, so pll_id is always zero
 * @param fref		: Input MCLK frequency
 * @param fout		: FsDM value
 * @return int		: Zero for success, negative error code for error
 *
 * Note: Supported PLL input frequencies are 11.2896MHz, 12MHz, 12.288MHz,
 *	 13MHz, 13.5MHz, 14.4MHz, 19.2MHz, 19.6MHz and 19.8MHz
 */
unsafe extern "C" fn da9055_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    fref: c_uint,
    fout: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let da9055 = snd_soc_component_get_drvdata(component) as *mut da9055_priv;
    let mut pll_frac_top: u8 = 0;
    let mut pll_frac_bot: u8 = 0;
    let mut pll_integer: u8 = 0;
    let mut cnt: usize = 0;

    /* Disable PLL before setting the divisors */
    snd_soc_component_update_bits(component, DA9055_PLL_CTRL, DA9055_PLL_EN, 0);

    /* In slave mode, there is only one set of divisors */
    if (*da9055).master == 0 && fout != 2822400 {
        dev_err((*codec_dai).dev, c"Error in setting up PLL\n".as_ptr());
        return -EINVAL;
    }

    /* Search pll div array for correct divisors */
    while cnt < da9055_pll_div.len() {
        /* Check fref, mode  and fout */
        if fref == da9055_pll_div[cnt].fref as c_uint
            && (*da9055).master == da9055_pll_div[cnt].mode as c_int
            && fout == da9055_pll_div[cnt].fout as c_uint
        {
            /* All match, pick up divisors */
            pll_frac_top = da9055_pll_div[cnt].frac_top;
            pll_frac_bot = da9055_pll_div[cnt].frac_bot;
            pll_integer = da9055_pll_div[cnt].integer;
            break;
        }
        cnt += 1;
    }
    if cnt >= da9055_pll_div.len() {
        dev_err((*codec_dai).dev, c"Error in setting up PLL\n".as_ptr());
        return -EINVAL;
    }

    /* Write PLL dividers */
    snd_soc_component_write(component, DA9055_PLL_FRAC_TOP, pll_frac_top as c_uint);
    snd_soc_component_write(component, DA9055_PLL_FRAC_BOT, pll_frac_bot as c_uint);
    snd_soc_component_write(component, DA9055_PLL_INTEGER, pll_integer as c_uint);

    0
}

/* DAI operations */
static da9055_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(da9055_hw_params),
    set_fmt: Some(da9055_set_dai_fmt),
    set_sysclk: Some(da9055_set_dai_sysclk),
    set_pll: Some(da9055_set_dai_pll),
    mute_stream: Some(da9055_mute),
    no_capture_mute: 1,
};

static mut da9055_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"da9055-hifi".as_ptr(),
    /* Playback Capabilities */
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: DA9055_FORMATS,
    },
    /* Capture Capabilities */
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: DA9055_FORMATS,
    },
    ops: &da9055_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn da9055_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON | snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                /* Enable VMID reference & master bias */
                snd_soc_component_update_bits(component, DA9055_REFERENCES,
                    DA9055_VMID_EN | DA9055_BIAS_EN, DA9055_VMID_EN | DA9055_BIAS_EN);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            /* Disable VMID reference & master bias */
            snd_soc_component_update_bits(component, DA9055_REFERENCES, DA9055_VMID_EN | DA9055_BIAS_EN, 0);
        }
    }
    0
}

unsafe extern "C" fn da9055_probe(component: *mut snd_soc_component) -> c_int {
    let da9055 = snd_soc_component_get_drvdata(component) as *mut da9055_priv;

    /* Enable all Gain Ramps */
    snd_soc_component_update_bits(component, DA9055_AUX_L_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_AUX_R_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_MIXIN_L_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_MIXIN_R_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_ADC_L_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_ADC_R_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_DAC_L_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_DAC_R_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_HP_L_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_HP_R_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);
    snd_soc_component_update_bits(component, DA9055_LINE_CTRL, DA9055_GAIN_RAMPING_EN, DA9055_GAIN_RAMPING_EN);

    /*
     * There are two separate control bits for input and output mixers.
     * One to enable corresponding amplifier and other to enable its
     * output. As amplifier bits are related to power control, they are
     * being managed by DAPM while other (non power related) bits are
     * enabled here
     */
    snd_soc_component_update_bits(component, DA9055_MIXIN_L_CTRL, DA9055_MIXIN_L_MIX_EN, DA9055_MIXIN_L_MIX_EN);
    snd_soc_component_update_bits(component, DA9055_MIXIN_R_CTRL, DA9055_MIXIN_R_MIX_EN, DA9055_MIXIN_R_MIX_EN);

    snd_soc_component_update_bits(component, DA9055_MIXOUT_L_CTRL, DA9055_MIXOUT_L_MIX_EN, DA9055_MIXOUT_L_MIX_EN);
    snd_soc_component_update_bits(component, DA9055_MIXOUT_R_CTRL, DA9055_MIXOUT_R_MIX_EN, DA9055_MIXOUT_R_MIX_EN);

    /* Set this as per your system configuration */
    snd_soc_component_write(component, DA9055_PLL_CTRL, DA9055_PLL_INDIV_10_20_MHZ);

    /* Set platform data values */
    if !(*da9055).pdata.is_null() {
        /* set mic bias source */
        if (*(*da9055).pdata).micbias_source != 0 {
            snd_soc_component_update_bits(component, DA9055_MIXIN_R_SELECT, DA9055_MICBIAS2_EN, DA9055_MICBIAS2_EN);
        } else {
            snd_soc_component_update_bits(component, DA9055_MIXIN_R_SELECT, DA9055_MICBIAS2_EN, 0);
        }
        /* set mic bias voltage */
        match (*(*da9055).pdata).micbias {
            DA9055_MICBIAS_2_2V | DA9055_MICBIAS_2_1V | DA9055_MICBIAS_1_8V | DA9055_MICBIAS_1_6V => {
                snd_soc_component_update_bits(component, DA9055_MIC_CONFIG,
                    DA9055_MICBIAS_LEVEL_MASK, ((*(*da9055).pdata).micbias as c_uint) << 4);
            }
            _ => {}
        }
    }
    0
}

static soc_component_dev_da9055: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(da9055_probe),
    set_bias_level: Some(da9055_set_bias_level),
    controls: da9055_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(da9055_snd_controls),
    dapm_widgets: da9055_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(da9055_dapm_widgets),
    dapm_routes: da9055_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(da9055_audio_map),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static da9055_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    reg_defaults: da9055_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(da9055_reg_defaults),
    volatile_reg: Some(da9055_volatile_register),
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn da9055_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut da9055: *mut da9055_priv;
    let pdata = dev_get_platdata(&mut (*i2c).dev) as *mut da9055_platform_data;
    let mut ret: c_int;

    da9055 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<da9055_priv>(), GFP_KERNEL) as *mut da9055_priv;
    if da9055.is_null() {
        return -ENOMEM;
    }

    if !pdata.is_null() {
        (*da9055).pdata = pdata;
    }

    i2c_set_clientdata(i2c, da9055 as *mut c_void);

    (*da9055).regmap = devm_regmap_init_i2c(i2c, &da9055_regmap_config);
    if IS_ERR((*da9055).regmap as *const c_void) {
        ret = PTR_ERR((*da9055).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, c"regmap_init() failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*i2c).dev,
        &soc_component_dev_da9055, &raw mut da9055_dai, 1);
    if ret < 0 {
        dev_err(&mut (*i2c).dev, c"Failed to register da9055 component: %d\n".as_ptr(), ret);
    }
    ret
}

/*
 * DO NOT change the device Ids. The naming is intentionally specific as both
 * the CODEC and PMIC parts of this chip are instantiated separately as I2C
 * devices (both have configurable I2C addresses, and are to all intents and
 * purposes separate). As a result there are specific DA9055 Ids for CODEC
 * and PMIC, which must be different to operate together.
 */
static da9055_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"da9055-codec\0\0\0\0\0\0\0" as [u8; 20] as [c_char; 20] },
    i2c_device_id { name: [0; 20] },
];
MODULE_DEVICE_TABLE!(i2c, da9055_i2c_id);

/* Original C conditional: #ifdef CONFIG_OF */
static da9055_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"dlg,da9055-codec".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];
MODULE_DEVICE_TABLE!(of, da9055_of_match);

/* I2C codec control layer */
static mut da9055_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"da9055-codec".as_ptr(),
        of_match_table: unsafe { of_match_ptr(da9055_of_match.as_ptr()) },
    },
    probe: Some(da9055_i2c_probe),
    id_table: da9055_i2c_id.as_ptr(),
};

module_i2c_driver!(da9055_i2c_driver);

MODULE_DESCRIPTION!("ASoC DA9055 Codec driver");
MODULE_AUTHOR!("David Chen, Ashish Chavan");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
